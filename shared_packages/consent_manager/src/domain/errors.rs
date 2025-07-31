//! Error types for the consent manager domain.

use thiserror::Error;

/// Errors that can occur in the consent manager domain
#[derive(Error, Debug, PartialEq)]
pub enum ConsentError {
    /// Consent level is the same as the current level
    #[error("No change in consent level")]
    NoChange,

    /// Invalid consent level for the operation
    #[error("Invalid consent level")]
    InvalidLevel,

    /// User not found
    #[error("User not found")]
    UserNotFound,

    /// Domain not found
    #[error("Domain not found")]
    DomainNotFound,

    /// Storage error
    #[error("Storage error: {0}")]
    StorageError(String),

    /// Validation error
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Unauthorized access
    #[error("Unauthorized access")]
    Unauthorized,

    /// Audit trail error
    #[error("Audit trail error: {0}")]
    AuditError(String),
}