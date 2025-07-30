# Migration Guide: CPC Sync Architecture Upgrade

This guide explains how to migrate existing code to use the new architectural improvements for the CPC sync infrastructure.

## Overview

The new architecture provides significant improvements in network resilience, error handling, and fault tolerance. This guide will help you upgrade your code to take advantage of these improvements.

## Before and After Comparison

### Old Approach

```rust
// Old approach - manual network handling
let preferences = if network_monitor.is_connected() {
    UserPreferencesImpl::Online(GrpcUserPreferences::new(client, user_id))
} else {
    UserPreferencesImpl::Offline(SledUserPreferences::new(db))
};

// Manual error handling with string errors
match preferences.set_preferred_currency(user_id, currency).await {
    Ok(_) => {},
    Err(e) => {
        // Error context is lost
        println!("Operation failed: {}", e);
    }
}
```

### New Approach

```rust
// New approach - automatic network handling
let preferences = UserPreferencesFactory::create(db, network_monitor, client);

// Proper error chaining with context preservation
match preferences.set_preferred_currency(user_id, currency).await {
    Ok(_) => {},
    Err(e) => {
        // Full error context preserved
        match e {
            PreferencesError::OnlineFailure(source) => {
                // Handle online service failure
            },
            PreferencesError::OfflineFailure(source) => {
                // Handle offline storage failure
            },
            PreferencesError::DualFailure { online, offline } => {
                // Handle both services failing
            }
        }
    }
}
```

## Migration Steps

### 1. Update Factory Usage

**Old:**
```rust
let preferences = UserPreferencesFactory::create(db, network_monitor, client, user_id);
```

**New:**
```rust
let preferences = UserPreferencesFactory::create(db, network_monitor, client);
// User ID is passed at operation level
preferences.set_preferred_currency(user_id, currency).await;
```

### 2. Update Error Handling

**Old:**
```rust
match operation.await {
    Ok(_) => {},
    Err(e) => println!("Error: {}", e)
}
```

**New:**
```rust
match operation.await {
    Ok(_) => {},
    Err(PreferencesError::OnlineFailure(source)) => {
        // Handle online service failure with full context
    },
    Err(PreferencesError::OfflineFailure(source)) => {
        // Handle offline storage failure with full context
    },
    Err(PreferencesError::DualFailure { online, offline }) => {
        // Handle both services failing with full context
    }
}
```

### 3. Update Sync Queue Usage

**Old:**
```rust
let queue = SyncQueue::new(db, resolver);
queue.enqueue(operation)?;
```

**New:**
```rust
let storage = SledQueueStorage::new(db)?;
let queue = SyncQueue::new(
    Box::new(storage),
    Arc::new(TimestampConflictResolver::new()),
    Box::new(ExponentialBackoff::default())
);

// Operations now include priority
let operation = SyncOperation::SetCurrency {
    user_id,
    currency: Currency::USD,
    priority: OperationPriority::High,  // New field
    attempts: 0,
    scheduled_at: SystemTime::now(),
};
queue.enqueue(operation)?;
```

### 4. Update Processing Logic

**Old:**
```rust
queue.process(&client).await?;
```

**New:**
```rust
let summary = queue.process(&client).await?;
println!("Processed {} operations successfully", summary.successful.len());
println!("{} operations scheduled for retry", summary.retried.len());
println!("{} operations failed permanently", summary.failed.len());
```

## Breaking Changes

### 1. Factory Interface
- Removed `user_id` parameter from `UserPreferencesFactory::create`
- User ID is now passed at the operation level

### 2. Error Types
- Replaced string errors with proper error types using `thiserror`
- Added `PreferencesError` enum with specific error variants

### 3. Sync Operation Structure
- Added `priority` field to `SyncOperation`
- Added `attempts` and `scheduled_at` fields
- Operations are now more structured

### 4. Queue Interface
- `SyncQueue::new` now takes storage, conflict resolver, and backoff strategy
- `process` method now returns `ProcessingSummary` instead of `()`

## New Features to Leverage

### 1. Operation Prioritization
```rust
let critical_op = SyncOperation::SetCurrency {
    user_id,
    currency: Currency::BTC,
    priority: OperationPriority::Critical,  // Highest priority
    attempts: 0,
    scheduled_at: SystemTime::now(),
};

let background_op = SyncOperation::SetCurrency {
    user_id,
    currency: Currency::USD,
    priority: OperationPriority::Low,  // Lowest priority
    attempts: 0,
    scheduled_at: SystemTime::now(),
};
```

### 2. Exponential Backoff
The queue automatically applies exponential backoff with jitter to failed operations, preventing service overload during outages.

### 3. Fault-Tolerant Processing
Processing continues even when individual operations fail, with detailed summaries of what succeeded, was retried, or failed permanently.

### 4. Network Status Monitoring
Automatic switching between online and offline implementations based on network status changes.

## Testing Migration

### Old Test Approach
```rust
#[test]
fn test_sync_queue() {
    let queue = SyncQueue::new(db, resolver);
    queue.enqueue(operation).unwrap();
}
```

### New Test Approach
```rust
#[tokio::test]
async fn test_sync_queue_with_backoff() {
    let storage = SledQueueStorage::new(db).unwrap();
    let queue = SyncQueue::new(
        Box::new(storage),
        Arc::new(TimestampConflictResolver::new()),
        Box::new(ExponentialBackoff::default())
    );
    
    queue.enqueue(operation).unwrap();
    
    let summary = queue.process(&client).await.unwrap();
    assert_eq!(summary.successful.len(), 1);
}
```

## Performance Considerations

### Memory Usage
The new architecture has slightly higher memory usage due to:
- Additional fields in `SyncOperation`
- Storage abstraction layer
- Error context preservation

However, this is offset by:
- More efficient processing (continues after individual failures)
- Better resource utilization (backoff prevents overwhelming services)

### Processing Overhead
Core operations have <10ms overhead compared to the previous implementation, with the benefits of:
- Automatic retry with backoff
- Priority-based processing
- Detailed processing summaries

## Best Practices with New Architecture

### 1. Leverage Operation Prioritization
Always set appropriate priorities for operations:
- `OperationPriority::Critical` for user-facing actions
- `OperationPriority::High` for important background tasks
- `OperationPriority::Medium` for regular sync operations
- `OperationPriority::Low` for analytics and non-critical data

### 2. Handle All Error Variants
```rust
match preferences.set_preferred_currency(user_id, currency).await {
    Ok(_) => {
        // Success case
    },
    Err(PreferencesError::OnlineFailure(e)) => {
        // Log online service failure but continue
        log::warn!("Online service failed: {}", e);
    },
    Err(PreferencesError::OfflineFailure(e)) => {
        // Offline storage failure is more serious
        log::error!("Offline storage failed: {}", e);
    },
    Err(PreferencesError::DualFailure { online, offline }) => {
        // Both services failed - requires manual intervention
        log::error!("Both services failed - online: {}, offline: {}", online, offline);
    }
}
```

### 3. Monitor Processing Summaries
Use the detailed summaries to monitor sync health:
```rust
let summary = queue.process(&client).await?;
if summary.failed.len() > 0 {
    log::error!("{} operations failed permanently", summary.failed.len());
    // Consider alerting or manual intervention
}
```

## Troubleshooting

### Common Migration Issues

1. **"Cannot infer type" errors**
   - Ensure you're importing the new error types
   - Check that you're handling all error variants

2. **"Field not found" errors**
   - Add the new required fields to `SyncOperation`
   - `priority`, `attempts`, and `scheduled_at` are now required

3. **"Method not found" errors**
   - Update factory calls to remove `user_id` parameter
   - Update queue construction to use new parameters

### Performance Issues

1. **High memory usage**
   - Check for operations being queued unnecessarily
   - Consider using `OperationPriority::Low` for non-critical operations

2. **Slow processing**
   - Verify backoff strategy is appropriate for your use case
   - Check for operations that are failing and being retried frequently

## Conclusion

The new architecture provides significant improvements in reliability, maintainability, and user experience. While there are breaking changes, the migration process is straightforward and the benefits far outweigh the costs.

The improvements directly support our cooperative values by ensuring access for all users regardless of network conditions, with special consideration for areas with unreliable connectivity.