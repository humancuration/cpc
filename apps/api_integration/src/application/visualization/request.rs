//! Visualization request structures

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use visualization_context::{AccessibilityMode, SharingScope};

/// Parameters for visualization requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationParameters {
    pub width: u32,
    pub height: u32,
    pub lod_level: u8,
    pub visualization_type: String,
    pub accessibility_mode: AccessibilityMode,
}

/// Context for visualization requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestContext {
    pub app_id: String,
    pub user_id: Uuid,
    pub sharing_scope: SharingScope,
    pub session_token: String,
}

/// Standardized visualization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationRequest {
    pub visualization_id: Uuid,
    pub parameters: VisualizationParameters,
    pub context: RequestContext,
}