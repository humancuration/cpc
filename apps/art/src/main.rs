//! Art App - Digital Creation Tools
//!
//! Entry point for the Art application, a professional-grade digital creation tool
//! similar to Photoshop, Krita, and Procreate.

use bevy::prelude::*;
use rendering::{ArtRenderingPlugin, RenderProject};
use tools::brush::BrushToolPlugin;
use core::models::{Project, Layer, LayerType};

mod core;
mod rendering;
mod tools;
mod persistence;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ArtRenderingPlugin)
        .add_plugins(BrushToolPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
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