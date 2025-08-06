//! GraphQL/gRPC-web client with offline support
//!
//! This module provides API client functionality with support for
//! both online and offline modes, automatic retries, and caching.

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::JsValue;
use web_sys::window;

/// Service for handling API calls with offline support
pub struct ApiClient {
    base_url: String,
    offline_mode: bool,
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
            base_url,
            offline_mode: false,
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
    
    /// Execute a GraphQL query
    pub async fn graphql_query<T: for<'de> Deserialize<'de>>(
        &self,
        query: &str,
        variables: Option<Value>,
    ) -> Result<ApiResponse<T>, String> {
        // Check if we're in offline mode
        if self.offline_mode {
            // Try to get cached response
            if let Some(cached) = self.get_cached_response(query)? {
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
        if let Err(e) = self.cache_response(query, &response_json) {
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
    
    /// Cache a response for offline use
    fn cache_response(&self, query: &str, response: &Value) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let key = format!("api_cache_{}", md5::compute(query));
                    let value = serde_json::to_string(response)
                        .map_err(|_| JsValue::from_str("Failed to serialize response"))?;
                    storage.set_item(&key, &value)?;
                }
            }
        }
        Ok(())
    }
    
    /// Get a cached response
    fn get_cached_response<T: for<'de> Deserialize<'de>>(&self, query: &str) -> Result<Option<T>, String> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let key = format!("api_cache_{}", md5::compute(query));
                    if let Ok(Some(cached_json)) = storage.get_item(&key) {
                        if let Ok(cached_data) = serde_json::from_str(&cached_json) {
                            return Ok(Some(cached_data));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new("http://localhost:3000".to_string())
    }
}