//! Error handling utilities for web applications
//!
//! This module provides common error handling patterns and utilities
//! for CPC web applications.

use wasm_bindgen::JsValue;
use web_sys::console;

/// A generic error type for web applications
#[derive(Debug)]
pub enum WebError {
    NetworkError(String),
    ParseError(String),
    StorageError(String),
    AuthenticationError(String),
    ValidationError(String),
    ApiError(String),
    RateLimitError(String),
    BatchError(String),
    GrpcError(String),
    ComponentError(String),
    ThemeError(String),
    UnknownError(String),
}

impl std::fmt::Display for WebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            WebError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            WebError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            WebError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            WebError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            WebError::ApiError(msg) => write!(f, "API error: {}", msg),
            WebError::RateLimitError(msg) => write!(f, "Rate limit error: {}", msg),
            WebError::BatchError(msg) => write!(f, "Batch error: {}", msg),
            WebError::GrpcError(msg) => write!(f, "gRPC error: {}", msg),
            WebError::ComponentError(msg) => write!(f, "Component error: {}", msg),
            WebError::ThemeError(msg) => write!(f, "Theme error: {}", msg),
            WebError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for WebError {}

/// Log an error to the console
pub fn log_error(error: &WebError) {
    console::error_1(&format!("{}", error).into());
}

/// Convert a JsValue error to a WebError
pub fn js_error_to_web_error(js_error: JsValue) -> WebError {
    WebError::UnknownError(format!("{:?}", js_error))
}

/// Wrap a Result with a fallback value for error handling
pub fn with_fallback<T>(result: Result<T, WebError>, fallback: T) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            log_error(&error);
            fallback
        }
    }
}

/// Handle an error with a callback
pub fn handle_error<E, F>(result: Result<(), E>, handler: F) 
where 
    E: std::fmt::Display,
    F: FnOnce(E),
{
    if let Err(error) = result {
        handler(error);
    }
}