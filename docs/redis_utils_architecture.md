# Enhanced Redis Package (`redis_utils`) Architecture

## Overview

The `redis_utils` package provides enhanced Redis functionality building on the existing usage. This package aims to provide efficient, reliable, and consistent Redis interactions across all CPC applications.

## Architecture

### Core Components

1. **Connection Management**
   - Connection pooling with bb8
   - Configuration management
   - Health checks

2. **Serialization Utilities**
   - JSON serialization/deserialization
   - Binary serialization (MessagePack, etc.)
   - Compression utilities

3. **Caching Layer**
   - Cache key generation
   - TTL management
   - Cache invalidation strategies

4. **Distributed Locking**
   - Lock acquisition and release
   - Lock timeout handling
   - Deadlock prevention

5. **Pub/Sub Utilities**
   - Message serialization
   - Channel management
   - Subscriber patterns

6. **Rate Limiting**
   - Token bucket implementation
   - Sliding window implementation
   - Rate limit configuration

7. **Session Management**
   - Session storage
   - Session expiration
   - Session validation

### Data Flow

```
App Configuration
       ↓
Redis Configuration
       ↓
Connection Pool (bb8)
       ↓
Redis Commands
       ↓
Redis Server
```

## Implementation Details

### Connection Management

Connection management uses bb8 for connection pooling:

```rust
pub struct RedisManager {
    pool: bb8::Pool<RedisConnectionManager>,
}

impl RedisManager {
    pub async fn new(config: &RedisConfig) -> Result<Self, RedisError>;
    pub async fn get_connection(&self) -> Result<bb8::PooledConnection<RedisConnectionManager>, RedisError>;
    pub async fn health_check(&self) -> Result<HealthStatus, RedisError>;
}
```

### Serialization Utilities

Serialization utilities provide multiple formats:

```rust
pub struct Serializer;

impl Serializer {
    pub fn to_json<T: Serialize>(value: &T) -> Result<String, RedisError>;
    pub fn from_json<T: DeserializeOwned>(data: &str) -> Result<T, RedisError>;
    pub fn to_msgpack<T: Serialize>(value: &T) -> Result<Vec<u8>, RedisError>;
    pub fn from_msgpack<T: DeserializeOwned>(data: &[u8]) -> Result<T, RedisError>;
    pub fn compress(data: &[u8]) -> Result<Vec<u8>, RedisError>;
    pub fn decompress(data: &[u8]) -> Result<Vec<u8>, RedisError>;
}
```

### Caching Layer

The caching layer provides high-level caching utilities:

```rust
pub struct CacheManager {
    redis: RedisManager,
    default_ttl: Duration,
}

impl CacheManager {
    pub async fn set<T: Serialize>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<(), RedisError>;
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError>;
    pub async fn delete(&self, key: &str) -> Result<(), RedisError>;
    pub async fn exists(&self, key: &str) -> Result<bool, RedisError>;
    pub async fn expire(&self, key: &str, ttl: Duration) -> Result<(), RedisError>;
}
```

### Distributed Locking

Distributed locking implementation:

```rust
pub struct DistributedLock {
    redis: RedisManager,
    key: String,
    ttl: Duration,
}

impl DistributedLock {
    pub fn new(redis: RedisManager, key: String, ttl: Duration) -> Self;
    pub async fn acquire(&self) -> Result<LockGuard, RedisError>;
    pub async fn release(&self, guard: LockGuard) -> Result<(), RedisError>;
    pub async fn extend(&self, guard: &LockGuard, additional_time: Duration) -> Result<(), RedisError>;
}
```

### Pub/Sub Utilities

Pub/Sub utilities with type safety:

```rust
pub struct PubSubManager {
    redis: RedisManager,
}

impl PubSubManager {
    pub async fn publish<T: Serialize>(&self, channel: &str, message: &T) -> Result<(), RedisError>;
    pub async fn subscribe<T: DeserializeOwned>(&self, channel: &str) -> Result<Subscriber<T>, RedisError>;
    pub async fn unsubscribe(&self, subscriber: Subscriber<T>) -> Result<(), RedisError>;
}
```

### Rate Limiting

Rate limiting implementations:

```rust
pub struct RateLimiter {
    redis: RedisManager,
}

impl RateLimiter {
    pub fn new(redis: RedisManager) -> Self;
    
    // Token bucket implementation
    pub async fn check_token_bucket(
        &self,
        key: &str,
        capacity: u64,
        refill_rate: u64,
    ) -> Result<RateLimitResult, RedisError>;
    
    // Sliding window implementation
    pub async fn check_sliding_window(
        &self,
        key: &str,
        max_requests: u64,
        window_size: Duration,
    ) -> Result<RateLimitResult, RedisError>;
}
```

### Session Management

Session management utilities:

```rust
pub struct SessionManager {
    cache: CacheManager,
}

impl SessionManager {
    pub fn new(cache: CacheManager) -> Self;
    pub async fn create_session<T: Serialize>(&self, data: &T, ttl: Duration) -> Result<String, RedisError>;
    pub async fn get_session<T: DeserializeOwned>(&self, session_id: &str) -> Result<Option<T>, RedisError>;
    pub async fn extend_session(&self, session_id: &str, ttl: Duration) -> Result<(), RedisError>;
    pub async fn destroy_session(&self, session_id: &str) -> Result<(), RedisError>;
}
```

## API Design

### Main Interface

```rust
// Initialize Redis manager
let redis_config = RedisConfig::from_env()?;
let redis_manager = RedisManager::new(&redis_config).await?;

// Use cache manager
let cache_manager = CacheManager::new(redis_manager.clone(), Duration::from_secs(300));
cache_manager.set("key", &value, Some(Duration::from_secs(60))).await?;

// Use distributed lock
let lock = DistributedLock::new(redis_manager.clone(), "lock_key".to_string(), Duration::from_secs(30));
let guard = lock.acquire().await?;
// Critical section
lock.release(guard).await?;

// Use pub/sub
let pubsub = PubSubManager::new(redis_manager);
pubsub.publish("channel", &message).await?;
let subscriber = pubsub.subscribe::<MyMessage>("channel").await?;

// Use rate limiter
let rate_limiter = RateLimiter::new(redis_manager);
let result = rate_limiter.check_token_bucket("user:123", 100, 10).await?;

// Use session manager
let session_manager = SessionManager::new(cache_manager);
let session_id = session_manager.create_session(&user_data, Duration::from_secs(3600)).await?;
```

## Error Handling

### RedisError
A unified error type for Redis operations:

```rust
#[derive(Debug, thiserror::Error)]
pub enum RedisError {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Connection error: {0}")]
    Connection(#[from] bb8::RunError<redis::RedisError>),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Lock acquisition timeout")]
    LockTimeout,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Session expired")]
    SessionExpired,
}
```

## Integration with Apps

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

## Testing Strategy

1. **Unit Tests**: Test individual components (serializers, cache managers)
2. **Integration Tests**: Test with actual Redis instances
3. **Performance Tests**: Benchmark Redis operations
4. **Failure Tests**: Test error handling and recovery
5. **Concurrency Tests**: Test concurrent access patterns

## Deployment Considerations

1. **Configuration**: Externalize Redis configuration for different environments
2. **Monitoring**: Export Redis metrics for monitoring systems
3. **Logging**: Comprehensive logging for debugging
4. **High Availability**: Redis cluster support for high availability
5. **Backup Strategy**: Redis persistence configuration