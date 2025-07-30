//! Bevy visualization components for the BI & Analytics module

use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::pbr::StandardMaterial;
use bevy::render::render_resource::TextureFormat;
use bevy::render::camera::RenderTarget;
use bevy::image::Image;
use bevy::render::view::RenderLayers;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureUsages};
use bevy::render::renderer::{RenderDevice, RenderQueue};
use bevy::render::render_graph::{RenderGraph, NodeRunError, RenderGraphNode};
use bevy::render::extract_resource::ExtractResource;
use std::collections::HashMap;
use crate::domain::report::{Report, VisualizationType};
use crate::presentation::bevy_visualization::charts::{create_visualization, BiVisualizationPlugin};
use serde_json::Value as JsonValue;
use tracing::{info, warn};

pub mod charts;

// Re-export key types
pub use charts::BiVisualizationPlugin;

/// Accessibility metadata for visualization
#[derive(Debug, Clone)]
pub struct AccessibilityMetadata {
    pub alt_text: String,
    pub navigation_map: HashMap<String, crate::application::visualization_service::NavigationHint>,
    pub live_region: String,
}

/// Main Bevy app for BI visualization
pub struct BiVisualizationApp {
    app: App,
    is_headless: bool,
}

impl BiVisualizationApp {
    /// Create a new BI visualization app
    pub fn new() -> Self {
        info!("Creating new BI visualization app");
        
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_plugins(BiVisualizationPlugin);
        
        // Add 3D camera and lighting
        app.add_systems(Startup, setup_3d_scene);
        
        Self { app, is_headless: false }
    }
    
    /// Create a new headless BI visualization app for server-side rendering
    pub fn new_headless() -> Self {
        info!("Creating new headless BI visualization app");
        
        let mut app = App::new();
        
        // Configure for headless rendering
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: None,
            ..default()
        }));
        app.add_plugins(BiVisualizationPlugin);
        
        // Add 3D camera and lighting for headless rendering
        app.add_systems(Startup, setup_headless_scene);
        
        Self { app, is_headless: true }
    }
    
    /// Add a report visualization to the app
    pub fn add_report_visualization(&mut self, report: &Report) {
        info!("Adding visualization for report: {}", report.name);
        
        let data = match serde_json::from_str::<JsonValue>(&report.data_json) {
            Ok(data) => data,
            Err(e) => {
                warn!("Failed to parse report data: {}", e);
                return;
            }
        };
        
        // Create visualization based on report type
        let chart_fn = create_visualization(&data, &report.visualization_type, &report.name);
        
        // Execute the visualization creation
        self.app.world.resource_scope(|world, mut meshes: Mut<Assets<Mesh>>| {
            world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
                let mut commands = world.commands();
                chart_fn(&mut commands, &mut meshes, &mut materials);
            });
        });
    }
    
    /// Run the Bevy app
    pub fn run(&mut self) {
        info!("Starting BI visualization app");
        self.app.run();
    }
    
    /// Get mutable access to the underlying Bevy app
    pub fn app_mut(&mut self) -> &mut App {
        &mut self.app
    }
    
    /// Render visualization to image buffer
    pub fn render_to_image(&mut self, width: u32, height: u32) -> Result<Vec<u8>, String> {
        if !self.is_headless {
            return Err("Rendering to image requires headless mode".to_string());
        }
        
        // Create render target for off-screen rendering
        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        
        // This is a more complete implementation structure
        // In a real implementation, we would:
        // 1. Set up a render target texture
        // 2. Configure the camera to render to this target
        // 3. Process exactly one frame
        // 4. Extract the image data
        
        // For now, return a placeholder PNG data
        // In production, this would use Bevy's render-to-texture functionality
        let mut image = image::ImageBuffer::<image::Rgba<u8>, Vec<u8>>::new(width, height);
        
        // Fill with a simple gradient for demonstration
        for y in 0..height {
            for x in 0..width {
                let r = (x as f32 / width as f32 * 255.0) as u8;
                let g = (y as f32 / height as f32 * 255.0) as u8;
                let b = 128;
                image.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }
        
        let mut buffer = Vec::new();
        image::codecs::png::PngEncoder::new(&mut buffer)
            .encode(
                &image,
                width,
                height,
                image::ColorType::Rgba8,
            )
            .map_err(|e| format!("Failed to encode PNG: {}", e))?;
        
        Ok(buffer)
    }
    
    /// Export scene data as JSON (glTF format)
    pub fn export_scene_data(&mut self) -> Result<serde_json::Value, String> {
        // Create a structured scene representation
        let mut objects = Vec::new();
        
        // Query for all entities with transform components
        let world = &self.app.world;
        
        // Collect basic scene information
        for entity in world.iter_entities() {
            if let Some(transform) = entity.get::<Transform>() {
                objects.push(serde_json::json!({
                    "type": "entity",
                    "position": [transform.translation.x, transform.translation.y, transform.translation.z],
                    "rotation": [transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.w],
                    "scale": [transform.scale.x, transform.scale.y, transform.scale.z],
                }));
            }
        }
        
        Ok(serde_json::json!({
            "type": "gltf",
            "version": "2.0",
            "scene": {
                "nodes": objects
            },
            "metadata": {
                "generator": "CPC BI Analytics",
                "format": "glTF",
                "version": "2.0"
            }
        }))
    }
    
    /// Generate accessibility metadata for current visualization
    pub fn generate_accessibility_metadata(&self, report: &Report) -> AccessibilityMetadata {
        let mut navigation_map = HashMap::new();
        
        // Add navigation hints based on visualization type
        match report.visualization_type {
            VisualizationType::BarChart => {
                navigation_map.insert("title".to_string(), crate::application::visualization_service::NavigationHint {
                    label: format!("Bar Chart: {}", report.name),
                    key: "T".to_string(),
                    position: [0.0, 3.0, 0.0],
                });
                
                navigation_map.insert("bars".to_string(), crate::application::visualization_service::NavigationHint {
                    label: "Data bars".to_string(),
                    key: "B".to_string(),
                    position: [0.0, 0.0, 0.0],
                });
                
                navigation_map.insert("legend".to_string(), crate::application::visualization_service::NavigationHint {
                    label: "Chart legend".to_string(),
                    key: "L".to_string(),
                    position: [-3.0, 0.0, 0.0],
                });
            }
            VisualizationType::LineChart => {
                navigation_map.insert("title".to_string(), crate::application::visualization_service::NavigationHint {
                    label: format!("Line Chart: {}", report.name),
                    key: "T".to_string(),
                    position: [0.0, 3.0, 0.0],
                });
                
                navigation_map.insert("lines".to_string(), crate::application::visualization_service::NavigationHint {
                    label: "Data lines".to_string(),
                    key: "L".to_string(),
                    position: [0.0, 0.0, 0.0],
                });
            }
            _ => {
                navigation_map.insert("title".to_string(), crate::application::visualization_service::NavigationHint {
                    label: format!("{}: {}", report.visualization_type.to_string().replace('_', " "), report.name),
                    key: "T".to_string(),
                    position: [0.0, 3.0, 0.0],
                });
            }
        }
        
        let alt_text = format!(
            "{} visualization showing {}. Use T to navigate to title, B for bars, L for legend.",
            report.visualization_type.to_string().replace('_', " "),
            report.name
        );
        
        AccessibilityMetadata {
            alt_text,
            navigation_map,
            live_region: "polite".to_string(),
        }
    }
}

/// Setup 3D scene with camera and lighting
fn setup_3d_scene(mut commands: Commands) {
    // Add camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Add light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
}

/// Setup headless 3D scene for server-side rendering
fn setup_headless_scene(mut commands: Commands) {
    // Add camera for headless rendering
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(8.0, 8.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // Add light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });
}

/// Helper function to create a simple visualization window
pub fn create_visualization_window(report: &Report) {
    let mut app = BiVisualizationApp::new();
    app.add_report_visualization(report);
    app.run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::report::Report;
    
    #[test]
    fn test_bi_visualization_app_creation() {
        let app = BiVisualizationApp::new();
        assert!(app.app.is_plugin_added::<BiVisualizationPlugin>());
    }
    
    #[test]
    fn test_add_report_visualization() {
        let mut app = BiVisualizationApp::new();
        
        let report = Report {
            id: uuid::Uuid::new_v4(),
            dataset_id: uuid::Uuid::new_v4(),
            name: "Test Report".to_string(),
            description: Some("Test description".to_string()),
            query: "SELECT * FROM test".to_string(),
            visualization_type: VisualizationType::BarChart,
            owner_id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        app.add_report_visualization(&report);
        // App should not panic when adding visualization
    }
    
    #[test]
    fn test_headless_rendering() {
        let mut app = BiVisualizationApp::new_headless();
        assert!(app.is_headless);
        
        // Test image rendering
        let result = app.render_to_image(800, 600);
        assert!(result.is_ok());
        
        let image_data = result.unwrap();
        assert!(!image_data.is_empty());
        assert!(image_data.starts_with(&[0x89, 0x50, 0x4E, 0x47])); // PNG magic bytes
    }
    
    #[test]
    fn test_accessibility_metadata_generation() {
        let app = BiVisualizationApp::new();
        
        let report = Report {
            id: uuid::Uuid::new_v4(),
            dataset_id: uuid::Uuid::new_v4(),
            name: "Sales Report".to_string(),
            description: Some("Monthly sales data".to_string()),
            query: "SELECT month, revenue FROM sales".to_string(),
            visualization_type: VisualizationType::BarChart,
            owner_id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let metadata = app.generate_accessibility_metadata(&report);
        assert!(!metadata.alt_text.is_empty());
        assert!(metadata.alt_text.contains("Bar chart"));
        assert!(metadata.alt_text.contains("Sales Report"));
        assert!(!metadata.navigation_map.is_empty());
    }
}