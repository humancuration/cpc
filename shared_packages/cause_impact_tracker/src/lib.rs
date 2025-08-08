//! # Cause Impact Tracker
//!
//! A comprehensive system for measuring the effectiveness of cause impact visualizations
//! and refining them based on real community feedback and usage patterns.
//!
//! This crate provides:
//! - Tracking of engagement with cause visualization components
//! - Measurement of correlation between visualization usage and cause engagement rates
//! - Monitoring of contribution effectiveness and community transformation metrics
//! - Recording of community validation of cause impact
//! - Privacy-preserving data collection respecting consent levels
//! - Integration with learning, volunteer, and financial impact metrics for cross-platform insights
//!
//! ## Modules
//!
//! - `tracker`: Core tracking functionality for cause impact metrics
//! - `analytics`: Analytics and dashboard functionality for cause coordinators
//! - `feedback`: Feedback collection and processing systems
//! - `improvement`: Continuous improvement mechanisms
//! - `integration`: Integration with broader impact ecosystem

/// Core tracking functionality for cause impact metrics
pub mod tracker;

/// Analytics and dashboard functionality for cause coordinators
pub mod analytics;

/// Feedback collection and processing systems
pub mod feedback;

/// Continuous improvement mechanisms
pub mod improvement;

/// Integration with broader impact ecosystem
pub mod integration;

// Re-export key types for easier access
pub use tracker::CauseImpactTracker;
pub use analytics::ImpactAnalyticsDashboard;
pub use feedback::FeedbackCollector;
pub use improvement::ImprovementEngine;
pub use integration::EcosystemIntegrator;