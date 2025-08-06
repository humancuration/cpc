# Enhanced Redis Package (`redis_utils`)

This package provides enhanced Redis functionality building on the existing usage. This package aims to provide efficient, reliable, and consistent Redis interactions across all CPC applications.

## Features

- Connection management with connection pooling
- Serialization/deserialization utilities
- Distributed locking mechanisms
- Caching utilities with automatic expiration
- Pub/sub utilities with type safety
- Rate limiting implementations
- Session management

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
redis_utils = { path = "../shared_packages/redis_utils" }
```

## Usage

### Connection Management

```rust
use redis_utils::{RedisConfig, RedisManager};

// Create Redis configuration
let redis_config = RedisConfig::from_env()?;

// Create Redis manager
let redis_manager = RedisManager::new(&redis_config).await?;
```

### Caching

```rust
use redis_utils::{RedisConfig, RedisManager, CacheManager};
use std::time::Duration;

// Create cache manager
let cache_manager = CacheManager::new(redis_manager, Duration::from_secs(300));

// Set a value in cache
cache_manager.set("key", &value, Some(Duration::from_secs(60))).await?;

// Get a value from cache
let cached_value: Option<MyType> = cache_manager.get("key").await?;
```

### Distributed Locking

```rust
use redis_utils::{RedisManager, DistributedLock};
use std::time::Duration;

// Create distributed lock
let lock = DistributedLock::new(redis_manager, "lock_key".to_string(), Duration::from_secs(30));

// Acquire lock
let guard = lock.acquire().await?;

// Critical section
// ... do work ...

// Release lock
lock.release(guard).await?;
```

### Pub/Sub

```rust
use redis_utils::{RedisManager, PubSubManager};

// Create pub/sub manager
let pubsub = PubSubManager::new(redis_manager);

// Publish a message
pubsub.publish("channel", &message).await?;

// Subscribe to a channel
let subscriber = pubsub.subscribe::<MyMessage>("channel").await?;
```

### Rate Limiting

```rust
use redis_utils::{RedisManager, RateLimiter};

// Create rate limiter
let rate_limiter = RateLimiter::new(redis_manager);

// Check rate limit using token bucket
let result = rate_limiter.check_token_bucket("user:123", 100, 10).await?;

if result.allowed {
    // Process request
} else {
    // Rate limit exceeded
}
```

### Session Management

```rust
use redis_utils::{RedisManager, CacheManager, SessionManager};
use std::time::Duration;

// Create session manager
let cache_manager = CacheManager::new(redis_manager, Duration::from_secs(300));
let session_manager = SessionManager::new(cache_manager);

// Create a session
let session_id = session_manager.create_session(&user_data, Duration::from_secs(3600)).await?;

// Get session data
let session_data: Option<UserData> = session_manager.get_session(&session_id).await?;

// Extend session
session_manager.extend_session(&session_id, Duration::from_secs(3600)).await?;
```

## Integration Examples

### Sheets App Integration

```rust
// Replace direct Redis usage with redis_utils
let redis_config = RedisConfig::from_env()?;
let redis_manager = RedisManager::new(&redis_config).await?;
let cache_manager = CacheManager::new(redis_manager, Duration::from_secs(300));

// Use in caching implementation
impl VisualizationCache {
    pub async fn store(&self, key: &str, data: &CacheEntry) -> Result<(), RedisError> {
        self.cache_manager.set(key, data, Some(Duration::from_secs(data.ttl))).await
    }
    
    pub async fn retrieve(&self, key: &str) -> Result<Option<CacheEntry>, RedisError> {
        self.cache_manager.get(key).await
    }
}
```

### Realtime Signaling Integration

```rust
// Enhance with additional Redis utilities
let redis_config = RedisConfig::from_env()?;
let redis_manager = RedisManager::new(&redis_config).await?;
let pubsub_manager = PubSubManager::new(redis_manager);

impl RedisSignalingService {
    pub async fn broadcast_message(&self, document_id: Uuid, message: &SignalingMessage) -> Result<(), RedisError> {
        let channel_name = self.get_channel_name(document_id);
        self.pubsub_manager.publish(&channel_name, message).await
    }
}
```

## Testing

The package includes both unit tests and integration tests. To run the tests:

```bash
cargo test
```

For integration tests with actual Redis instances, you'll need to have Redis available.

## Performance Considerations

1. **Connection Pooling**: bb8 connection pooling for efficient connection reuse
2. **Serialization**: Efficient serialization formats (MessagePack) for binary data
3. **Compression**: Optional compression for large values
4. **Pipelining**: Redis command pipelining for batch operations
5. **Caching Strategies**: LRU, TTL, and other caching strategies
6. **Pub/Sub Patterns**: Efficient subscriber patterns for messaging

## Security Considerations

1. **Authentication**: Redis authentication support
2. **Encryption**: TLS support for encrypted connections
3. **Data Protection**: Encryption of sensitive data at rest
4. **Access Control**: Redis ACL for fine-grained access control
5. **Session Security**: Secure session management with proper expiration