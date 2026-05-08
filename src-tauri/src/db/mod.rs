use std::sync::Mutex;

use chrono::Utc;
use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::error::{AppError, AppResult};
use crate::types::{Message, Session};

pub struct DbState(pub Mutex<Connection>);

pub fn init_db(app: &AppHandle) -> AppResult<()> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| AppError::Database("Failed to get app data dir".into()))?;

    if !app_dir.exists() {
        std::fs::create_dir_all(&app_dir)?;
    }

    let db_path = app_dir.join("happy-friday.db");
    let conn = Connection::open(&db_path)?;

    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA foreign_keys=ON;",
    )?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL DEFAULT '新对话',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_messages_session_id ON messages(session_id);
        CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);",
    )?;

    app.manage(DbState(Mutex::new(conn)));
    Ok(())
}

pub fn create_session(conn: &Connection) -> AppResult<Session> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO sessions (id, title, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        (&id, "新对话", &now, &now),
    )?;

    Ok(Session {
        id,
        title: "新对话".to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub fn get_sessions(conn: &Connection) -> AppResult<Vec<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM sessions ORDER BY updated_at DESC",
    )?;

    let sessions = stmt
        .query_map([], |row| {
            Ok(Session {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions)
}

pub fn get_session(conn: &Connection, session_id: &str) -> AppResult<Option<Session>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, updated_at FROM sessions WHERE id = ?1",
    )?;

    let mut sessions = stmt
        .query_map([session_id], |row| {
            Ok(Session {
                id: row.get(0)?,
                title: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(sessions.pop())
}

pub fn delete_session(conn: &Connection, session_id: &str) -> AppResult<()> {
    conn.execute("DELETE FROM sessions WHERE id = ?1", [session_id])?;
    Ok(())
}

pub fn update_session_title(conn: &Connection, session_id: &str, title: &str) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE sessions SET title = ?1, updated_at = ?2 WHERE id = ?3",
        (title, &now, session_id),
    )?;
    Ok(())
}

pub fn update_session_timestamp(conn: &Connection, session_id: &str) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE sessions SET updated_at = ?1 WHERE id = ?2",
        (&now, session_id),
    )?;
    Ok(())
}

pub fn save_message(conn: &Connection, session_id: &str, role: &str, content: &str) -> AppResult<Message> {
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO messages (session_id, role, content, created_at) VALUES (?1, ?2, ?3, ?4)",
        (session_id, role, content, &now),
    )?;

    let id = conn.last_insert_rowid();

    Ok(Message {
        id,
        session_id: session_id.to_string(),
        role: role.to_string(),
        content: content.to_string(),
        created_at: now,
    })
}

pub fn get_messages(conn: &Connection, session_id: &str) -> AppResult<Vec<Message>> {
    let mut stmt = conn.prepare(
        "SELECT id, session_id, role, content, created_at FROM messages WHERE session_id = ?1 ORDER BY id ASC",
    )?;

    let messages = stmt
        .query_map([session_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                session_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(messages)
}

pub fn rollback_session(conn: &Connection, session_id: &str, message_id: i64) -> AppResult<()> {
    conn.execute(
        "DELETE FROM messages WHERE session_id = ?1 AND id > ?2",
        (session_id, message_id),
    )?;
    update_session_timestamp(conn, session_id)?;
    Ok(())
}

#[cfg(test)]
fn init_test_db(conn: &Connection) {
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA foreign_keys=ON;",
    )
    .unwrap();

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS sessions (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL DEFAULT '新对话',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            session_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_messages_session_id ON messages(session_id);
        CREATE INDEX IF NOT EXISTS idx_messages_created_at ON messages(created_at);",
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_test_db(&conn);
        conn
    }

    #[test]
    fn test_create_session() {
        let conn = get_test_conn();
        let session = create_session(&conn).unwrap();

        assert!(!session.id.is_empty());
        assert_eq!(session.title, "新对话");
        assert!(!session.created_at.is_empty());

        let sessions = get_sessions(&conn).unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, session.id);
    }

    #[test]
    fn test_get_session_by_id() {
        let conn = get_test_conn();
        let session = create_session(&conn).unwrap();

        let found = get_session(&conn, &session.id).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, session.id);

        let not_found = get_session(&conn, "non-existent-id").unwrap();
        assert!(not_found.is_none());
    }

    #[test]
    fn test_get_sessions_order_by_updated_at_desc() {
        let conn = get_test_conn();
        let s1 = create_session(&conn).unwrap();
        let _s2 = create_session(&conn).unwrap();
        let _s3 = create_session(&conn).unwrap();

        update_session_title(&conn, &s1.id, "updated-first").unwrap();

        let sessions = get_sessions(&conn).unwrap();
        assert_eq!(sessions.len(), 3);
        assert_eq!(sessions[0].id, s1.id);
        assert_eq!(sessions[0].title, "updated-first");
    }

    #[test]
    fn test_delete_session_cascades_messages() {
        let conn = get_test_conn();
        let session = create_session(&conn).unwrap();

        save_message(&conn, &session.id, "user", "hello").unwrap();
        save_message(&conn, &session.id, "assistant", "hi").unwrap();

        delete_session(&conn, &session.id).unwrap();

        let sessions = get_sessions(&conn).unwrap();
        assert!(sessions.is_empty());

        let messages = get_messages(&conn, &session.id).unwrap();
        assert!(messages.is_empty());
    }

    #[test]
    fn test_update_session_title() {
        let conn = get_test_conn();
        let session = create_session(&conn).unwrap();

        update_session_title(&conn, &session.id, "新主题").unwrap();

        let updated = get_session(&conn, &session.id).unwrap().unwrap();
        assert_eq!(updated.title, "新主题");
    }

    #[test]
    fn test_save_and_get_messages_ordered() {
        let conn = get_test_conn();
        let session = create_session(&conn).unwrap();

        let msg1 = save_message(&conn, &session.id, "user", "第一条消息").unwrap();
        let msg2 = save_message(&conn, &session.id, "assistant", "第二条消息").unwrap();
        let msg3 = save_message(&conn, &session.id, "user", "第三条消息").unwrap();

        assert!(msg1.id < msg2.id);
        assert!(msg2.id < msg3.id);

        let messages = get_messages(&conn, &session.id).unwrap();
        assert_eq!(messages.len(), 3);
        assert_eq!(messages[0].content, "第一条消息");
        assert_eq!(messages[1].content, "第二条消息");
        assert_eq!(messages[2].content, "第三条消息");

        for (i, msg) in messages.iter().enumerate() {
            assert_eq!(msg.session_id, session.id);
            assert!(msg.created_at.len() > 0);
            if i == 0 || i == 2 {
                assert_eq!(msg.role, "user");
            } else {
                assert_eq!(msg.role, "assistant");
            }
        }
    }

    #[test]
    fn test_rollback_session_deletes_after_target() {
        let conn = get_test_conn();
        let session = create_session(&conn).unwrap();

        let _m1 = save_message(&conn, &session.id, "user", "msg1").unwrap();
        let m2 = save_message(&conn, &session.id, "assistant", "msg2").unwrap();
        let _m3 = save_message(&conn, &session.id, "user", "msg3").unwrap();
        let _m4 = save_message(&conn, &session.id, "assistant", "msg4").unwrap();

        rollback_session(&conn, &session.id, m2.id).unwrap();

        let messages = get_messages(&conn, &session.id).unwrap();
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].content, "msg1");
        assert_eq!(messages[1].content, "msg2");
    }
  
}
