//! Extended AR integration plugin
//!
//! Combines all AR features into a single plugin for easy integration
//! with the Bevy engine and Tauri backend.

use bevy::prelude::*;
use crate::{
    bevy_integration::ARIntegrationPlugin,
    ar_tracking::ARTrackingPlugin,
    ar_rendering::ARRenderingPlugin,
    experience_preview::PreviewPlugin,
};

/// Main AR plugin that combines all AR functionality
pub struct ARPlugin;

impl Plugin for ARPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ARIntegrationPlugin,
            ARTrackingPlugin,
            ARRenderingPlugin,
            PreviewPlugin,
        ));
    }
}

/// AR plugin configuration
#[derive(Resource, Debug, Clone)]
pub struct ARPluginConfig {
    pub enable_tracking: bool,
    pub enable_rendering: bool,
    pub enable_previews: bool,
    pub debug_mode: bool,
    pub performance_mode: PerformanceMode,
}

/// Performance optimization modes
#[derive(Debug, Clone, Copy)]
pub enum PerformanceMode {
    Low,
    Medium,
    High,
    Ultra,
}

impl Default for ARPluginConfig {
    fn default() -> Self {
        Self {
            enable_tracking: true,
            enable_rendering: true,
            enable_previews: true,
            debug_mode: false,
            performance_mode: PerformanceMode::High,
        }
    }
}