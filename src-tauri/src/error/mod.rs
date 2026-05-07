use serde::{Deserialize, Serialize, Serializer};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization Error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Config Error: {0}")]
    Config(String),
    #[error("Database Error: {0}")]
    Database(String),
    #[error("LLM Error: {0}")]
    Llm(String),
    #[error("Request Error: {0}")]
    Request(#[from] reqwest::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StreamError {
    pub code: Option<String>,
    pub message: String,
}
