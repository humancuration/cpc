//! # Volunteer Impact Tracker
//!
//! A comprehensive system for measuring the effectiveness of volunteer impact visualizations
//! and refining them based on real community feedback and usage patterns.
//!
//! This crate provides:
//! - Tracking of engagement with volunteer visualization components
//! - Measurement of correlation between visualization usage and volunteer retention
//! - Monitoring of task completion rates and quality
//! - Recording of community validation of volunteer impact
//! - Privacy-preserving data collection respecting consent levels
//! - Integration with learning impact metrics for cross-platform insights

/// Core tracking functionality for volunteer impact metrics
pub mod tracker;

/// Analytics and dashboard functionality for volunteer coordinators
pub mod analytics;

/// Feedback collection and processing systems
pub mod feedback;

/// Continuous improvement mechanisms
pub mod improvement;

/// Integration with broader impact ecosystem
pub mod integration;

// Re-export key types for easier access
pub use tracker::VolunteerImpactTracker;
pub use analytics::ImpactAnalyticsDashboard;
pub use feedback::FeedbackCollector;
pub use improvement::ImprovementEngine;
pub use integration::EcosystemIntegrator;