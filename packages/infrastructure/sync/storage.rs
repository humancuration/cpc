//! Storage abstraction for the sync queue
//!
//! This module provides a storage abstraction that allows the sync queue to work
//! with different storage backends. The default implementation uses Sled, but
//! alternative implementations could use other databases.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;
use packages::domains::finance::domain::primitives::Currency;
use thiserror::Error;

/// Key type for queue operations
pub type QueueKey = Vec<u8>;

/// Priority levels for sync operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum OperationPriority {
    Critical = 100,  // Payments, security-critical actions
    High = 75,       // User-facing actions
    Medium = 50,     // Background sync
    Low = 25,        // Analytics, non-critical data
}

/// Operations that can be queued for sync
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncOperation {
    SetCurrency {
        user_id: Uuid,
        currency: Currency,
        priority: OperationPriority,
        attempts: u32,
        scheduled_at: SystemTime,
    },
}

impl SyncOperation {
    /// Get the number of attempts for this operation
    pub fn attempts(&self) -> u32 {
        match self {
            SyncOperation::SetCurrency { attempts, .. } => *attempts,
        }
    }
    
    /// Get the priority of this operation
    pub fn priority(&self) -> &OperationPriority {
        match self {
            SyncOperation::SetCurrency { priority, .. } => priority,
        }
    }
    
    /// Get the scheduled time for this operation
    pub fn scheduled_at(&self) -> SystemTime {
        match self {
            SyncOperation::SetCurrency { scheduled_at, .. } => *scheduled_at,
        }
    }
    
    /// Update the operation with incremented attempts
    pub fn with_incremented_attempts(&self) -> Self {
        match self {
            SyncOperation::SetCurrency { user_id, currency, priority, attempts, scheduled_at } => {
                SyncOperation::SetCurrency {
                    user_id: *user_id,
                    currency: currency.clone(),
                    priority: priority.clone(),
                    attempts: attempts + 1,
                    scheduled_at: *scheduled_at,
                }
            }
        }
    }
}

/// Result of conflict resolution
#[derive(Debug, Clone)]
pub enum ResolutionResult {
    UseLocal,
    UseRemote,
    Merge(Box<dyn std::any::Any>), // For complex merge scenarios
}

/// Policy for conflict resolution
#[derive(Debug, Clone)]
pub enum ResolutionPolicy {
    LastWriteWins,
    Custom,
}

/// Error types for queue operations
#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Operation not found")]
    NotFound,
}

/// Trait for queue storage implementations
pub trait QueueStorage: Send + Sync {
    /// Add an operation to the queue
    fn enqueue(&self, operation: SyncOperation) -> Result<QueueKey, QueueError>;
    
    /// Get all operations ready for processing (sorted by priority)
    fn dequeue_ready(&self) -> Result<Vec<(QueueKey, SyncOperation)>, QueueError>;
    
    /// Mark an operation as being processed
    fn mark_as_processing(&self, key: &QueueKey) -> Result<(), QueueError>;
    
    /// Mark an operation as completed
    fn complete(&self, key: &QueueKey) -> Result<(), QueueError>;
    
    /// Schedule an operation for retry
    fn schedule_retry(
        &self, 
        key: &QueueKey, 
        operation: &SyncOperation, 
        next_attempt: SystemTime
    ) -> Result<(), QueueError>;
    
    /// Get failed operations
    fn get_failed_operations(&self) -> Result<Vec<(QueueKey, SyncOperation)>, QueueError>;
    
    /// Clear failed operations
    fn clear_failed(&self) -> Result<(), QueueError>;
}

/// Sled implementation of QueueStorage
pub struct SledQueueStorage {
    tree: sled::Tree,
}

impl SledQueueStorage {
    /// Create a new SledQueueStorage instance
    pub fn new(db: &sled::Db) -> Result<Self, QueueError> {
        let tree = db.open_tree("sync_queue")
            .map_err(|e| QueueError::StorageError(format!("Failed to open sync queue: {}", e)))?;
        Ok(Self { tree })
    }
    
    /// Generate a unique key for queue operations
    fn generate_unique_key(&self) -> Vec<u8> {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_nanos() as u64;
        
        timestamp.to_be_bytes().to_vec()
    }
}

impl QueueStorage for SledQueueStorage {
    fn enqueue(&self, operation: SyncOperation) -> Result<QueueKey, QueueError> {
        let key = self.generate_unique_key();
        let value = bincode::serialize(&operation)
            .map_err(|e| QueueError::SerializationError(format!("Serialization error: {}", e)))?;
        
        self.tree.insert(&key, value)
            .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?;
        
        Ok(key)
    }
    
    fn dequeue_ready(&self) -> Result<Vec<(QueueKey, SyncOperation)>, QueueError> {
        let mut operations = Vec::new();
        
        for result in self.tree.iter() {
            let (key, value) = result
                .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?;
            
            let operation: SyncOperation = bincode::deserialize(&value)
                .map_err(|e| QueueError::SerializationError(format!("Deserialization error: {}", e)))?;
            
            // For now, we'll return all operations
            // In a more sophisticated implementation, we'd check scheduled_at
            operations.push((key.to_vec(), operation));
        }
        
        // Sort by priority (highest first)
        operations.sort_by(|a, b| {
            b.1.priority().cmp(a.1.priority()) // Reverse order for highest priority first
        });
        
        Ok(operations)
    }
    
    fn mark_as_processing(&self, key: &QueueKey) -> Result<(), QueueError> {
        // In a real implementation, we might update the operation status
        // For now, we'll just ensure it exists
        self.tree.get(key)
            .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?
            .ok_or(QueueError::NotFound)?;
        
        Ok(())
    }
    
    fn complete(&self, key: &QueueKey) -> Result<(), QueueError> {
        self.tree.remove(key)
            .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?;
        
        Ok(())
    }
    
    fn schedule_retry(
        &self, 
        key: &QueueKey, 
        operation: &SyncOperation, 
        _next_attempt: SystemTime
    ) -> Result<(), QueueError> {
        let updated_operation = operation.with_incremented_attempts();
        let value = bincode::serialize(&updated_operation)
            .map_err(|e| QueueError::SerializationError(format!("Serialization error: {}", e)))?;
        
        self.tree.insert(key, value)
            .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?;
        
        Ok(())
    }
    
    fn get_failed_operations(&self) -> Result<Vec<(QueueKey, SyncOperation)>, QueueError> {
        // For now, we'll return all operations
        // In a real implementation, we'd filter by attempt count
        let mut operations = Vec::new();
        
        for result in self.tree.iter() {
            let (key, value) = result
                .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?;
            
            let operation: SyncOperation = bincode::deserialize(&value)
                .map_err(|e| QueueError::SerializationError(format!("Deserialization error: {}", e)))?;
            
            operations.push((key.to_vec(), operation));
        }
        
        Ok(operations)
    }
    
    fn clear_failed(&self) -> Result<(), QueueError> {
        // For now, we'll clear all operations
        // In a real implementation, we'd only clear failed ones
        self.tree.clear()
            .map_err(|e| QueueError::StorageError(format!("Storage error: {}", e)))?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::time::Duration;

    #[test]
    fn test_sled_queue_storage_enqueue() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let result = storage.enqueue(operation);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_sled_queue_storage_dequeue_ready() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        // Enqueue a few operations with different priorities
        let user_id = Uuid::new_v4();
        
        let low_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::Low,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        storage.enqueue(low_priority_op).expect("Failed to enqueue low priority operation");
        
        let high_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::EUR,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        storage.enqueue(high_priority_op).expect("Failed to enqueue high priority operation");
        
        // Dequeue operations - should be sorted by priority
        let operations = storage.dequeue_ready().expect("Failed to dequeue operations");
        assert_eq!(operations.len(), 2);
        
        // High priority should come first
        match &operations[0].1 {
            SyncOperation::SetCurrency { currency, priority, .. } => {
                assert_eq!(currency, &Currency::EUR);
                assert_eq!(priority, &OperationPriority::High);
            }
        }
        
        // Low priority should come second
        match &operations[1].1 {
            SyncOperation::SetCurrency { currency, priority, .. } => {
                assert_eq!(currency, &Currency::USD);
                assert_eq!(priority, &OperationPriority::Low);
            }
        }
    }
    
    #[test]
    fn test_sled_queue_storage_complete() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let key = storage.enqueue(operation).expect("Failed to enqueue operation");
        
        // Verify operation exists
        let operations = storage.dequeue_ready().expect("Failed to dequeue operations");
        assert_eq!(operations.len(), 1);
        
        // Complete the operation
        storage.complete(&key).expect("Failed to complete operation");
        
        // Verify operation is removed
        let operations = storage.dequeue_ready().expect("Failed to dequeue operations");
        assert_eq!(operations.len(), 0);
    }
    
    #[test]
    fn test_sled_queue_storage_schedule_retry() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let key = storage.enqueue(operation).expect("Failed to enqueue operation");
        
        // Schedule for retry (increments attempts)
        let next_attempt = SystemTime::now() + Duration::from_secs(60);
        storage.schedule_retry(&key, &storage.dequeue_ready().unwrap()[0].1, next_attempt)
            .expect("Failed to schedule retry");
        
        // Verify attempts were incremented
        let operations = storage.dequeue_ready().expect("Failed to dequeue operations");
        assert_eq!(operations.len(), 1);
        assert_eq!(operations[0].1.attempts(), 1);
    }
}