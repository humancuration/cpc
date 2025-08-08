//! # Learning Impact Tracker
//!
//! A comprehensive system for measuring the effectiveness of impact visualizations
//! and refining them based on real community feedback and usage patterns.
//!
//! This crate provides:
//! - Tracking of engagement with visualization components
//! - Measurement of correlation between visualization usage and course completion
//! - Monitoring of transitions from learning to volunteer activities
//! - Recording of community validation interactions
//! - Privacy-preserving data collection respecting consent levels

/// Core tracking functionality for learning impact metrics
pub mod tracker;

/// Analytics and dashboard functionality for educators and coordinators
pub mod analytics;

/// Feedback collection and processing systems
pub mod feedback;

/// Continuous improvement mechanisms
pub mod improvement;

/// Integration with broader impact ecosystem
pub mod integration;

// Re-export key types for easier access
pub use tracker::LearningImpactTracker;
pub use analytics::ImpactAnalyticsDashboard;
pub use feedback::FeedbackCollector;
pub use improvement::ImprovementEngine;
pub use integration::EcosystemIntegrator;