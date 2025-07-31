//! Sync queue for managing offline operations with prioritization and backoff
//!
//! This module implements a queue for storing and processing operations
//! that need to be synchronized with the backend when connectivity is restored.
//! It supports operation prioritization, exponential backoff, and fault-tolerant
//! processing to ensure reliable synchronization even in challenging network conditions.

use std::sync::Arc;
use std::time::SystemTime;
use packages::domains::finance::domain::primitives::Currency;
use crate::storage::{QueueStorage, SyncOperation, QueueKey, QueueError};
use crate::backoff::{BackoffStrategy, ExponentialBackoff};
use crate::conflict::{TimestampConflictResolver, ResolutionResult};

/// Summary of processing results
#[derive(Debug)]
pub struct ProcessingSummary {
    /// Operations successfully processed
    pub successful: Vec<SyncOperation>,
    
    /// Operations scheduled for retry
    pub retried: Vec<(SyncOperation, std::time::Duration)>,
    
    /// Operations that failed permanently
    pub failed: Vec<SyncOperation>,
}

impl ProcessingSummary {
    /// Create a new ProcessingSummary
    pub fn new() -> Self {
        Self {
            successful: Vec::new(),
            retried: Vec::new(),
            failed: Vec::new(),
        }
    }
}

/// Trait for user preferences client
#[async_trait::async_trait]
pub trait UserPreferencesClient: Send + Sync {
    async fn set_preferred_currency(&self, user_id: uuid::Uuid, currency: Currency) -> Result<(), String>;
}

/// Sync queue for managing offline operations
pub struct SyncQueue {
    storage: Box<dyn QueueStorage>,
    conflict_resolver: Arc<TimestampConflictResolver>,
    backoff_strategy: Box<dyn BackoffStrategy>,
}

impl SyncQueue {
    /// Create a new sync queue
    pub fn new(
        storage: Box<dyn QueueStorage>,
        conflict_resolver: Arc<TimestampConflictResolver>,
        backoff_strategy: Box<dyn BackoffStrategy>,
    ) -> Self {
        Self {
            storage,
            conflict_resolver,
            backoff_strategy,
        }
    }
    
    /// Create a new sync queue with default components
    pub fn with_defaults(storage: Box<dyn QueueStorage>) -> Self {
        Self {
            storage,
            conflict_resolver: Arc::new(TimestampConflictResolver::new()),
            backoff_strategy: Box::new(ExponentialBackoff::default()),
        }
    }
    
    /// Add an operation to the sync queue
    pub fn enqueue(&self, operation: SyncOperation) -> Result<(), QueueError> {
        self.storage.enqueue(operation).map(|_| ())
    }
    
    /// Process all pending operations in the queue
    pub async fn process<T>(&self, client: &T) -> Result<ProcessingSummary, String>
    where
        T: UserPreferencesClient,
    {
        let mut summary = ProcessingSummary::new();
        
        // Get all operations ready for processing (sorted by priority)
        let operations = self.storage.dequeue_ready()
            .map_err(|e| format!("Failed to dequeue operations: {}", e))?;
        
        // Process each operation
        for (key, operation) in operations {
            // Mark as processing
            self.storage.mark_as_processing(&key)
                .map_err(|e| format!("Failed to mark operation as processing: {}", e))?;
            
            match self.attempt_sync(&operation, client).await {
                Ok(_) => {
                    // Remove from queue on success
                    self.storage.complete(&key)
                        .map_err(|e| format!("Failed to complete operation: {}", e))?;
                    summary.successful.push(operation);
                }
                Err(e) => {
                    // Handle retry/backoff
                    if operation.attempts() >= 3 { // MAX_RETRIES
                        // Mark as failed permanently
                        self.storage.complete(&key) // Remove from queue
                            .map_err(|e| format!("Failed to mark operation as failed: {}", e))?;
                        summary.failed.push(operation);
                    } else {
                        // Schedule for retry with backoff
                        let next_attempt = SystemTime::now() + self.backoff_strategy.next_delay(&operation);
                        self.storage.schedule_retry(&key, &operation, next_attempt)
                            .map_err(|e| format!("Failed to schedule retry: {}", e))?;
                        summary.retried.push((operation, self.backoff_strategy.next_delay(&operation)));
                    }
                    // Continue processing other operations even if one fails
                }
            }
        }
        
        Ok(summary)
    }
    
    /// Attempt to sync a single operation
    async fn attempt_sync<T>(
        &self,
        operation: &SyncOperation,
        client: &T,
    ) -> Result<(), String>
    where
        T: UserPreferencesClient,
    {
        match operation {
            SyncOperation::SetCurrency { user_id, currency, .. } => {
                client.set_preferred_currency(*user_id, currency.clone()).await
            }
        }
    }
    
    /// Get failed operations
    pub fn get_failed_operations(&self) -> Result<Vec<SyncOperation>, QueueError> {
        let operations = self.storage.get_failed_operations()?;
        Ok(operations.into_iter().map(|(_, op)| op).collect())
    }
    
    /// Clear failed operations
    pub fn clear_failed(&self) -> Result<(), QueueError> {
        self.storage.clear_failed()
    }
}