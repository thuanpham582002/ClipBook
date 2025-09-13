use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ClipBookError {
    #[error("Clipboard access denied: {0}")]
    ClipboardError(String),
    
    #[error("System error: {0}")]
    SystemError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Database: {0}")]
    Database(String),
    
    #[error("IO error: {0}")]
    IoError(String),
    
    #[error("Performance threshold exceeded: {0}ms")]
    PerformanceError(u64),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<arboard::Error> for ClipBookError {
    fn from(err: arboard::Error) -> Self {
        ClipBookError::ClipboardError(err.to_string())
    }
}

impl From<sqlx::Error> for ClipBookError {
    fn from(err: sqlx::Error) -> Self {
        ClipBookError::DatabaseError(err.to_string())
    }
}

impl From<std::io::Error> for ClipBookError {
    fn from(err: std::io::Error) -> Self {
        ClipBookError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for ClipBookError {
    fn from(err: serde_json::Error) -> Self {
        ClipBookError::SerializationError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ClipBookError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ClipBookError::SystemError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for ClipBookError {
    fn from(err: std::num::ParseIntError) -> Self {
        ClipBookError::SystemError(err.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorReport {
    pub operation: String,
    pub error: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub user_context: Option<String>,
}

impl ErrorReport {
    pub fn new(operation: &str, error: &ClipBookError) -> Self {
        Self {
            operation: operation.to_string(),
            error: error.to_string(),
            timestamp: chrono::Utc::now(),
            user_context: None,
        }
    }
    
    pub fn with_context(mut self, context: String) -> Self {
        self.user_context = Some(context);
        self
    }
}

pub type Result<T> = std::result::Result<T, ClipBookError>;