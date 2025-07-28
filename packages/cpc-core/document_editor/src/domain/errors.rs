use thiserror::Error;

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("Document not found: {0}")]
    DocumentNotFound(String),
    
    #[error("Document access denied")]
    AccessDenied,
    
    #[error("Invalid document title: {0}")]
    InvalidTitle(String),
    
    #[error("Invalid permission level: {0}")]
    InvalidPermission(String),
    
    #[error("Version conflict: {0}")]
    VersionConflict(String),
    
    #[error("Export failed: {0}")]
    ExportFailed(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Media processing error: {0}")]
    MediaError(String),
}