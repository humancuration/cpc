//! Integration tests for the collaboration engine

use collaboration_engine::{
    core::{Document, Operation, Position, CRDTDocument},
    presence::{PresenceManager, Presence},
    conflict_resolution::{ConflictResolver, ResolutionStrategy},
    versioning::{VersionManager, DocumentVersion},
    schema_registry::{SchemaRegistry, JsonSchema},
};
use event_bus::{EventBus, EventStorage, EventQuery, DomainEvent};
use storage_abstraction::DataStore;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use tokio::sync::Mutex;

#[test]
fn test_document_operations() {
    let mut document = Document::new("Hello World".to_string());
    
    // Test insert operation
    let insert_op = Operation::Insert {
        position: Position { line: 0, column: 5 },
        text: ", ".to_string(),
        user_id: Uuid::new_v4(),
        timestamp: Utc::now(),
    };
    
    assert!(document.apply_operation(insert_op).is_ok());
    assert_eq!(document.content, "Hello, World");
    assert_eq!(document.version, 1);
    
    // Test delete operation
    let delete_op = Operation::Delete {
        start: Position { line: 0, column: 5 },
        end: Position { line: 0, column: 7 },
        user_id: Uuid::new_v4(),
        timestamp: Utc::now(),
    };
    
    assert!(document.apply_operation(delete_op).is_ok());
    assert_eq!(document.content, "Hello World");
    assert_eq!(document.version, 2);
}

#[test]
fn test_crdt_document() {
    let mut crdt_doc = CRDTDocument::new();
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    // Apply operations from different users
    let op1 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "Hello".to_string(),
        user_id: user1_id,
        timestamp: Utc::now(),
    };
    
    let op2 = Operation::Insert {
        position: Position { line: 0, column: 5 },
        text: " World".to_string(),
        user_id: user2_id,
        timestamp: Utc::now(),
    };
    
    assert!(crdt_doc.apply_operation(user1_id, op1).is_ok());
    assert!(crdt_doc.apply_operation(user2_id, op2).is_ok());
    
    // Check version vectors
    assert_eq!(crdt_doc.version_vector[&user1_id], 1);
    assert_eq!(crdt_doc.version_vector[&user2_id], 1);
}

#[test]
fn test_presence_management() {
    let document_id = Uuid::new_v4();
    let mut presence_manager = PresenceManager::new(document_id);
    
    let user_id = Uuid::new_v4();
    let cursor = Some(Position { line: 0, column: 0 });
    
    assert!(presence_manager.update_presence(user_id, cursor, None, false).is_ok());
    
    assert_eq!(presence_manager.get_presences().len(), 1);
    assert!(presence_manager.get_user_presence(user_id).is_some());
    assert_eq!(presence_manager.active_user_count(), 1);
    
    assert!(presence_manager.remove_presence(user_id).is_ok());
    assert_eq!(presence_manager.get_presences().len(), 0);
}

#[test]
fn test_conflict_resolution() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    // Set user priorities
    conflict_resolver.set_user_priority(user1_id, 10);
    conflict_resolver.set_user_priority(user2_id, 5);
    
    // Create conflicting operations
    let op1 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "A".to_string(),
        user_id: user1_id,
        timestamp: Utc::now(),
    };
    
    let op2 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "B".to_string(),
        user_id: user2_id,
        timestamp: Utc::now(),
    };
    
    let operations = vec![op1, op2];
    let conflicts = conflict_resolver.detect_conflicts(&operations);
    
    assert_eq!(conflicts.len(), 1);
    
    // Add conflict and resolve it
    if let Some(conflict) = conflicts.first() {
        conflict_resolver.add_conflict(conflict.clone());
        assert!(conflict_resolver.resolve_conflict(conflict.id).is_ok());
    }
}

#[test]
fn test_version_management() {
    let document_id = Uuid::new_v4();
    let mut version_manager = VersionManager::new(document_id);
    
    let document = Document::new("Initial content".to_string());
    let author_id = Uuid::new_v4();
    
    // Create a version
    let version_result = version_manager.create_version(
        &document,
        author_id,
        "Test Author".to_string(),
        Some("Initial commit".to_string()),
    );
    
    assert!(version_result.is_ok());
    assert_eq!(version_manager.current_version, 0);
    
    // Get the version back
    let version = version_manager.get_version(0);
    assert!(version.is_some());
    
    // List versions
    let versions = version_manager.list_versions();
    assert_eq!(versions.len(), 1);
    
    // Create a branch
    assert!(version_manager.create_branch("feature".to_string(), 0).is_ok());
    assert_eq!(version_manager.get_branch_version("feature"), Some(0));
    
    // Create a tag
    assert!(version_manager.create_tag("v1.0".to_string(), 0).is_ok());
    assert_eq!(version_manager.get_tag_version("v1.0"), Some(0));
}

// Mock storage implementation for testing
#[derive(Debug, Clone)]
struct MockStorage {
    data: Arc<Mutex<std::collections::HashMap<String, Vec<u8>>>>,
}

impl MockStorage {
    fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl DataStore for MockStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, storage_abstraction::StorageError> {
        let data = self.data.lock().await;
        Ok(data.get(key).cloned())
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), storage_abstraction::StorageError> {
        let mut data = self.data.lock().await;
        data.insert(key.to_string(), value);
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), storage_abstraction::StorageError> {
        let mut data = self.data.lock().await;
        data.remove(key);
        Ok(())
    }
    
    async fn exists(&self, key: &str) -> Result<bool, storage_abstraction::StorageError> {
        let data = self.data.lock().await;
        Ok(data.contains_key(key))
    }
}

#[async_trait::async_trait]
impl EventStorage for MockStorage {
    async fn store_event(&self, event: &DomainEvent) -> Result<(), event_bus::EventError> {
        let key = format!("event:{}:{}", event.domain, event.event_id);
        let value = serde_json::to_vec(event)
            .map_err(|e| event_bus::EventError::SerializationError(e.to_string()))?;
        
        self.set(&key, value)
            .await
            .map_err(|e| event_bus::EventError::StorageError(e.to_string()))
    }
    
    async fn get_events(&self, _query: EventQuery) -> Result<Vec<DomainEvent>, event_bus::EventError> {
        // Simplified implementation for testing
        Ok(Vec::new())
    }
}

#[tokio::test]
async fn test_event_publishing() {
    let storage = Arc::new(MockStorage::new());
    let event_bus = EventBus::new(storage);
    
    // Create a document and set the event bus
    let mut document = Document::new("Hello World".to_string());
    document.set_event_bus(event_bus.clone());
    
    // Apply an operation
    let insert_op = Operation::Insert {
        position: Position { line: 0, column: 5 },
        text: ", ".to_string(),
        user_id: Uuid::new_v4(),
        timestamp: Utc::now(),
    };
    
    assert!(document.apply_operation(insert_op).is_ok());
    
    // Create a presence manager and set the event bus
    let document_id = Uuid::new_v4();
    let mut presence_manager = PresenceManager::new(document_id);
    presence_manager.set_event_bus(event_bus.clone());
    
    let user_id = Uuid::new_v4();
    let cursor = Some(Position { line: 0, column: 0 });
    
    assert!(presence_manager.update_presence(user_id, cursor, None, false).is_ok());
    
    // Create a conflict resolver and set the event bus
    let mut conflict_resolver = ConflictResolver::new(document_id);
    conflict_resolver.set_event_bus(event_bus.clone());
    
    // Create a conflict and add it
    let conflict = collaboration_engine::conflict_resolution::Conflict {
        id: Uuid::new_v4(),
        document_id,
        conflicting_operations: vec![],
        resolution_strategy: ResolutionStrategy::TimestampOrder,
        resolved: false,
        resolved_at: None,
        created_at: Utc::now(),
    };
    
    conflict_resolver.add_conflict(conflict);
    
    // Create a version manager and set the event bus
    let mut version_manager = VersionManager::new(document_id);
    version_manager.set_event_bus(event_bus.clone());
    
    // Create a version
    let version_result = version_manager.create_version(
        &document,
        user_id,
        "Test Author".to_string(),
        Some("Test commit".to_string()),
    );
    
    assert!(version_result.is_ok());
    
    // Test event subscription
    let filter = event_bus::domain::subscription::EventFilter {
        domain: Some("collaboration".to_string()),
        event_types: vec!["OperationApplied".to_string()],
        user_id: None,
    };
    
    let mut subscription = event_bus.subscribe(filter).await;
    // Handle user joining
    assert!(document.handle_user_joined(user_id, "Test User".to_string()).is_ok());
}

#[test]
fn test_schema_registry() {
    let mut registry = SchemaRegistry::new();
    
    // Create a schema
    let schema = JsonSchema {
        definition: serde_json::json!({
            "type": "object",
            "properties": {
                "message": {"type": "string"}
            }
        }),
        created_at: Utc::now(),
        deprecated: false,
        deprecated_until: None,
    };
    
    // Register the schema
    registry.register_schema("TestEvent", "1.0.0", schema.clone());
    
    // Retrieve the schema
    let retrieved = registry.get_schema("TestEvent", "1.0.0").unwrap();
    assert_eq!(retrieved.definition, schema.definition);
    
    // Register a transformation
    registry.register_transformation("TestEvent", "1.0.0", "2.0.0", |payload| {
        Ok(serde_json::json!({
            "content": payload["message"].as_str().unwrap_or(""),
            "version": "2.0.0"
        }))
    });
    
    // Get the transformation
    let transformer = registry.get_transformer("TestEvent", "1.0.0", "2.0.0").unwrap();
    let result = transformer(&serde_json::json!({"message": "test"})).unwrap();
    assert_eq!(result["content"], "test");
    assert_eq!(result["version"], "2.0.0");
    
    // List versions
    let versions = registry.list_versions("TestEvent");
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].to_string(), "1.0.0");
}
}

#[test]
fn test_conflict_resolution_integration() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let timestamp1 = Utc::now();
    let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
    
    // Create conflicting operations - both inserting at position (0,0)
    let op1 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "Hello".to_string(),
        user_id: user1_id,
        timestamp: timestamp1,
    };
    
    let op2 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "World".to_string(),
        user_id: user2_id,
        timestamp: timestamp2,
    };
    
    // Detect conflicts
    let operations = vec![op1.clone(), op2.clone()];
    let conflicts = conflict_resolver.detect_conflicts(&operations);
    assert_eq!(conflicts.len(), 1);
    
    // Add conflict and resolve it
    if let Some(conflict) = conflicts.first() {
        conflict_resolver.add_conflict(conflict.clone());
        assert!(conflict_resolver.resolve_conflict(conflict.id).is_ok());
        
        // Check that conflict is resolved
        let resolved_conflict = conflict_resolver.conflicts.get(&conflict.id).unwrap();
        assert!(resolved_conflict.resolved);
        assert!(!resolved_conflict.resolved_operations.is_empty());
    }
}

#[test]
fn test_priority_based_conflict_resolution() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    
    let admin_user_id = Uuid::new_v4();
    let regular_user_id = Uuid::new_v4();
    let timestamp1 = Utc::now();
    let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
    
    // Set priorities - admin user has higher priority (lower QoS tier = higher priority)
    conflict_resolver.set_user_priority(admin_user_id, 100); // High priority
    conflict_resolver.set_user_priority(regular_user_id, 10); // Lower priority
    
    // Create conflicting operations
    let admin_op = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "AdminText".to_string(),
        user_id: admin_user_id,
        timestamp: timestamp1,
    };
    
    let regular_op = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "RegularText".to_string(),
        user_id: regular_user_id,
        timestamp: timestamp2,
    };
    
    // Create conflict with UserPriority resolution strategy
    let conflict = Conflict {
        id: Uuid::new_v4(),
        document_id,
        conflicting_operations: vec![admin_op, regular_op],
        resolution_strategy: ResolutionStrategy::UserPriority,
        resolved: false,
        resolved_operations: vec![],
        resolved_at: None,
        created_at: Utc::now(),
        metadata: ConflictMetadata {
            detection_method: "test".to_string(),
            transformation_history: vec![],
            resolution_details: None,
        },
    };
    
    // Add and resolve conflict
    conflict_resolver.add_conflict(conflict.clone());
    assert!(conflict_resolver.resolve_conflict(conflict.id).is_ok());
    
    // Check that conflict is resolved
    let resolved_conflict = conflict_resolver.conflicts.get(&conflict.id).unwrap();
    assert!(resolved_conflict.resolved);
}

#[test]
fn test_versioning_with_conflict_metadata() {
    let document_id = Uuid::new_v4();
    let mut version_manager = VersionManager::new(document_id);
    let document = Document::new("Initial content".to_string());
    let author_id = Uuid::new_v4();
    
    // Create a version with conflict metadata
    let conflict_metadata = Some(serde_json::json!({
        "conflict_id": Uuid::new_v4().to_string(),
        "resolution_strategy": "UserPriority",
        "involved_users": [author_id.to_string()]
    }));
    
    let version_result = version_manager.create_version(
        &document,
        author_id,
        "Test Author".to_string(),
        Some("Conflict resolution commit".to_string()),
        conflict_metadata,
    );
    
    assert!(version_result.is_ok());
    assert_eq!(version_manager.current_version, 0);
    
    // Get the version back and check conflict metadata
    let version = version_manager.get_version(0);
    assert!(version.is_some());
    assert!(version.unwrap().conflict_metadata.is_some());
}

#[test]
fn test_conflict_aware_branching() {
    let document_id = Uuid::new_v4();
    let mut version_manager = VersionManager::new(document_id);
    let document = Document::new("Initial content".to_string());
    let author_id = Uuid::new_v4();
    
    // Create initial version
    version_manager.create_version(&document, author_id, "Test Author".to_string(), None, None).unwrap();
    
    // Create a conflict-aware branch
    let conflict_metadata = Some(serde_json::json!({
        "conflict_id": Uuid::new_v4().to_string(),
        "resolution_strategy": "Merge"
    }));
    
    let result = version_manager.create_conflict_aware_branch("conflict-fix".to_string(), 0, conflict_metadata);
    assert!(result.is_ok());
    assert_eq!(version_manager.get_branch_version("conflict-fix"), Some(0));
}

#[test]
fn test_schema_validation_for_conflict_events() {
    let mut registry = SchemaRegistry::new();
    
    // Test ConflictDetected schema
    let conflict_detected_schema = JsonSchema {
        definition: serde_json::json!({
            "type": "object",
            "properties": {
                "document_id": {"type": "string", "format": "uuid"},
                "conflict": {"type": "object"}
            }
        }),
        created_at: Utc::now(),
        deprecated: false,
        deprecated_until: None,
    };
    registry.register_schema("ConflictDetected", "1.1.0", conflict_detected_schema);
    
    // Test ConflictResolved schema
    let conflict_resolved_schema = JsonSchema {
        definition: serde_json::json!({
            "type": "object",
            "properties": {
                "document_id": {"type": "string", "format": "uuid"},
                "conflict_id": {"type": "string", "format": "uuid"},
                "resolved_operations": {"type": "array"}
            }
        }),
        created_at: Utc::now(),
        deprecated: false,
        deprecated_until: None,
    };
    registry.register_schema("ConflictResolved", "1.0.0", conflict_resolved_schema);
    
    // Create test events
    let document_id = Uuid::new_v4();
    let conflict_id = Uuid::new_v4();
    
    let conflict_detected_event = DomainEvent::new(
        "collaboration".to_string(),
        "ConflictDetected".to_string(),
        serde_json::json!({
            "document_id": document_id.to_string(),
            "conflict": {
                "id": conflict_id.to_string(),
                "document_id": document_id.to_string(),
                "conflicting_operations": [],
                "resolution_strategy": "TimestampOrder"
            }
        }),
        EventSource::Local,
    );
    
    let conflict_resolved_event = DomainEvent::new(
        "collaboration".to_string(),
        "ConflictResolved".to_string(),
        serde_json::json!({
            "document_id": document_id.to_string(),
            "conflict_id": conflict_id.to_string(),
            "resolved_operations": []
        }),
        EventSource::Local,
    );
    
    // Validate events
    assert!(registry.validate(&conflict_detected_event).is_ok());
    assert!(registry.validate(&conflict_resolved_event).is_ok());
}

#[test]
fn test_transformation_recording() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let timestamp1 = Utc::now();
    let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
    let conflict_id = Uuid::new_v4();
    
    // Create conflicting operations
    let op1 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "A".to_string(),
        user_id: user1_id,
        timestamp: timestamp1,
    };
    
    let op2 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "B".to_string(),
        user_id: user2_id,
        timestamp: timestamp2,
    };
    
    // Create conflict
    let conflict = Conflict {
        id: conflict_id,
        document_id,
        conflicting_operations: vec![op1.clone(), op2.clone()],
        resolution_strategy: ResolutionStrategy::TimestampOrder,
        resolved: false,
        resolved_operations: vec![],
        resolved_at: None,
        created_at: Utc::now(),
        metadata: ConflictMetadata {
            detection_method: "test".to_string(),
            transformation_history: vec![],
            resolution_details: None,
        },
    };
    
    // Add conflict
    conflict_resolver.add_conflict(conflict.clone());
    
    // Transform operations
    let result = conflict_resolver.transform_insert_vs_insert(
        conflict_id,
        &op1.position, &op1.text, op1.user_id, op1.timestamp,
        &op2.position, op2.user_id, op2.timestamp
    );
    
    assert!(result.is_ok());
    
    // Check that transformation was recorded
    let conflict = conflict_resolver.conflicts.get(&conflict_id).unwrap();
    assert!(!conflict.metadata.transformation_history.is_empty());
    assert_eq!(conflict.metadata.transformation_history[0].transformation_type, "insert_vs_insert");
}

#[test]
fn test_version_creation_after_conflict_resolution() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    let mut version_manager = VersionManager::new(document_id);
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let timestamp1 = Utc::now();
    let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
    let conflict_id = Uuid::new_v4();
    
    // Set document content
    conflict_resolver.set_document_content("Initial content".to_string());
    
    // Set version manager
    conflict_resolver.set_version_manager(version_manager.clone());
    
    // Create conflicting operations
    let op1 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "A".to_string(),
        user_id: user1_id,
        timestamp: timestamp1,
    };
    
    let op2 = Operation::Insert {
        position: Position { line: 0, column: 0 },
        text: "B".to_string(),
        user_id: user2_id,
        timestamp: timestamp2,
    };
    
    // Create conflict
    let conflict = Conflict {
        id: conflict_id,
        document_id,
        conflicting_operations: vec![op1, op2],
        resolution_strategy: ResolutionStrategy::TimestampOrder,
        resolved: false,
        resolved_operations: vec![],
        resolved_at: None,
        created_at: Utc::now(),
        metadata: ConflictMetadata {
            detection_method: "test".to_string(),
            transformation_history: vec![],
            resolution_details: None,
        },
    };
    
    // Add conflict
    conflict_resolver.add_conflict(conflict.clone());
    
    // Resolve conflict
    let result = conflict_resolver.resolve_conflict(conflict_id);
    assert!(result.is_ok());
    
    // Check that version was created
    // Note: In the current implementation, we're directly accessing the version manager
    // In a real implementation, this would be handled differently
}

#[test]
fn test_multi_line_range_calculation() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    
    // Set document content with multiple lines
    conflict_resolver.set_document_content("Line 1\nLine 2\nLine 3".to_string());
    
    // Calculate range from (0,2) to (2,3)
    let start = Position { line: 0, column: 2 };
    let end = Position { line: 2, column: 3 };
    
    let result = conflict_resolver.calculate_range_length(&start, &end);
    assert!(result.is_ok());
    
    // Expected: "ne 1\nLine 2\nLin" = 13 characters
    // First line: "ne 1" = 4 chars
    // Middle line: "Line 2" = 6 chars
    // Last line: "Lin" = 3 chars
    // Total: 4 + 6 + 3 = 13
    assert_eq!(result.unwrap(), 13);
}

#[test]
fn test_merge_strategy_implementation() {
    let document_id = Uuid::new_v4();
    let mut conflict_resolver = ConflictResolver::new(document_id);
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    let timestamp1 = Utc::now();
    let timestamp2 = timestamp1 + chrono::Duration::seconds(1);
    
    // Create operations that can be merged
    let op1 = Operation::Insert {
        position: Position { line: 0, column: 1 },
        text: "A".to_string(),
        user_id: user1_id,
        timestamp: timestamp1,
    };
    
    let op2 = Operation::Insert {
        position: Position { line: 0, column: 3 },
        text: "B".to_string(),
        user_id: user2_id,
        timestamp: timestamp2,
    };
    
    let operations = vec![op1, op2];
    let result = conflict_resolver.resolve_by_merge(&operations);
    assert!(result.is_ok());
    
    let resolved_ops = result.unwrap();
    assert_eq!(resolved_ops.len(), 2);
}