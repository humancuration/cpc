//! Tests for error handling utilities
//!
//! This module contains tests for the error handling functionality.

use wasm_bindgen_test::*;
use web_core::utils::error_handling::{WebError, log_error, js_error_to_web_error};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_web_error_creation() {
    let network_error = WebError::NetworkError("Connection failed".to_string());
    let parse_error = WebError::ParseError("Invalid JSON".to_string());
    let storage_error = WebError::StorageError("LocalStorage unavailable".to_string());
    let auth_error = WebError::AuthenticationError("Invalid credentials".to_string());
    let validation_error = WebError::ValidationError("Field is required".to_string());
    let api_error = WebError::ApiError("Server error".to_string());
    let rate_limit_error = WebError::RateLimitError("Too many requests".to_string());
    let batch_error = WebError::BatchError("Batch processing failed".to_string());
    let grpc_error = WebError::GrpcError("gRPC call failed".to_string());
    let component_error = WebError::ComponentError("Component render failed".to_string());
    let theme_error = WebError::ThemeError("Theme loading failed".to_string());
    let unknown_error = WebError::UnknownError("Unexpected error".to_string());
    
    // Test that all error types can be created
    assert!(matches!(network_error, WebError::NetworkError(_)));
    assert!(matches!(parse_error, WebError::ParseError(_)));
    assert!(matches!(storage_error, WebError::StorageError(_)));
    assert!(matches!(auth_error, WebError::AuthenticationError(_)));
    assert!(matches!(validation_error, WebError::ValidationError(_)));
    assert!(matches!(api_error, WebError::ApiError(_)));
    assert!(matches!(rate_limit_error, WebError::RateLimitError(_)));
    assert!(matches!(batch_error, WebError::BatchError(_)));
    assert!(matches!(grpc_error, WebError::GrpcError(_)));
    assert!(matches!(component_error, WebError::ComponentError(_)));
    assert!(matches!(theme_error, WebError::ThemeError(_)));
    assert!(matches!(unknown_error, WebError::UnknownError(_)));
}

#[wasm_bindgen_test]
fn test_web_error_display() {
    let error = WebError::NetworkError("Connection failed".to_string());
    let display = format!("{}", error);
    assert_eq!(display, "Network error: Connection failed");
    
    let error = WebError::ParseError("Invalid JSON".to_string());
    let display = format!("{}", error);
    assert_eq!(display, "Parse error: Invalid JSON");
    
    let error = WebError::StorageError("LocalStorage unavailable".to_string());
    let display = format!("{}", error);
    assert_eq!(display, "Storage error: LocalStorage unavailable");
}

#[wasm_bindgen_test]
fn test_js_error_conversion() {
    let js_error = wasm_bindgen::JsValue::from_str("JavaScript error");
    let web_error = js_error_to_web_error(js_error);
    
    // Should convert to UnknownError
    assert!(matches!(web_error, WebError::UnknownError(_)));
}

#[wasm_bindgen_test]
fn test_log_error() {
    let error = WebError::NetworkError("Test error".to_string());
    
    // This should not panic
    log_error(&error);
    
    // If we get here, the test passes
    assert!(true);
}