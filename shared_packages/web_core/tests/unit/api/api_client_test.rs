//! Tests for the API client
//!
//! This module contains tests for the API client functionality.

use wasm_bindgen_test::*;
use web_core::api_client::{ApiClient, ApiResponse};
use web_core::api_client::queue::{BatchQueue, BatchRequest, HttpMethod};
use web_core::api_client::rate_limit::{RateLimiter, RateLimitConfig, RateLimitResult};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_api_client_creation() {
    let client = ApiClient::new("https://api.example.com".to_string());
    assert_eq!(client.base_url(), "https://api.example.com");
}

#[wasm_bindgen_test]
fn test_batch_queue_functionality() {
    let mut queue = BatchQueue::new(5, 1000);
    
    let request = BatchRequest {
        id: "1".to_string(),
        endpoint: "/api/test".to_string(),
        method: HttpMethod::Get,
        body: None,
        headers: std::collections::HashMap::new(),
        queued_at: 0,
    };
    
    queue.add_request(request);
    assert_eq!(queue.pending_count(), 1);
    assert!(queue.has_pending_requests());
}

#[wasm_bindgen_test]
fn test_rate_limiter_functionality() {
    let config = RateLimitConfig {
        max_requests: 2,
        time_window_ms: 1000,
        queue_excess: false,
        max_queue_size: 0,
    };
    
    let mut limiter = RateLimiter::new(config);
    
    // First request should be allowed
    assert_eq!(limiter.is_allowed("/api/test"), RateLimitResult::Allowed);
    
    // Second request should be allowed
    assert_eq!(limiter.is_allowed("/api/test"), RateLimitResult::Allowed);
    
    // Third request should be rejected
    assert_eq!(limiter.is_allowed("/api/test"), RateLimitResult::Rejected);
}

#[wasm_bindgen_test]
fn test_offline_mode() {
    let mut client = ApiClient::new("https://api.example.com".to_string());
    
    // Should be offline by default
    assert_eq!(client.is_offline_mode(), false);
    
    // Enable offline mode
    client.set_offline_mode(true);
    assert_eq!(client.is_offline_mode(), true);
    
    // Disable offline mode
    client.set_offline_mode(false);
    assert_eq!(client.is_offline_mode(), false);
}

#[wasm_bindgen_test]
fn test_grpc_config() {
    let client = ApiClient::new("https://api.example.com".to_string());
    let grpc_client = client.grpc_client();
    
    // The base URL should match
    assert_eq!(grpc_client.base_url(), "https://api.example.com");
}