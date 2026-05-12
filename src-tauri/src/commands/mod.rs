use tauri::{AppHandle, Emitter, Manager, State, command};

use crate::config::{load_config, save_config};
use crate::db::{self, DbState};
use crate::error::AppResult;
use crate::events::{CHAT_DONE, CONFIG_CHANGED, SESSION_TITLE_UPDATED};
use crate::llm;
use crate::types::{
    AppConfig, ChatDonePayload, ChatMessage, ChatResult, Message, ModelConfig, Note, ScheduleEvent, Session,
};

#[command]
pub fn get_config(app: AppHandle) -> AppResult<AppConfig> {
    load_config(&app)
}

#[command]
pub fn update_config(app: AppHandle, config: AppConfig) -> AppResult<()> {
    save_config(&app, &config)?;
    app.emit(CONFIG_CHANGED, config).unwrap_or(());
    Ok(())
}

#[command]
pub fn get_sessions(db: State<'_, DbState>) -> AppResult<Vec<Session>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_sessions(&conn)
}

#[command]
pub fn create_session(db: State<'_, DbState>) -> AppResult<Session> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::create_session(&conn)
}

#[command]
pub fn delete_session(db: State<'_, DbState>, session_id: String) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::delete_session(&conn, &session_id)
}

#[command]
pub fn rollback_session(db: State<'_, DbState>, session_id: String, message_id: i64) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::rollback_session(&conn, &session_id, message_id)
}

#[command]
pub fn get_session_messages(db: State<'_, DbState>, session_id: String) -> AppResult<Vec<Message>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_messages(&conn, &session_id)
}

#[command]
pub fn update_session_title(
    db: State<'_, DbState>,
    session_id: String,
    title: String,
) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::update_session_title(&conn, &session_id, &title)
}

#[command]
pub async fn chat_with_memory(
    app: AppHandle,
    db: State<'_, DbState>,
    request_id: String,
    session_id: String,
    model: ModelConfig,
    message: String,
) -> AppResult<ChatResult> {
    let (session, history_messages, is_new_session, user_message_id) = {
        let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;

        let (session, is_new) = if session_id.is_empty() {
            (db::create_session(&conn)?, true)
        } else {
            let existing = db::get_session(&conn, &session_id)?
                .ok_or_else(|| crate::error::AppError::Database("Session not found".into()))?;
            (existing, false)
        };

        let user_msg = db::save_message(&conn, &session.id, "user", &message)?;
        db::update_session_timestamp(&conn, &session.id)?;

        let db_messages = db::get_messages(&conn, &session.id)?;
        let history: Vec<ChatMessage> = db_messages
            .into_iter()
            .map(|m| ChatMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        (session, history, is_new, user_msg.id)
    };

    let app_config = load_config(&app)?;
    let mut all_messages = vec![ChatMessage {
        role: "system".to_string(),
        content: app_config.system_prompt,
    }];
    all_messages.extend(history_messages);

    let full_content = llm::stream_chat(
        &app,
        all_messages,
        &model,
        &request_id,
        Some(&session.id),
    )
    .await?;

    let assistant_message = {
        let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
        let msg = db::save_message(&conn, &session.id, "assistant", &full_content)?;
        db::update_session_timestamp(&conn, &session.id)?;
        msg
    };

    let _ = app.emit(
        CHAT_DONE,
        ChatDonePayload {
            request_id: request_id.clone(),
            session_id: Some(session.id.clone()),
            full_content: full_content.clone(),
            message_id: Some(assistant_message.id),
            user_message_id: Some(user_message_id),
        },
    );

    if is_new_session {
        let app_clone = app.clone();
        let model_clone = model.clone();
        let session_id_clone = session.id.clone();
        let user_msg = message.clone();
        tokio::spawn(async move {
            match llm::generate_title(&model_clone, &user_msg).await {
                Ok(title) => {
                    if let Some(db_state) = app_clone.try_state::<DbState>() {
                        if let Ok(conn) = db_state.0.lock() {
                            let _ = db::update_session_title(&conn, &session_id_clone, &title);
                        }
                    }
                    let _ = app_clone.emit(
                        SESSION_TITLE_UPDATED,
                        crate::types::SessionTitlePayload {
                            session_id: session_id_clone,
                            title,
                        },
                    );
                }
                Err(_) => {}
            }
        });
    }

    Ok(ChatResult {
        session_id: session.id,
    })
}

#[command]
pub async fn chat_without_memory(
    app: AppHandle,
    request_id: String,
    model: ModelConfig,
    message: String,
) -> AppResult<()> {
    let app_config = load_config(&app)?;
    let messages = vec![
        ChatMessage {
            role: "system".to_string(),
            content: app_config.system_prompt,
        },
        ChatMessage {
            role: "user".to_string(),
            content: message,
        },
    ];

    let full_content = llm::stream_chat(&app, messages, &model, &request_id, None).await?;

    let _ = app.emit(
        CHAT_DONE,
        ChatDonePayload {
            request_id: request_id.clone(),
            session_id: None,
            full_content,
            message_id: None,
            user_message_id: None,
        },
    );

    Ok(())
}

#[command]
pub fn get_notes(db: State<'_, DbState>, knowledge_base_id: Option<String>) -> AppResult<Vec<Note>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_notes(&conn, knowledge_base_id.as_deref())
}

#[command]
pub fn get_note(db: State<'_, DbState>, note_id: String) -> AppResult<Option<Note>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_note(&conn, &note_id)
}

#[command]
pub fn create_note(
    db: State<'_, DbState>,
    knowledge_base_id: Option<String>,
    title: Option<String>,
) -> AppResult<Note> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::create_note(&conn, knowledge_base_id.as_deref(), title.as_deref().unwrap_or("新建笔记"))
}

#[command]
pub fn update_note(
    db: State<'_, DbState>,
    note_id: String,
    title: String,
    content: String,
    content_text: String,
) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::update_note(&conn, &note_id, &title, &content, &content_text)
}

#[command]
pub fn delete_note(db: State<'_, DbState>, note_id: String) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::soft_delete_note(&conn, &note_id)
}

#[command]
pub fn search_notes(db: State<'_, DbState>, query: String) -> AppResult<Vec<Note>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::search_notes(&conn, &query)
}

#[command]
pub fn get_schedule_events(db: State<'_, DbState>) -> AppResult<Vec<ScheduleEvent>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_schedule_events(&conn)
}

#[command]
pub fn get_schedule_events_by_date_range(
    db: State<'_, DbState>,
    start: String,
    end: String,
) -> AppResult<Vec<ScheduleEvent>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_schedule_events_by_date_range(&conn, &start, &end)
}

#[command]
pub fn get_schedule_event(db: State<'_, DbState>, event_id: String) -> AppResult<Option<ScheduleEvent>> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::get_schedule_event(&conn, &event_id)
}

#[command]
pub fn create_schedule_event(
    db: State<'_, DbState>,
    title: String,
    start_date: String,
    end_date: String,
    start_time: String,
    end_time: String,
    all_day: bool,
    description: String,
    color: String,
    reminder: bool,
    completed: bool,
) -> AppResult<ScheduleEvent> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::create_schedule_event(
        &conn, &title, &start_date, &end_date, &start_time, &end_time,
        all_day, &description, &color, reminder, completed,
    )
}

#[command]
pub fn update_schedule_event(
    db: State<'_, DbState>,
    event_id: String,
    title: String,
    start_date: String,
    end_date: String,
    start_time: String,
    end_time: String,
    all_day: bool,
    description: String,
    color: String,
    reminder: bool,
    completed: bool,
) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::update_schedule_event(
        &conn, &event_id, &title, &start_date, &end_date, &start_time, &end_time,
        all_day, &description, &color, reminder, completed,
    )
}

#[command]
pub fn delete_schedule_event(db: State<'_, DbState>, event_id: String) -> AppResult<()> {
    let conn = db.0.lock().map_err(|e| crate::error::AppError::Database(e.to_string()))?;
    db::delete_schedule_event(&conn, &event_id)
}

pub fn get_handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        get_config,
        update_config,
        get_sessions,
        create_session,
        delete_session,
        rollback_session,
        get_session_messages,
        update_session_title,
        chat_with_memory,
        chat_without_memory,
        get_notes,
        get_note,
        create_note,
        update_note,
        delete_note,
        search_notes,
        get_schedule_events,
        get_schedule_events_by_date_range,
        get_schedule_event,
        create_schedule_event,
        update_schedule_event,
        delete_schedule_event
    ]
}
