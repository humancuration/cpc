//! Integration tests for real-time synchronization

use cpc_document_editor::collaboration::sync::SyncManager;
use cpc_document_editor::crdt::operations::{DocumentOperation, FormatType};
use cpc_document_editor::crdt::id::CRDTId;
use collaboration_engine::core::Position;
use uuid::Uuid;

#[test]
fn test_sync_manager_apply_operation() {
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
fn test_sync_manager_presence_updates() {
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

#[test]
fn test_sync_manager_multiple_users() {
    let document_id = Uuid::new_v4();
    let sync_manager = SyncManager::new(document_id);
    
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    let position1 = Position { line: 0, column: 5 };
    let position2 = Position { line: 1, column: 10 };
    
    // Update presence for both users
    assert!(sync_manager.update_presence(user1_id, Some(position1.clone()), None, true).is_ok());
    assert!(sync_manager.update_presence(user2_id, Some(position2.clone()), None, false).is_ok());
    
    // Check that both presences were updated
    let presences = sync_manager.get_presences();
    assert_eq!(presences.len(), 2);
    
    // Find each user's presence
    let user1_presence = presences.iter().find(|p| p.user_id == user1_id).unwrap();
    let user2_presence = presences.iter().find(|p| p.user_id == user2_id).unwrap();
    
    assert_eq!(user1_presence.cursor, Some(position1));
    assert!(user1_presence.is_typing);
    
    assert_eq!(user2_presence.cursor, Some(position2));
    assert!(!user2_presence.is_typing);
}

#[test]
fn test_sync_manager_conflict_detection() {
    let document_id = Uuid::new_v4();
    let sync_manager = SyncManager::new(document_id);
    let user_id = Uuid::new_v4();
    
    // Create conflicting operations (same position)
    let doc_op1 = DocumentOperation::Insert {
        position: 0,
        value: serde_json::Value::String("A".to_string()),
        id: CRDTId::new(user_id, 1234567890),
        parent_id: None,
    };
    
    let doc_op2 = DocumentOperation::Insert {
        position: 0,
        value: serde_json::Value::String("B".to_string()),
        id: CRDTId::new(user_id, 1234567891),
        parent_id: None,
    };
    
    // Apply both operations
    assert!(sync_manager.apply_operation(doc_op1).is_ok());
    assert!(sync_manager.apply_operation(doc_op2).is_ok());
    
    // Check that both operations were stored
    let operations = sync_manager.get_operations();
    assert_eq!(operations.len(), 2);
}

#[test]
fn test_sync_manager_format_operations() {
    let document_id = Uuid::new_v4();
    let sync_manager = SyncManager::new(document_id);
    let user_id = Uuid::new_v4();
    
    // Create a format operation
    let doc_op = DocumentOperation::Format {
        range: (Position { line: 0, column: 0 }, Position { line: 0, column: 5 }),
        format: FormatType::Bold,
    };
    
    // Apply the operation
    assert!(sync_manager.apply_operation(doc_op).is_ok());
    
    // Check that the operation was stored
    let operations = sync_manager.get_operations();
    assert_eq!(operations.len(), 1);
}

#[test]
fn test_sync_manager_user_priority() {
    let document_id = Uuid::new_v4();
    let sync_manager = SyncManager::new(document_id);
    let user_id = Uuid::new_v4();
    
    // Set user priority
    sync_manager.set_user_priority(user_id, 10);
    
    // Create an operation and apply it
    let doc_op = DocumentOperation::Insert {
        position: 0,
        value: serde_json::Value::String("Hello".to_string()),
        id: CRDTId::new(user_id, 1234567890),
        parent_id: None,
    };
    
    assert!(sync_manager.apply_operation(doc_op).is_ok());
    
    // Check that the operation was stored
    let operations = sync_manager.get_operations();
    assert_eq!(operations.len(), 1);
}

#[test]
fn test_sync_manager_remove_presence() {
    let document_id = Uuid::new_v4();
    let sync_manager = SyncManager::new(document_id);
    let user_id = Uuid::new_v4();
    let position = Position { line: 0, column: 5 };
    
    // Update user presence
    assert!(sync_manager.update_presence(user_id, Some(position), None, true).is_ok());
    
    // Check that presence was updated
    let presences = sync_manager.get_presences();
    assert_eq!(presences.len(), 1);
    
    // Remove user presence
    assert!(sync_manager.remove_presence(user_id).is_ok());
    
    // Check that presence was removed
    let presences = sync_manager.get_presences();
    assert_eq!(presences.len(), 0);
}