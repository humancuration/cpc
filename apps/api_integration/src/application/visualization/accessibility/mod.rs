//! Accessibility adapters for different apps

use crate::application::visualization::response::AccessibilityMetadata;
use visualization_context::VisualizationContext;

pub mod dashboard;
pub mod reporting;
pub mod collaboration;

pub use dashboard::DashboardAdapter;
pub use reporting::ReportingAdapter;
pub use collaboration::CollaborationAdapter;

/// Trait for app-specific accessibility adapters
pub trait AccessibilityAdapter {
    /// Enhance accessibility metadata based on app context
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        context: &VisualizationContext,
    ) -> AccessibilityMetadata;
}

/// Factory for creating accessibility adapters
pub struct AccessibilityAdapterFactory;

impl AccessibilityAdapterFactory {
    /// Create an adapter for the given app
    pub fn create_adapter(app_name: &str) -> Box<dyn AccessibilityAdapter> {
        match app_name {
            "dashboard" => Box::new(DashboardAdapter),
            "reporting" => Box::new(ReportingAdapter),
            "collaboration" => Box::new(CollaborationAdapter),
            _ => Box::new(DefaultAdapter),
        }
    }
}

/// Default adapter for unknown apps
pub struct DefaultAdapter;

impl AccessibilityAdapter for DefaultAdapter {
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        _context: &VisualizationContext,
    ) -> AccessibilityMetadata {
        base_metadata
    }
}