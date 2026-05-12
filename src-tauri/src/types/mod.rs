use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub theme: String,
    #[serde(default = "default_system_prompt")]
    pub system_prompt: String,
}

fn default_system_prompt() -> String {
    "你是 Friday，一个定制化个人知识智能服务助手。你友好、专业，善于帮助用户解答问题和完成任务。".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
            theme: "light".to_string(),
            system_prompt: default_system_prompt(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub id: String,
    pub provider: String,
    #[serde(rename = "providerLabel")]
    pub provider_label: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: i64,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub role: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatChunkPayload {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatDonePayload {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "fullContent")]
    pub full_content: String,
    #[serde(rename = "messageId")]
    pub message_id: Option<i64>,
    #[serde(rename = "userMessageId")]
    pub user_message_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatErrorPayload {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: String,
    #[serde(rename = "knowledgeBaseId")]
    pub knowledge_base_id: Option<String>,
    pub title: String,
    pub content: String,
    #[serde(rename = "contentText")]
    pub content_text: String,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionTitlePayload {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduleEvent {
    pub id: String,
    pub title: String,
    #[serde(rename = "start")]
    pub start_date: String,
    #[serde(rename = "end")]
    pub end_date: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "allDay")]
    pub all_day: bool,
    pub description: String,
    pub color: String,
    pub reminder: bool,
    pub completed: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
