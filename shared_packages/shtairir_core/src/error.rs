//! Error handling framework for Shtairir Core

use thiserror::Error;

/// Result type for Shtairir Core operations
pub type ShtairirResult<T> = Result<T, ShtairirError>;

/// Error types for Shtairir Core
#[derive(Error, Debug, Clone)]
pub enum ShtairirError {
    /// Registry-related errors
    #[error("Registry error: {0}")]
    Registry(String),

    /// Adapter-related errors
    #[error("Adapter error: {0}")]
    Adapter(String),

    /// Event bus errors
    #[error("Event bus error: {0}")]
    EventBus(String),

    /// Type system errors
    #[error("Type error: {0}")]
    Type(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// Network/communication errors
    #[error("Network error: {0}")]
    Network(String),

    /// Redis connection errors
    #[error("Redis error: {0}")]
    Redis(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(String),
}

impl From<serde_json::Error> for ShtairirError {
    fn from(err: serde_json::Error) -> Self {
        ShtairirError::Serialization(err.to_string())
    }
}

impl From<bb8::RunError<bb8_redis::redis::RedisError>> for ShtairirError {
    fn from(err: bb8::RunError<bb8_redis::redis::RedisError>) -> Self {
        ShtairirError::Redis(err.to_string())
    }
}

impl From<bb8_redis::redis::RedisError> for ShtairirError {
    fn from(err: bb8_redis::redis::RedisError) -> Self {
        ShtairirError::Redis(err.to_string())
    }
}

impl From<std::io::Error> for ShtairirError {
    fn from(err: std::io::Error) -> Self {
        ShtairirError::Io(err.to_string())
    }
}