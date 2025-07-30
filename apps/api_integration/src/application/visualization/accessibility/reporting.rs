//! Reporting-specific accessibility adapter

use crate::application::visualization::response::{AccessibilityMetadata, NavigationHint};
use visualization_context::VisualizationContext;

/// Reporting-specific accessibility enhancements
pub struct ReportingAdapter;

impl super::AccessibilityAdapter for ReportingAdapter {
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        context: &VisualizationContext,
    ) -> AccessibilityMetadata {
        let enhanced_alt_text = format!(
            "Report visualization - {}. {}",
            context.originating_app,
            base_metadata.alt_text
        );
        
        let mut enhanced_navigation_map = base_metadata.navigation_map.clone();
        
        // Add reporting-specific navigation hints
        enhanced_navigation_map.insert("report_summary".to_string(), NavigationHint {
            label: "Report summary".to_string(),
            key: "R".to_string(),
            position: [-2.0, 2.0, 0.0],
        });
        
        enhanced_navigation_map.insert("data_source".to_string(), NavigationHint {
            label: "View data source".to_string(),
            key: "D".to_string(),
            position: [2.0, 2.0, 0.0],
        });
        
        AccessibilityMetadata {
            alt_text: enhanced_alt_text,
            navigation_map: enhanced_navigation_map,
            aria_properties: base_metadata.aria_properties,
        }
    }
}