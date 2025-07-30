//! Request structures for visualization endpoints

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use visualization_context::{AccessibilityMode, SharingScope};

/// Standardized request for visualization endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationRequest {
    /// Unique identifier for the visualization/report
    pub visualization_id: Uuid,
    /// Visualization parameters
    pub parameters: VisualizationParameters,
    /// Context information for the request
    pub context: RequestContext,
}

/// Parameters for customizing the visualization output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationParameters {
    /// Width of the output in pixels
    pub width: u32,
    /// Height of the output in pixels
    pub height: u32,
    /// Level of detail (0-5, where 0 is lowest quality)
    pub lod_level: u8,
    /// Accessibility mode preference
    pub accessibility_mode: AccessibilityMode,
    /// Output format preference
    pub format: VisualizationFormat,
    /// Additional parameters as key-value pairs
    pub custom_params: std::collections::HashMap<String, String>,
}

/// Output format options
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VisualizationFormat {
    /// 3D scene data (glTF format)
    Scene3D,
    /// Static image (PNG format)
    Image,
    /// WebSocket stream
    Stream,
}

/// Context information for the request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    /// Originating application ID
    pub app_id: String,
    /// User ID making the request
    pub user_id: Uuid,
    /// Sharing scope for the visualization
    pub sharing_scope: SharingScope,
    /// Session token for authentication
    pub session_token: String,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl Default for VisualizationParameters {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            lod_level: 2,
            accessibility_mode: AccessibilityMode::Standard,
            format: VisualizationFormat::Scene3D,
            custom_params: std::collections::HashMap::new(),
        }
    }
}