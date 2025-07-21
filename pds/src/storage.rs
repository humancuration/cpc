//! Network storage interface

use crate::metadata::{FileMetadata, ChunkMetadata};

/// Comprehensive error type for network operations
#[derive(Debug)]
pub enum NetworkError {
    StorageError(String),
    NotFound,
    ConnectionError,
    SerializationError,
    VerificationFailed(String),
    DecryptionError(String),
    StorageFull,
    InvalidData,
    // Add more as needed
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            NetworkError::NotFound => write!(f, "Resource not found"),
            NetworkError::ConnectionError => write!(f, "Connection error"),
            NetworkError::SerializationError => write!(f, "Serialization error"),
            NetworkError::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            NetworkError::DecryptionError(msg) => write!(f, "Decryption error: {}", msg),
            NetworkError::StorageFull => write!(f, "Storage full"),
            NetworkError::InvalidData => write!(f, "Invalid data"),
        }
    }
}

/// Network trait extension for storage operations
pub trait Network {
    /// Store a file's metadata in the network
    async fn store_metadata(&mut self, metadata: &FileMetadata) -> Result<(), NetworkError>;
    
    /// Retrieve a file's metadata by content address
    async fn get_metadata(&self, content_address: &str) -> Result<FileMetadata, NetworkError>;
    
    /// Store an encrypted chunk in the network using content address
    async fn store_chunk(&mut self, content_address: &str, chunk: &[u8]) -> Result<(), NetworkError>;
    
    /// Retrieve a chunk by its content address
    async fn get_chunk(&self, content_address: &str) -> Result<Vec<u8>, NetworkError>;
}