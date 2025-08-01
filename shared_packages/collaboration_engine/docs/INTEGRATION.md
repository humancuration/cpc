# Collaboration Engine Integration Guide

## Overview
The Collaboration Engine integrates with several CPC systems to provide real-time collaborative editing capabilities across applications.

## Integration Points

### Event Bus Integration
The Collaboration Engine uses the event bus for real-time messaging between users:

**Published Events:**
- `OperationApplied` - When an operation is applied to a document
- `PresenceUpdated` - When a user's presence changes
- `ConflictDetected` - When a conflict is detected
- `VersionCreated` - When a new document version is created
- `SchemaRegistered` - When a new schema is registered
- `SchemaTransformed` - When an event is transformed between versions

**Subscribed Events:**
- `UserJoinedDocument` - When a user joins a collaborative session
- `UserLeftDocument` - When a user leaves a collaborative session
- `RemoteOperation` - Operations from other users

**Usage:**
To use the event bus integration, you need to set the event bus for each component:

```rust
use collaboration_engine::{core::Document, presence::PresenceManager};
use event_bus::EventBus;

// Create an event bus instance
let event_bus = EventBus::new(storage);

// Set the event bus for a document
let mut document = Document::new("Initial content".to_string());
document.set_event_bus(event_bus.clone());

// Set the event bus for a presence manager
let mut presence_manager = PresenceManager::new(document.id);
presence_manager.set_event_bus(event_bus.clone());

// Set the event bus for a conflict resolver
let mut conflict_resolver = ConflictResolver::new(document.id);
conflict_resolver.set_event_bus(event_bus.clone());

// Set the event bus for a version manager
let mut version_manager = VersionManager::new(document.id);
version_manager.set_event_bus(event_bus.clone());
```

**Event Handling:**
To handle events from other users, you can subscribe to specific event types:

```rust
use event_bus::domain::subscription::EventFilter;

// Subscribe to remote operations
let filter = EventFilter {
    domain: Some("collaboration".to_string()),
    event_types: vec!["RemoteOperation".to_string()],
    user_id: None,
};

let mut subscription = event_bus.subscribe(filter).await;

// Handle events in a loop
while let Some(event) = subscription.recv().await {
    match event.event_type.as_str() {
        "RemoteOperation" => {
            // Handle remote operation
            if let Ok(operation) = serde_json::from_value::<Operation>(event.payload) {
                document.handle_remote_operation(operation)?;
            }
        },
        "UserJoinedDocument" => {
            // Handle user joining
            if let Some(user_id) = event.payload.get("user_id") {
                if let Some(user_id) = user_id.as_str() {
                    if let (Some(user_id), Some(user_name)) = (
                        event.payload.get("user_id").and_then(|v| v.as_str()),
                        event.payload.get("user_name").and_then(|v| v.as_str()),
                    ) {
                        document.handle_user_joined(
                            Uuid::parse_str(user_id)?,
                            user_name.to_string()
                        )?;
                    }
                }
            }
        },
        _ => {
            // Handle other events
        }
    }
}
```

### Storage Abstraction Integration
The Collaboration Engine uses the storage abstraction for data persistence:

**Storage Requirements:**
- Document metadata and content
- Operation history for conflict resolution
- Presence information
- Version history

**Storage Strategies:**
- Sled for local caching and offline editing
- PostgreSQL for cloud synchronization
- In-memory for testing

### Task Manager Integration
The Collaboration Engine can be used by the Task Manager for collaborative task planning:

**Use Cases:**
- Collaborative task description editing
- Shared task notes
- Real-time task assignment updates

### Messenger Integration
The Collaboration Engine integrates with Messenger for collaborative messaging:

**Use Cases:**
- Collaborative document sharing in chats
- Real-time editing of shared notes
- Comment threads on documents

## API Usage

### Initializing Collaboration
```rust
use collaboration_engine::{core::Document, presence::PresenceManager};

let document = Document::new("Initial content".to_string());
let mut presence_manager = PresenceManager::new(document.id);
```

### Applying Operations
```rust
use collaboration_engine::core::{Operation, Position};

let operation = Operation::Insert {
    position: Position { line: 0, column: 0 },
    text: "Hello".to_string(),
    user_id: user_id,
    timestamp: Utc::now(),
};

document.apply_operation(operation)?;
```

### Managing Presence
```rust
use collaboration_engine::presence::Presence;

let presence = Presence {
    user_id,
    user_name: "Alice".to_string(),
    cursor_position: Position { line: 0, column: 5 },
    selection_start: None,
    selection_end: None,
    last_seen: Utc::now(),
    is_typing: true,
    color: "#FF0000".to_string(),
};

presence_manager.update_presence(presence);
```

## Performance Considerations

### Network Efficiency
- Batch operations to reduce network traffic
- Use delta compression for operation transmission
- Implement operation garbage collection

### Storage Optimization
- Store only necessary operation history
- Use efficient serialization formats
- Implement lazy loading for version history

### Real-time Updates
- Use efficient data structures for presence tracking
- Implement throttling for cursor updates
- Prioritize critical operations over cosmetic updates

### Schema Registry Integration
The Schema Registry provides event validation and versioning capabilities:

**Use Cases:**
- Validating events before processing
- Transforming events between different versions
- Managing schema deprecation

**Usage:**
```rust
use collaboration_engine::schema_registry::{SchemaRegistry, JsonSchema};
use event_bus::DomainEvent;

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

// Validate an event
let event = DomainEvent::new_local(
    "collaboration".to_string(),
    "UserMessage".to_string(),
    serde_json::json!({"message": "Hello"}),
);

match registry.validate(&event) {
    Ok(()) => println!("Event is valid"),
    Err(e) => println!("Validation error: {}", e),
}
```