// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
use thiserror::Error;
use uuid::Uuid;

/// Network error types for the QUIC transport implementation
#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("QUIC transport error: {0}")]
    QuicError(String),
    
    #[error("STUN protocol error: {0}")]
    StunError(String),
    
    #[error("TURN protocol error: {0}")]
    TurnError(String),
    
    #[error("Connection timeout for peer {0}")]
    ConnectionTimeout(Uuid),
    
    #[error("Connection refused by peer {0}")]
    ConnectionRefused(Uuid),
    
    #[error("NAT traversal failed: {0}")]
    NatTraversalFailed(String),
    
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Certificate validation failed: {0}")]
    CertificateError(String),
    
    #[error("Network unreachable: {0}")]
    NetworkUnreachable(String),
    
    #[error("Resource limit exceeded: {0}")]
    ResourceLimitExceeded(String),
    
    #[error("Operation not supported: {0}")]
    UnsupportedOperation(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl From<quinn::ConnectError> for NetworkError {
    fn from(err: quinn::ConnectError) -> Self {
        NetworkError::QuicError(format!("Connection error: {}", err))
    }
}

impl From<quinn::ConnectionError> for NetworkError {
    fn from(err: quinn::ConnectionError) -> Self {
        NetworkError::QuicError(format!("Connection error: {}", err))
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> Self {
        NetworkError::QuicError(format!("IO error: {}", err))
    }
}