//! Error types for the social graph package

use thiserror::Error;

/// Errors that can occur in the social graph package
#[derive(Debug, Error)]
pub enum SocialGraphError {
    /// Consent-related errors
    #[error("Consent error: {0}")]
    Consent(#[from] crate::domain::service::consent_service::ConsentError),
    
    /// Repository-related errors
    #[error("Repository error: {0}")]
    Repository(#[from] crate::infrastructure::repositories::social_post_repository::RepositoryError),
    
    /// Provider-related errors
    #[error("Provider error: {0}")]
    Provider(#[from] crate::domain::model::ContentProviderError),
    
    /// Generic error for other issues
    #[error("Social graph operation failed: {0}")]
    Other(String),
}