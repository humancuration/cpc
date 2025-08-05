//! Brush tool implementation for the Art application
//!
//! This module provides the core functionality for brush-based painting,
//! including stroke rendering, pressure sensitivity, and brush dynamics.

use bevy::prelude::*;
use crate::core::models::{Brush, Project, Action};
use std::collections::HashMap;

/// Plugin for brush tool functionality
pub struct BrushToolPlugin;

impl Plugin for BrushToolPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BrushTool>()
            .init_resource::<BrushStrokes>()
            .add_systems(Update, (handle_brush_input, update_brush_cursor));
    }
}

/// Current state of the brush tool
#[derive(Resource)]
pub struct BrushTool {
    pub active_brush: Brush,
    pub is_drawing: bool,
    pub current_stroke: Vec<(f32, f32)>,
}

impl Default for BrushTool {
    fn default() -> Self {
        Self {
            active_brush: Brush::new("Default Brush".to_string()),
            is_drawing: false,
            current_stroke: Vec::new(),
        }
    }
}

/// Collection of all brush strokes in the current session
#[derive(Resource, Default)]
pub struct BrushStrokes {
    pub strokes: HashMap<uuid::Uuid, Vec<BrushStroke>>,
}

/// Representation of a single brush stroke
pub struct BrushStroke {
    pub points: Vec<(f32, f32)>,
    pub brush: Brush,
    pub layer_id: uuid::Uuid,
}

/// Component for visualizing the brush cursor
#[derive(Component)]
pub struct BrushCursor;

/// Handle brush input (mouse/touch)
fn handle_brush_input(
    mut brush_tool: ResMut<BrushTool>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    
    // Get mouse position in world coordinates
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        
        // Check for mouse button press/release
        if mouse_input.just_pressed(MouseButton::Left) {
            brush_tool.is_drawing = true;
            brush_tool.current_stroke.clear();
            brush_tool.current_stroke.push((world_position.x, world_position.y));
        } else if mouse_input.just_released(MouseButton::Left) {
            brush_tool.is_drawing = false;
            // In a real implementation, we would apply the stroke to the current layer
            brush_tool.current_stroke.clear();
        }
        
        // Add point to current stroke if drawing
        if brush_tool.is_drawing {
            brush_tool.current_stroke.push((world_position.x, world_position.y));
            
            // Visualize the stroke
            if brush_tool.current_stroke.len() > 1 {
                let points = &brush_tool.current_stroke;
                for i in 1..points.len() {
                    let start = Vec2::new(points[i-1].0, points[i-1].1);
                    let end = Vec2::new(points[i].0, points[i].1);
                    gizmos.line_2d(start, end, Color::WHITE);
                }
            }
        }
    }
}

/// Update the brush cursor visualization
fn update_brush_cursor(
    mut commands: Commands,
    brush_tool: Res<BrushTool>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut cursor_query: Query<&mut Transform, With<BrushCursor>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    
    // Get mouse position in world coordinates
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {
        
        // Spawn or update cursor visualization
        if let Ok(mut transform) = cursor_query.get_single_mut() {
            transform.translation.x = world_position.x;
            transform.translation.y = world_position.y;
        } else {
            // Spawn new cursor
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                        custom_size: Some(Vec2::new(brush_tool.active_brush.size, brush_tool.active_brush.size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(world_position.x, world_position.y, 10.0),
                    ..default()
                },
                BrushCursor,
            ));
        }
    }
}

/// Apply a brush stroke to a layer
pub fn apply_brush_stroke(
    project: &mut Project,
    layer_id: uuid::Uuid,
    points: Vec<(f32, f32)>,
    brush: Brush,
) -> Result<(), String> {
    // Verify the layer exists
    if project.get_layer(layer_id).is_none() {
        return Err("Layer not found".to_string());
    }
    
    // Apply the stroke to the layer's pixel data
    // In a real implementation, this would involve:
    // 1. Converting world coordinates to pixel coordinates
    // 2. Rasterizing the brush stroke onto the layer's pixel buffer
    // 3. Handling brush dynamics (pressure, tilt, etc.)
    
    // For now, we'll just add the action to history
    project.history.push(Action::BrushStroke {
        layer_id,
        points,
        brush,
    });
    
    Ok(())
}

/// Create a brush with specific properties
pub fn create_brush(name: &str, size: f32, hardness: f32, opacity: f32) -> Brush {
    let mut brush = Brush::new(name.to_string());
    brush.set_size(size);
    brush.set_hardness(hardness);
    brush.set_opacity(opacity);
    brush
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Layer, LayerType};
    
    #[test]
    fn test_brush_tool_default() {
        let brush_tool = BrushTool::default();
        assert_eq!(brush_tool.active_brush.name, "Default Brush");
        assert_eq!(brush_tool.is_drawing, false);
        assert_eq!(brush_tool.current_stroke.len(), 0);
    }
    
    #[test]
    fn test_create_brush() {
        let brush = create_brush("Test Brush", 20.0, 0.8, 0.9);
        assert_eq!(brush.name, "Test Brush");
        assert_eq!(brush.size, 20.0);
        assert_eq!(brush.hardness, 0.8);
        assert_eq!(brush.opacity, 0.9);
    }
    
    #[test]
    fn test_apply_brush_stroke() {
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        let layer = Layer::new("Test Layer".to_string(), 800, 600, LayerType::Raster);
        let layer_id = layer.id;
        project.add_layer(layer);
        
        let brush = create_brush("Test Brush", 10.0, 1.0, 1.0);
        let points = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 20.0)];
        
        let result = apply_brush_stroke(&mut project, layer_id, points.clone(), brush);
        assert!(result.is_ok());
        
        // Check that the action was added to history
        assert_eq!(project.history.len(), 2); // 1 for layer addition, 1 for brush stroke
        match &project.history[1] {
            Action::BrushStroke { layer_id: stroke_layer_id, points: stroke_points, .. } => {
                assert_eq!(*stroke_layer_id, layer_id);
                assert_eq!(*stroke_points, points);
            }
            _ => panic!("Expected BrushStroke action"),
        }
    }
    
    #[test]
    fn test_apply_brush_stroke_invalid_layer() {
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        let invalid_layer_id = uuid::Uuid::new_v4();
        let brush = create_brush("Test Brush", 10.0, 1.0, 1.0);
        let points = vec![(0.0, 0.0), (10.0, 10.0)];
        
        let result = apply_brush_stroke(&mut project, invalid_layer_id, points, brush);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Layer not found");
    }
}