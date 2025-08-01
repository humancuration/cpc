//! Real-time synchronization with collaboration engine

use crate::crdt::operations::DocumentOperation;
use crate::domain::errors::DocumentError;
use crate::collaboration::conversion; // Import our conversion utilities
use collaboration_engine::{ConflictResolver, PresenceManager};
use collaboration_engine::core::{Operation, Position};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Manages real-time synchronization between document operations and collaboration engine
pub struct SyncManager {
    document_id: Uuid,
    conflict_resolver: Arc<Mutex<ConflictResolver>>,
    presence_manager: Arc<Mutex<PresenceManager>>,
    operations: Arc<Mutex<Vec<DocumentOperation>>>,
}

impl SyncManager {
    /// Create a new synchronization manager
    pub fn new(document_id: Uuid) -> Self {
        let conflict_resolver = Arc::new(Mutex::new(ConflictResolver::new(document_id)));
        let presence_manager = Arc::new(Mutex::new(PresenceManager::new(document_id)));
        let operations = Arc::new(Mutex::new(Vec::new()));
        
        Self {
            document_id,
            conflict_resolver,
            presence_manager,
            operations,
        }
    }
    
    /// Apply a document operation to the collaboration engine
    pub fn apply_operation(&self, doc_op: DocumentOperation) -> Result<(), DocumentError> {
        // Convert document operation to collaboration engine operation
        let engine_op: Operation = doc_op.clone().into();
        
        // Store the operation
        {
            let mut ops = self.operations.lock().unwrap();
            ops.push(doc_op);
        }
        
        // Detect conflicts
        let conflicts = {
            let resolver = self.conflict_resolver.lock().unwrap();
            resolver.detect_conflicts(&[engine_op.clone()])
        };
        
        // Resolve conflicts if any
        if !conflicts.is_empty() {
            let mut resolver = self.conflict_resolver.lock().unwrap();
            for conflict in conflicts {
                resolver.add_conflict(conflict.clone());
                // In a real implementation, we would resolve the conflict
                // For now, we'll just log it
                tracing::debug!("Detected conflict: {:?}", conflict.id);
            }
        }
        
        // Update presence if this is a cursor movement
        match &engine_op {
            Operation::Insert { position, user_id, .. } |
            Operation::Delete { start: position, user_id, .. } |
            Operation::Replace { start: position, .., user_id, .. } => {
                let mut presence = self.presence_manager.lock().unwrap();
                let _ = presence.update_presence(
                    *user_id,
                    Some(position.clone()),
                    None, // No selection for now
                    true, // Assume user is typing when applying operations
                    0,    // Highest QoS tier
                );
            }
        }
        
        Ok(())
    }
    
    /// Get current presence information for all users
    pub fn get_presences(&self) -> Vec<collaboration_engine::presence::UserPresence> {
        let presence = self.presence_manager.lock().unwrap();
        presence.get_presences()
    }
    
    /// Update a user's presence information
    pub fn update_presence(
        &self,
        user_id: Uuid,
        cursor: Option<Position>,
        selection: Option<(Position, Position)>,
        is_typing: bool,
    ) -> Result<(), DocumentError> {
        let mut presence = self.presence_manager.lock().unwrap();
        presence.update_presence(user_id, cursor, selection, is_typing, 0) // QoS tier 0 (highest)
            .map_err(|e| DocumentError::SyncError(format!("Failed to update presence: {:?}", e)))
    }
    
    /// Remove a user's presence (when they leave the document)
    pub fn remove_presence(&self, user_id: Uuid) -> Result<(), DocumentError> {
        let mut presence = self.presence_manager.lock().unwrap();
        presence.remove_presence(user_id)
            .map_err(|e| DocumentError::SyncError(format!("Failed to remove presence: {:?}", e)))
    }
    
    /// Get all operations applied to this document
    pub fn get_operations(&self) -> Vec<DocumentOperation> {
        let ops = self.operations.lock().unwrap();
        ops.clone()
    }
    
    /// Set user priority for conflict resolution
    pub fn set_user_priority(&self, user_id: Uuid, priority: i32) {
        let mut resolver = self.conflict_resolver.lock().unwrap();
        resolver.set_user_priority(user_id, priority);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crdt::id::CRDTId;
    use chrono::Utc;
    
    #[test]
    fn test_sync_manager_creation() {
        let document_id = Uuid::new_v4();
        let sync_manager = SyncManager::new(document_id);
        
        assert_eq!(sync_manager.document_id, document_id);
    }
    
    #[test]
    fn test_apply_operation() {
        let document_id = Uuid::new_v4();
        let sync_manager = SyncManager::new(document_id);
        let user_id = Uuid::new_v4();
        
        // Create a simple insert operation
        let doc_op = DocumentOperation::Insert {
            position: 0,
            value: serde_json::Value::String("Hello".to_string()),
            id: CRDTId::new(user_id, 1234567890),
            parent_id: None,
        };
        
        // Apply the operation
        assert!(sync_manager.apply_operation(doc_op).is_ok());
        
        // Check that the operation was stored
        let operations = sync_manager.get_operations();
        assert_eq!(operations.len(), 1);
    }
    
    #[test]
    fn test_presence_updates() {
        let document_id = Uuid::new_v4();
        let sync_manager = SyncManager::new(document_id);
        let user_id = Uuid::new_v4();
        let position = Position { line: 0, column: 5 };
        
        // Update user presence
        assert!(sync_manager.update_presence(user_id, Some(position.clone()), None, true).is_ok());
        
        // Check that presence was updated
        let presences = sync_manager.get_presences();
        assert_eq!(presences.len(), 1);
        assert_eq!(presences[0].user_id, user_id);
        assert_eq!(presences[0].cursor, Some(position));
        assert!(presences[0].is_typing);
    }
}