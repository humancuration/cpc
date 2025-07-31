//! Error types for the audit framework

/// Error type for audit operations
#[derive(thiserror::Error, Debug)]
pub enum AuditError {
    #[error("Audit serialization failed: {0}")]
    SerializationError(String),
    
    #[error("Audit deserialization failed: {0}")]
    DeserializationError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Audit event not found")]
    AuditEventNotFound,
    
    #[error("Invalid audit query: {0}")]
    InvalidQuery(String),
    
    #[error("Compliance check failed: {0}")]
    ComplianceCheckFailed(String),
}