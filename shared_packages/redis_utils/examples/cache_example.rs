//! Example of using the redis_utils package for caching

use redis_utils::{RedisConfig, RedisManager, CacheManager};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create Redis configuration
    let redis_config = RedisConfig::from_env()?;

    // Create Redis manager
    println!("Creating Redis manager...");
    let redis_manager = RedisManager::new(&redis_config).await?;

    // Create cache manager
    let cache_manager = CacheManager::new(redis_manager, Duration::from_secs(300));

    // Create a user object
    let user = User {
        id: 123,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    let cache_key = "user:123";

    // Store user in cache
    println!("Storing user in cache...");
    cache_manager.set(cache_key, &user, Some(Duration::from_secs(60))).await?;
    println!("User stored in cache");

    // Retrieve user from cache
    println!("Retrieving user from cache...");
    let cached_user: Option<User> = cache_manager.get(cache_key).await?;
    
    match cached_user {
        Some(u) => println!("Retrieved user: {:?}", u),
        None => println!("User not found in cache"),
    }

    // Check if key exists
    let exists = cache_manager.exists(cache_key).await?;
    println!("Key exists: {}", exists);

    // Delete user from cache
    println!("Deleting user from cache...");
    cache_manager.delete(cache_key).await?;
    println!("User deleted from cache");

    // Try to retrieve again
    let cached_user: Option<User> = cache_manager.get(cache_key).await?;
    match cached_user {
        Some(u) => println!("Retrieved user: {:?}", u),
        None => println!("User not found in cache (as expected)"),
    }

    println!("Cache example completed successfully!");
    Ok(())
}