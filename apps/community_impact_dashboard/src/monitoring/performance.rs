//! Performance monitoring for the Unified Community Impact Dashboard
//!
//! This module provides performance monitoring capabilities for the dashboard.

use tracing::{info, warn, error};
use std::time::Instant;

/// Performance monitor for tracking dashboard performance
pub struct PerformanceMonitor {
    start_time: Instant,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }
    
    /// Start timing an operation
    pub fn start_operation(&mut self, operation_name: &str) {
        self.start_time = Instant::now();
        info!("Starting operation: {}", operation_name);
    }
    
    /// End timing an operation and log the duration
    pub fn end_operation(&self, operation_name: &str) {
        let duration = self.start_time.elapsed();
        info!("Operation '{}' completed in {:?}", operation_name, duration);
        
        // Warn if operation took longer than expected
        if duration.as_millis() > 1000 {
            warn!("Operation '{}' took longer than expected: {:?}", operation_name, duration);
        }
    }
    
    /// Log an error
    pub fn log_error(&self, operation_name: &str, error: &str) {
        error!("Error in operation '{}': {}", operation_name, error);
    }
    
    /// Log a warning
    pub fn log_warning(&self, operation_name: &str, warning: &str) {
        warn!("Warning in operation '{}': {}", operation_name, warning);
    }
    
    /// Log an info message
    pub fn log_info(&self, operation_name: &str, message: &str) {
        info!("Info for operation '{}': {}", operation_name, message);
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}