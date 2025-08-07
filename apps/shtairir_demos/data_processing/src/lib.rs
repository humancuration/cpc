//! Real-time Data Processing Pipeline
//!
//! A stream processing workflow that ingests sensor data, processes it, and generates analytics.
//! This module demonstrates how to compose Shtairir standard library blocks into a complex workflow.

pub mod pipeline;
pub mod metrics;

// Re-export key types and functions
pub use pipeline::{SensorReading, ProcessingMetrics, execute_pipeline, generate_mock_data};
pub use metrics::MetricsCollector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}