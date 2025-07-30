//! Presentation layer for the BI & Analytics module

pub mod web;
pub mod bevy_visualization;

// Re-export key types
pub use web::BiAnalyticsGraphQLSchema;
pub use bevy_visualization::BiVisualizationPlugin;