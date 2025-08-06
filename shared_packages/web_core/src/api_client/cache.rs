//! API client caching implementation
//!
//! This module provides caching functionality for the API client with support
//! for TTL, LRU eviction, and multiple storage backends.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::window;

/// Cache configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries in memory cache
    pub max_memory_entries: usize,
    
    /// Maximum number of entries in disk cache
    pub max_disk_entries: usize,
    
    /// Default TTL in milliseconds
    pub default_ttl_ms: u64,
    
    /// Eviction policy
    pub eviction_policy: EvictionPolicy,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_memory_entries: 1000,
            max_disk_entries: 10000,
            default_ttl_ms: 5 * 60 * 1000, // 5 minutes
            eviction_policy: EvictionPolicy::LRU,
        }
    }
}

/// Eviction policies
#[derive(Debug, Clone)]
pub enum EvictionPolicy {
    /// Least Recently Used
    LRU,
    
    /// First In First Out
    FIFO,
    
    /// Time-based eviction
    TimeBased,
}

/// Cache entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    /// Cached data
    pub data: T,
    
    /// Timestamp when entry was created
    pub timestamp: u64,
    
    /// Time to live in milliseconds (None means no expiration)
    pub ttl: Option<u64>,
    
    /// Number of times this entry has been accessed
    pub accessed_count: u32,
}

impl<T> CacheEntry<T> {
    /// Create a new cache entry
    pub fn new(data: T, ttl: Option<u64>) -> Self {
        Self {
            data,
            timestamp: Self::current_timestamp(),
            ttl,
            accessed_count: 0,
        }
    }
    
    /// Check if the entry has expired
    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl {
            let now = Self::current_timestamp();
            now - self.timestamp > ttl
        } else {
            false
        }
    }
    
    /// Mark the entry as accessed
    pub fn mark_accessed(&mut self) {
        self.accessed_count += 1;
    }
    
    /// Get the current timestamp in milliseconds
    fn current_timestamp() -> u64 {
        // In a real implementation, we would use:
        // web_sys::window().unwrap().performance().unwrap().now() as u64
        // For now, we'll use a mock timestamp
        0
    }
}

/// In-memory cache implementation
#[derive(Debug)]
pub struct MemoryCache<T> {
    /// Cache entries
    entries: HashMap<String, CacheEntry<T>>,
    
    /// Cache configuration
    config: CacheConfig,
}

impl<T> MemoryCache<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    /// Create a new memory cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            entries: HashMap::new(),
            config,
        }
    }
    
    /// Get a value from the cache
    pub fn get(&mut self, key: &str) -> Option<T> {
        if let Some(entry) = self.entries.get_mut(key) {
            if entry.is_expired() {
                self.entries.remove(key);
                None
            } else {
                entry.mark_accessed();
                Some(entry.data.clone())
            }
        } else {
            None
        }
    }
    
    /// Set a value in the cache
    pub fn set(&mut self, key: String, value: T, ttl: Option<u64>) {
        // Check if we need to evict entries
        if self.entries.len() >= self.config.max_memory_entries {
            self.evict_entries();
        }
        
        let entry = CacheEntry::new(value, ttl.or(Some(self.config.default_ttl_ms)));
        self.entries.insert(key, entry);
    }
    
    /// Remove a value from the cache
    pub fn remove(&mut self, key: &str) -> Option<T> {
        self.entries.remove(key).map(|entry| entry.data)
    }
    
    /// Clear all entries from the cache
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    
    /// Evict entries based on the eviction policy
    fn evict_entries(&mut self) {
        match self.config.eviction_policy {
            EvictionPolicy::LRU => {
                // Remove the entry with the lowest accessed count
                if let Some(key) = self
                    .entries
                    .iter()
                    .min_by_key(|(_, entry)| entry.accessed_count)
                    .map(|(key, _)| key.clone())
                {
                    self.entries.remove(&key);
                }
            }
            EvictionPolicy::FIFO => {
                // Remove the oldest entry
                if let Some(key) = self
                    .entries
                    .iter()
                    .min_by_key(|(_, entry)| entry.timestamp)
                    .map(|(key, _)| key.clone())
                {
                    self.entries.remove(&key);
                }
            }
            EvictionPolicy::TimeBased => {
                // Remove the entry with the oldest timestamp
                let now = CacheEntry::<T>::current_timestamp();
                if let Some(key) = self
                    .entries
                    .iter()
                    .min_by_key(|(_, entry)| entry.timestamp)
                    .map(|(key, _)| key.clone())
                {
                    self.entries.remove(&key);
                }
            }
        }
    }
}

/// IndexedDB cache implementation
#[derive(Debug)]
pub struct IndexedDBCache {
    /// Database name
    db_name: String,
    
    /// Cache configuration
    config: CacheConfig,
}

impl IndexedDBCache {
    /// Create a new IndexedDB cache
    pub fn new(db_name: String, config: CacheConfig) -> Self {
        Self { db_name, config }
    }
    
    /// Get a value from the cache
    pub async fn get<T>(&self, key: &str) -> Result<Option<T>, JsValue>
    where
        T: for<'de> Deserialize<'de>,
    {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let storage_key = format!("{}_{}", self.db_name, key);
                if let Ok(Some(json_str)) = storage.get_item(&storage_key) {
                    if let Ok(entry) = serde_json::from_str::<CacheEntry<String>>(&json_str) {
                        if !entry.is_expired() {
                            if let Ok(data) = serde_json::from_str(&entry.data) {
                                return Ok(Some(data));
                            }
                        }
                    }
                }
            }
        }
        Ok(None)
    }
    
    /// Set a value in the cache
    pub async fn set<T>(&self, key: String, value: T, ttl: Option<u64>) -> Result<(), JsValue>
    where
        T: Serialize,
    {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let json_value = serde_json::to_string(&value)
                    .map_err(|_| JsValue::from_str("Failed to serialize value"))?;
                
                let entry = CacheEntry::new(json_value, ttl.or(Some(self.config.default_ttl_ms)));
                let json_entry = serde_json::to_string(&entry)
                    .map_err(|_| JsValue::from_str("Failed to serialize entry"))?;
                
                let storage_key = format!("{}_{}", self.db_name, key);
                storage.set_item(&storage_key, &json_entry)?;
            }
        }
        Ok(())
    }
    
    /// Remove a value from the cache
    pub async fn remove(&self, key: &str) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let storage_key = format!("{}_{}", self.db_name, key);
                storage.remove_item(&storage_key)?;
            }
        }
        Ok(())
    }
    
    /// Clear all entries from the cache
    pub async fn clear(&self) -> Result<(), JsValue> {
        // This is a simplified implementation
        // In a real implementation, we would need to iterate through all items
        Ok(())
    }
}

/// Cache manager that combines memory and disk caching
#[derive(Debug)]
pub struct CacheManager<T> {
    /// Memory cache
    memory_cache: MemoryCache<T>,
    
    /// Disk cache
    disk_cache: IndexedDBCache,
    
    /// Cache configuration
    config: CacheConfig,
}

impl<T> CacheManager<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    /// Create a new cache manager
    pub fn new(config: CacheConfig) -> Self {
        let memory_cache = MemoryCache::new(config.clone());
        let disk_cache = IndexedDBCache::new("web_core_cache".to_string(), config.clone());
        
        Self {
            memory_cache,
            disk_cache,
            config,
        }
    }
    
    /// Get a value from the cache
    pub async fn get(&mut self, key: &str) -> Result<Option<T>, JsValue> {
        // First check memory cache
        if let Some(value) = self.memory_cache.get(key) {
            return Ok(Some(value));
        }
        
        // Then check disk cache
        if let Some(value) = self.disk_cache.get(key).await? {
            // Promote to memory cache
            self.memory_cache.set(key.to_string(), value.clone(), None);
            return Ok(Some(value));
        }
        
        Ok(None)
    }
    
    /// Set a value in the cache
    pub async fn set(&mut self, key: String, value: T, ttl: Option<u64>) -> Result<(), JsValue> {
        // Set in both caches
        self.memory_cache.set(key.clone(), value.clone(), ttl);
        self.disk_cache.set(key, value, ttl).await?;
        Ok(())
    }
    
    /// Remove a value from the cache
    pub async fn remove(&mut self, key: &str) -> Result<(), JsValue> {
        // Remove from both caches
        self.memory_cache.remove(key);
        self.disk_cache.remove(key).await?;
        Ok(())
    }
    
    /// Clear all entries from the cache
    pub async fn clear(&mut self) -> Result<(), JsValue> {
        self.memory_cache.clear();
        self.disk_cache.clear().await?;
        Ok(())
    }
}

impl<T> Default for CacheManager<T>
where
    T: Clone + Serialize + for<'de> Deserialize<'de>,
{
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}