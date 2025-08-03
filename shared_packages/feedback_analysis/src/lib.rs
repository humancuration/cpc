//! Shared analysis components for the feedback system
//!
//! This crate provides common statistical analysis functions that can be used
//! across both reviews and survey packages.

pub mod stats;
pub mod distributions;
pub mod trends;
// Note: We're not implementing sentiment analysis as per the requirement in reviews/analytics.rs

// Re-export commonly used types
pub use stats::calculate_correlation;
pub use distributions::RatingDistribution;
pub use trends::TrendResult;