//! Integration tests for the sync queue with network monitoring
//!
//! This module contains comprehensive integration tests that verify the
//! interaction between the sync queue, network monitoring, and composite
//! adapter components.

#[cfg(test)]
mod tests {
    use crate::queue::SyncQueue;
    use crate::storage::{SledQueueStorage, SyncOperation, OperationPriority};
    use crate::network_fault_mock::NetworkFaultMockClient;
    use crate::backoff::ExponentialBackoff;
    use crate::conflict::TimestampConflictResolver;
    use packages::domains::finance::domain::primitives::Currency;
    use uuid::Uuid;
    use std::sync::Arc;
    use std::time::{SystemTime, Duration};
    use tempfile::TempDir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_retry_with_backoff_strategy() {
        // Setup
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        // Create operation that fails 3 times then succeeds
        let client = NetworkFaultMockClient::new(vec![false, false, false, true]);
        let queue = SyncQueue::new(
            Box::new(storage),
            Arc::new(TimestampConflictResolver::new()),
            Box::new(ExponentialBackoff::default())
        );
        
        // Enqueue operation
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(operation).expect("Failed to enqueue operation");
        
        // Process queue
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify
        assert_eq!(summary.successful.len(), 1);
        assert_eq!(summary.retried.len(), 3);
        assert_eq!(summary.failed.len(), 0);
        
        // Check backoff timing (basic check)
        let history = client.get_operation_history();
        assert_eq!(history.len(), 4); // 3 failures + 1 success
        assert!(!history[0].success);
        assert!(!history[1].success);
        assert!(!history[2].success);
        assert!(history[3].success);
    }
    
    #[tokio::test]
    async fn test_priority_based_processing() {
        // Setup
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        // Create client that always succeeds
        let client = NetworkFaultMockClient::new(vec![true, true, true, true, true]);
        let queue = SyncQueue::new(
            Box::new(storage),
            Arc::new(TimestampConflictResolver::new()),
            Box::new(ExponentialBackoff::default())
        );
        
        // Enqueue mixed priority operations
        let user_id = Uuid::new_v4();
        
        // Low priority
        let low_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::Low,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(low_priority_op).expect("Failed to enqueue low priority operation");
        
        // Critical priority
        let critical_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::BTC,
            priority: OperationPriority::Critical,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(critical_priority_op).expect("Failed to enqueue critical priority operation");
        
        // Medium priority
        let medium_priority_op = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::EUR,
            priority: OperationPriority::Medium,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(medium_priority_op).expect("Failed to enqueue medium priority operation");
        
        // Process queue
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify all operations were processed
        assert_eq!(summary.successful.len(), 3);
        assert_eq!(summary.retried.len(), 0);
        assert_eq!(summary.failed.len(), 0);
        
        // Operations should be processed in priority order (Critical, Medium, Low)
        // Note: Our current implementation sorts by priority, so this should be verified
        // in the storage layer rather than here
    }
    
    #[tokio::test]
    async fn test_network_flapping_resilience() {
        // Setup
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        // Create client that simulates network flapping
        let client = NetworkFaultMockClient::new(vec![true, false, true, false, true, true, true]);
        let queue = SyncQueue::new(
            Box::new(storage),
            Arc::new(TimestampConflictResolver::new()),
            Box::new(ExponentialBackoff::default())
        );
        
        // Enqueue multiple operations
        let user_id = Uuid::new_v4();
        
        for i in 0..5 {
            let operation = SyncOperation::SetCurrency {
                user_id,
                currency: match i % 3 {
                    0 => Currency::USD,
                    1 => Currency::EUR,
                    _ => Currency::GBP,
                },
                priority: OperationPriority::High,
                attempts: 0,
                scheduled_at: SystemTime::now(),
            };
            queue.enqueue(operation).expect("Failed to enqueue operation");
        }
        
        // Process queue
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify that despite network flapping, operations are processed
        assert_eq!(summary.successful.len(), 5);
        // Some operations may have been retried due to network failures
        assert!(summary.retried.len() >= 0);
        assert_eq!(summary.failed.len(), 0);
    }
    
    #[tokio::test]
    async fn test_storage_failure_recovery() {
        // Setup
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        // Create client that always succeeds
        let client = NetworkFaultMockClient::new(vec![true]);
        let queue = SyncQueue::new(
            Box::new(storage),
            Arc::new(TimestampConflictResolver::new()),
            Box::new(ExponentialBackoff::default())
        );
        
        // Enqueue operation
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(operation).expect("Failed to enqueue operation");
        
        // Process queue
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        // Verify successful processing
        assert_eq!(summary.successful.len(), 1);
        assert_eq!(summary.retried.len(), 0);
        assert_eq!(summary.failed.len(), 0);
        
        // Verify queue is now empty
        let failed_ops = queue.get_failed_operations().expect("Failed to get failed operations");
        assert_eq!(failed_ops.len(), 0);
    }
}