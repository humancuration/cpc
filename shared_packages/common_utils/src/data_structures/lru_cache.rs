//! LRU (Least Recently Used) Cache implementation
//!
//! This module provides a thread-safe LRU cache implementation that can be
//! used to cache frequently accessed data.

use std::collections::HashMap;
use std::hash::Hash;
use tokio::sync::RwLock;
use crate::error::Result;

/// LRU Cache implementation
pub struct LruCache<K, V> {
    cache: RwLock<HashMap<K, V>>,
    capacity: usize,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone,
{
    /// Create a new LRU cache with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            capacity,
        }
    }

    /// Get a value from the cache
    pub async fn get(&self, key: &K) -> Option<V> {
        let cache = self.cache.read().await;
        cache.get(key).cloned()
    }

    /// Insert a value into the cache
    pub async fn put(&self, key: K, value: V) {
        let mut cache = self.cache.write().await;
        
        // If we're at capacity, remove the oldest entry
        if cache.len() >= self.capacity {
            if let Some(oldest_key) = cache.keys().next().cloned() {
                cache.remove(&oldest_key);
            }
        }
        
        cache.insert(key, value);
    }

    /// Remove a value from the cache
    pub async fn remove(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.write().await;
        cache.remove(key)
    }

    /// Check if the cache contains a key
    pub async fn contains(&self, key: &K) -> bool {
        let cache = self.cache.read().await;
        cache.contains_key(key)
    }

    /// Get the current size of the cache
    pub async fn len(&self) -> usize {
        let cache = self.cache.read().await;
        cache.len()
    }

    /// Check if the cache is empty
    pub async fn is_empty(&self) -> bool {
        let cache = self.cache.read().await;
        cache.is_empty()
    }

    /// Clear the cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lru_cache_basic() {
        let cache = LruCache::new(3);
        
        cache.put("key1", "value1").await;
        cache.put("key2", "value2").await;
        cache.put("key3", "value3").await;
        
        assert_eq!(cache.get(&"key1").await, Some("value1"));
        assert_eq!(cache.get(&"key2").await, Some("value2"));
        assert_eq!(cache.get(&"key3").await, Some("value3"));
    }
    
    #[tokio::test]
    async fn test_lru_cache_capacity() {
        let cache = LruCache::new(2);
        
        cache.put("key1", "value1").await;
        cache.put("key2", "value2").await;
        cache.put("key3", "value3").await; // This should evict key1
        
        assert_eq!(cache.get(&"key1").await, None); // key1 should be evicted
        assert_eq!(cache.get(&"key2").await, Some("value2"));
        assert_eq!(cache.get(&"key3").await, Some("value3"));
    }
    
    #[tokio::test]
    async fn test_lru_cache_remove() {
        let cache = LruCache::new(3);
        
        cache.put("key1", "value1").await;
        cache.put("key2", "value2").await;
        
        let removed = cache.remove(&"key1").await;
        assert_eq!(removed, Some("value1"));
        assert_eq!(cache.get(&"key1").await, None);
        assert_eq!(cache.get(&"key2").await, Some("value2"));
    }
    
    #[tokio::test]
    async fn test_lru_cache_clear() {
        let cache = LruCache::new(3);
        
        cache.put("key1", "value1").await;
        cache.put("key2", "value2").await;
        
        assert!(!cache.is_empty().await);
        
        cache.clear().await;
        assert!(cache.is_empty().await);
    }
}