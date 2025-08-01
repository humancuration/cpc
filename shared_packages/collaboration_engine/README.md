# Collaboration Engine

A shared package providing real-time document collaboration capabilities for the CPC ecosystem.

## Features

- **Real-time Collaboration**: Operational Transformation/CRDT algorithms for conflict-free editing
- **Presence Tracking**: Shared cursor positions and user activity monitoring
- **Conflict Resolution**: Automatic detection and resolution of editing conflicts
- **Version History**: Complete document versioning with branching and tagging
- **Event Integration**: Works with the CPC event bus for real-time updates
- **Storage Abstraction**: Integrates with CPC storage systems (Sled, PostgreSQL)
- **Schema Registry**: Manages event schemas and version transformations

## Architecture

The Collaboration Engine follows hexagonal architecture principles with clearly separated layers:

```
[User Interface] ↔ [Application Services] ↔ [Domain Model]
       ↑                    ↑                     ↑
       |                    |                     |
[Event Bus]         [Real-time Sync]        [Storage Abstraction]
[Presence Tracking] [Conflict Resolution]   [Version History]
```

## Modules

- `core`: Core OT/CRDT algorithms and document models
- `presence`: Shared cursor tracking and user presence
- `conflict_resolution`: Conflict detection and resolution strategies
- `versioning`: Document version history management
- `schema_registry`: Event schema management and versioning

## Installation

Add to your Cargo.toml:

```toml
[dependencies]
collaboration_engine = { path = "../shared_packages/collaboration_engine" }
```

## Usage

### Basic Document Creation

```rust
use collaboration_engine::core::Document;

let mut document = Document::new("Hello, World!".to_string());
```

### Real-time Collaboration

```rust
use collaboration_engine::core::CRDTDocument;

let mut crdt_doc = CRDTDocument::new();
```

### Presence Tracking

```rust
use collaboration_engine::presence::PresenceManager;
use collaboration_engine::core::Position;
use uuid::Uuid;

let mut presence_manager = PresenceManager::new(document_id);
let user_id = Uuid::new_v4();

// Update user presence
presence_manager.update_presence(
    user_id,
    Some(Position { line: 5, column: 10 }),
    None,
    true
).expect("Failed to update presence");
```

### Schema Registry

```rust
use collaboration_engine::schema_registry::SchemaRegistry;

let mut registry = SchemaRegistry::new();
```

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Domain Model](docs/DOMAIN.md)
- [Integration Guide](docs/INTEGRATION.md)
- [Usage Guide](docs/USAGE.md)
- [Schema Registry](docs/SCHEMA_REGISTRY.md)
- [Presence Management](docs/PRESENCE.md)

## Testing

Run tests with:

```bash
cargo test
```

## Integration Points

- **Event Bus**: For real-time messaging between users
- **Storage Abstraction**: For data persistence
- **Task Manager**: For collaborative task planning
- **Messenger**: For collaborative messaging

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.