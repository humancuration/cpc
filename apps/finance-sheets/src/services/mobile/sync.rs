//! Mobile sync service for Finance-Sheets
//!
//! This module provides functionality for synchronizing data between
//! local storage and remote servers when connectivity is available.

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::services::mobile::storage::{ChangeRecord, StorageError};

/// Error types for sync operations
#[derive(Debug, Clone, PartialEq)]
pub enum SyncError {
    /// Network error
    NetworkError(String),
    
    /// Conflict detected during sync
    ConflictDetected(String),
    
    /// Storage error
    StorageError(StorageError),
    
    /// Serialization error
    SerializationError(String),
}

/// Sync queue for managing pending operations
pub struct SyncQueue {
    /// Queue of pending sync operations
    pending_operations: VecDeque<SyncOperation>,
    
    /// Maximum number of operations to queue
    max_queue_size: usize,
}

/// Types of sync operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncOperation {
    /// Upload local changes
    UploadChanges {
        /// Sheet identifier
        sheet_id: String,
        
        /// Changes to upload
        changes: Vec<ChangeRecord>,
    },
    
    /// Download remote changes
    DownloadChanges {
        /// Sheet identifier
        sheet_id: String,
        
        /// Last sync timestamp
        last_sync_timestamp: u64,
    },
    
    /// Resolve conflicts
    ResolveConflicts {
        /// Sheet identifier
        sheet_id: String,
        
        /// Conflicting changes
        conflicts: Vec<Conflict>,
    },
}

/// Conflict information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conflict {
    /// Local change
    pub local_change: ChangeRecord,
    
    /// Remote change
    pub remote_change: ChangeRecord,
    
    /// Resolution strategy
    pub resolution: ConflictResolution,
}

/// Conflict resolution strategies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    /// Use local change (last writer wins)
    UseLocal,
    
    /// Use remote change (last writer wins)
    UseRemote,
    
    /// Manual resolution required
    Manual,
}

impl SyncQueue {
    /// Create a new sync queue
    pub fn new(max_queue_size: usize) -> Self {
        Self {
            pending_operations: VecDeque::new(),
            max_queue_size,
        }
    }
    
    /// Add an operation to the queue
    pub fn add_operation(&mut self, operation: SyncOperation) -> Result<(), SyncError> {
        if self.pending_operations.len() >= self.max_queue_size {
            return Err(SyncError::NetworkError("Sync queue is full".to_string()));
        }
        
        self.pending_operations.push_back(operation);
        Ok(())
    }
    
    /// Get the next operation from the queue
    pub fn next_operation(&mut self) -> Option<SyncOperation> {
        self.pending_operations.pop_front()
    }
    
    /// Check if there are pending operations
    pub fn has_pending_operations(&self) -> bool {
        !self.pending_operations.is_empty()
    }
    
    /// Get the number of pending operations
    pub fn pending_count(&self) -> usize {
        self.pending_operations.len()
    }
    
    /// Clear all pending operations
    pub fn clear_operations(&mut self) {
        self.pending_operations.clear();
    }
}

/// Conflict resolver for handling sync conflicts
pub struct ConflictResolver;

impl ConflictResolver {
    /// Resolve a conflict using the specified strategy
    pub fn resolve_conflict(conflict: &Conflict) -> Result<ChangeRecord, SyncError> {
        match &conflict.resolution {
            ConflictResolution::UseLocal => Ok(conflict.local_change.clone()),
            ConflictResolution::UseRemote => Ok(conflict.remote_change.clone()),
            ConflictResolution::Manual => {
                // In a real implementation, this would trigger UI for manual resolution
                Err(SyncError::ConflictDetected("Manual resolution required".to_string()))
            }
        }
    }
    
    /// Auto-resolve conflicts using last-write-wins strategy
    pub fn auto_resolve_conflicts(conflicts: Vec<Conflict>) -> Vec<ChangeRecord> {
        conflicts
            .into_iter()
            .filter_map(|conflict| {
                // Use the change with the later timestamp
                if conflict.local_change.timestamp >= conflict.remote_change.timestamp {
                    Some(conflict.local_change)
                } else {
                    Some(conflict.remote_change)
                }
            })
            .collect()
    }
    
    /// Create a conflict from two conflicting changes
    pub fn create_conflict(local: ChangeRecord, remote: ChangeRecord) -> Conflict {
        Conflict {
            local_change: local,
            remote_change: remote,
            resolution: ConflictResolution::UseLocal, // Default to local
        }
    }
}

/// Sync manager for coordinating sync operations
pub struct SyncManager {
    /// Sync queue
    queue: SyncQueue,
    
    /// Whether sync is currently in progress
    is_syncing: bool,
}

impl SyncManager {
    /// Create a new sync manager
    pub fn new() -> Self {
        Self {
            queue: SyncQueue::new(100), // Max 100 operations in queue
            is_syncing: false,
        }
    }
    
    /// Add changes to be synced
    pub fn queue_changes(&mut self, sheet_id: String, changes: Vec<ChangeRecord>) -> Result<(), SyncError> {
        let operation = SyncOperation::UploadChanges {
            sheet_id,
            changes,
        };
        
        self.queue.add_operation(operation)
    }
    
    /// Check for remote changes
    pub fn check_remote_changes(&mut self, sheet_id: String, last_sync_timestamp: u64) -> Result<(), SyncError> {
        let operation = SyncOperation::DownloadChanges {
            sheet_id,
            last_sync_timestamp,
        };
        
        self.queue.add_operation(operation)
    }
    
    /// Process pending sync operations
    pub async fn process_sync_queue(&mut self) -> Result<usize, SyncError> {
        if self.is_syncing {
            return Ok(0); // Already syncing
        }
        
        self.is_syncing = true;
        let mut processed_count = 0;
        
        // Process all pending operations
        while let Some(operation) = self.queue.next_operation() {
            match self.process_operation(operation).await {
                Ok(_) => processed_count += 1,
                Err(e) => {
                    self.is_syncing = false;
                    return Err(e);
                }
            }
        }
        
        self.is_syncing = false;
        Ok(processed_count)
    }
    
    /// Process a single sync operation
    async fn process_operation(&mut self, operation: SyncOperation) -> Result<(), SyncError> {
        match operation {
            SyncOperation::UploadChanges { sheet_id: _, changes: _ } => {
                // In a real implementation, this would upload changes to the server
                // For now, we'll just simulate success
                Ok(())
            }
            SyncOperation::DownloadChanges { sheet_id: _, last_sync_timestamp: _ } => {
                // In a real implementation, this would download changes from the server
                // For now, we'll just simulate success
                Ok(())
            }
            SyncOperation::ResolveConflicts { sheet_id: _, conflicts } => {
                // Auto-resolve conflicts
                let _resolved = ConflictResolver::auto_resolve_conflicts(conflicts);
                Ok(())
            }
        }
    }
    
    /// Check if there are pending sync operations
    pub fn has_pending_syncs(&self) -> bool {
        self.queue.has_pending_operations()
    }
}

impl Default for SyncManager {
    fn default() -> Self {
        Self::new()
    }
}