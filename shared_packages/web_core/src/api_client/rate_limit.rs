//! API rate limiting functionality
//!
//! This module provides rate limiting for API requests to prevent overwhelming
//! the server and to ensure fair usage.

use std::collections::HashMap;
use wasm_bindgen::JsValue;

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests allowed per time window
    pub max_requests: u32,
    
    /// Time window in milliseconds
    pub time_window_ms: u64,
    
    /// Whether to queue requests that exceed the rate limit
    pub queue_excess: bool,
    
    /// Maximum number of queued requests
    pub max_queue_size: usize,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            time_window_ms: 60000, // 1 minute
            queue_excess: true,
            max_queue_size: 50,
        }
    }
}

/// Rate limiter for API requests
pub struct RateLimiter {
    /// Configuration for the rate limiter
    config: RateLimitConfig,
    
    /// Request timestamps for each endpoint
    request_timestamps: HashMap<String, Vec<u64>>,
    
    /// Queued requests for each endpoint
    queued_requests: HashMap<String, Vec<QueuedRequest>>,
}

/// A queued request
#[derive(Debug, Clone)]
pub struct QueuedRequest {
    /// Timestamp when the request was queued
    pub queued_at: u64,
    
    /// Request data
    pub data: serde_json::Value,
}

impl RateLimiter {
    /// Create a new rate limiter with the given configuration
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            request_timestamps: HashMap::new(),
            queued_requests: HashMap::new(),
        }
    }
    
    /// Check if a request is allowed for the given endpoint
    pub fn is_allowed(&mut self, endpoint: &str) -> RateLimitResult {
        let now = self.current_timestamp();
        let window_start = now - self.config.time_window_ms;
        
        // Clean up old timestamps
        if let Some(timestamps) = self.request_timestamps.get_mut(endpoint) {
            timestamps.retain(|&timestamp| timestamp >= window_start);
        }
        
        // Check if we're within the rate limit
        let request_count = self.request_timestamps
            .get(endpoint)
            .map_or(0, |timestamps| timestamps.len()) as u32;
        
        if request_count < self.config.max_requests {
            // Record the request
            self.request_timestamps
                .entry(endpoint.to_string())
                .or_insert_with(Vec::new)
                .push(now);
            
            RateLimitResult::Allowed
        } else {
            // Check if we should queue the request
            if self.config.queue_excess {
                // Check if we have space in the queue
                let queue_size = self.queued_requests
                    .get(endpoint)
                    .map_or(0, |queue| queue.len());
                
                if queue_size < self.config.max_queue_size {
                    // Queue the request
                    let queued_request = QueuedRequest {
                        queued_at: now,
                        data: serde_json::Value::Null,
                    };
                    
                    self.queued_requests
                        .entry(endpoint.to_string())
                        .or_insert_with(Vec::new)
                        .push(queued_request);
                    
                    RateLimitResult::Queued
                } else {
                    RateLimitResult::Rejected
                }
            } else {
                RateLimitResult::Rejected
            }
        }
    }
    
    /// Get the number of requests in the current time window for an endpoint
    pub fn request_count(&self, endpoint: &str) -> u32 {
        let now = self.current_timestamp();
        let window_start = now - self.config.time_window_ms;
        
        self.request_timestamps
            .get(endpoint)
            .map_or(0, |timestamps| {
                timestamps.iter().filter(|&&timestamp| timestamp >= window_start).count() as u32
            })
    }
    
    /// Get the number of queued requests for an endpoint
    pub fn queued_count(&self, endpoint: &str) -> usize {
        self.queued_requests
            .get(endpoint)
            .map_or(0, |queue| queue.len())
    }
    
    /// Process queued requests for an endpoint
    pub fn process_queue(&mut self, endpoint: &str) -> Vec<QueuedRequest> {
        self.queued_requests.remove(endpoint).unwrap_or_else(Vec::new)
    }
    
    /// Clear all rate limiting data for an endpoint
    pub fn clear_endpoint(&mut self, endpoint: &str) {
        self.request_timestamps.remove(endpoint);
        self.queued_requests.remove(endpoint);
    }
    
    /// Clear all rate limiting data
    pub fn clear_all(&mut self) {
        self.request_timestamps.clear();
        self.queued_requests.clear();
    }
    
    /// Get the current timestamp in milliseconds
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, we would use:
        // web_sys::window().unwrap().performance().unwrap().now() as u64
        // For now, we'll use a mock timestamp
        0
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new(RateLimitConfig::default())
    }
}

/// Result of a rate limiting check
#[derive(Debug, Clone, PartialEq)]
pub enum RateLimitResult {
    /// Request is allowed
    Allowed,
    
    /// Request is queued due to rate limiting
    Queued,
    
    /// Request is rejected due to rate limiting
    Rejected,
}

/// Trait for services that support rate limiting
pub trait RateLimitedService {
    /// Get the rate limiter for this service
    fn rate_limiter(&self) -> &RateLimiter;
    
    /// Get a mutable reference to the rate limiter for this service
    fn rate_limiter_mut(&mut self) -> &mut RateLimiter;
    
    /// Check if a request is allowed for the given endpoint
    fn check_rate_limit(&mut self, endpoint: &str) -> RateLimitResult {
        self.rate_limiter_mut().is_allowed(endpoint)
    }
}

/// Rate limiting middleware
pub struct RateLimitMiddleware<T> {
    /// The underlying service
    service: T,
    
    /// Rate limiter
    rate_limiter: RateLimiter,
}

impl<T> RateLimitMiddleware<T> {
    /// Create a new rate limiting middleware
    pub fn new(service: T, rate_limiter: RateLimiter) -> Self {
        Self { service, rate_limiter }
    }
    
    /// Get a reference to the underlying service
    pub fn inner(&self) -> &T {
        &self.service
    }
    
    /// Get a mutable reference to the underlying service
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.service
    }
    
    /// Consume the middleware and return the underlying service
    pub fn into_inner(self) -> T {
        self.service
    }
}

impl<T> RateLimitedService for RateLimitMiddleware<T> {
    fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }
    
    fn rate_limiter_mut(&mut self) -> &mut RateLimiter {
        &mut self.rate_limiter
    }
}