//! Sled implementation for storage abstraction
//! 
//! This module provides a Sled-based implementation of the DataStore trait.

use async_trait::async_trait;
use sled::Db;
use crate::domain::traits::{DataStore, StorageError};

/// Sled-based storage implementation
pub struct SledStore {
    db: Db,
}

impl SledStore {
    /// Create a new Sled store
    pub fn new(path: &str) -> Result<Self, StorageError> {
        let db = sled::open(path)
            .map_err(|e| StorageError::ConnectionError(format!("Failed to open Sled database: {}", e)))?;
        
        Ok(Self { db })
    }
    
    /// Create a new Sled store with default configuration
    pub fn new_default() -> Result<Self, StorageError> {
        let db = sled::Config::new()
            .temporary(true)
            .open()
            .map_err(|e| StorageError::ConnectionError(format!("Failed to open Sled database: {}", e)))?;
        
        Ok(Self { db })
    }
}

#[async_trait]
impl DataStore for SledStore {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        match self.db.get(key) {
            Ok(Some(value)) => Ok(Some(value.to_vec())),
            Ok(None) => Ok(None),
            Err(e) => Err(StorageError::OperationFailed(format!("Failed to get key: {}", e))),
        }
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        self.db.insert(key, value)
            .map_err(|e| StorageError::OperationFailed(format!("Failed to set key: {}", e)))?;
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.db.remove(key)
            .map_err(|e| StorageError::OperationFailed(format!("Failed to delete key: {}", e)))?;
        Ok(())
    }
}