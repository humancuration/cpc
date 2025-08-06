//! Main API client implementation
//!
//! This module provides the main API client that combines GraphQL, gRPC-web,
//! request batching, and rate limiting functionality.

use super::cache::CacheManager;
use super::grpc::GrpcClient;
use super::queue::BatchQueue;
use super::rate_limit::RateLimiter;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::JsValue;
use web_sys::window;
/// Main service for handling API calls with all features
pub struct ApiClient {
    base_url: String,
    offline_mode: bool,
    grpc_client: GrpcClient,
    batch_queue: BatchQueue,
    rate_limiter: RateLimiter,
    cache_manager: CacheManager<Value>,
}
}

/// Response from an API call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<String>>,
    pub cached: bool,
}

impl ApiClient {
    /// Create a new API client
    pub fn new(base_url: String) -> Self {
        Self {
            base_url: base_url.clone(),
            offline_mode: false,
            grpc_client: GrpcClient::new(super::grpc::GrpcConfig {
                base_url,
                timeout_ms: Some(5000),
                use_tls: false,
            }),
            batch_queue: BatchQueue::default(),
            rate_limiter: RateLimiter::default(),
            cache_manager: CacheManager::default(),
        }
    }
    
    /// Enable or disable offline mode
    pub fn set_offline_mode(&mut self, offline: bool) {
        self.offline_mode = offline;
    }
    
    /// Check if we're currently in offline mode
    pub fn is_offline_mode(&self) -> bool {
        self.offline_mode
    }
    
    /// Check if we're currently online
    pub fn is_online(&self) -> bool {
        // In a real implementation, we would check navigator.onLine
        // For now, we'll return the opposite of offline mode
        !self.offline_mode
    }
    
    /// Set online status
    pub fn set_online(&mut self, online: bool) {
        self.offline_mode = !online;
    }
    
    /// Get a reference to the gRPC client
    pub fn grpc_client(&self) -> &GrpcClient {
        &self.grpc_client
    }
    
    /// Get a mutable reference to the gRPC client
    pub fn grpc_client_mut(&mut self) -> &mut GrpcClient {
        &mut self.grpc_client
    }
    
    /// Get a reference to the batch queue
    pub fn batch_queue(&self) -> &BatchQueue {
        &self.batch_queue
    }
    
    /// Get a mutable reference to the batch queue
    pub fn batch_queue_mut(&mut self) -> &mut BatchQueue {
        &mut self.batch_queue
    }
    
    /// Get a reference to the rate limiter
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }
    
    /// Get a mutable reference to the rate limiter
    pub fn rate_limiter_mut(&mut self) -> &mut RateLimiter {
        &mut self.rate_limiter
    }
    
    /// Execute a GraphQL query
    pub async fn graphql_query<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Option<Value>,
    ) -> Result<ApiResponse<T>, String> {
        // Check rate limiting
        let rate_limit_result = self.rate_limiter.is_allowed("graphql");
        if rate_limit_result == super::rate_limit::RateLimitResult::Rejected {
            return Err("Rate limit exceeded".to_string());
        }
        
        // Check if we're in offline mode
        if self.offline_mode {
            // Try to get cached response
            if let Ok(Some(cached)) = self.cache_manager.get(&self.get_cache_key(query)).await {
                return Ok(ApiResponse {
                    data: Some(cached),
                    errors: None,
                    cached: true,
                });
            }
            
            return Err("Offline mode: No cached response available".to_string());
        }
        
        let mut request_body = serde_json::json!({
            "query": query
        });
        
        if let Some(vars) = variables {
            request_body["variables"] = vars;
        }
        
        let response = Request::post(&format!("{}/graphql", self.base_url))
            .header("Content-Type", "application/json")
            .body(request_body.to_string())
            .send()
            .await
            .map_err(|e| format!("Network error: {:?}", e))?;
            
        let response_text = response.text().await
            .map_err(|e| format!("Failed to read response: {:?}", e))?;
            
        // Try to parse as JSON
        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {:?}", e))?;
            
        // Cache the response for offline use
        if let Err(e) = self.cache_manager.set(self.get_cache_key(query), response_json.clone(), None).await {
            // Log error but don't fail the request
            web_sys::console::warn_1(&format!("Failed to cache response: {:?}", e).into());
        }
            
        // Try to extract data field
        if let Some(data) = response_json.get("data") {
            if let Ok(parsed_data) = serde_json::from_value(data.clone()) {
                return Ok(ApiResponse {
                    data: Some(parsed_data),
                    errors: response_json.get("errors").map(|e| vec![e.to_string()]),
                    cached: false,
                });
            }
        }
        
        // If we can't parse the data field, return the whole response
        if let Ok(parsed_data) = serde_json::from_value(response_json.clone()) {
            Ok(ApiResponse {
                data: Some(parsed_data),
                errors: None,
                cached: false,
            })
        } else {
            Err(format!("Failed to parse response: {}", response_text))
        }
    }
    
    /// Execute a GraphQL mutation
    pub async fn graphql_mutation<T: for<'de> Deserialize<'de>>(
        &self,
        mutation: &str,
        variables: Option<Value>,
    ) -> Result<ApiResponse<T>, String> {
        self.graphql_query(mutation, variables).await
    }
    
    /// Execute a gRPC call
    pub async fn grpc_call<T, U>(
        &self,
        service_name: &str,
        method_name: &str,
        request: T,
    ) -> Result<U, String>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        // Check rate limiting
        let rate_limit_result = self.rate_limiter.is_allowed(&format!("grpc.{}.{}", service_name, method_name));
        if rate_limit_result == super::rate_limit::RateLimitResult::Rejected {
            return Err("Rate limit exceeded".to_string());
        }
        
        self.grpc_client
            .execute_call(service_name, method_name, request)
            .await
            .map_err(|e| format!("gRPC error: {}", e))
    }
    
    /// Add a request to the batch queue
    pub fn queue_request(&mut self, request: super::queue::BatchRequest) {
        // If we're offline, queue the request for later processing
        if self.offline_mode {
            // In a real implementation, we would store the request persistently
            // For now, we'll just add it to the in-memory queue
            self.batch_queue.add_request(request);
        } else {
            self.batch_queue.add_request(request);
        }
    }
    
    /// Process the next batch of requests
    pub async fn process_next_batch(&mut self) -> Option<Result<super::queue::BatchResult, String>> {
        if let Some(batch) = self.batch_queue.get_next_batch() {
            // If we're offline, we can't process the batch
            if self.offline_mode {
                return Some(Err("Offline mode: Cannot process batch requests".to_string()));
            }
            
            // Create the batch request payload
            let batch_payload = serde_json::json!({
                "requests": batch.requests
            });
            
            // Send the batch request to the server
            match Request::post(&format!("{}/api/batch", self.base_url))
                .header("Content-Type", "application/json")
                .body(batch_payload.to_string())
                .send()
                .await
            {
                Ok(response) => {
                    match response.json::<super::queue::BatchResult>().await {
                        Ok(batch_result) => Some(Ok(batch_result)),
                        Err(e) => Some(Err(format!("Failed to parse batch response: {:?}", e))),
                    }
                }
                Err(e) => Some(Err(format!("Failed to send batch request: {:?}", e))),
            }
        } else {
            None
        }
    }
    
    /// Generate a cache key for a query
    fn get_cache_key(&self, query: &str) -> String {
        // In a real implementation, we would use a proper hash function
        // For now, we'll use a simple approach
        format!("api_cache_{}", query)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new("http://localhost:3000".to_string())
    }
}