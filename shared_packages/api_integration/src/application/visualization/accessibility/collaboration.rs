//! Collaboration-specific accessibility adapter

use crate::application::visualization::response::{AccessibilityMetadata, NavigationHint};
use visualization_context::VisualizationContext;

/// Collaboration-specific accessibility enhancements
pub struct CollaborationAdapter;

impl super::AccessibilityAdapter for CollaborationAdapter {
    fn enhance_metadata(
        &self,
        base_metadata: AccessibilityMetadata,
        context: &VisualizationContext,
    ) -> AccessibilityMetadata {
        let enhanced_alt_text = format!(
            "Collaborative visualization - {}. {}",
            context.originating_app,
            base_metadata.alt_text
        );
        
        let mut enhanced_navigation_map = base_metadata.navigation_map.clone();
        
        // Add collaboration-specific navigation hints
        enhanced_navigation_map.insert("collaborators".to_string(), NavigationHint {
            label: "View active collaborators".to_string(),
            key: "C".to_string(),
            position: [-1.0, 2.5, 0.0],
        });
        
        enhanced_navigation_map.insert("shared_cursors".to_string(), NavigationHint {
            label: "Navigate to shared cursors".to_string(),
            key: "X".to_string(),
            position: [1.0, 2.5, 0.0],
        });
        
        enhanced_navigation_map.insert("chat".to_string(), NavigationHint {
            label: "Open collaboration chat".to_string(),
            key: "H".to_string(),
            position: [0.0, 3.0, 0.0],
        });
        
        AccessibilityMetadata {
            alt_text: enhanced_alt_text,
            navigation_map: enhanced_navigation_map,
            aria_properties: base_metadata.aria_properties,
        }
    }
}