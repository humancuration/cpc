#[derive(Debug, thiserror::Error)]
pub enum PublishError {
    #[error("Serialization failed: {0}")]
    Serialization(#[source] anyhow::Error),
    #[error("Network error: {0}")]
    Network(#[source] anyhow::Error),
    #[error("PDS processing error: {0}")]
    PdsProcessing(String),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("JWT validation failed: {0}")]
    JwtValidation(String),
    #[error("Token expired")]
    TokenExpired,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Configuration error: {0}")]
    Config(String),
}