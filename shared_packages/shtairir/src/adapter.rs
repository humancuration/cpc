//! Adapter patterns for external system integration in Shtairir
//! 
//! This module defines adapters that enable integration with external systems,
//! including connection pooling, serialization, authentication, and response processing.

use crate::block::{Block, Value, ExecutionContext, BlockResult, BlockOutputs, BlockInputs};
use async_trait::async_trait;
use std::sync::Arc;
use shtairir_core::error::ShtairirError;

/// External system adapter
pub struct ExternalSystemAdapter {
    /// Adapter specification
    spec: ExternalSystemAdapterSpec,
    
    /// Connection pool
    // connection_pool: Arc<dyn ConnectionPool>,
    
    /// Message serializer/deserializer
    codec: Box<dyn MessageCodec>,
    
    /// Authentication provider
    // auth: Arc<dyn AuthenticationProvider>,
    
    /// Configuration
    config: ExternalSystemAdapterConfig,
}

impl ExternalSystemAdapter {
    /// Create a new external system adapter
    pub fn new(
        spec: ExternalSystemAdapterSpec,
        codec: Box<dyn MessageCodec>,
        config: ExternalSystemAdapterConfig,
    ) -> Self {
        Self {
            spec,
            codec,
            // connection_pool,
            // auth,
            config,
        }
    }
    
    /// Send a request to the external system
    pub async fn send_request(&self, request: &Value) -> Result<Value, ProcessingError> {
        // TODO: Implement actual request sending logic
        // This would involve:
        // 1. Getting a connection from the pool
        // 2. Serializing the request
        // 3. Adding authentication headers
        // 4. Sending the request
        // 5. Receiving and deserializing the response
        // 6. Returning the response
        
        // For now, we'll just return a placeholder
        Ok(Value::string("response from external system"))
    }
}

/// External system adapter specification
#[derive(Debug, Clone)]
pub struct ExternalSystemAdapterSpec {
    /// Adapter name
    pub name: String,
    
    /// Target system URL or endpoint
    pub target: String,
    
    /// Protocol (HTTP, gRPC, WebSocket, etc.)
    pub protocol: String,
    
    /// Supported operations
    pub operations: Vec<String>,
    
    /// Required authentication method
    pub auth_method: Option<String>,
    
    /// Supported content types
    pub content_types: Vec<String>,
}

/// External system adapter configuration
#[derive(Debug, Clone)]
pub struct ExternalSystemAdapterConfig {
    /// Timeout for requests
    pub timeout_ms: u64,
    
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    
    /// Retry configuration
    pub retry_config: RetryConfig,
    
    /// Custom headers
    pub custom_headers: std::collections::HashMap<String, String>,
    
    /// SSL/TLS configuration
    pub tls_config: Option<TlsConfig>,
}

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    
    /// Initial delay between retries (milliseconds)
    pub initial_delay_ms: u64,
    
    /// Maximum delay between retries (milliseconds)
    pub max_delay_ms: u64,
    
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

/// TLS configuration
#[derive(Debug, Clone)]
pub struct TlsConfig {
    /// Whether to verify certificates
    pub verify_certificates: bool,
    
    /// Path to certificate file
    pub cert_file: Option<std::path::PathBuf>,
    
    /// Path to private key file
    pub key_file: Option<std::path::PathBuf>,
    
    /// Path to CA certificate file
    pub ca_file: Option<std::path::PathBuf>,
}

/// Message codec trait for serialization/deserialization
pub trait MessageCodec: Send + Sync {
    /// Serialize a value to bytes
    fn serialize(&self, value: &Value) -> Result<Vec<u8>, CodecError>;
    
    /// Deserialize bytes to a value
    fn deserialize(&self, data: &[u8]) -> Result<Value, CodecError>;
    
    /// Get the content type
    fn content_type(&self) -> &str;
}

/// JSON codec implementation
pub struct JsonCodec;

impl MessageCodec for JsonCodec {
    fn serialize(&self, value: &Value) -> Result<Vec<u8>, CodecError> {
        // TODO: Implement JSON serialization using serde_json
        // For now, we'll just return placeholder data
        Ok(serde_json::to_vec(value).map_err(|e| CodecError::new(format!("JSON serialization error: {}", e)))?)
    }
    
    fn deserialize(&self, data: &[u8]) -> Result<Value, CodecError> {
        // TODO: Implement JSON deserialization using serde_json
        // For now, we'll just return placeholder data
        Ok(serde_json::from_slice(data).map_err(|e| CodecError::new(format!("JSON deserialization error: {}", e)))?)
    }
    
    fn content_type(&self) -> &str {
        "application/json"
    }
}

/// Codec error
#[derive(Debug, Clone)]
pub struct CodecError {
    /// Error message
    pub message: String,
}

impl CodecError {
    /// Create a new codec error
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CodecError: {}", self.message)
    }
}

impl std::error::Error for CodecError {}

/// External system block
pub struct ExternalSystemBlock {
    /// Block specification
    // spec: BlockSpec, // TODO: Import from registry
    
    /// External system adapter
    adapter: Arc<ExternalSystemAdapter>,
    
    /// Operation to perform
    operation: String,
    
    /// Request template
    request_template: ValueTemplate,
    
    /// Response processor
    response_processor: Arc<dyn ResponseProcessor>,
}

#[async_trait]
impl Block for ExternalSystemBlock {
    // fn spec(&self) -> &BlockSpec {
    //     &self.spec
    // }
    
    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> BlockResult<BlockOutputs> {
        // Build request from inputs and template
        let request = self.build_request(inputs)?;
        
        // Send request to external system
        let response = self.adapter.send_request(&request).await
            .map_err(|e| ShtairirError::Adapter(e.message))?;
        
        // Process response
        let processed_response = self.response_processor.process_response(&response, _context).await
            .map_err(|e| ShtairirError::Adapter(e.message))?;
        
        // Return outputs
        Ok(BlockOutputs::new().with_output("response".to_string(), processed_response))
    }
    
    // fn validate(&self, params: &BlockParams) -> Result<(), ValidationError> {
    //     // TODO: Implement validation
    //     Ok(())
    // }
    
    // fn purity(&self) -> Purity {
    //     Purity::Effect
    // }
    
    // fn determinism(&self) -> Determinism {
    //     Determinism::Nondeterministic
    // }
}

impl ExternalSystemBlock {
    /// Create a new external system block
    pub fn new(
        // spec: BlockSpec,
        adapter: Arc<ExternalSystemAdapter>,
        operation: String,
        request_template: ValueTemplate,
        response_processor: Arc<dyn ResponseProcessor>,
    ) -> Self {
        Self {
            // spec,
            adapter,
            operation,
            request_template,
            response_processor,
        }
    }
    
    /// Build request from inputs and template
    fn build_request(&self, inputs: &BlockInputs) -> Result<Value, ProcessingError> {
        // TODO: Implement request building logic
        // This would involve merging the inputs with the request template
        // For now, we'll just return the template
        Ok(self.request_template.clone())
    }
}

/// Value template for request building
#[derive(Debug, Clone)]
pub struct ValueTemplate(Value);

impl ValueTemplate {
    /// Create a new value template
    pub fn new(value: Value) -> Self {
        Self(value)
    }
    
    /// Get the underlying value
    pub fn value(&self) -> &Value {
        &self.0
    }
}

impl std::ops::Deref for ValueTemplate {
    type Target = Value;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Response processor trait
#[async_trait]
pub trait ResponseProcessor: Send + Sync {
    /// Process a response from the external system
    async fn process_response(&self, response: &Value, context: &ExecutionContext) -> Result<Value, ProcessingError>;
    
    /// Get the processor specification
    // fn spec(&self) -> &ResponseProcessorSpec;
}

/// Default response processor that just returns the response as-is
pub struct DefaultResponseProcessor;

#[async_trait]
impl ResponseProcessor for DefaultResponseProcessor {
    async fn process_response(&self, response: &Value, _context: &ExecutionContext) -> Result<Value, ProcessingError> {
        Ok(response.clone())
    }
}

/// Processing error
#[derive(Debug, Clone)]
pub struct ProcessingError {
    /// Error message
    pub message: String,
    
    /// Error details
    pub details: Option<Value>,
}

impl ProcessingError {
    /// Create a new processing error
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: None,
        }
    }
    
    /// Create a new processing error with details
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

impl std::fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProcessingError: {}", self.message)?;
        if let Some(details) = &self.details {
            write!(f, " ({:?})", details)?;
        }
        Ok(())
    }
}

impl std::error::Error for ProcessingError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_retry_config() {
        let config = RetryConfig {
            max_attempts: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            backoff_multiplier: 2.0,
        };
        
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay_ms, 100);
        assert_eq!(config.max_delay_ms, 5000);
        assert_eq!(config.backoff_multiplier, 2.0);
    }
    
    #[test]
    fn test_tls_config() {
        let config = TlsConfig {
            verify_certificates: true,
            cert_file: Some(std::path::PathBuf::from("/path/to/cert.pem")),
            key_file: Some(std::path::PathBuf::from("/path/to/key.pem")),
            ca_file: Some(std::path::PathBuf::from("/path/to/ca.pem")),
        };
        
        assert_eq!(config.verify_certificates, true);
        assert_eq!(config.cert_file, Some(std::path::PathBuf::from("/path/to/cert.pem")));
        assert_eq!(config.key_file, Some(std::path::PathBuf::from("/path/to/key.pem")));
        assert_eq!(config.ca_file, Some(std::path::PathBuf::from("/path/to/ca.pem")));
    }
    
    #[test]
    fn test_codec_error() {
        let error = CodecError::new("Test codec error".to_string());
        assert_eq!(error.message, "Test codec error");
        
        let error_string = format!("{}", error);
        assert_eq!(error_string, "CodecError: Test codec error");
    }
    
    #[test]
    fn test_processing_error() {
        let error = ProcessingError::new("Test processing error".to_string())
            .with_details(Value::string("Test details"));
        
        assert_eq!(error.message, "Test processing error");
        assert_eq!(error.details, Some(Value::string("Test details")));
        
        let error_string = format!("{}", error);
        assert!(error_string.contains("ProcessingError: Test processing error"));
        assert!(error_string.contains("Test details"));
    }
}