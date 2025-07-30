# CPC Infrastructure Components

This crate provides infrastructure implementations for various services including database adapters, network clients, and sync mechanisms.

## Overview

The infrastructure layer provides implementations for core domain services, focusing on:

1. **Network Resilience** - Automatic switching between online/offline modes
2. **Data Synchronization** - Reliable offline-first synchronization
3. **Error Handling** - Proper error chaining and recovery
4. **Fault Tolerance** - Graceful degradation under adverse conditions

## Key Components

### Network Monitoring

The `core::network::monitor` module provides network status monitoring with an observer pattern:

```rust
use packages::infra::core::network::monitor::NetworkStatusMonitor;
use std::sync::Arc;

let monitor = Arc::new(NetworkStatusMonitor::new());
monitor.set_connected(false); // Simulate network disconnection
```

### Composite Adapters

The `core::adapters::composite` module implements the composite adapter pattern with automatic network-aware switching:

```rust
use packages::infra::core::adapters::composite::CompositeUserPreferences;
use packages::infra::grpc::clients::user_preferences::GrpcUserPreferences;
use packages::infra::sled::adapters::user_preferences::SledUserPreferences;

let composite = CompositeUserPreferences::new(online_service, offline_service);
// Automatically switches implementations based on network status
```

### Sync Queue

The `sync` module provides a robust synchronization queue with:

- **Operation Prioritization** - Critical operations processed first
- **Exponential Backoff** - Prevents overwhelming services during outages
- **Fault-Tolerant Processing** - Continues processing even when individual operations fail
- **Storage Abstraction** - Pluggable storage backends

```rust
use packages::infra::sync::queue::SyncQueue;
use packages::infra::sync::storage::SledQueueStorage;
use packages::infra::sync::backoff::ExponentialBackoff;
use packages::infra::sync::conflict::TimestampConflictResolver;

let storage = SledQueueStorage::new(&db)?;
let queue = SyncQueue::new(
    Box::new(storage),
    Arc::new(TimestampConflictResolver::new()),
    Box::new(ExponentialBackoff::default())
);
```

## Usage Example

```rust
// Create network monitor
let network_monitor = Arc::new(NetworkStatusMonitor::new());

// Create services
let online_service = GrpcUserPreferences::new(grpc_client, user_id);
let offline_service = SledUserPreferences::new(&db);

// Create composite adapter
let preferences = CompositeUserPreferences::new(online_service, offline_service);

// Use the service (automatically switches based on network status)
preferences.set_preferred_currency(user_id, Currency::USD).await?;

// If offline, operation is queued for sync
let queue = SyncQueue::with_defaults(Box::new(storage));
queue.enqueue(operation)?;
```

## Testing

The infrastructure components include comprehensive tests for:

- Network fault simulation
- Retry strategies with backoff
- Priority-based processing
- Conflict resolution
- Storage failure recovery

Run tests with:

```bash
cargo test
```

## Design Principles

1. **Hexagonal Architecture** - Clear separation between domain logic and infrastructure
2. **Observer Pattern** - Reactive to network status changes
3. **Storage Abstraction** - Pluggable storage backends
4. **Error Chaining** - Preserves original error context
5. **Fault Tolerance** - Graceful degradation under adverse conditions

## Cooperative Values Alignment

These infrastructure components align with our cooperative values by:

- Ensuring access for all users regardless of network conditions
- Prioritizing user actions over background tasks
- Making error recovery paths transparent to users
- Supporting low-connectivity areas with reliable offline functionality