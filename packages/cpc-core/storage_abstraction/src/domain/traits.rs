//! Core storage interfaces
//! 
//! This module defines the core traits that all storage implementations must implement.

use std::future::Future;
use std::pin::Pin;

/// Error type for storage operations
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Storage operation failed: {0}")]
    OperationFailed(String),
    
    #[error("Data not found")]
    NotFound,
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// Core storage interface
/// 
/// This trait defines the basic operations that any storage backend must implement.
#[async_trait::async_trait]
pub trait DataStore: Send + Sync {
    /// Get a value by key
    /// 
    /// # Arguments
    /// * `key` - The key to retrieve
    /// 
    /// # Returns
    /// * `Ok(Some(value))` if the key exists
    /// * `Ok(None)` if the key does not exist
    /// * `Err(StorageError)` if an error occurred
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    
    /// Set a value by key
    /// 
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to store
    /// 
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(StorageError)` if an error occurred
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError>;
    
    /// Delete a value by key
    /// 
    /// # Arguments
