//! Bevy interactive components
//! 
//! This module provides integration with Bevy for interactive visualizations.

use bevy::prelude::*;
use crate::domain::{
    chart::InteractiveConfig,
    data::DataSeries,
};

/// Bevy plugin for BI visualization
pub struct BiVisualizationPlugin;

impl Plugin for BiVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_charts)
            .add_systems(Update, handle_interactions);
    }
}

/// Component for interactive charts
#[derive(Component)]
pub struct InteractiveChart {
    /// Title of the chart
    pub title: String,
    
    /// Data series
    pub data: DataSeries,
    
    /// Interactive elements
    pub interactive_elements: Vec<crate::domain::chart::InteractiveElement>,
    
    /// Current zoom level
    pub zoom: f32,
    
    /// Current pan offset
    pub pan: Vec2,
}

impl InteractiveChart {
    /// Create a new interactive chart
    pub fn new(
        title: String,
        data: DataSeries,
        interactive_elements: Vec<crate::domain::chart::InteractiveElement>,
    ) -> Self {
        Self {
            title,
            data,
            interactive_elements,
            zoom: 1.0,
            pan: Vec2::ZERO,
        }
    }
}

/// Component for chart rendering
#[derive(Component)]
pub struct ChartRenderer {
    /// Texture handle for the chart
    pub texture: Handle<Image>,
}

/// System to update charts
fn update_charts(
    mut charts: Query<(&InteractiveChart, &mut ChartRenderer)>,
    mut images: ResMut<Assets<Image>>,
) {
    for (chart, mut renderer) in charts.iter_mut() {
        // In a real implementation, this would re-render the chart when data changes
        // For now, we'll just log that an update is needed
        info!("Chart '{}' needs update", chart.title);
    }
}

/// System to handle user interactions
fn handle_interactions(
    mut charts: Query<&mut InteractiveChart>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut chart in charts.iter_mut() {
        // Handle zoom
        if mouse_input.just_pressed(MouseButton::WheelUp) {
            chart.zoom *= 1.1;
        }
        
        if mouse_input.just_pressed(MouseButton::WheelDown) {
            chart.zoom *= 0.9;
        }
        
        // Handle pan
        if mouse_input.pressed(MouseButton::Left) {
            // In a real implementation, we would track mouse movement
            // For now, we'll just log that panning is active
            info!("Panning chart '{}'", chart.title);
        }
        
        // Handle other interactions based on interactive elements
        for element in &chart.interactive_elements {
            match element {
                crate::domain::chart::InteractiveElement::Tooltip => {
                    // Handle tooltip interactions
                }
                crate::domain::chart::InteractiveElement::Zoom => {
                    // Zoom is already handled above
                }
                crate::domain::chart::InteractiveElement::Pan => {
                    // Pan is already handled above
                }
                crate::domain::chart::InteractiveElement::Selection => {
                    // Handle selection interactions
                }
            }
        }
    }
}

/// Create an interactive chart bundle
pub fn create_interactive_chart_bundle(
    config: InteractiveConfig,
    data: DataSeries,
    asset_server: &AssetServer,
) -> impl Bundle {
    let chart = InteractiveChart::new(
        config.title,
        data,
        config.interactive_elements,
    );
    
    let renderer = ChartRenderer {
        texture: asset_server.load("textures/chart_placeholder.png"),
    };
    
    (chart, renderer)
}