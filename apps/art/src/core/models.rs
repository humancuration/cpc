//! Core data models for the Art application
//!
//! This module defines the fundamental data structures used throughout the application:
//! - Project: The top-level container for all artwork
//! - Layer: Individual layers that make up a project
//! - Brush: Brush definitions for painting tools

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::core::selection::SelectionState;
use crate::core::effects::LayerEffect;

/// Supported color modes for projects
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorMode {
    RGB,
    CMYK,
    Grayscale,
}

/// Blend modes for layers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    SoftLight,
    HardLight,
    ColorDodge,
    ColorBurn,
    Darken,
    Lighten,
    Difference,
    Exclusion,
}

/// Types of layers that can exist in a project
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LayerType {
    Raster,
    Vector,
    Text,
    Adjustment,
}

/// Rectangle bounds for positioning
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Layer representation in a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: Uuid,
    pub name: String,
    pub kind: LayerType,
    pub opacity: f32,
    pub visible: bool,
    pub blend_mode: BlendMode,
    pub metadata: serde_json::Value,
    pub pixels: Vec<u8>,
    pub bounds: Rect,
    pub effects: Vec<LayerEffect>,
}

impl Layer {
    /// Create a new layer
    pub fn new(name: String, width: u32, height: u32, kind: LayerType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            kind,
            opacity: 1.0,
            visible: true,
            blend_mode: BlendMode::Normal,
            metadata: serde_json::Value::Null,
            pixels: vec![0; (width * height * 4) as usize], // RGBA format
            bounds: Rect {
                x: 0.0,
                y: 0.0,
                width: width as f32,
                height: height as f32,
            },
            effects: Vec::new(),
        }
    }

    /// Set the blend mode for this layer
    pub fn set_blend_mode(&mut self, mode: BlendMode) {
        self.blend_mode = mode;
    }

    /// Set the opacity for this layer (0.0 to 1.0)
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }

    /// Toggle layer visibility
    pub fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}

/// Brush dynamics for pressure/tilt sensitivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrushDynamics {
    pub pressure_size: bool,
    pub pressure_opacity: bool,
    pub tilt_size: bool,
    pub tilt_opacity: bool,
}

impl Default for BrushDynamics {
    fn default() -> Self {
        Self {
            pressure_size: false,
            pressure_opacity: false,
            tilt_size: false,
            tilt_opacity: false,
        }
    }
}

/// Brush representation for painting tools
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brush {
    pub id: Uuid,
    pub name: String,
    pub size: f32,
    pub hardness: f32,
    pub opacity: f32,
    pub spacing: f32,
    pub texture: Option<Uuid>, // Reference to texture ID
    pub dynamics: BrushDynamics,
}

impl Brush {
    /// Create a new brush
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            size: 10.0,
            hardness: 1.0,
            opacity: 1.0,
            spacing: 0.25,
            texture: None,
            dynamics: BrushDynamics::default(),
        }
    }

    /// Set brush size
    pub fn set_size(&mut self, size: f32) {
        self.size = size.max(1.0);
    }

    /// Set brush hardness (0.0 to 1.0)
    pub fn set_hardness(&mut self, hardness: f32) {
        self.hardness = hardness.clamp(0.0, 1.0);
    }

    /// Set brush opacity (0.0 to 1.0)
    pub fn set_opacity(&mut self, opacity: f32) {
        self.opacity = opacity.clamp(0.0, 1.0);
    }
}

/// Action for undo/redo history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    BrushStroke {
        layer_id: Uuid,
        points: Vec<(f32, f32)>,
        brush: Brush,
    },
    LayerAdded {
        layer: Layer,
    },
    LayerRemoved {
        layer_id: Uuid,
    },
    LayerReordered {
        layer_id: Uuid,
        from_index: usize,
        to_index: usize,
    },
}

/// Project representation - the top-level container for artwork
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub color_mode: ColorMode,
    pub resolution: f32, // Pixels per inch
    pub layers: Vec<Layer>,
    pub history: Vec<Action>,
    pub metadata: serde_json::Value,
    pub selection_state: SelectionState,
}

impl Project {
    /// Create a new project
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            width,
            height,
            color_mode: ColorMode::RGB,
            resolution: 72.0,
            layers: vec![],
            history: vec![],
            metadata: serde_json::Value::Null,
            selection_state: SelectionState::new(),
        }
    }

    /// Add a new layer to the project
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
        // Add to history for undo/redo
        self.history.push(Action::LayerAdded {
            layer: layer.clone(),
        });
    }

    /// Remove a layer from the project
    pub fn remove_layer(&mut self, layer_id: Uuid) -> Option<Layer> {
        if let Some(index) = self.layers.iter().position(|l| l.id == layer_id) {
            let layer = self.layers.remove(index);
            // Add to history for undo/redo
            self.history.push(Action::LayerRemoved { layer_id });
            Some(layer)
        } else {
            None
        }
    }

    /// Reorder layers
    pub fn reorder_layer(&mut self, layer_id: Uuid, to_index: usize) {
        if let Some(from_index) = self.layers.iter().position(|l| l.id == layer_id) {
            if from_index != to_index && to_index < self.layers.len() {
                let layer = self.layers.remove(from_index);
                self.layers.insert(to_index, layer);
                // Add to history for undo/redo
                self.history.push(Action::LayerReordered {
                    layer_id,
                    from_index,
                    to_index,
                });
            }
        }
    }

    /// Get a mutable reference to a layer by ID
    pub fn get_layer_mut(&mut self, layer_id: Uuid) -> Option<&mut Layer> {
        self.layers.iter_mut().find(|l| l.id == layer_id)
    }

    /// Get a reference to a layer by ID
    pub fn get_layer(&self, layer_id: Uuid) -> Option<&Layer> {
        self.layers.iter().find(|l| l.id == layer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_layer() {
        let layer = Layer::new("Test Layer".to_string(), 100, 100, LayerType::Raster);
        assert_eq!(layer.name, "Test Layer");
        assert_eq!(layer.kind, LayerType::Raster);
        assert_eq!(layer.opacity, 1.0);
        assert!(layer.visible);
        assert_eq!(layer.pixels.len(), 100 * 100 * 4); // RGBA
    }

    #[test]
    fn test_create_brush() {
        let mut brush = Brush::new("Test Brush".to_string());
        assert_eq!(brush.name, "Test Brush");
        assert_eq!(brush.size, 10.0);

        brush.set_size(20.0);
        assert_eq!(brush.size, 20.0);

        brush.set_hardness(0.5);
        assert_eq!(brush.hardness, 0.5);

        brush.set_opacity(0.8);
        assert_eq!(brush.opacity, 0.8);
    }

    #[test]
    fn test_create_project() {
        let project = Project::new("Test Project".to_string(), 800, 600);
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.width, 800);
        assert_eq!(project.height, 600);
        assert_eq!(project.color_mode, ColorMode::RGB);
    }

    #[test]
    fn test_layer_operations() {
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        let layer = Layer::new("Layer 1".to_string(), 800, 600, LayerType::Raster);
        let layer_id = layer.id;

        // Add layer
        project.add_layer(layer);
        assert_eq!(project.layers.len(), 1);

        // Get layer
        assert!(project.get_layer(layer_id).is_some());

        // Remove layer
        let removed = project.remove_layer(layer_id);
        assert!(removed.is_some());
        assert_eq!(project.layers.len(), 0);
    }
}