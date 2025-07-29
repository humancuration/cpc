# Storage Abstraction Layer

Provides unified storage interfaces with smart routing between different storage backends (Sled for edge, PostgreSQL for cloud), addressing the dual-write pattern.

## Overview

The Storage Abstraction Layer implements a storage abstraction that allows applications to store data without needing to know the underlying storage mechanism. It provides smart routing based on data sensitivity and access patterns, and implements dual-write patterns with fallback strategies.

## Features

- **Unified Interface**: Single API for all storage operations
- **Smart Routing**: Automatic routing based on data patterns
- **Dual-Write Pattern**: Ensures data consistency during migrations
- **Multiple Backends**: Support for Sled (edge) and PostgreSQL (cloud)
- **Caching**: In-memory caching strategies
- **Error Handling**: Comprehensive error types and handling
- **Tracing**: Detailed logging for monitoring and debugging

## Architecture

The module follows hexagonal architecture principles with clear separation of concerns:

```
Domain Layer
├── traits.rs       # Core storage interfaces
├── routing.rs      # Routing logic
└── errors.rs       # Error types

Application Layer
├── manager.rs      # StorageManager orchestrator
└── cache.rs        # Caching strategies

Infrastructure Layer
├── sled.rs         # Sled implementation
├── postgres.rs     # PostgreSQL implementation
├── in_memory.rs    # Test implementation
└── dual_write.rs   # Dual-write pattern
```

## Usage

### Basic Usage

```rust
use storage_abstraction::{
    DataStore,
    StorageManager,
    StorageConfig,
    SledStore,
    PostgresStore,
};
use std::sync::Arc;

// Create storage backends
let edge_store = Arc::new(SledStore::new("edge_data")?);
let cloud_store = Arc::new(PostgresStore::new("postgresql://...").await?);

// Create storage manager
let config = StorageConfig {
    router: RoutingConfig::default(),
};
let manager = StorageManager::new(edge_store, cloud_store, config);

// Store data
manager.set("user:123:profile", profile_data).await?;

// Retrieve data
let profile = manager.get("user:123:profile").await?;
```

### Routing Configuration

```rust
use storage_abstraction::{StorageConfig, RoutingConfig, StorageLocation};

let mut router_config = RoutingConfig::default();

// Configure patterns for edge storage
router_config.edge_patterns = vec![
    "temp:*".to_string(),
    "cache:*".to_string(),
    "session:*".to_string(),
    "wearable:*".to_string(),
];

// Configure patterns for cloud storage
router_config.cloud_patterns = vec![
    "user:*".to_string(),
    "consent:*".to_string(),
    "finance:*".to_string(),
    "health:*".to_string(),
];

let config = StorageConfig {
    router: router_config,
};
```

## Integration Examples

See the examples directory for integration examples with:
- Basic usage
- Health module integration
- Finance module integration

Run examples with:
```bash
cargo run --example basic_usage
cargo run --example health_integration
cargo run --example finance_integration
```

## Testing

Run tests with:
```bash
cargo test
```

## Migration

See [MIGRATION.md](MIGRATION.md) for detailed migration guidance for existing modules.

## Dependencies

- **tokio**: Async runtime
- **serde**: Serialization framework
- **sled**: Embedded database for edge storage
- **sqlx**: PostgreSQL client for cloud storage
- **tracing**: Logging and monitoring

## License

This module is part of the CPC software ecosystem and is licensed under the CPC license.