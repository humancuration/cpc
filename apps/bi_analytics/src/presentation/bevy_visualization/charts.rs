//! Chart visualization components for Bevy

use bevy::prelude::*;
use serde_json::Value as JsonValue;
use crate::domain::report::VisualizationType;
use tracing::{info, warn};

/// Bevy plugin for BI visualization
pub struct BiVisualizationPlugin;

impl Plugin for BiVisualizationPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding BI Visualization plugin to Bevy app");
        app.add_systems(Update, update_visualizations);
    }
}

/// System to update visualizations
fn update_visualizations() {
    // In a real implementation, this would update visualization components
    // based on data changes
}

/// Create a bar chart visualization
pub fn create_bar_chart(
    data: &JsonValue,
    title: &str,
) -> impl Fn(&mut Commands) {
    let title = title.to_string();
    move |commands: &mut Commands| {
        info!("Creating bar chart: {}", title);
        
        // Extract data from JSON
        let data_points = if let JsonValue::Array(arr) = data {
            arr.iter().map(|item| {
                if let JsonValue::Object(obj) = item {
                    let x = obj.get("x").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
                    let y = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    (x, y)
                } else {
                    ("Unknown".to_string(), 0.0)
                }
            }).collect::<Vec<_>>()
        } else {
            warn!("Invalid data format for bar chart");
            vec![]
        };
        
        // Create chart components
        create_chart_base(commands, &title);
        
        // Create bars
        for (i, (label, value)) in data_points.iter().enumerate() {
            create_bar(commands, i as f32, *value as f32, label);
        }
    }
}

/// Create a line chart visualization
pub fn create_line_chart(
    data: &JsonValue,
    title: &str,
) -> impl Fn(&mut Commands) {
    let title = title.to_string();
    move |commands: &mut Commands| {
        info!("Creating line chart: {}", title);
        
        // Extract data from JSON
        let data_points = if let JsonValue::Array(arr) = data {
            arr.iter().map(|item| {
                if let JsonValue::Object(obj) = item {
                    let x = obj.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    let y = obj.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
                    (x, y)
                } else {
                    (0.0, 0.0)
                }
            }).collect::<Vec<_>>()
        } else {
            warn!("Invalid data format for line chart");
            vec![]
        };
        
        // Create chart components
        create_chart_base(commands, &title);
        
        // Create line
        create_line(commands, &data_points);
    }
}

/// Create a pie chart visualization
pub fn create_pie_chart(
    data: &JsonValue,
    title: &str,
) -> impl Fn(&mut Commands) {
    let title = title.to_string();
    move |commands: &mut Commands| {
        info!("Creating pie chart: {}", title);
        
        // Extract data from JSON
        let data_points = if let JsonValue::Array(arr) = data {
            arr.iter().map(|item| {
                if let JsonValue::Object(obj) = item {
                    let label = obj.get("label").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
                    let value = obj.get("value").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    (label, value)
                } else {
                    ("Unknown".to_string(), 0.0)
                }
            }).collect::<Vec<_>>()
        } else {
            warn!("Invalid data format for pie chart");
            vec![]
        };
        
        // Create chart components
        create_chart_base(commands, &title);
        
        // Create pie slices
        create_pie(commands, &data_points);
    }
}

/// Create base chart components
fn create_chart_base(commands: &mut Commands, title: &str) {
    // Create chart container
    commands.spawn((
        Name::new(format!("Chart: {}", title)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}

/// Create a bar for bar chart
fn create_bar(commands: &mut Commands, x: f32, height: f32, label: &str) {
    commands.spawn((
        Name::new(format!("Bar: {}", label)),
        Transform::from_xyz(x, height / 2.0, 0.0),
        GlobalTransform::default(),
        // In a real implementation, we would add mesh and material components
    ));
}

/// Create a line for line chart
fn create_line(commands: &mut Commands, points: &[(f32, f32)]) {
    commands.spawn((
        Name::new("Line Chart Line"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        // In a real implementation, we would add mesh and material components
    ));
}

/// Create pie slices for pie chart
fn create_pie(commands: &mut Commands, data_points: &[(String, f64)]) {
    let total: f64 = data_points.iter().map(|(_, value)| value).sum();
    
    commands.spawn((
        Name::new("Pie Chart"),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        // In a real implementation, we would add mesh and material components
    ));
}

/// Create visualization based on report data and type
pub fn create_visualization(
    data: &JsonValue,
    visualization_type: &VisualizationType,
    title: &str,
) -> impl Fn(&mut Commands) {
    match visualization_type {
        VisualizationType::BarChart => create_bar_chart(data, title),
        VisualizationType::LineChart => create_line_chart(data, title),
        VisualizationType::PieChart => create_pie_chart(data, title),
        _ => {
            // For other visualization types, create a basic chart
            let title = title.to_string();
            move |commands: &mut Commands| {
                warn!("Unsupported visualization type, creating basic chart: {}", title);
                create_chart_base(commands, &title);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_create_bar_chart() {
        let data = json!([
            {"x": "A", "y": 10},
            {"x": "B", "y": 20},
            {"x": "C", "y": 15}
        ]);
        
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(BiVisualizationPlugin);
        
        // This test would require a more complex setup to verify the actual Bevy entities
        // For now, we'll just ensure it doesn't panic
        let chart_fn = create_bar_chart(&data, "Test Bar Chart");
        // chart_fn would be called with commands in a real scenario
    }
    
    #[test]
    fn test_create_line_chart() {
        let data = json!([
            {"x": 1, "y": 10},
            {"x": 2, "y": 20},
            {"x": 3, "y": 15}
        ]);
        
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(BiVisualizationPlugin);
        
        let chart_fn = create_line_chart(&data, "Test Line Chart");
        // chart_fn would be called with commands in a real scenario
    }
    
    #[test]
    fn test_create_pie_chart() {
        let data = json!([
            {"label": "A", "value": 30},
            {"label": "B", "value": 50},
            {"label": "C", "value": 20}
        ]);
        
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(BiVisualizationPlugin);
        
        let chart_fn = create_pie_chart(&data, "Test Pie Chart");
        // chart_fn would be called with commands in a real scenario
    }
}