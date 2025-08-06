//! Art App - Digital Creation Tools
//!
//! Entry point for the Art application, a professional-grade digital creation tool
//! similar to Photoshop, Krita, and Procreate.

use bevy::prelude::*;
use rendering::{ArtRenderingPlugin, RenderProject};
use tools::brush::BrushToolPlugin;
use core::models::{Project, Layer, LayerType};

// Renderer backends
use rendering_core::Renderer;
use opengl_renderer::OpenGLRenderer;
use vulkan_renderer::VulkanRenderer;
use rendering::render_manager::RenderManager;

mod core;
mod rendering;
mod tools;
mod persistence;

/// Available rendering backends
#[derive(Debug, Clone, PartialEq)]
pub enum RenderBackend {
    Bevy,
    OpenGL,
    Vulkan,
}
fn main() {
    // For now, we'll default to Bevy renderer
    // In a real implementation, this would be configurable
    let backend = RenderBackend::Bevy;
    
    // Create render manager
    let mut render_manager = RenderManager::new();
    render_manager.switch_backend(backend.clone());
    render_manager.init_renderer();
    
    match backend {
        RenderBackend::Bevy => {
            App::new()
                .add_plugins(DefaultPlugins)
                .add_plugins(ArtRenderingPlugin)
                .add_plugins(BrushToolPlugin)
                .add_systems(Startup, setup)
                .add_systems(Update, update)
                .run();
        },
        RenderBackend::OpenGL => {
            // Initialize OpenGL renderer
            // This would require a different initialization approach
            println!("Using OpenGL renderer");
        },
        RenderBackend::Vulkan => {
            // Initialize Vulkan renderer
            // This would require a different initialization approach
            println!("Using Vulkan renderer");
        },
    }
}
}

fn setup(
    mut commands: Commands,
) {
    // Create a sample project
    let mut project = Project::new("Sample Project".to_string(), 800, 600);
    
    // Create a sample layer
    let mut layer = Layer::new("Layer 1".to_string(), 800, 600, LayerType::Raster);
    
    // Add some sample pixel data (red color)
    for i in 0..layer.pixels.len() {
        if i % 4 == 0 {
            layer.pixels[i] = 255; // Red
        } else if i % 4 == 1 {
            layer.pixels[i] = 0; // Green
        } else if i % 4 == 2 {
            layer.pixels[i] = 0; // Blue
        } else {
            layer.pixels[i] = 255; // Alpha
        }
    }
    
    project.add_layer(layer);
    
    // Insert the project as a resource for rendering
    commands.insert_resource(RenderProject { project });
}

fn update() {
    // Main update loop for the art application
    // Will be expanded with actual art functionality
}