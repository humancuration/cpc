# Migration Guide for Storage Abstraction Layer

This document provides guidance on migrating existing modules to use the new Storage Abstraction Layer.

## Overview

The Storage Abstraction Layer provides a unified interface for storage operations with smart routing between different storage backends (Sled for edge, PostgreSQL for cloud) and implements dual-write patterns with fallback strategies.

## Migration Steps

### 1. Update Cargo.toml

Add the storage abstraction dependency to your module's Cargo.toml:

```toml
[dependencies]
storage_abstraction = { path = "../storage_abstraction" }
```

### 2. Replace Direct Storage Implementations

#### Before (Consent Manager Example)
```rust
// Direct Sled usage
use sled::Db;

struct ConsentStorage {
    db: Db,
}

impl ConsentStorage {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let key = format!("consent:{}:{}", user_id, domain.as_str());
        match self.db.get(key) {
            Ok(Some(value)) => {
                let profile = serde_json::from_slice(&value)
                    .map_err(|e| ConsentError::SerializationError(e.to_string()))?;
                Ok(Some(profile))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(ConsentError::StorageError(e.to_string())),
        }
    }
}
```

#### After (Using Storage Abstraction)
```rust
// Using Storage Abstraction
use storage_abstraction::{DataStore, StorageError};

struct ConsentStorage {
    storage: Arc<dyn DataStore>,
}

impl ConsentStorage {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let key = format!("consent:{}:{}", user_id, domain.as_str());
        match self.storage.get(&key).await {
            Ok(Some(value)) => {
                let profile = serde_json::from_slice(&value)
                    .map_err(|e| ConsentError::SerializationError(e.to_string()))?;
                Ok(Some(profile))
            },
            Ok(None) => Ok(None),
            Err(e) => Err(ConsentError::StorageError(e.to_string())),
        }
    }
}
```

### 3. Update Service Initialization

#### Before (Consent Manager Example)
```rust
// Direct Sled initialization
let db = sled::open("consent_data")?;
let storage = ConsentStorage::new(db);
let service = ConsentService::new(Box::new(storage));
```

#### After (Using Storage Abstraction)
```rust
// Using Storage Abstraction
use storage_abstraction::{StorageManager, SledStore, PostgresStore, StorageConfig};
use std::sync::Arc;

// Create storage backends
let edge_store = Arc::new(SledStore::new("edge_data")?);
let cloud_store = Arc::new(PostgresStore::new("postgresql://...").await?);

// Create storage manager
let config = StorageConfig {
    router: RoutingConfig::default(),
};
let storage_manager = StorageManager::new(edge_store, cloud_store, config);

// Use in service
let storage = ConsentStorage::new(Arc::new(storage_manager));
let service = ConsentService::new(Box::new(storage));
```

## Dual-Write Pattern Implementation

For modules that need to maintain compatibility during migration, implement a dual-write pattern:

```rust
use storage_abstraction::{DataStore, StorageError};
use std::sync::Arc;

/// Wrapper service that handles dual-write during migration
pub struct DualWriteStorage {
    /// The new storage abstraction
    new_storage: Arc<dyn DataStore>,
    /// The legacy storage (if still needed during migration)
    legacy_storage: Option<Arc<dyn DataStore>>,
}

impl DualWriteStorage {
    pub fn new(
        new_storage: Arc<dyn DataStore>,
        legacy_storage: Option<Arc<dyn DataStore>>,
    ) -> Self {
        Self {
            new_storage,
            legacy_storage,
        }
    }
}

#[async_trait::async_trait]
impl DataStore for DualWriteStorage {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        // Try the new storage first
        match self.new_storage.get(key).await {
            Ok(Some(value)) => Ok(Some(value)),
            Ok(None) => {
                // If not found in new storage, try legacy storage
                if let Some(legacy) = &self.legacy_storage {
                    legacy.get(key).await
                } else {
                    Ok(None)
                }
            },
            Err(e) => {
                // If new storage fails, try legacy storage
                if let Some(legacy) = &self.legacy_storage {
                    legacy.get(key).await
                } else {
                    Err(e)
                }
            },
        }
    }

    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        // Write to the new storage
        self.new_storage.set(key, value.clone()).await?;
        
        // Also write to the legacy storage during migration
        if let Some(legacy) = &self.legacy_storage {
            if let Err(e) = legacy.set(key, value).await {
                // Log the error but don't fail the operation
                tracing::warn!("Warning: Failed to write to legacy storage: {:?}", e);
            }
        }
        
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        // Delete from the new storage
        self.new_storage.delete(key).await?;
        
        // Also delete from the legacy storage during migration
        if let Some(legacy) = &self.legacy_storage {
            if let Err(e) = legacy.delete(key).await {
                // Log the error but don't fail the operation
                tracing::warn!("Warning: Failed to delete from legacy storage: {:?}", e);
            }
        }
        
        Ok(())
    }
}
```

## Routing Configuration

Configure routing based on your data sensitivity requirements:

```rust
use storage_abstraction::{StorageConfig, RoutingConfig, StorageLocation};

let mut router_config = RoutingConfig::default();

// Override default patterns for your specific needs
router_config.edge_patterns = vec![
    "temp:*".to_string(),
    "cache:*".to_string(),
    "session:*".to_string(),
    "wearable:*".to_string(), // Health module wearable data
];

router_config.cloud_patterns = vec![
    "user:*".to_string(),
    "consent:*".to_string(),
    "finance:*".to_string(),
    "health:*".to_string(),
    "transaction:*".to_string(),
];

let config = StorageConfig {
    router: router_config,
};
```

## Testing During Migration

Use the in-memory storage implementation for testing:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use storage_abstraction::InMemoryStore;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_with_in_memory_storage() {
        let storage = Arc::new(InMemoryStore::new());
        let service = MyService::new(storage);
        
        // Test your service logic
        // ...
    }
}
```

## Performance Considerations

1. The Storage Abstraction Layer implements dual-write with background operations for the secondary storage, minimizing impact on primary operations.

2. For read operations, data is retrieved from the primary storage location determined by the routing configuration.

3. Caching strategies can be implemented at the application layer for frequently accessed data.

## Troubleshooting

### Common Issues

1. **Serialization Errors**: Ensure all data structures implement serde serialization correctly.

2. **Connection Errors**: Verify database connection strings and network connectivity for PostgreSQL.

3. **Routing Issues**: Check routing configuration patterns match your key naming conventions.

### Logging and Monitoring

The Storage Abstraction Layer uses tracing for logging. Enable tracing in your application to monitor storage operations:

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // ... rest of your application
}
```

This will provide detailed logs of storage operations, including routing decisions and any errors that occur.