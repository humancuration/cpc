//! Common error types for the CPC platform
//!
//! This module provides a unified error type that can be used across all modules
//! in the common_utils package. It follows the pattern established in the wallet
//! module for consistency.

use thiserror::Error;
use std::fmt;

/// Common error types for utility operations
#[derive(Error, Debug)]
pub enum CommonError {
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    /// MessagePack serialization/deserialization error
    #[error("MessagePack error: {0}")]
    MessagePack(#[from] rmp_serde::encode::Error),
    
    /// MessagePack deserialization error
    #[error("MessagePack decode error: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),
    
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// DateTime parsing error
    #[error("DateTime parsing error: {0}")]
    DateTime(#[from] chrono::ParseError),
    
    /// Cryptography error
    #[error("Crypto error: {0}")]
    Crypto(String),
    
    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Timeout error
    #[error("Operation timed out")]
    Timeout,
    
    /// Generic error with custom message
    #[error("Generic error: {0}")]
    Generic(String),
}

impl CommonError {
    /// Create a new generic error with a custom message
    pub fn generic(message: impl Into<String>) -> Self {
        CommonError::Generic(message.into())
    }
    
    /// Create a new crypto error with a custom message
    pub fn crypto(message: impl Into<String>) -> Self {
        CommonError::Crypto(message.into())
    }
    
    /// Create a new invalid input error with a custom message
    pub fn invalid_input(message: impl Into<String>) -> Self {
        CommonError::InvalidInput(message.into())
    }
}

/// Result type alias for common utility operations
pub type Result<T> = std::result::Result<T, CommonError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = CommonError::generic("test error");
        assert_eq!(format!("{}", err), "Generic error: test error");
        
        let err = CommonError::crypto("crypto error");
        assert_eq!(format!("{}", err), "Crypto error: crypto error");
        
        let err = CommonError::invalid_input("invalid input");
        assert_eq!(format!("{}", err), "Invalid input: invalid input");
    }
    
    #[test]
    fn test_error_from_json() {
        let json_err = serde_json::Error::syntax(serde_json::error::SyntaxError::EofWhileParsingValue, 0, 0);
        let common_err: CommonError = json_err.into();
        assert!(matches!(common_err, CommonError::Json(_)));
    }
}