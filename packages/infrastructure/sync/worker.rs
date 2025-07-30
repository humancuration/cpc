//! Sync worker that processes the queue periodically
//!
//! This module implements a worker that periodically processes the sync queue,
//! handling network status changes and ensuring reliable synchronization.

use crate::queue::SyncQueue;
use crate::storage::QueueStorage;
use tokio::time::{interval, Duration};
use std::sync::Arc;

/// Sync worker that processes the queue periodically
pub struct SyncWorker {
    queue: Arc<SyncQueue>,
    interval: Duration,
}

impl SyncWorker {
    /// Create a new sync worker
    pub fn new(queue: Arc<SyncQueue>, interval: Duration) -> Self {
        Self { queue, interval }
    }
    
    /// Start the worker (runs indefinitely)
    pub async fn start<T>(&self, client: T) 
    where 
        T: crate::queue::UserPreferencesClient + Clone + Send + Sync + 'static,
    {
        let mut interval_timer = interval(self.interval);
        let queue = self.queue.clone();
        
        loop {
            interval_timer.tick().await;
            
            // Process the queue
            match queue.process(&client).await {
                Ok(summary) => {
                    if summary.successful.len() > 0 {
                        tracing::info!("Processed {} operations successfully", summary.successful.len());
                    }
                    if summary.retried.len() > 0 {
                        tracing::info!("{} operations scheduled for retry", summary.retried.len());
                    }
                    if summary.failed.len() > 0 {
                        tracing::warn!("{} operations failed permanently", summary.failed.len());
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to process sync queue: {}", e);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::{SledQueueStorage, SyncOperation, OperationPriority};
    use crate::backoff::ExponentialBackoff;
    use crate::conflict::TimestampConflictResolver;
    use crate::network_fault_mock::NetworkFaultMockClient;
    use packages::domains::finance::domain::primitives::Currency;
    use uuid::Uuid;
    use std::time::SystemTime;
    use tempfile::TempDir;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_sync_worker_processing() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let storage = SledQueueStorage::new(&db).expect("Failed to create storage");
        
        let queue = Arc::new(SyncQueue::new(
            Box::new(storage),
            Arc::new(TimestampConflictResolver::new()),
            Box::new(ExponentialBackoff::default())
        ));
        
        let client = NetworkFaultMockClient::new(vec![true]);
        
        // Enqueue an operation
        let user_id = Uuid::new_v4();
        let operation = SyncOperation::SetCurrency {
            user_id,
            currency: Currency::USD,
            priority: OperationPriority::High,
            attempts: 0,
            scheduled_at: SystemTime::now(),
        };
        queue.enqueue(operation).expect("Failed to enqueue operation");
        
        // Process once
        let summary = queue.process(&client).await.expect("Failed to process queue");
        
        assert_eq!(summary.successful.len(), 1);
        assert_eq!(summary.retried.len(), 0);
        assert_eq!(summary.failed.len(), 0);
    }
}