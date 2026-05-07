use futures_util::StreamExt;
use reqwest::Client;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter};

use crate::error::AppError;
use crate::events::{CHAT_CHUNK, CHAT_DONE, CHAT_ERROR};
use crate::types::{
    ChatChunkPayload, ChatDonePayload, ChatErrorPayload, ChatMessage, ModelConfig,
};

fn build_api_url(base_url: &str) -> String {
    format!("{}/chat/completions", base_url.trim_end_matches('/'))
}

pub async fn stream_chat(
    app: &AppHandle,
    messages: Vec<ChatMessage>,
    model: &ModelConfig,
    request_id: &str,
    session_id: Option<&str>,
) -> Result<String, AppError> {
    let url = build_api_url(&model.base_url);

    let messages_json: Vec<Value> = messages
        .iter()
        .map(|m| json!({"role": m.role, "content": m.content}))
        .collect();

    let body = json!({
        "model": model.model_name,
        "messages": messages_json,
        "stream": true
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        let error_msg = format!("API request failed ({}): {}", status, error_text);
        let _ = app.emit(
            CHAT_ERROR,
            ChatErrorPayload {
                request_id: request_id.to_string(),
                session_id: session_id.map(|s| s.to_string()),
                error: error_msg.clone(),
            },
        );
        return Err(AppError::Llm(error_msg));
    }

    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    let mut full_content = String::new();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();

            if line.is_empty() {
                continue;
            }

            if let Some(data) = line.strip_prefix("data: ") {
                let data = data.trim();

                if data == "[DONE]" {
                    let _ = app.emit(
                        CHAT_DONE,
                        ChatDonePayload {
                            request_id: request_id.to_string(),
                            session_id: session_id.map(|s| s.to_string()),
                            full_content: full_content.clone(),
                            message_id: None,
                        },
                    );
                    return Ok(full_content);
                }

                if let Ok(parsed) = serde_json::from_str::<Value>(data) {
                    if let Some(error) = parsed.get("error") {
                        let error_msg = error["message"]
                            .as_str()
                            .unwrap_or("Unknown API error")
                            .to_string();
                        let _ = app.emit(
                            CHAT_ERROR,
                            ChatErrorPayload {
                                request_id: request_id.to_string(),
                                session_id: session_id.map(|s| s.to_string()),
                                error: error_msg.clone(),
                            },
                        );
                        return Err(AppError::Llm(error_msg));
                    }

                    if let Some(content) = parsed["choices"][0]["delta"]["content"].as_str() {
                        full_content.push_str(content);

                        let _ = app.emit(
                            CHAT_CHUNK,
                            ChatChunkPayload {
                                request_id: request_id.to_string(),
                                session_id: session_id.map(|s| s.to_string()),
                                content: content.to_string(),
                            },
                        );
                    }
                }
            }
        }
    }

    let _ = app.emit(
        CHAT_DONE,
        ChatDonePayload {
            request_id: request_id.to_string(),
            session_id: session_id.map(|s| s.to_string()),
            full_content: full_content.clone(),
            message_id: None,
        },
    );

    Ok(full_content)
}

pub async fn generate_title(
    model: &ModelConfig,
    user_message: &str,
) -> Result<String, AppError> {
    let url = build_api_url(&model.base_url);

    let body = json!({
        "model": model.model_name,
        "messages": [
            {"role": "system", "content": "请用5-10个字总结以下用户问题的主题，只返回主题文字，不要加引号或其他格式。"},
            {"role": "user", "content": user_message}
        ],
        "stream": false,
        "max_tokens": 50
    });

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", model.api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    if !status.is_success() {
        return Ok("新对话".to_string());
    }

    let parsed: Value = response.json().await.unwrap_or_default();
    let title = parsed["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("新对话")
        .trim()
        .to_string();

    if title.is_empty() {
        Ok("新对话".to_string())
    } else {
        Ok(title)
    }
}
