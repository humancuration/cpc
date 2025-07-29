//! Caching strategies for storage operations
//! 
//! This module implements caching layers to improve performance of storage operations.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::traits::StorageError;

/// In-memory cache for storage operations
pub struct InMemoryCache {
    /// The actual cache storage
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Maximum number of entries in the cache
    max_size: usize,
}

impl InMemoryCache {
    /// Create a new in-memory cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }
    
    /// Get a value from the cache
    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }
    
    /// Set a value in the cache
    pub async fn set(&self, key: String, value: Vec<u8>) -> Result<(), StorageError> {
        let mut cache = self.cache.write().await;
        
        // If we're at max size, remove a random entry
        if cache.len() >= self.max_size {
            if let Some(key) = cache.keys().next().cloned() {
                cache.remove(&key);
            }
        }
        
        cache.insert(key, value);
        Ok(())
    }
    
    /// Remove a value from the cache
    pub async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }
    
    /// Clear the cache
    pub async fn clear(&self) -> Result<(), StorageError> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }
}