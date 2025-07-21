//! Storage abstractions for Cooperative Peer Cloud
//!
//! Provides:
//! - Content-addressable storage interface
//! - LRU cache implementation
//! - Storage metrics
//!
//! Example usage:
//! ```
//! use cpc_lib::storage::{ContentStorage, LruStorage, StorageMetrics};
//! use cpc_lib::crypto::hash_content;
//!
//! let mut storage = LruStorage::new(1024 * 1024 * 100); // 100 MB
//! let data = b"Hello, world!";
//! let content_id = hash_content(data);
//!
//! // Store content
//! storage.put(&content_id, data.to_vec()).unwrap();
//!
//! // Retrieve content
//! let retrieved = storage.get(&content_id).unwrap();
//! assert_eq!(retrieved, data);
//!
//! // Check metrics
//! let metrics = storage.metrics();
//! println!("Storage hits: {}", metrics.hits);
//! ```

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lru::LruCache;
use metrics::{Counter, Gauge};
use std::num::NonZeroUsize;

/// Content-addressable storage interface
pub trait ContentStorage {
    /// Store content and return its content ID
    fn put(&mut self, content_id: &[u8; 32], data: Vec<u8>) -> Result<(), StorageError>;
    
    /// Retrieve content by content ID
    fn get(&mut self, content_id: &[u8; 32]) -> Result<Vec<u8>, StorageError>;
    
    /// Check if content exists
    fn exists(&self, content_id: &[u8; 32]) -> bool;
    
    /// Delete content by content ID
    fn delete(&mut self, content_id: &[u8; 32]) -> Result<(), StorageError>;
    
    /// Get storage metrics
    fn metrics(&self) -> StorageMetrics;
}

/// Storage errors
#[derive(Debug)]
pub enum StorageError {
    NotFound,
    CapacityExceeded,
    IoError(String),
}

/// Storage metrics
#[derive(Default, Clone)]
pub struct StorageMetrics {
    pub items: u64,
    pub size_bytes: u64,
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
}

/// In-memory LRU storage implementation
pub struct LruStorage {
    cache: LruCache<[u8; 32], Vec<u8>>,
    metrics: StorageMetrics,
    max_size: u64,
    current_size: u64,
}

impl LruStorage {
    /// Create new LRU storage with specified maximum size in bytes
    pub fn new(max_size: u64) -> Self {
        let cache = LruCache::new(NonZeroUsize::new(1000).unwrap()); // Default capacity
        LruStorage {
            cache,
            metrics: StorageMetrics::default(),
            max_size,
            current_size: 0,
        }
    }
}

impl ContentStorage for LruStorage {
    fn put(&mut self, content_id: &[u8; 32], data: Vec<u8>) -> Result<(), StorageError> {
        let data_size = data.len() as u64;
        
        // Check capacity
        if data_size > self.max_size {
            return Err(StorageError::CapacityExceeded);
        }
        
        // Evict items until there's enough space
        while self.current_size + data_size > self.max_size {
            if let Some((_, evicted_data)) = self.cache.pop_lru() {
                self.current_size -= evicted_data.len() as u64;
                self.metrics.evictions += 1;
            } else {
                return Err(StorageError::CapacityExceeded);
            }
        }
        
        // Store the item
        self.cache.put(*content_id, data.clone());
        self.current_size += data_size;
        self.metrics.items += 1;
        self.metrics.size_bytes += data_size;
        Ok(())
    }

    fn get(&mut self, content_id: &[u8; 32]) -> Result<Vec<u8>, StorageError> {
        if let Some(data) = self.cache.get(content_id) {
            self.metrics.hits += 1;
            Ok(data.clone())
        } else {
            self.metrics.misses += 1;
            Err(StorageError::NotFound)
        }
    }

    fn exists(&self, content_id: &[u8; 32]) -> bool {
        self.cache.contains(content_id)
    }

    fn delete(&mut self, content_id: &[u8; 32]) -> Result<(), StorageError> {
        if let Some(data) = self.cache.pop(content_id) {
            self.current_size -= data.len() as u64;
            self.metrics.items -= 1;
            self.metrics.size_bytes -= data.len() as u64;
            Ok(())
        } else {
            Err(StorageError::NotFound)
        }
    }

    fn metrics(&self) -> StorageMetrics {
        self.metrics.clone()
    }
}

/// Metrics collector for storage
pub struct StorageMetricsCollector {
    metrics: Arc<Mutex<StorageMetrics>>,
    items_gauge: Gauge,
    size_gauge: Gauge,
    hits_counter: Counter,
    misses_counter: Counter,
    evictions_counter: Counter,
}

impl StorageMetricsCollector {
    pub fn new() -> Self {
        // Initialize metrics (actual implementation would register with metrics registry)
        StorageMetricsCollector {
            metrics: Arc::new(Mutex::new(StorageMetrics::default())),
            items_gauge: Gauge::noop(),
            size_gauge: Gauge::noop(),
            hits_counter: Counter::noop(),
            misses_counter: Counter::noop(),
            evictions_counter: Counter::noop(),
        }
    }

    pub fn update(&self) {
        let metrics = self.metrics.lock().unwrap();
        self.items_gauge.set(metrics.items as f64);
        self.size_gauge.set(metrics.size_bytes as f64);
        // Counters are cumulative so we don't need to update them here
    }
}