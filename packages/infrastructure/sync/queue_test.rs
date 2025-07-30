//! Tests for the sync queue

use crate::queue::{SyncQueue, UserPreferencesClient};
use crate::storage::{SledQueueStorage, SyncOperation, OperationPriority};
use packages::domains::finance::domain::primitives::Currency;
use uuid::Uuid;
use tempfile::TempDir;
use std::time::SystemTime;

// Mock client for testing
#[derive(Clone)]
struct MockClient;

#[async_trait::async_trait]
impl UserPreferencesClient for MockClient {
    async fn set_preferred_currency(&self, _user_id: Uuid, _currency: Currency) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sync_queue_enqueue() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let queue = SyncQueue::with_defaults(Box::new(storage));
        
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        let result = queue.enqueue(operation);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_sync_queue_process() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let queue = SyncQueue::with_defaults(Box::new(storage));
        let client = MockClient;
        
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        // Enqueue operation
        queue.enqueue(operation).expect("Failed to enqueue operation");
        
        // Process queue
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify results
        assert_eq!(summary.successful.len(), 1);
        assert_eq!(summary.retried.len(), 0);
        assert_eq!(summary.failed.len(), 0);
    }
    
    #[tokio::test]
    async fn test_sync_queue_priority_processing() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let queue = SyncQueue::with_defaults(Box::new(storage));
        let client = MockClient;
        
        let user_id = Uuid::new_v4();
        
        // Enqueue low priority operation
        let low_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::Low,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(low_priority_op).expect("Failed to enqueue low priority operation");
        
        // Enqueue high priority operation
        let high_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::EUR,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(high_priority_op).expect("Failed to enqueue high priority operation");
        
        // Process queue
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify both operations were processed
        assert_eq!(summary.successful.len(), 2);
        assert_eq!(summary.retried.len(), 0);
        assert_eq!(summary.failed.len(), 0);
    }
    
    #[tokio::test]
    async fn test_sync_queue_with_network_faults() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let queue = SyncQueue::with_defaults(Box::new(storage));
        let client = crate::network_fault_mock::NetworkFaultMockClient::new(vec![false, false, true]);
        
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        
        // Enqueue operation
        queue.enqueue(operation).expect("Failed to enqueue operation");
        
        // Process queue - first two attempts should fail, third should succeed
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify results
        assert_eq!(summary.successful.len(), 1);
        assert_eq!(summary.retried.len(), 2); // Two retries
        assert_eq!(summary.failed.len(), 0);
        
        // Check that the operation was retried with backoff
        assert!(!summary.retried.is_empty());
    }
}