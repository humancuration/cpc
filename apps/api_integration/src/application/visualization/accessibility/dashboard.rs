//! Dashboard-specific accessibility adapter

use crate::application::visualization::response::{AccessibilityMetadata, NavigationHint};
use visualization_context::VisualizationContext;

/// Dashboard-specific accessibility enhancements
pub struct DashboardAdapter;

impl super::AccessibilityAdapter for DashboardAdapter {
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        context: &VisualizationContext,
    ) -> AccessibilityMetadata {
        let enhanced_alt_text = format!(
            "Dashboard section - {} visualization. {}",
            context.originating_app,
            base_metadata.alt_text
        );
        
        let mut enhanced_navigation_map = base_metadata.navigation_map.clone();
        
        // Add dashboard-specific navigation hints
        enhanced_navigation_map.insert("dashboard_home".to_string(), NavigationHint {
            label: "Return to dashboard home".to_string(),
            key: "H".to_string(),
            position: [-3.0, 3.0, 0.0],
        });
        
        enhanced_navigation_map.insert("dashboard_settings".to_string(), NavigationHint {
            label: "Dashboard settings".to_string(),
            key: "S".to_string(),
            position: [3.0, 3.0, 0.0],
        });
        
        AccessibilityMetadata {
            alt_text: enhanced_alt_text,
            navigation_map: enhanced_navigation_map,
            aria_properties: base_metadata.aria_properties,
        }
    }
}