//! Visualization response structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Navigation hint for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationHint {
    pub label: String,
    pub key: String,
    pub position: [f32; 3],
}

/// ARIA properties for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriaProperties {
    pub role: String,
    pub live_region: String,
    pub keyboard_shortcuts: Vec<String>,
}

/// Accessibility metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityMetadata {
    pub alt_text: String,
    pub navigation_map: HashMap<String, NavigationHint>,
    pub aria_properties: AriaProperties,
}

/// Visualization data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VisualizationData {
    Scene3D {
        payload: serde_json::Value,
        accessibility: AccessibilityMetadata,
    },
    Image {
        payload: String, // Base64 encoded image
        format: String,
        accessibility: AccessibilityMetadata,
    },
    Stream {
        endpoint: String,
        stream_id: String,
        accessibility: AccessibilityMetadata,
    },
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub cache_ttl: u64,
    pub lod_config: LodConfig,
    pub compliance_flags: Vec<String>,
}

/// Level of detail configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LodConfig {
    pub level: u8,
    pub max_points: u32,
}

/// Standardized visualization response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationResponse {
    pub visualization_data: VisualizationData,
    pub metadata: ResponseMetadata,
}

impl Default for ResponseMetadata {
    fn default() -> Self {
        Self {
            cache_ttl: 300,
            lod_config: LodConfig {
                level: 2,
                max_points: 1000,
            },
            compliance_flags: vec![],
        }
    }
}

impl Default for AriaProperties {
    fn default() -> Self {
        Self {
            role: "application".to_string(),
            live_region: "off".to_string(),
            keyboard_shortcuts: vec![],
        }
    }
}