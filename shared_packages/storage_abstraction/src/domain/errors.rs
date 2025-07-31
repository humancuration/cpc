//! Error types for the storage abstraction layer

/// Error type for storage operations
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Storage operation failed: {0}")]
    OperationFailed(String),
    
    #[error("Data not found")]
    NotFound,
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}