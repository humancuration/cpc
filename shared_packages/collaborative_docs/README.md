# Collaborative Documentation System

A Rust-based collaborative documentation system with CRDT support, versioning, and access control.

## Features

- **CRDT-based Collaboration**: Real-time collaborative editing using Automerge
- **Version Control**: Full document history with diff tracking
- **Access Control**: Consent-based permissions system
- **Multiple Storage Backends**: PostgreSQL and Sled support
- **Content Provider Integration**: Works with the social graph content provider system
- **Hexagonal Architecture**: Clean separation of concerns with mockable components

## Architecture

The system follows a hexagonal architecture with the following components:

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Services                     │
├─────────────────────────────────────────────────────────────┤
│                    CollaborativeDocService                  │
├─────────────────────────────────────────────────────────────┤
│  Core     │  CRDT     │  Versioning  │  Access   │  Storage │
│           │           │              │           │          │
│ Document  │ Automerge │ History      │ Consent   │ Provider │
│ Service   │           │ Management   │ Control   │          │
│           │           │              │           │          │
│           │           │              │           │          │
└─────────────────────────────────────────────────────────────┘
                           │
                  ┌─────────────────┐
                  │   PostgreSQL    │
                  │     Sled        │
                  └─────────────────┘
```

## Modules

### Core (`core.rs`)
Defines the main traits and data structures:
- `DocumentService`: Main service trait for document operations
- `DocProvider`: Storage provider trait
- Document metadata, content, and permission structures
- Error types

### CRDT (`crdt.rs`)
Implements collaborative editing using Automerge:
- `CrdtDocument`: Wrapper around Automerge document
- `ConflictResolver`: Handles conflict resolution between documents
- Integration with serde for JSON serialization

### Versioning (`versioning.rs`)
Manages document history and version control:
- `VersionHistory`: Tracks document versions
- `DocumentDiff`: Represents changes between versions
- `DiffCalculator`: Calculates differences between document states

### Access Control (`access.rs`)
Implements consent-based access control:
- `ConsentAccessController`: Manages user consents
- `DocumentAccessChecker`: Checks access permissions
- Consent records and policies

### Storage Adapters
- `postgres_store.rs`: PostgreSQL implementation of `DocProvider`
- `sled_store.rs`: Sled (local) implementation of `DocProvider`

### Content Provider (`content_provider.rs`)
Integrates with the social graph content provider system:
- `CollaborativeDocProvider`: Implements `ContentProvider` trait
- Converts documents to feed items

## Usage

### Creating a Document Service

```rust
use collaborative_docs::{
    CollaborativeDocService, 
    PostgresDocStore, 
    DocumentAccessChecker
};
use std::sync::Arc;

// Create storage provider
let store = Arc::new(PostgresDocStore::new(pool));

// Create access checker
let access_checker = DocumentAccessChecker::new(None);

// Create document service
let doc_service = Arc::new(CollaborativeDocService::new(
    store, 
    access_checker
));
```

### Creating a Document

```rust
use serde_json::json;
use uuid::Uuid;

let owner_id = Uuid::new_v4();
let content = DocumentContent {
    data: json!({"text": "Hello, world!"}),
    format: "json".to_string(),
};

let metadata = doc_service.create_document(
    owner_id,
    "My Document".to_string(),
    content,
    vec!["example".to_string()]
).await?;
```

### Collaborative Editing

```rust
// Create a CRDT document
let metadata = doc_service.create_crdt_document(
    owner_id,
    "Collaborative Document".to_string(),
    json!({"content": "Initial content"}),
    vec![]
).await?;

// Update with CRDT operations
let updates = json!({
    "content": "Updated content",
    "author": "user1"
});

doc_service.update_crdt_document(
    metadata.id,
    user_id,
    updates
).await?;
```

### Access Control

```rust
// Grant access to another user
doc_service.grant_access(
    document_id,
    owner_id,
    collaborator_id,
    AccessLevel::Write
).await?;

// Check access
let has_access = access_checker.check_access(
    document_id,
    user_id,
    AccessLevel::Write,
    context
).await?;
```

## Integration with Social Graph

The package includes a `CollaborativeDocProvider` that implements the `ContentProvider` trait from the social graph package, allowing documents to appear in user feeds.

### Provider Registration

```rust
use collaborative_docs::{
    CollaborativeDocProvider,
    CollaborativeDocProviderMetadata
};
use std::sync::Arc;
use uuid::Uuid;

// Create the content provider
let doc_provider = Arc::new(CollaborativeDocProvider::new(doc_service));

// Prepare provider metadata
let metadata = CollaborativeDocProviderMetadata {
    provider_id: Uuid::new_v4(),
    name: "Collaborative Document Provider".to_string(),
    version: "1.0.0".to_string(),
    description: "Provider for collaborative documents with CRDT support".to_string(),
};

// Register with the content provider registry
let provider_id = doc_provider.register_provider(&registry, metadata)?;
```

See `examples/provider_registration.rs` for a complete example.

## Testing

The package includes comprehensive unit tests for all modules:

```bash
cargo test
```

## Dependencies

- `automerge-rs`: CRDT implementation
- `sqlx`: PostgreSQL database access
- `sled`: Embedded database
- `social_graph`: Content provider integration
- `serde`: Serialization
- `uuid`: Unique identifiers
- `chrono`: Time handling
- `tokio`: Async runtime

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.