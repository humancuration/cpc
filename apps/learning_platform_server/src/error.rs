use thiserror::Error;
use tonic::{Status, Code};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(_) => Status::internal("Database error"),
            AppError::Config(_) => Status::internal("Configuration error"),
            AppError::Auth(msg) => Status::unauthenticated(msg),
            AppError::Validation(msg) => Status::invalid_argument(msg),
            AppError::NotFound(msg) => Status::not_found(msg),
            AppError::AlreadyExists(msg) => Status::already_exists(msg),
            AppError::Internal(msg) => Status::internal(msg),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;