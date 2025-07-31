//! Circuit breaker implementation for network operations
//! 
//! This module provides a circuit breaker pattern to prevent cascading
//! failures when network operations repeatedly fail.

use std::time::{Duration, Instant};
use std::sync::Arc;
use tracing::{warn, error};

/// Configuration for the circuit breaker
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures that will trip the circuit
    pub failure_threshold: usize,
    
    /// Time window for counting failures
    pub failure_window: Duration,
    
    /// Time to wait before attempting to close the circuit again
    pub reset_timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            failure_window: Duration::from_secs(60), // 1 minute
            reset_timeout: Duration::from_secs(120), // 2 minutes
        }
    }
}

/// State of the circuit breaker
#[derive(Debug, Clone, PartialEq)]
enum CircuitState {
    /// Circuit is closed and functioning normally
    Closed,
    
    /// Circuit is open due to too many failures
    Open,
    
    /// Circuit is half-open, allowing limited attempts to test if service is restored
    HalfOpen,
}

/// Circuit breaker for network operations
pub struct NetworkCircuitBreaker {
    /// Circuit breaker configuration
    config: CircuitBreakerConfig,
    
    /// Current state of the circuit
    state: CircuitState,
    
    /// Time when the circuit opened
    open_time: Option<Instant>,
    
    /// Recent failure timestamps
    failure_timestamps: Vec<Instant>,
}

impl NetworkCircuitBreaker {
    /// Create a new network circuit breaker with default configuration
    pub fn new() -> Self {
        Self::with_config(CircuitBreakerConfig::default())
    }
    
    /// Create a new network circuit breaker with custom configuration
    pub fn with_config(config: CircuitBreakerConfig) -> Self {
        Self {
            config,
            state: CircuitState::Closed,
            open_time: None,
            failure_timestamps: Vec::new(),
        }
    }
    
    /// Check if operation can proceed (circuit is closed or half-open)
    pub fn can_execute(&mut self) -> bool {
        // Check circuit state
        if self.state == CircuitState::Open {
            // Check if we should attempt to close the circuit
            if let Some(open_time) = self.open_time {
                if open_time.elapsed() >= self.config.reset_timeout {
                    self.state = CircuitState::HalfOpen;
                    return true;
                } else {
                    // Circuit is still open, operation cannot proceed
                    return false;
                }
            } else {
                // Circuit is open but we don't know when it opened
                return false;
            }
        }
        
        // Circuit is closed or half-open, operation can proceed
        true
    }
    
    /// Record a successful operation
    pub fn on_success(&mut self) {
        if self.state == CircuitState::HalfOpen {
            // Success in half-open state, close the circuit
            self.state = CircuitState::Closed;
            self.failure_timestamps.clear();
            self.open_time = None;
        }
        // In closed state, we don't need to do anything special on success
    }
    
    /// Record a failed operation
    pub fn on_failure(&mut self) {
        let now = Instant::now();
        
        // Remove old failures outside the window
        self.failure_timestamps.retain(|&timestamp| {
            now.duration_since(timestamp) <= self.config.failure_window
        });
        
        // Add the new failure
        self.failure_timestamps.push(now);
        
        // Check if we should trip the circuit
        if self.failure_timestamps.len() >= self.config.failure_threshold {
            if self.state != CircuitState::Open {
                warn!("Circuit breaker tripped due to {} failures in {:?}",
                      self.failure_timestamps.len(), self.config.failure_window);
                self.state = CircuitState::Open;
                self.open_time = Some(now);
            }
        } else if self.state == CircuitState::HalfOpen {
            // Failure in half-open state, open the circuit again
            self.state = CircuitState::Open;
            self.open_time = Some(now);
        }
    }
    
    /// Get the current state of the circuit breaker
    pub fn state(&self) -> &CircuitState {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_circuit_breaker_opens_after_failures() {
        let mut circuit_breaker = NetworkCircuitBreaker::with_config(CircuitBreakerConfig {
            failure_threshold: 3,
            failure_window: Duration::from_secs(1),
            reset_timeout: Duration::from_secs(2),
        });
        
        // First two failures - circuit should remain closed
        circuit_breaker.on_failure();
        assert_eq!(*circuit_breaker.state(), CircuitState::Closed);
        
        circuit_breaker.on_failure();
        assert_eq!(*circuit_breaker.state(), CircuitState::Closed);
        
        // Third failure - should trip the circuit
        circuit_breaker.on_failure();
        assert_eq!(*circuit_breaker.state(), CircuitState::Open);
    }
    
    #[test]
    fn test_circuit_breaker_resets_after_timeout() {
        let mut circuit_breaker = NetworkCircuitBreaker::with_config(CircuitBreakerConfig {
            failure_threshold: 1,
            failure_window: Duration::from_millis(100),
            reset_timeout: Duration::from_millis(200),
        });
        
        // Trip the circuit
        circuit_breaker.on_failure();
        assert_eq!(*circuit_breaker.state(), CircuitState::Open);
        assert!(!circuit_breaker.can_execute());
        
        // Wait for reset timeout
        thread::sleep(Duration::from_millis(250));
        
        // Should be able to execute (half-open state)
        assert!(circuit_breaker.can_execute());
        assert_eq!(*circuit_breaker.state(), CircuitState::HalfOpen);
    }
    
    #[test]
    fn test_circuit_breaker_closes_after_success_in_half_open() {
        let mut circuit_breaker = NetworkCircuitBreaker::with_config(CircuitBreakerConfig {
            failure_threshold: 1,
            failure_window: Duration::from_millis(100),
            reset_timeout: Duration::from_millis(200),
        });
        
        // Trip the circuit
        circuit_breaker.on_failure();
        assert_eq!(*circuit_breaker.state(), CircuitState::Open);
        
        // Wait for reset timeout
        thread::sleep(Duration::from_millis(250));
        
        // Should be in half-open state
        assert!(circuit_breaker.can_execute());
        assert_eq!(*circuit_breaker.state(), CircuitState::HalfOpen);
        
        // Success should close the circuit
        circuit_breaker.on_success();
        assert_eq!(*circuit_breaker.state(), CircuitState::Closed);
    }
}