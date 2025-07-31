//! Authentication error types

use thiserror::Error;
use serde::{Deserialize, Serialize};

/// Authentication errors
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuthError {
    /// Unsupported OAuth provider
    #[error("Unsupported OAuth provider: {0}")]
    UnsupportedProvider(String),
    
    /// Invalid authorization code
    #[error("Invalid authorization code")]
    InvalidAuthorizationCode,
    
    /// Invalid state parameter
    #[error("Invalid state parameter")]
    InvalidState,
    
    /// Token exchange failed
    #[error("Token exchange failed: {0}")]
    TokenExchangeFailed(String),
    
    /// Token refresh failed
    #[error("Token refresh failed: {0}")]
    TokenRefreshFailed(String),
    
    /// Token expired
    #[error("Token expired")]
    TokenExpired,
    
    /// Invalid token
    #[error("Invalid token")]
    InvalidToken,
    
    /// Encryption error
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),
    
    /// Consent denied
    #[error("Consent denied for required scopes")]
    ConsentDenied,
    
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// Provider-specific error
    #[error("Provider error: {0}")]
    ProviderError(String),
    
    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl AuthError {
    /// Check if the error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AuthError::NetworkError(_) | 
            AuthError::TokenRefreshFailed(_) | 
            AuthError::ProviderError(_)
        )
    }
}