//! Accessibility enhancement adapters

pub mod dashboard;
pub mod reporting;
pub mod collaboration;

use crate::application::visualization::response::AccessibilityMetadata;
use visualization_context::{AccessibilityMode, VisualizationContext};

/// Trait for app-specific accessibility adapters
pub trait AccessibilityAdapter {
    /// Enhance accessibility metadata with app-specific context
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        context: &VisualizationContext,
    ) -> AccessibilityMetadata;
}

/// Factory for creating appropriate accessibility adapter
pub fn create_adapter(app_id: &str) -> Box<dyn AccessibilityAdapter> {
    match app_id {
        "dashboard" => Box::new(dashboard::DashboardAdapter),
        "reporting" => Box::new(reporting::ReportingAdapter),
        "collaboration" => Box::new(collaboration::CollaborationAdapter),
        _ => Box::new(DefaultAdapter),
    }
}

/// Default adapter for unknown apps
struct DefaultAdapter;

impl AccessibilityAdapter for DefaultAdapter {
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        _context: &VisualizationContext,
    ) -> AccessibilityMetadata {
        base_metadata
    }
}