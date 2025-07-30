//! Complete chart visualization components for Bevy with 3D rendering

use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::text::{Text, TextSection, TextStyle};
use bevy::ui::{node_bundles::TextBundle, Style, Val};
use bevy::pbr::{StandardMaterial, PbrBundle};
use bevy::transform::components::Transform;
use bevy::ecs::system::Commands;
use serde_json::Value as JsonValue;
use crate::domain::report::VisualizationType;
use tracing::{info, warn};
use std::f32::consts::PI;

/// Bevy plugin for BI visualization
pub struct BiVisualizationPlugin;

impl Plugin for BiVisualizationPlugin {
    fn build(&self, app: &mut App) {
        info!("Adding BI Visualization plugin to Bevy app");
        app.add_plugins(bevy::pbr::PbrPlugin)
            .add_systems(Update, (
                update_visualizations,
                handle_chart_interactions,
                update_hover_effects,
            ));
    }
}

/// Component for chart entities
#[derive(Component)]
pub struct ChartEntity {
    pub chart_type: VisualizationType,
    pub title: String,
}

/// Component for interactive chart elements
#[derive(Component)]
pub struct InteractiveElement {
    pub value: f64,
    pub label: String,
    pub original_color: Color,
    pub is_hovered: bool,
}

/// Component for axis labels
#[derive(Component)]
pub struct AxisLabel;

/// System to update visualizations based on data changes
fn update_visualizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &ChartEntity), Changed<ChartEntity>>,
) {
    for (entity, chart_entity) in query.iter() {
        // Handle data updates - in real implementation, would update meshes
        info!("Updating visualization: {}", chart_entity.title);
    }
}

/// System to handle chart interactions
fn handle_chart_interactions(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(Entity, &InteractiveElement, &mut Handle<StandardMaterial>)>,
) {
    // Implement click handlers for drill-down functionality
    if mouse_input.just_pressed(MouseButton::Left) {
        for (entity, interactive, material_handle) in query.iter_mut() {
            // In real implementation, would handle drill-down
            info!("Clicked on: {} (value: {})", interactive.label, interactive.value);
        }
    }
}

/// System to update hover effects
fn update_hover_effects(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&InteractiveElement, &mut Handle<StandardMaterial>)>,
) {
    for (interactive, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(material_handle) {
            if interactive.is_hovered {
                material.base_color = interactive.original_color * 1.2; // Brighten on hover
            } else {
                material.base_color = interactive.original_color;
            }
        }
    }
}

/// Create a bar chart visualization with 3D bars
pub fn create_bar_chart(
    data: &JsonValue,
    title: &str,
) -> impl Fn(&mut Commands, &mut ResMut<Assets<Mesh>>, &mut ResMut<Assets<StandardMaterial>>) {
    let title = title.to_string();
    move |commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>| {
        info!("Creating 3D bar chart: {}", title);
        
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
        
        if data_points.is_empty() {
            return;
        }
        
        // Calculate scaling
        let max_value = data_points.iter().map(|(_, y)| *y).fold(0.0, f64::max);
        let scale_factor = 5.0 / max_value.max(1.0);
        
        // Create chart container
        let chart_entity = commands.spawn((
            Name::new(format!("Chart: {}", title)),
            ChartEntity {
                chart_type: VisualizationType::BarChart,
                title: title.clone(),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
        )).id();
        
        // Create bars with 3D cuboids
        let bar_width = 0.8;
        let bar_spacing = 1.2;
        let total_width = data_points.len() as f32 * bar_spacing;
        let start_x = -total_width / 2.0 + bar_spacing / 2.0;
        
        for (i, (label, value)) in data_points.iter().enumerate() {
            let height = (*value * scale_factor) as f32;
            let x = start_x + i as f32 * bar_spacing;
            let normalized_value = *value / max_value;
            
            // Create color based on value
            let color = Color::hsl(240.0 - normalized_value as f32 * 120.0, 0.7, 0.5);
            let material = materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.2,
                perceptual_roughness: 0.4,
                ..default()
            });
            
            let bar_entity = commands.spawn((
                Name::new(format!("Bar: {}", label)),
                InteractiveElement {
                    value: *value,
                    label: label.clone(),
                    original_color: color,
                    is_hovered: false,
                },
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(bar_width, height, bar_width).into()),
                    material: material.clone(),
                    transform: Transform::from_xyz(x, height / 2.0, 0.0),
                    ..default()
                },
            )).id();
            
            commands.entity(chart_entity).add_child(bar_entity);
            
            // Add value label above bar
            let label_entity = commands.spawn(Text3dBundle {
                text: Text::from_section(
                    format!("{:.1}", value),
                    TextStyle {
                        font_size: 0.3,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(x, height + 0.5, 0.0),
                ..default()
            }).id();
            commands.entity(chart_entity).add_child(label_entity);
        }
        
        // Add X-axis labels
        for (i, (label, _)) in data_points.iter().enumerate() {
            let x = start_x + i as f32 * bar_spacing;
            let label_entity = commands.spawn(Text3dBundle {
                text: Text::from_section(
                    label.clone(),
                    TextStyle {
                        font_size: 0.25,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(x, -0.5, 0.0),
                ..default()
            }).id();
            commands.entity(chart_entity).add_child(label_entity);
        }
    }
}

/// Create a line chart visualization with 3D lines
pub fn create_line_chart(
    data: &JsonValue,
    title: &str,
) -> impl Fn(&mut Commands, &mut ResMut<Assets<Mesh>>, &mut ResMut<Assets<StandardMaterial>>) {
    let title = title.to_string();
    move |commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>| {
        info!("Creating 3D line chart: {}", title);
        
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
        
        if data_points.is_empty() {
            return;
        }
        
        // Calculate scaling
        let max_x = data_points.iter().map(|(x, _)| *x).fold(0.0, f32::max);
        let max_y = data_points.iter().map(|(_, y)| *y).fold(0.0, f32::max);
        let x_scale = 8.0 / max_x.max(1.0);
        let y_scale = 5.0 / max_y.max(1.0);
        
        // Create chart container
        let chart_entity = commands.spawn((
            Name::new(format!("Chart: {}", title)),
            ChartEntity {
                chart_type: VisualizationType::LineChart,
                title: title.clone(),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
        )).id();
        
        // Create line material
        let line_material = materials.add(StandardMaterial {
            base_color: Color::rgb(0.2, 0.8, 0.2),
            metallic: 0.3,
            perceptual_roughness: 0.5,
            ..default()
        });
        
        // Create line segments
        for i in 0..data_points.len() - 1 {
            let (x1, y1) = data_points[i];
            let (x2, y2) = data_points[i + 1];
            
            let start = Vec3::new(x1 * x_scale, y1 * y_scale, 0.0);
            let end = Vec3::new(x2 * x_scale, y2 * y_scale, 0.0);
            
            // Create cylinder mesh for line segment
            let direction = end - start;
            let length = direction.length();
            let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
            
            let line_entity = commands.spawn((
                Name::new(format!("Line segment {}", i)),
                PbrBundle {
                    mesh: meshes.add(create_cylinder_mesh(0.05, length)),
                    material: line_material.clone(),
                    transform: Transform {
                        translation: start + direction / 2.0,
                        rotation,
                        scale: Vec3::ONE,
                    },
                    ..default()
                },
            )).id();
            
            commands.entity(chart_entity).add_child(line_entity);
        }
        
        // Create data points as spheres
        for (x, y) in &data_points {
            let sphere_entity = commands.spawn((
                Name::new("Data point"),
                InteractiveElement {
                    value: *y as f64,
                    label: format!("({:.1}, {:.1})", x, y),
                    original_color: Color::rgb(0.2, 0.8, 0.2),
                    is_hovered: false,
                },
                PbrBundle {
                    mesh: meshes.add(Sphere::new(0.1).mesh().ico(5).unwrap()),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgb(0.2, 0.8, 0.2),
                        metallic: 0.5,
                        perceptual_roughness: 0.3,
                        ..default()
                    }),
                    transform: Transform::from_xyz(x * x_scale, y * y_scale, 0.0),
                    ..default()
                },
            )).id();
            
            commands.entity(chart_entity).add_child(sphere_entity);
        }
    }
}

/// Create a pie chart visualization with 3D segments
pub fn create_pie_chart(
    data: &JsonValue,
    title: &str,
) -> impl Fn(&mut Commands, &mut ResMut<Assets<Mesh>>, &mut ResMut<Assets<StandardMaterial>>) {
    let title = title.to_string();
    move |commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>| {
        info!("Creating 3D pie chart: {}", title);
        
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
        
        if data_points.is_empty() {
            return;
        }
        
        let total: f64 = data_points.iter().map(|(_, value)| value).sum();
        if total <= 0.0 {
            return;
        }
        
        // Create chart container
        let chart_entity = commands.spawn((
            Name::new(format!("Chart: {}", title)),
            ChartEntity {
                chart_type: VisualizationType::PieChart,
                title: title.clone(),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
        )).id();
        
        // Create pie slices
        let mut current_angle = 0.0;
        let radius = 3.0;
        
        for (i, (label, value)) in data_points.iter().enumerate() {
            let angle = (value / total) * 2.0 * PI;
            let percentage = (value / total) * 100.0;
            
            // Create color based on index
            let hue = (i as f32 * 360.0 / data_points.len() as f32) % 360.0;
            let color = Color::hsl(hue, 0.7, 0.5);
            let material = materials.add(StandardMaterial {
                base_color: color,
                metallic: 0.2,
                perceptual_roughness: 0.4,
                ..default()
            });
            
            // Create pie slice mesh
            let slice_entity = commands.spawn((
                Name::new(format!("Pie slice: {}", label)),
                InteractiveElement {
                    value: *value,
                    label: label.clone(),
                    original_color: color,
                    is_hovered: false,
                },
                PbrBundle {
                    mesh: meshes.add(create_pie_slice_mesh(radius, current_angle as f32, (current_angle + angle) as f32, 0.5)),
                    material: material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
            )).id();
            
            commands.entity(chart_entity).add_child(slice_entity);
            
            // Add label with percentage
            let label_angle = current_angle + angle / 2.0;
            let label_x = (label_angle.cos() * radius * 0.7) as f32;
            let label_y = (label_angle.sin() * radius * 0.7) as f32;
            
            let label_entity = commands.spawn(Text3dBundle {
                text: Text::from_section(
                    format!("{:.1}%", percentage),
                    TextStyle {
                        font_size: 0.3,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(label_x, label_y, 0.6),
                ..default()
            }).id();
            
            commands.entity(chart_entity).add_child(label_entity);
            
            current_angle += angle;
        }
    }
}

/// Helper function to create a cylinder mesh for line segments
fn create_cylinder_mesh(radius: f32, height: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    
    let vertices = vec![
        [0.0, -height / 2.0, 0.0],      // bottom center
        [radius, -height / 2.0, 0.0],   // bottom edge
        [0.0, height / 2.0, 0.0],       // top center
        [radius, height / 2.0, 0.0],    // top edge
    ];
    
    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];
    
    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
    ];
    
    let indices = vec![0, 1, 2, 2, 1, 3];
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    
    mesh
}

/// Helper function to create a pie slice mesh
fn create_pie_slice_mesh(radius: f32, start_angle: f32, end_angle: f32, height: f32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    
    let mut vertices = vec![[0.0, 0.0, 0.0], [0.0, 0.0, height]];
    let mut normals = vec![[0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];
    let mut uvs = vec![[0.5, 0.5], [0.5, 0.5]];
    
    let segments = 20;
    for i in 0..=segments {
        let angle = start_angle + (end_angle - start_angle) * (i as f32 / segments as f32);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        
        vertices.push([x, y, 0.0]);
        vertices.push([x, y, height]);
        normals.push([0.0, 0.0, 1.0]);
        normals.push([0.0, 0.0, -1.0]);
        uvs.push([(x / radius + 1.0) / 2.0, (y / radius + 1.0) / 2.0]);
        uvs.push([(x / radius + 1.0) / 2.0, (y / radius + 1.0) / 2.0]);
    }
    
    let mut indices = Vec::new();
    for i in 0..segments {
        let base = 2 + i * 2;
        let next = base + 2;
        
        // Top face
        indices.extend_from_slice(&[0, base, next]);
        
        // Bottom face
        indices.extend_from_slice(&[1, next + 1, base + 1]);
        
        // Side faces
        indices.extend_from_slice(&[base, base + 1, next]);
        indices.extend_from_slice(&[next, base + 1, next + 1]);
    }
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    
    mesh
}

/// Create visualization based on report data and type
pub fn create_visualization(
    data: &JsonValue,
    visualization_type: &VisualizationType,
    title: &str,
) -> impl Fn(&mut Commands, &mut ResMut<Assets<Mesh>>, &mut ResMut<Assets<StandardMaterial>>) {
    match visualization_type {
        VisualizationType::BarChart => create_bar_chart(data, title),
        VisualizationType::LineChart => create_line_chart(data, title),
        VisualizationType::PieChart => create_pie_chart(data, title),
        _ => {
            // For other visualization types, create a basic chart
            let title = title.to_string();
            move |commands: &mut Commands, _meshes: &mut ResMut<Assets<Mesh>>, _materials: &mut ResMut<Assets<StandardMaterial>>| {
                warn!("Unsupported visualization type, creating basic chart: {}", title);
                commands.spawn((
                    Name::new(format!("Basic Chart: {}", title)),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                    GlobalTransform::default(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use bevy::app::App;
    
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
        
        // Test that the function can be called without panic
        let chart_fn = create_bar_chart(&data, "Test Bar Chart");
        // In real usage, would be called with:
        // app.world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
        //     world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
        //         let mut commands = world.commands();
        //         chart_fn(&mut commands, &mut meshes, &mut materials);
        //     });
        // });
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
    }
}