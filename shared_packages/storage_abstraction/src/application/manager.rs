//! StorageManager orchestrator
//! 
//! This module contains the StorageManager which is the primary entry point for storage operations.

use std::sync::Arc;
use crate::domain::{
    traits::{DataStore, StorageError},
    routing::{StorageRouter, StorageLocation},
};
use tracing::trace;

/// Configuration for the storage manager
pub struct StorageConfig {
    /// Router configuration
    pub router: crate::domain::routing::RoutingConfig,
}

/// Storage manager that orchestrates storage operations
/// 
/// This struct provides a unified interface for storage operations, handling
/// routing between different storage backends and implementing dual-write patterns.
pub struct StorageManager {
    /// Edge storage backend (Sled)
    edge_store: Arc<dyn DataStore>,
    /// Cloud storage backend (PostgreSQL)
    cloud_store: Arc<dyn DataStore>,
    /// Router for determining storage locations
    router: StorageRouter,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new(
        edge_store: Arc<dyn DataStore>,
        cloud_store: Arc<dyn DataStore>,
        config: StorageConfig,
    ) -> Self {
        let router = StorageRouter::new(config.router);
        
        Self {
            edge_store,
            cloud_store,
            router,
        }
    }
    
    /// Get a value by key
    /// 
    /// This method implements smart routing based on the key and routing configuration.
    /// 
    /// # Arguments
    /// * `key` - The key to retrieve
    /// 
    /// # Returns
    /// * `Ok(Some(value))` if the key exists
    /// * `Ok(None)` if the key does not exist
    /// * `Err(StorageError)` if an error occurred
    pub async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        trace!("Getting value for key: {}", key);
        
        let location = self.router.route_read(key);
        trace!("Routing read request for key '{}' to {:?}", key, location);
        
        match location {
            StorageLocation::Edge => self.edge_store.get(key).await,
            StorageLocation::Cloud => self.cloud_store.get(key).await,
        }
    }
    
    /// Set a value by key
    /// 
    /// This method implements dual-write with fallback strategies. It writes to the
    /// primary storage location first, then asynchronously writes to the secondary
    /// location in the background.
    /// 
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to store
    /// 
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(StorageError)` if an error occurred
    pub async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        trace!("Setting value for key: {}", key);
        
        let location = self.router.route_write(key);
        trace!("Routing write request for key '{}' to {:?}", key, location);
        
        match location {
            StorageLocation::Edge => {
                // Write to edge store first
                self.edge_store.set(key, value.clone()).await?;
                
                // Background write to cloud store
                let cloud_store = self.cloud_store.clone();
                let key = key.to_string();
                tokio::spawn(async move {
                    if let Err(e) = cloud_store.set(&key, value).await {
                        tracing::warn!("Failed to write to cloud store in background: {}", e);
                    }
                });
            },
            StorageLocation::Cloud => {
                // Write to cloud store first
                self.cloud_store.set(key, value.clone()).await?;
                
                // Background write to edge store
                let edge_store = self.edge_store.clone();
                let key = key.to_string();
                tokio::spawn(async move {
                    if let Err(e) = edge_store.set(&key, value).await {
                        tracing::warn!("Failed to write to edge store in background: {}", e);
                    }
                });
            },
        }
        
        Ok(())
    }
    
    /// Delete a value by key
    /// 
    /// This method deletes a value from both storage locations.
    /// 
    /// # Arguments
    /// * `key` - The key to delete
    /// 
    /// # Returns
    /// * `Ok(())` if the operation was successful
    /// * `Err(StorageError)` if an error occurred
    pub async fn delete(&self, key: &str) -> Result<(), StorageError> {
        trace!("Deleting value for key: {}", key);
        
        // Delete from both stores
        let edge_result = self.edge_store.delete(key).await;
        let cloud_result = self.cloud_store.delete(key).await;
        
        // Return the first error if any
        edge_result?;
        cloud_result?;
        
        Ok(())
    }
}