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

## Usage

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