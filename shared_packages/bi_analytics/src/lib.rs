//! High-performance data analytics framework using Polars for the CPC ecosystem
//!
//! This module provides a comprehensive analytics engine that:
//! - Uses Polars as the underlying data processing engine
//! - Implements lazy evaluation for complex queries
//! - Supports both in-memory and streaming data processing
//! - Provides standardized error handling for data operations
//! - Includes cooperative values-aware data normalization
//!
//! The framework is designed to work across multiple domains in the CPC ecosystem,
//! including cause management, skill development, volunteer coordination, financial
//! analysis, and feedback analysis.

pub mod engine;
pub mod pipeline;
pub mod visualization;
pub mod privacy;
pub mod cooperative_values;
pub mod error;

// Re-export key types
pub use engine::AnalyticsEngine;
pub use pipeline::{DataPipeline, DataSourceAdapter};
pub use error::AnalyticsError;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}