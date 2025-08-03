//! Shared visualization components for the feedback system
//!
//! This crate provides reusable visualization components that can be used
//! across different feedback-related packages.

pub mod heatmap;
pub mod trend_comparison;
pub mod correlation_matrix;

// Re-export commonly used types
pub use heatmap::Heatmap;
pub use trend_comparison::TrendComparison;
pub use correlation_matrix::CorrelationMatrix;