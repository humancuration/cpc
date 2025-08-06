//! gRPC-web client implementation
//!
//! This module provides gRPC-web client functionality for the API client.

use serde::{Deserialize, Serialize};
use tonic::{codegen::InterceptedService, transport::Channel, Request, Status};
use wasm_bindgen::JsValue;

/// gRPC client configuration
#[derive(Debug, Clone)]
pub struct GrpcConfig {
    /// Base URL for the gRPC service
    pub base_url: String,
    
    /// Timeout for requests in milliseconds
    pub timeout_ms: Option<u64>,
    
    /// Whether to use TLS
    pub use_tls: bool,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            timeout_ms: Some(5000),
            use_tls: false,
        }
    }
}

/// gRPC client wrapper
pub struct GrpcClient {
    /// Configuration for the client
    config: GrpcConfig,
}

impl GrpcClient {
    /// Create a new gRPC client with the given configuration
    pub fn new(config: GrpcConfig) -> Self {
        Self { config }
    }
    
    /// Get the base URL for the gRPC service
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }
    
    /// Execute a gRPC call
    pub async fn execute_call<T, U>(
        &self,
        service_name: &str,
        method_name: &str,
        request: T,
    ) -> Result<U, Status>
    where
        T: Serialize,
        U: for<'de> Deserialize<'de>,
    {
        // Serialize the request
        let request_json = serde_json::to_string(&request)
            .map_err(|e| Status::internal(format!("Serialization error: {}", e)))?;
        
        // Create the gRPC-web endpoint URL
        let endpoint = format!("{}/{}/{}", self.config.base_url, service_name, method_name);
        
        // Make the HTTP request
        let response = gloo_net::http::Request::post(&endpoint)
            .header("Content-Type", "application/json")
            .header("X-GRPC-WEB", "1")
            .body(request_json)
            .map_err(|e| Status::internal(format!("Request creation error: {}", e)))?
            .send()
            .await
            .map_err(|e| Status::internal(format!("Network error: {}", e)))?;
        
        // Check the response status
        if !response.ok() {
            return Err(Status::internal(format!(
                "HTTP error {}: {}",
                response.status(),
                response.status_text()
            )));
        }
        
        // Parse the response
        let response_text = response.text().await
            .map_err(|e| Status::internal(format!("Response read error: {}", e)))?;
        
        let response: U = serde_json::from_str(&response_text)
            .map_err(|e| Status::internal(format!("Deserialization error: {}", e)))?;
        
        Ok(response)
    }
}

/// Convert a tonic Status to a JsValue for error handling
pub fn status_to_js_value(status: Status) -> JsValue {
    JsValue::from_str(&format!("gRPC error: {}: {}", status.code(), status.message()))
}

/// Generic gRPC service client
pub struct GrpcServiceClient<T> {
    /// The underlying tonic client
    client: T,
    
    /// Service name for logging and debugging
    service_name: String,
}

impl<T> GrpcServiceClient<T> {
    /// Create a new gRPC service client
    pub fn new(client: T, service_name: String) -> Self {
        Self { client, service_name }
    }
    
    /// Get a reference to the underlying client
    pub fn inner(&self) -> &T {
        &self.client
    }
    
    /// Get a mutable reference to the underlying client
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.client
    }
    
    /// Consume the wrapper and return the underlying client
    pub fn into_inner(self) -> T {
        self.client
    }
    
    /// Get the service name
    pub fn service_name(&self) -> &str {
        &self.service_name
    }
}

/// Trait for gRPC services that can be intercepted
pub trait InterceptableService {
    /// Apply an interceptor to the service
    fn with_interceptor<I>(self, interceptor: I) -> InterceptedService<Self, I>
    where
        Self: Sized,
        I: Fn(Request<()>) -> Result<Request<()>, Status>,
    {
        InterceptedService::new(self, interceptor)
    }
}