//! Domain models for the BI visualization toolkit
//!
//! This module contains the core business logic and entities for data visualization.

pub mod chart;
pub mod data;
pub mod errors;
pub mod confidence_interval;

pub use errors::VisualizationError;
pub use confidence_interval::{ConfidenceIntervalConfig, SignificanceIndicator, SignificanceLevel, StatisticalChartConfig};