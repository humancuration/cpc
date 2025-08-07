//! Memory management for the data lakehouse
//!
//! This module provides memory management capabilities for handling large datasets
//! in web contexts with appropriate limits and downsampling.

use crate::domain::models::{DataCapabilities, DataAsset};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Memory management service for data operations
pub struct MemoryManager {
    capabilities: DataCapabilities,
}

impl MemoryManager {
    /// Create a new memory manager with the specified capabilities
    pub fn new(capabilities: DataCapabilities) -> Self {
        Self { capabilities }
    }
    
    /// Create a new memory manager with web defaults
    pub fn new_web_default() -> Self {
        Self::new(DataCapabilities::new_web_default())
    }
    
    /// Create a new memory manager with desktop defaults
    pub fn new_desktop_default() -> Self {
        Self::new(DataCapabilities::new_desktop_default())
    }
    
    /// Check if a data asset can be loaded within memory constraints
    pub fn can_load_asset(&self, asset: &DataAsset, estimated_size_bytes: usize) -> Result<bool, String> {
        // Check memory limit
        if self.capabilities.exceeds_memory_limit(estimated_size_bytes) {
            return Ok(false);
        }
        
        // Additional checks could be added here
        Ok(true)
    }
    
    /// Get recommended processing strategy for a data asset
    pub fn get_processing_strategy(&self, row_count: usize, estimated_size_bytes: usize) -> ProcessingStrategy {
        // Check memory constraints first
        if self.capabilities.exceeds_memory_limit(estimated_size_bytes) {
            return ProcessingStrategy::Stream;
        }
        
        // Check row count constraints
        if self.capabilities.exceeds_row_limit(row_count) {
            if self.capabilities.auto_downsample {
                if let Some(sample_size) = self.capabilities.recommended_sample_size(row_count) {
                    return ProcessingStrategy::Downsample(sample_size);
                }
            }
            return ProcessingStrategy::Stream;
        }
        
        ProcessingStrategy::LoadFull
    }
    
    /// Estimate memory usage for a given number of rows
    pub fn estimate_memory_usage(&self, row_count: usize, avg_row_size_bytes: usize) -> usize {
        row_count * avg_row_size_bytes
    }
}

/// Processing strategies for handling data assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingStrategy {
    /// Load the full dataset into memory
    LoadFull,
    
    /// Downsample the dataset to a smaller size
    Downsample(usize),
    
    /// Stream the dataset in chunks
    Stream,
}

/// Web Worker message for data processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkerMessage {
    /// Request to process a data asset
    ProcessAsset {
        asset_id: uuid::Uuid,
        strategy: ProcessingStrategy,
    },
    
    /// Request to load a sample of data
    LoadSample {
        asset_id: uuid::Uuid,
        sample_size: usize,
    },
    
    /// Request to stream data in chunks
    StreamData {
        asset_id: uuid::Uuid,
        chunk_size: usize,
    },
    
    /// Progress update from worker
    Progress {
        asset_id: uuid::Uuid,
        processed_rows: usize,
        total_rows: usize,
        message: String,
    },
    
    /// Result from worker processing
    Result {
        asset_id: uuid::Uuid,
        success: bool,
        data: Option<Vec<u8>>, // Serialized data result
        error: Option<String>,
    },
}

/// Web Worker interface for data processing
#[cfg(target_arch = "wasm32")]
pub struct WebWorkerInterface {
    worker: web_sys::Worker,
}

#[cfg(target_arch = "wasm32")]
impl WebWorkerInterface {
    /// Create a new web worker interface
    pub fn new(script_url: &str) -> Result<Self, String> {
        let worker = web_sys::Worker::new(script_url)
            .map_err(|e| format!("Failed to create web worker: {:?}", e))?;
        Ok(Self { worker })
    }
    
    /// Send a message to the web worker
    pub fn send_message(&self, message: &WorkerMessage) -> Result<(), String> {
        let json_message = serde_json::to_string(message)
            .map_err(|e| format!("Failed to serialize message: {}", e))?;
        
        self.worker.post_message(&json_message.into())
            .map_err(|e| format!("Failed to send message to worker: {:?}", e))
    }
    
    /// Set up message handler for worker responses
    pub fn set_on_message<F>(&self, callback: F) -> Result<(), String>
    where
        F: Fn(WorkerMessage) + 'static,
    {
        let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            if let Ok(text) = event.data().as_string() {
                if let Ok(message) = serde_json::from_str::<WorkerMessage>(&text) {
                    callback(message);
                }
            }
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);
        
        self.worker.set_onmessage(Some(callback.as_ref().unchecked_ref()));
        callback.forget(); // Prevent cleanup
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use serde_json::json;
    
    #[test]
    fn test_memory_manager_creation() {
        let manager = MemoryManager::new_web_default();
        assert!(manager.capabilities.streaming);
        assert_eq!(manager.capabilities.max_rows, Some(5000));
        assert_eq!(manager.capabilities.memory_limit_bytes, Some(5 * 1024 * 1024));
    }
    
    #[test]
    fn test_processing_strategy() {
        let manager = MemoryManager::new_web_default();
        
        // Test full load within limits
        let strategy = manager.get_processing_strategy(1000, 1024);
        assert!(matches!(strategy, ProcessingStrategy::LoadFull));
        
        // Test downsampling when exceeding row limit
        let strategy = manager.get_processing_strategy(10000, 1024);
        assert!(matches!(strategy, ProcessingStrategy::Downsample(5000)));
        
        // Test streaming when exceeding memory limit
        let strategy = manager.get_processing_strategy(1000, 10 * 1024 * 1024);
        assert!(matches!(strategy, ProcessingStrategy::Stream));
    }
    
    #[test]
    fn test_memory_estimation() {
        let manager = MemoryManager::new_web_default();
        let estimated = manager.estimate_memory_usage(1000, 1024);
        assert_eq!(estimated, 1000 * 1024);
    }
}