//! Memory management for Shtairir execution
//!
//! This module provides memory pool management, garbage collection,
//! and memory profiling capabilities for efficient execution of Shtairir programs.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use shtairir_core::error::ShtairirError;
use shtairir_registry::value::Value;

/// Memory manager for Shtairir execution
pub struct MemoryManager {
    /// Memory pools for different types of data
    pools: Arc<Mutex<HashMap<MemoryPoolId, MemoryPool>>>,
    
    /// Garbage collector
    garbage_collector: Arc<Mutex<GarbageCollector>>,
    
    /// Memory statistics
    stats: Arc<Mutex<MemoryStats>>,
}

/// Unique identifier for a memory pool
pub type MemoryPoolId = String;

/// Memory pool for managing allocations of a specific type
pub struct MemoryPool {
    /// Pool identifier
    id: MemoryPoolId,
    
    /// Allocated blocks
    blocks: HashSet<usize>,
    
    /// Pool configuration
    config: MemoryPoolConfig,
    
    /// Last cleanup time
    last_cleanup: Instant,
}

/// Configuration for a memory pool
#[derive(Debug, Clone)]
pub struct MemoryPoolConfig {
    /// Maximum size of the pool
    pub max_size: usize,
    
    /// Block size for allocations
    pub block_size: usize,
    
    /// Cleanup interval
    pub cleanup_interval: Duration,
    
    /// Whether to enable garbage collection for this pool
    pub gc_enabled: bool,
}

impl Default for MemoryPoolConfig {
    fn default() -> Self {
        Self {
            max_size: 1024 * 1024, // 1MB default
            block_size: 1024,      // 1KB blocks
            cleanup_interval: Duration::from_secs(30),
            gc_enabled: true,
        }
    }
}

/// Garbage collector for automatic memory management
pub struct GarbageCollector {
    /// Tracked objects
    tracked_objects: HashSet<usize>,
    
    /// Reference counts
    ref_counts: HashMap<usize, usize>,
    
    /// Collection threshold
    collection_threshold: usize,
    
    /// Last collection time
    last_collection: Instant,
}

impl GarbageCollector {
    /// Create a new garbage collector
    pub fn new(threshold: usize) -> Self {
        Self {
            tracked_objects: HashSet::new(),
            ref_counts: HashMap::new(),
            collection_threshold: threshold,
            last_collection: Instant::now(),
        }
    }
    
    /// Track an object
    pub fn track_object(&mut self, obj_id: usize) {
        self.tracked_objects.insert(obj_id);
        self.ref_counts.insert(obj_id, 1);
    }
    
    /// Increment reference count
    pub fn increment_ref(&mut self, obj_id: usize) {
        if let Some(count) = self.ref_counts.get_mut(&obj_id) {
            *count += 1;
        }
    }
    
    /// Decrement reference count
    pub fn decrement_ref(&mut self, obj_id: usize) {
        if let Some(count) = self.ref_counts.get_mut(&obj_id) {
            *count = count.saturating_sub(1);
        }
    }
    
    /// Collect garbage
    pub fn collect_garbage(&mut self) -> Vec<usize> {
        let mut collected = Vec::new();
        
        // Find objects with zero references
        let to_remove: Vec<usize> = self.ref_counts
            .iter()
            .filter(|(_, &count)| count == 0)
            .map(|(&id, _)| id)
            .collect();
        
        // Remove collected objects
        for obj_id in to_remove {
            self.tracked_objects.remove(&obj_id);
            self.ref_counts.remove(&obj_id);
            collected.push(obj_id);
        }
        
        self.last_collection = Instant::now();
        collected
    }
    
    /// Check if collection is needed
    pub fn should_collect(&self) -> bool {
        self.tracked_objects.len() >= self.collection_threshold ||
            self.last_collection.elapsed() > Duration::from_secs(60)
    }
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    /// Total allocated memory
    pub total_allocated: usize,
    
    /// Total freed memory
    pub total_freed: usize,
    
    /// Current memory usage
    pub current_usage: usize,
    
    /// Number of garbage collections
    pub gc_count: usize,
    
    /// Total objects collected
    pub total_collected: usize,
}

impl MemoryStats {
    /// Create new memory statistics
    pub fn new() -> Self {
        Self {
            total_allocated: 0,
            total_freed: 0,
            current_usage: 0,
            gc_count: 0,
            total_collected: 0,
        }
    }
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
            garbage_collector: Arc::new(Mutex::new(GarbageCollector::new(1000))),
            stats: Arc::new(Mutex::new(MemoryStats::new())),
        }
    }
    
    /// Create a new memory pool
    pub fn create_pool(&self, id: MemoryPoolId, config: MemoryPoolConfig) -> Result<(), ShtairirError> {
        let mut pools = self.pools.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        pools.insert(id, MemoryPool {
            id: config.clone(),
            blocks: HashSet::new(),
            config,
            last_cleanup: Instant::now(),
        });
        Ok(())
    }
    
    /// Allocate memory from a pool
    pub fn allocate(&self, pool_id: &MemoryPoolId, size: usize) -> Result<usize, ShtairirError> {
        let mut pools = self.pools.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        let pool = pools.get_mut(pool_id)
            .ok_or_else(|| ShtairirError::Io("Pool not found".to_string()))?;
        
        // Check if cleanup is needed
        if pool.last_cleanup.elapsed() > pool.config.cleanup_interval {
            self.cleanup_pool(pool_id)?;
        }
        
        // Simple allocation strategy - just return a unique identifier
        // In a real implementation, this would allocate actual memory
        let block_id = pool.blocks.len();
        pool.blocks.insert(block_id);
        
        // Update statistics
        let mut stats = self.stats.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        stats.total_allocated += size;
        stats.current_usage += size;
        
        Ok(block_id)
    }
    
    /// Free memory from a pool
    pub fn free(&self, pool_id: &MemoryPoolId, block_id: usize, size: usize) -> Result<(), ShtairirError> {
        let mut pools = self.pools.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        let pool = pools.get_mut(pool_id)
            .ok_or_else(|| ShtairirError::Io("Pool not found".to_string()))?;
        
        if pool.blocks.remove(&block_id) {
            // Update statistics
            let mut stats = self.stats.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
            stats.total_freed += size;
            stats.current_usage = stats.current_usage.saturating_sub(size);
            Ok(())
        } else {
            Err(ShtairirError::Io("Block not found in pool".to_string()))
        }
    }
    
    /// Cleanup a pool by removing unused blocks
    pub fn cleanup_pool(&self, pool_id: &MemoryPoolId) -> Result<(), ShtairirError> {
        let mut pools = self.pools.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        let pool = pools.get_mut(pool_id)
            .ok_or_else(|| ShtairirError::Io("Pool not found".to_string()))?;
        
        // In a real implementation, this would actually free memory
        // For now, we just update the cleanup time
        pool.last_cleanup = Instant::now();
        Ok(())
    }
    
    /// Track an object for garbage collection
    pub fn track_object(&self, obj_id: usize) -> Result<(), ShtairirError> {
        let mut gc = self.garbage_collector.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        gc.track_object(obj_id);
        Ok(())
    }
    
    /// Increment reference count for an object
    pub fn increment_ref(&self, obj_id: usize) -> Result<(), ShtairirError> {
        let mut gc = self.garbage_collector.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        gc.increment_ref(obj_id);
        Ok(())
    }
    
    /// Decrement reference count for an object
    pub fn decrement_ref(&self, obj_id: usize) -> Result<(), ShtairirError> {
        let mut gc = self.garbage_collector.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        gc.decrement_ref(obj_id);
        
        // Check if garbage collection is needed
        if gc.should_collect() {
            let collected = gc.collect_garbage();
            
            // Update statistics
            let mut stats = self.stats.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
            stats.gc_count += 1;
            stats.total_collected += collected.len();
        }
        
        Ok(())
    }
    
    /// Get current memory statistics
    pub fn get_stats(&self) -> Result<MemoryStats, ShtairirError> {
        let stats = self.stats.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        Ok(stats.clone())
    }
    
    /// Force garbage collection
    pub fn force_gc(&self) -> Result<usize, ShtairirError> {
        let mut gc = self.garbage_collector.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        let collected = gc.collect_garbage();
        
        // Update statistics
        let mut stats = self.stats.lock().map_err(|e| ShtairirError::Io(e.to_string()))?;
        stats.gc_count += 1;
        stats.total_collected += collected.len();
        
        Ok(collected.len())
    }
}

impl Default for MemoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory profiler for monitoring memory usage
pub struct MemoryProfiler {
    /// Profiling start time
    start_time: Instant,
    
    /// Memory usage samples
    samples: Vec<(Instant, usize)>,
    
    /// Sampling interval
    sampling_interval: Duration,
}

impl MemoryProfiler {
    /// Create a new memory profiler
    pub fn new(sampling_interval: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            samples: Vec::new(),
            sampling_interval,
        }
    }
    
    /// Take a memory usage sample
    pub fn sample(&mut self, memory_manager: &MemoryManager) -> Result<(), ShtairirError> {
        let stats = memory_manager.get_stats()?;
        let now = Instant::now();
        self.samples.push((now, stats.current_usage));
        Ok(())
    }
    
    /// Get memory usage over time
    pub fn get_usage_over_time(&self) -> &[(Instant, usize)] {
        &self.samples
    }
    
    /// Get average memory usage
    pub fn average_usage(&self) -> usize {
        if self.samples.is_empty() {
            0
        } else {
            self.samples.iter().map(|(_, usage)| usage).sum::<usize>() / self.samples.len()
        }
    }
    
    /// Get peak memory usage
    pub fn peak_usage(&self) -> usize {
        self.samples.iter().map(|(_, usage)| usage).max().copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_manager_creation() {
        let manager = MemoryManager::new();
        assert!(manager.pools.lock().unwrap().is_empty());
    }
    
    #[test]
    fn test_memory_pool_config_default() {
        let config = MemoryPoolConfig::default();
        assert_eq!(config.max_size, 1024 * 1024);
        assert_eq!(config.block_size, 1024);
        assert_eq!(config.cleanup_interval, Duration::from_secs(30));
        assert_eq!(config.gc_enabled, true);
    }
    
    #[test]
    fn test_garbage_collector() {
        let mut gc = GarbageCollector::new(100);
        gc.track_object(1);
        gc.track_object(2);
        
        assert!(gc.tracked_objects.contains(&1));
        assert!(gc.tracked_objects.contains(&2));
        assert_eq!(gc.ref_counts.get(&1), Some(&1));
        assert_eq!(gc.ref_counts.get(&2), Some(&1));
        
        gc.increment_ref(1);
        assert_eq!(gc.ref_counts.get(&1), Some(&2));
        
        gc.decrement_ref(1);
        assert_eq!(gc.ref_counts.get(&1), Some(&1));
        
        gc.decrement_ref(1);
        gc.decrement_ref(2);
        
        let collected = gc.collect_garbage();
        assert_eq!(collected.len(), 2);
        assert!(!gc.tracked_objects.contains(&1));
        assert!(!gc.tracked_objects.contains(&2));
    }
    
    #[test]
    fn test_memory_stats() {
        let stats = MemoryStats::new();
        assert_eq!(stats.total_allocated, 0);
        assert_eq!(stats.total_freed, 0);
        assert_eq!(stats.current_usage, 0);
        assert_eq!(stats.gc_count, 0);
        assert_eq!(stats.total_collected, 0);
    }
    
    #[test]
    fn test_memory_profiler() {
        let mut profiler = MemoryProfiler::new(Duration::from_secs(1));
        let manager = MemoryManager::new();
        
        // Take a sample
        profiler.sample(&manager).unwrap();
        
        assert_eq!(profiler.get_usage_over_time().len(), 1);
        assert_eq!(profiler.average_usage(), 0);
        assert_eq!(profiler.peak_usage(), 0);
    }
}