//! Performance metrics collection for the data processing pipeline

use std::time::Instant;
use tracing::info;

/// Metrics collector for tracking pipeline performance
#[derive(Debug)]
pub struct MetricsCollector {
    start_time: Instant,
    processing_times: Vec<f64>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            processing_times: Vec::new(),
        }
    }
    
    /// Record the start of a processing operation
    pub fn start_operation(&mut self) -> Instant {
        Instant::now()
    }
    
    /// Record the completion of a processing operation
    pub fn end_operation(&mut self, start: Instant) {
        let duration_ms = start.elapsed().as_micros() as f64 / 1000.0;
        self.processing_times.push(duration_ms);
        info!("Operation completed in {:.2}ms", duration_ms);
    }
    
    /// Get average processing time
    pub fn average_processing_time(&self) -> f64 {
        if self.processing_times.is_empty() {
            0.0
        } else {
            self.processing_times.iter().sum::<f64>() / self.processing_times.len() as f64
        }
    }
    
    /// Get total execution time
    pub fn total_execution_time(&self) -> f64 {
        self.start_time.elapsed().as_micros() as f64 / 1000.0
    }
    
    /// Get processing time statistics
    pub fn processing_time_stats(&self) -> (f64, f64, f64) {
        if self.processing_times.is_empty() {
            (0.0, 0.0, 0.0)
        } else {
            let min = self.processing_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = self.processing_times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            let avg = self.average_processing_time();
            (min, avg, max)
        }
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}