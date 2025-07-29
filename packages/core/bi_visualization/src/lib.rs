//! # BI Visualization Toolkit
//!
//! Provides standardized data visualization capabilities across applications
//! using Plotters, with Bevy integration for interactive components.

/// Domain layer containing core business logic and entities
pub mod domain;

/// Application layer containing use cases and service orchestration
pub mod application;

/// Infrastructure layer containing adapters for external systems
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{chart::ChartConfig, data::DataSeries, VisualizationError};
pub use application::service::VisualizationService;