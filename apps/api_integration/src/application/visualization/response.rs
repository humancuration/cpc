//! Response structures for visualization endpoints

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standardized response for visualization endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationResponse {
    /// The visualization data payload
    pub visualization_data: VisualizationData,
    /// Metadata about the response
    pub metadata: ResponseMetadata,
}

/// The actual visualization data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum VisualizationData {
    /// 3D scene data in glTF format
    Scene3D {
        /// Serialized glTF scene data
        payload: serde_json::Value,
        /// Accessibility metadata
        accessibility: AccessibilityMetadata,
    },
    /// Static image data
    Image {
        /// Base64-encoded image data
        payload: String,
        /// Image format (e.g., "png", "jpeg")
        format: String,
        /// Accessibility metadata
        accessibility: AccessibilityMetadata,
    },
    /// WebSocket stream connection
    Stream {
        /// WebSocket endpoint URL
        endpoint: String,
        /// Stream ID for connection
        stream_id: String,
        /// Accessibility metadata
        accessibility: AccessibilityMetadata,
    },
}

/// Accessibility metadata for screen readers and keyboard navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityMetadata {
    /// Alt text describing the visualization
    pub alt_text: String,
    /// Navigation map for keyboard shortcuts
    pub navigation_map: HashMap<String, NavigationHint>,
    /// ARIA properties for accessibility
    pub aria_properties: AriaProperties,
}

/// Navigation hint for keyboard accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationHint {
    /// Human-readable label
    pub label: String,
    /// Keyboard shortcut key
    pub key: String,
    /// Position coordinates [x, y, z]
    pub position: [f32; 3],
}

/// ARIA properties for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriaProperties {
    /// ARIA role
    pub role: String,
    /// Live region behavior
    pub live_region: String,
    /// Available keyboard shortcuts
    pub keyboard_shortcuts: Vec<String>,
}

/// Metadata about the response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Cache TTL in seconds
    pub cache_ttl: u64,
    /// Level of detail configuration
    pub lod_config: LodConfig,
    /// Compliance flags
    pub compliance_flags: Vec<String>,
    /// Performance metrics
    pub performance: PerformanceMetrics,
}

/// Level of detail configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LodConfig {
    /// Current LOD level
    pub level: u8,
    /// Maximum points for this LOD
    pub max_points: u32,
    /// Progressive loading enabled
    pub progressive: bool,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Time to generate in milliseconds
    pub generation_time_ms: u64,
    /// Cache hit status
    pub cache_hit: bool,
    /// Size of the response in bytes
    pub response_size_bytes: u64,
}