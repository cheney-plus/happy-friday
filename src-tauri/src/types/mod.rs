
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: String,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            language: "zh-CN".to_string(),
            theme: "light".to_string(),
        }
    }
}
