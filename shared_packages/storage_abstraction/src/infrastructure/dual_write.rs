//! Dual-write pattern implementation
//! 
//! This module provides a dual-write implementation that writes to both edge and cloud storage
//! during migration periods, ensuring data consistency.

use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::traits::{DataStore, StorageError};

/// Dual-write storage implementation
/// 
/// This implementation writes to both edge and cloud storage during migration periods,
/// ensuring data consistency between the two storage systems.
pub struct DualWriteStore {
    /// Primary storage backend
    primary: Arc<dyn DataStore>,
    /// Secondary storage backend
    secondary: Arc<dyn DataStore>,
}

impl DualWriteStore {
    /// Create a new dual-write store
    pub fn new(primary: Arc<dyn DataStore>, secondary: Arc<dyn DataStore>) -> Self {
        Self { primary, secondary }
    }
}

#[async_trait]
impl DataStore for DualWriteStore {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        // Try primary first
        match self.primary.get(key).await {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => {
                // If not found in primary, try secondary
                self.secondary.get(key).await
            },
            Err(e) => {
                // If primary fails, try secondary
                tracing::warn!("Primary storage failed: {}, trying secondary", e);
                self.secondary.get(key).await
            },
        }
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        // Write to primary first
        self.primary.set(key, value.clone()).await?;
        
        // Write to secondary
        if let Err(e) = self.secondary.set(key, value).await {
            tracing::warn!("Failed to write to secondary storage: {}", e);
            // Don't fail the operation if secondary write fails
        }
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        // Delete from primary first
        self.primary.delete(key).await?;
        
        // Delete from secondary
        if let Err(e) = self.secondary.delete(key).await {
            tracing::warn!("Failed to delete from secondary storage: {}", e);
            // Don't fail the operation if secondary delete fails
        }
        
        Ok(())
    }
}