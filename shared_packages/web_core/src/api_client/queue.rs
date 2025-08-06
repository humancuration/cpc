//! API request batching queue
//!
//! This module provides functionality for batching API requests to improve performance
//! and reduce the number of network calls.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use wasm_bindgen::JsValue;

/// A batched API request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest {
    /// Unique identifier for the request
    pub id: String,
    
    /// The API endpoint
    pub endpoint: String,
    
    /// HTTP method
    pub method: HttpMethod,
    
    /// Request body (if applicable)
    pub body: Option<serde_json::Value>,
    
    /// Request headers
    pub headers: std::collections::HashMap<String, String>,
    
    /// Timestamp when the request was added to the queue
    pub queued_at: u64,
}

/// HTTP methods supported for batching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HttpMethod {
    /// GET request
    Get,
    
    /// POST request
    Post,
    
    /// PUT request
    Put,
    
    /// PATCH request
    Patch,
    
    /// DELETE request
    Delete,
}

/// A batch of requests to be sent together
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBatch {
    /// The requests in this batch
    pub requests: Vec<BatchRequest>,
    
    /// Timestamp when the batch was created
    pub created_at: u64,
    
    /// Maximum number of requests allowed in a batch
    pub max_size: usize,
}

/// Request batching queue
pub struct BatchQueue {
    /// Queue of pending requests
    queue: VecDeque<BatchRequest>,
    
    /// Maximum batch size
    max_batch_size: usize,
    
    /// Maximum time to wait before sending a batch (in milliseconds)
    max_wait_time_ms: u64,
    
    /// Timestamp of when the current batch started forming
    batch_start_time: Option<u64>,
}

impl BatchQueue {
    /// Create a new batch queue with the specified configuration
    pub fn new(max_batch_size: usize, max_wait_time_ms: u64) -> Self {
        Self {
            queue: VecDeque::new(),
            max_batch_size,
            max_wait_time_ms,
            batch_start_time: None,
        }
    }
    
    /// Add a request to the queue
    pub fn add_request(&mut self, request: BatchRequest) {
        self.queue.push_back(request);
        
        // Set the batch start time if this is the first request
        if self.batch_start_time.is_none() {
            self.batch_start_time = Some(self.current_timestamp());
        }
    }
    
    /// Get the next batch of requests that are ready to be sent
    pub fn get_next_batch(&mut self) -> Option<RequestBatch> {
        if self.queue.is_empty() {
            return None;
        }
        
        let now = self.current_timestamp();
        let should_send_batch = 
            // Send if we've reached the maximum batch size
            self.queue.len() >= self.max_batch_size ||
            // Send if we've exceeded the maximum wait time
            self.batch_start_time.map_or(false, |start| {
                now - start >= self.max_wait_time_ms
            }) ||
            // Send if there are requests and we're past the wait time
            self.batch_start_time.is_none();
        
        if should_send_batch {
            let batch_size = std::cmp::min(self.queue.len(), self.max_batch_size);
            let requests: Vec<BatchRequest> = self.queue.drain(..batch_size).collect();
            
            // Reset the batch start time
            self.batch_start_time = if self.queue.is_empty() {
                None
            } else {
                Some(now)
            };
            
            Some(RequestBatch {
                requests,
                created_at: now,
                max_size: self.max_batch_size,
            })
        } else {
            None
        }
    }
    
    /// Get the number of pending requests in the queue
    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
    
    /// Check if there are any pending requests
    pub fn has_pending_requests(&self) -> bool {
        !self.queue.is_empty()
    }
    
    /// Clear all pending requests
    pub fn clear(&mut self) {
        self.queue.clear();
        self.batch_start_time = None;
    }
    
    /// Get the current timestamp in milliseconds
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, we would use:
        // web_sys::window().unwrap().performance().unwrap().now() as u64
        // For now, we'll use a mock timestamp
        0
    }
}

impl Default for BatchQueue {
    fn default() -> Self {
        Self::new(10, 1000) // Default to 10 requests or 1 second
    }
}

/// Batch processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    /// Results for each request in the batch
    pub results: Vec<BatchRequestResult>,
    
    /// Whether the batch was processed successfully
    pub success: bool,
    
    /// Any error that occurred during batch processing
    pub error: Option<String>,
}

/// Result for an individual request in a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequestResult {
    /// The ID of the request this result is for
    pub request_id: String,
    
    /// The response data (if successful)
    pub data: Option<serde_json::Value>,
    
    /// Any error that occurred for this request
    pub error: Option<String>,
    
    /// HTTP status code
    pub status_code: Option<u16>,
}

/// Trait for batchable API services
#[async_trait(?Send)]
pub trait BatchableService {
    /// Process a batch of requests
    async fn process_batch(&self, batch: RequestBatch) -> Result<BatchResult, JsValue>;
    
    /// Process a single request (fallback when batching is not possible)
    async fn process_single(&self, request: BatchRequest) -> Result<BatchRequestResult, JsValue>;
}