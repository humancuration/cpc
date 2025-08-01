# Collaboration Engine Usage Guide

## Overview
This guide explains how to use the Collaboration Engine to implement real-time collaborative editing features in CPC applications.

## Getting Started

### Adding to Your Project
Add the collaboration engine to your Cargo.toml:

```toml
[dependencies]
collaboration_engine = { path = "../shared_packages/collaboration_engine" }
```

### Basic Document Creation
```rust
use collaboration_engine::core::Document;

// Create a new document
let mut document = Document::new("Hello, World!".to_string());

// Apply operations to the document
let operation = Operation::Insert {
    position: Position { line: 0, column: 5 },
    text: ",".to_string(),
    user_id: Uuid::new_v4(),
    timestamp: Utc::now(),
};

document.apply_operation(operation)?;
```

## Core Features

### Real-time Collaboration
The Collaboration Engine supports real-time collaborative editing using CRDT algorithms:

```rust
use collaboration_engine::core::CRDTDocument;

let mut crdt_doc = CRDTDocument::new();
let user_id = Uuid::new_v4();

// Apply operations from different users
let operation = Operation::Insert {
    position: Position { line: 0, column: 0 },
    text: "Hello".to_string(),
    user_id,
    timestamp: Utc::now(),
};

crdt_doc.apply_operation(user_id, operation)?;
```

### Presence Tracking
Track user presence and cursor positions in real-time:

```rust
use collaboration_engine::presence::{PresenceManager, Presence};

let document_id = Uuid::new_v4();
let mut presence_manager = PresenceManager::new(document_id);

let presence = Presence {
    user_id: Uuid::new_v4(),
    user_name: "Alice".to_string(),
    cursor_position: Position { line: 0, column: 0 },
    selection_start: None,
    selection_end: None,
    last_seen: Utc::now(),
    is_typing: false,
    color: "#FF0000".to_string(),
};

presence_manager.update_presence(presence);
```

### Conflict Resolution
Detect and resolve conflicts between concurrent operations:

```rust
use collaboration_engine::conflict_resolution::{ConflictResolver, ResolutionStrategy};

let document_id = Uuid::new_v4();
let mut conflict_resolver = ConflictResolver::new(document_id);

// Set user priorities for conflict resolution
conflict_resolver.set_user_priority(user_id, 10);

// Detect conflicts between operations
let conflicts = conflict_resolver.detect_conflicts(&operations);

// Resolve conflicts
for conflict in conflicts {
    conflict_resolver.add_conflict(conflict);
    conflict_resolver.resolve_conflict(conflict.id)?;
}
```

### Version History
Manage document versions and branching:

```rust
use collaboration_engine::versioning::VersionManager;

let document_id = Uuid::new_v4();
let mut version_manager = VersionManager::new(document_id);

// Create a new version
let version_number = version_manager.create_version(
    &document,
    author_id,
    "Author Name".to_string(),
    Some("Commit message".to_string()),
)?;

// Create branches and tags
version_manager.create_branch("feature-branch".to_string(), version_number)?;
version_manager.create_tag("v1.0".to_string(), version_number)?;
```

### Schema Management
Manage event schemas and version transformations:

```rust
use collaboration_engine::schema_registry::{SchemaRegistry, JsonSchema};

let mut registry = SchemaRegistry::new();

// Register a schema
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

registry.register_schema("UserMessage", "1.0.0", schema);

// Register a transformation function
registry.register_transformation("UserMessage", "1.0.0", "2.0.0", |payload| {
    Ok(serde_json::json!({
        "content": payload["message"].as_str().unwrap_or(""),
        "timestamp": Utc::now().to_rfc3339()
    }))
});

// Validate events
let event = DomainEvent::new_local(
    "collaboration".to_string(),
    "UserMessage".to_string(),
    serde_json::json!({"message": "Hello"}),
);

match registry.validate(&event) {
    Ok(()) => println!("Event is valid"),
    Err(e) => println!("Validation error: {}", e),
}

// Transform events
match registry.transform(&event, "2.0.0") {
    Ok(transformed) => println!("Transformed event: {}", transformed),
    Err(e) => println!("Transformation error: {}", e),
}
```

## Advanced Usage

### Custom Conflict Resolution Strategies
Implement custom conflict resolution strategies:

```rust
use collaboration_engine::conflict_resolution::{Conflict, ResolutionStrategy};

impl ConflictResolver {
    pub fn resolve_with_custom_strategy(&mut self, conflict: &Conflict) -> Result<(), CollaborationError> {
        match conflict.resolution_strategy {
            ResolutionStrategy::Merge => {
                // Implement custom merge logic
                // This could involve semantic merging based on document structure
                Ok(())
            }
            _ => self.resolve_conflict(conflict.id),
        }
    }
}
```

### Performance Optimization
For large documents, consider these optimization techniques:

1. **Operation Compression**: Compress operation history periodically
2. **Lazy Loading**: Load version history on demand
3. **Batching**: Batch operations to reduce network traffic

```rust
// Example of batching operations
fn apply_batch_operations(document: &mut Document, operations: Vec<Operation>) -> Result<(), CollaborationError> {
    for operation in operations {
        document.apply_operation(operation)?;
    }
    Ok(())
}
```

## Error Handling
The Collaboration Engine provides comprehensive error handling:

```rust
use collaboration_engine::core::CollaborationError;

match document.apply_operation(operation) {
    Ok(_) => println!("Operation applied successfully"),
    Err(CollaborationError::InvalidPosition) => println!("Invalid position in document"),
    Err(CollaborationError::InvalidRange) => println!("Invalid range in document"),
    Err(CollaborationError::OperationConflict) => println!("Operation conflict detected"),
    Err(CollaborationError::DocumentNotFound) => println!("Document not found"),
    Err(_) => println!("Unknown error occurred"),
}
```

## Testing
The Collaboration Engine includes comprehensive tests:

```bash
cd shared_packages/collaboration_engine
cargo test
```

## Best Practices

### For Real-time Collaboration
1. Use CRDT documents for distributed editing
2. Implement proper presence cleanup to avoid memory leaks
3. Use efficient serialization for network transmission

### For Conflict Resolution
1. Set appropriate user priorities based on role or expertise
2. Implement user-friendly conflict resolution UI
3. Log conflicts for analytics and debugging

### For Version Management
1. Create meaningful commit messages
2. Use branches for experimental features
3. Tag stable versions for easy reference

### For Schema Management
1. Register schemas for all event types
2. Implement transformation functions for backward compatibility
3. Mark deprecated fields but keep them for 2 major versions
4. Validate events before processing to ensure data integrity