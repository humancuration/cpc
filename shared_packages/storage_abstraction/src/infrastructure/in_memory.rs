//! In-memory implementation for storage abstraction
//! 
//! This module provides an in-memory implementation of the DataStore trait for testing purposes.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::traits::{DataStore, StorageError};

/// In-memory storage implementation for testing
pub struct InMemoryStore {
    data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl InMemoryStore {
    /// Create a new in-memory store
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DataStore for InMemoryStore {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let data = self.data.read().await;
        Ok(data.get(key).cloned())
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value);
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        let mut data = self.data.write().await;
        data.remove(key);
        Ok(())
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}