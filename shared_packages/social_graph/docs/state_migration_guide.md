# State Migration Guide

This guide explains how to implement and use state migration features in the social graph content provider system.

## Overview

The content provider system supports state migration between different versions of providers. This allows for seamless updates without losing important state information.

## Key Components

### ProviderMetadata

The `ProviderMetadata` struct has been extended with new fields for migration support:

- `state_schema_version`: Version of the state schema
- `compatible_previous_versions`: List of previous versions this provider can migrate from
- `required_interfaces`: List of interfaces this provider implements

### ContentProvider Trait

The `ContentProvider` trait now includes two new methods:

```rust
/// Serialize the provider's state for migration purposes
fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Default implementation returns empty state
    Ok(Vec::new())
}

/// Deserialize state into the provider
fn deserialize_state(&self, _data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    // Default implementation does nothing
    Ok(())
}
```

### DependencyResolver

The `DependencyResolver` handles dependency validation with version checking using the `semver` crate.

## Implementing State Migration

To implement state migration in your content provider:

1. Add state storage to your provider
2. Implement `serialize_state` to convert your state to bytes
3. Implement `deserialize_state` to restore state from bytes

Example:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProviderState {
    last_processed_id: Option<Uuid>,
    processed_count: u64,
    custom_config: String,
}

impl ContentProvider for MyContentProvider {
    // ... other methods ...
    
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let state = self.state.lock().unwrap();
        let serialized = serde_json::to_vec(&*state)?;
        Ok(serialized)
    }
    
    fn deserialize_state(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let state: ProviderState = serde_json::from_slice(data)?;
        let mut self_state = self.state.lock().unwrap();
        *self_state = state;
        Ok(())
    }
}
```

## Using the Update Provider Feature

To update a provider with state migration:

```rust
let registry = ContentProviderRegistry::new(consent_service);

// Register initial provider
let provider_v1 = Arc::new(MyContentProvider::new());
let metadata_v1 = ProviderMetadata {
    id: Uuid::new_v4(),
    name: "MyProvider".to_string(),
    version: "1.0.0".to_string(),
    state_schema_version: "1.0.0".to_string(),
    compatible_previous_versions: vec![],
    // ... other fields
};

registry.register_provider(provider_v1, metadata_v1)?;

// Update to new version
let provider_v2 = Arc::new(MyContentProviderV2::new());
let metadata_v2 = ProviderMetadata {
    version: "2.0.0".to_string(),
    state_schema_version: "2.0.0".to_string(),
    compatible_previous_versions: vec!["1.0.0".to_string()],
    // ... other fields (keep same ID)
};

registry.update_provider(provider_v2, metadata_v2)?;
```

## Dependency Validation

The system now validates dependencies when registering providers:

```rust
let metadata = ProviderMetadata {
    dependencies: vec!["OtherProvider@1.2.3".to_string()],
    // ... other fields
};
```

Dependencies can specify version requirements using the `@` syntax.

## Testing

The registry includes tests for dependency resolution and state migration scenarios.

See `shared_packages/social_graph/examples/state_migration_example.rs` for a complete example.

## Error Handling During Migration

### Serialization and Deserialization Errors

The migration process handles errors during state serialization and deserialization through specialized error variants:

```rust
enum MigrationError {
    StateSerializationError(String),
    StateDeserializationError(String),
    // other errors...
}
```

- **StateSerializationError**: Occurs when the old provider fails to serialize its state
- **StateDeserializationError**: Occurs when the new provider fails to deserialize the state

These errors include detailed messages for troubleshooting. When encountered, the migration process is aborted and the old provider remains active.

### Rollback Mechanisms

For failed migrations, the registry implements an atomic rollback process:

1. If any step fails (dependency validation, serialization, or deserialization), the entire update is aborted
2. The old provider remains registered with its original state intact
3. The registry logs detailed error information for debugging
4. Client applications receive clear error messages about the failure cause

This all-or-nothing approach ensures system stability during provider updates.

### Concurrent Access Patterns

The registry uses a read-write lock to handle concurrent access during migrations:

1. During serialization:
   - Acquires a read lock on providers map
   - Allows other threads to continue reading providers
2. During state transfer:
   - Briefly upgrades to write lock for atomic provider swap
   - Minimizes write lock duration to reduce contention
3. After swap:
   - Releases write lock and resumes normal read operations

This design ensures safe concurrent access while maintaining performance.

## Testing State Migration

The package includes comprehensive tests for state migration scenarios:

- Successful state migration between compatible providers
- Error handling during serialization failures
- Error handling during deserialization failures
- Rollback mechanism verification
- Concurrent access during migration

See `shared_packages/social_graph/examples/state_migration_example.rs` for a complete example
demonstrating these features.