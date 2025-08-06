//! Error recovery utilities
//!
//! This module provides utilities for recovering from errors and
//! implementing retry mechanisms.

use crate::utils::error_handling::WebError;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use wasm_bindgen::JsValue;

/// Error recovery strategy
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// Retry immediately
    Immediate,
    
    /// Retry after a fixed delay
    FixedDelay(u64), // milliseconds
    
    /// Retry with exponential backoff
    ExponentialBackoff {
        initial_delay_ms: u64,
        max_delay_ms: u64,
        multiplier: f64,
    },
    
    /// Retry with a custom delay function
    Custom(Box<dyn Fn(usize) -> u64>),
}

/// Error recovery configuration
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    
    /// Strategy for retry delays
    pub strategy: RecoveryStrategy,
    
    /// Whether to retry on specific error types
    pub retry_on: Vec<WebErrorType>,
}

/// Types of errors that can be retried
#[derive(Debug, Clone, PartialEq)]
pub enum WebErrorType {
    Network,
    Api,
    RateLimit,
    Storage,
    Unknown,
}

impl From<&WebError> for WebErrorType {
    fn from(error: &WebError) -> Self {
        match error {
            WebError::NetworkError(_) => WebErrorType::Network,
            WebError::ApiError(_) => WebErrorType::Api,
            WebError::RateLimitError(_) => WebErrorType::RateLimit,
            WebError::StorageError(_) => WebErrorType::Storage,
            _ => WebErrorType::Unknown,
        }
    }
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            strategy: RecoveryStrategy::ExponentialBackoff {
                initial_delay_ms: 1000,
                max_delay_ms: 30000,
                multiplier: 2.0,
            },
            retry_on: vec![
                WebErrorType::Network,
                WebErrorType::Api,
                WebErrorType::RateLimit,
            ],
        }
    }
}

/// Error recovery service
pub struct ErrorRecovery {
    /// Configuration for error recovery
    config: RecoveryConfig,
}

impl ErrorRecovery {
    /// Create a new error recovery service with the given configuration
    pub fn new(config: RecoveryConfig) -> Self {
        Self { config }
    }
    
    /// Execute an operation with retry logic
    pub async fn execute_with_retry<T, F, Fut>(
        &self,
        operation: F,
    ) -> Result<T, WebError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, WebError>>,
    {
        let mut attempt = 0;
        
        loop {
            attempt += 1;
            
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    // Check if we should retry
                    if attempt >= self.config.max_attempts {
                        return Err(error);
                    }
                    
                    // Check if the error type is retryable
                    let error_type = WebErrorType::from(&error);
                    if !self.config.retry_on.contains(&error_type) {
                        return Err(error);
                    }
                    
                    // Calculate delay before retry
                    let delay_ms = self.calculate_delay(attempt);
                    if delay_ms > 0 {
                        self.sleep(delay_ms).await;
                    }
                }
            }
        }
    }
    
    /// Calculate the delay before the next retry attempt
    fn calculate_delay(&self, attempt: usize) -> u64 {
        match &self.config.strategy {
            RecoveryStrategy::Immediate => 0,
            RecoveryStrategy::FixedDelay(delay_ms) => *delay_ms,
            RecoveryStrategy::ExponentialBackoff {
                initial_delay_ms,
                max_delay_ms,
                multiplier,
            } => {
                let delay = (*initial_delay_ms as f64) * multiplier.powi(attempt as i32 - 1);
                std::cmp::min(delay as u64, *max_delay_ms)
            }
            RecoveryStrategy::Custom(delay_fn) => delay_fn(attempt),
        }
    }
    
    /// Sleep for the specified number of milliseconds
    async fn sleep(&self, ms: u64) {
        // In a real implementation, we would use:
        // gloo_timers::future::TimeoutFuture::new(ms).await;
        // For now, we'll use a mock implementation
        let _ = ms;
    }
    
    /// Recover from an error by providing a fallback value
    pub fn recover_with_fallback<T, F>(
        &self,
        result: Result<T, WebError>,
        fallback: F,
    ) -> Result<T, WebError>
    where
        F: FnOnce() -> T,
    {
        match result {
            Ok(value) => Ok(value),
            Err(error) => {
                // Check if the error type allows recovery
                let error_type = WebErrorType::from(&error);
                if self.config.retry_on.contains(&error_type) {
                    Ok(fallback())
                } else {
                    Err(error)
                }
            }
        }
    }
    
    /// Recover from an error by providing a default value
    pub fn recover_with_default<T>(&self, result: Result<T, WebError>) -> Result<T, WebError>
    where
        T: Default,
    {
        self.recover_with_fallback(result, T::default)
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new(RecoveryConfig::default())
    }
}

/// Trait for operations that can be retried
pub trait RetryableOperation<T> {
    /// Execute the operation with retry logic
    async fn retry(self, recovery: &ErrorRecovery) -> Result<T, WebError>;
}

impl<F, T, Fut> RetryableOperation<T> for F
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, WebError>>,
{
    async fn retry(self, recovery: &ErrorRecovery) -> Result<T, WebError> {
        recovery.execute_with_retry(self).await
    }
}

/// Error recovery context
pub struct RecoveryContext {
    /// Error recovery service
    recovery: ErrorRecovery,
    
    /// Operation timeout in milliseconds
    timeout_ms: Option<u64>,
}

impl RecoveryContext {
    /// Create a new recovery context
    pub fn new(recovery: ErrorRecovery, timeout_ms: Option<u64>) -> Self {
        Self { recovery, timeout_ms }
    }
    
    /// Execute an operation with both retry and timeout logic
    pub async fn execute_with_retry_and_timeout<T, F, Fut>(
        &self,
        operation: F,
    ) -> Result<T, WebError>
    where
        F: Fn() -> Fut + Copy,
        Fut: Future<Output = Result<T, WebError>>,
    {
        // If a timeout is specified, wrap the operation with a timeout
        if let Some(timeout_ms) = self.timeout_ms {
            // In a real implementation, we would use:
            // gloo_timers::future::TimeoutFuture::new(timeout_ms)
            // For now, we'll just execute without timeout
            let _ = timeout_ms;
        }
        
        self.recovery.execute_with_retry(operation).await
    }
}

impl Default for RecoveryContext {
    fn default() -> Self {
        Self::new(ErrorRecovery::default(), Some(30000)) // 30 second default timeout
    }
}

/// Circuit breaker pattern for error recovery
pub struct CircuitBreaker {
    /// Maximum number of failures before opening the circuit
    max_failures: usize,
    
    /// Timeout before attempting to close the circuit again
    timeout_ms: u64,
    
    /// Current failure count
    failure_count: usize,
    
    /// Timestamp when the circuit was last opened
    last_failure_time: Option<u64>,
    
    /// Current state of the circuit
    state: CircuitState,
}

/// State of the circuit breaker
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitState {
    /// Circuit is closed, operations can proceed normally
    Closed,
    
    /// Circuit is open, operations are failing fast
    Open,
    
    /// Circuit is half-open, testing if operations can proceed
    HalfOpen,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(max_failures: usize, timeout_ms: u64) -> Self {
        Self {
            max_failures,
            timeout_ms,
            failure_count: 0,
            last_failure_time: None,
            state: CircuitState::Closed,
        }
    }
    
    /// Execute an operation with circuit breaker protection
    pub async fn execute<T, F, Fut>(
        &mut self,
        operation: F,
    ) -> Result<T, WebError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, WebError>>,
    {
        // Check circuit state
        match self.state {
            CircuitState::Open => {
                // Check if timeout has expired
                if let Some(last_failure) = self.last_failure_time {
                    let now = self.current_timestamp();
                    if now - last_failure >= self.timeout_ms {
                        self.state = CircuitState::HalfOpen;
                    } else {
                        return Err(WebError::NetworkError("Circuit breaker is open".to_string()));
                    }
                } else {
                    return Err(WebError::NetworkError("Circuit breaker is open".to_string()));
                }
            }
            CircuitState::HalfOpen => {
                // Only one operation is allowed in half-open state
            }
            CircuitState::Closed => {
                // Proceed normally
            }
        }
        
        // Execute the operation
        match operation().await {
            Ok(result) => {
                // Success - reset circuit
                self.on_success();
                Ok(result)
            }
            Err(error) => {
                // Failure - record it
                self.on_failure();
                Err(error)
            }
        }
    }
    
    /// Handle a successful operation
    fn on_success(&mut self) {
        self.failure_count = 0;
        self.last_failure_time = None;
        self.state = CircuitState::Closed;
    }
    
    /// Handle a failed operation
    fn on_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(self.current_timestamp());
        
        if self.failure_count >= self.max_failures {
            self.state = CircuitState::Open;
        }
    }
    
    /// Get the current state of the circuit
    pub fn state(&self) -> &CircuitState {
        &self.state
    }
    
    /// Get the current failure count
    pub fn failure_count(&self) -> usize {
        self.failure_count
    }
    
    /// Get the current timestamp in milliseconds
    fn current_timestamp(&self) -> u64 {
        // In a real implementation, we would use:
        // web_sys::window().unwrap().performance().unwrap().now() as u64
        // For now, we'll use a mock timestamp
        0
    }
}

impl Default for CircuitBreaker {
    fn default() -> Self {
        Self::new(5, 60000) // 5 failures, 1 minute timeout
    }
}