use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User not found")]
    UserNotFound,
    #[error("Email already exists")]
    EmailExists,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Internal server error")]
    InternalError,
}

impl AuthError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AuthError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AuthError::UserNotFound => StatusCode::NOT_FOUND,
            AuthError::EmailExists => StatusCode::CONFLICT,
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::TokenExpired => StatusCode::UNAUTHORIZED,
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}