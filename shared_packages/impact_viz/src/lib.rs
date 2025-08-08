//! # Community Impact Visualization Framework
//! 
//! This crate provides intuitive, values-aligned visualizations that translate complex 
//! mathematical results into clear community impact metrics that all cooperative members can 
//! understand and act upon.
//! 
//! ## Modules
//! 
//! - `core`: Core visualization components and traits
//! - `volunteer`: Volunteer impact dashboard visualizations
//! - `financial`: Financial health visualization components
//! - `skill`: Skill development mapping visualizations
//! - `cause`: Cause impact storytelling visualizations
//! - `values`: Cooperative values translation layer
//! - `accessibility`: Accessibility-first design principles implementation

/// Core visualization components and traits
pub mod core;

/// Volunteer impact dashboard visualizations
pub mod volunteer;

/// Financial health visualization components
pub mod financial;

/// Skill development mapping visualizations
pub mod skill;

/// Cause impact storytelling visualizations
pub mod cause;

/// Cooperative values translation layer
pub mod values;

/// Accessibility-first design principles implementation
pub mod accessibility;

/// Reusable UI components for impact visualizations
pub mod components;

// Re-export commonly used items
pub use core::{ImpactVisualization, VisualizationStyle, ImpactMetric, CommunityStory};
pub use values::ValuesTranslator;
pub use accessibility::AccessibilityOptions;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // This is a placeholder test to ensure modules compile
        // Actual tests will be in individual module test files
        assert!(true);
    }
}