//! Text creation tool for the Art application
//!
//! This module implements the text creation tool which allows users
//! to create new text layers on the canvas.

use crate::tools::trait::{Tool, ToolResult, CursorType};
use crate::core::models::{Project, Layer, LayerType};

/// Text creation tool
pub struct TextCreateTool {
    /// Whether we're currently creating text
    is_creating: bool,
    /// Position where text creation started
    x: f32,
    y: f32,
    /// Text content being created
    text_content: String,
}

impl TextCreateTool {
    /// Create a new text creation tool
    pub fn new() -> Self {
        Self {
            is_creating: false,
            x: 0.0,
            y: 0.0,
            text_content: String::new(),
        }
    }
}

impl Tool for TextCreateTool {
    fn name(&self) -> &str {
        "Text Create"
    }
    
    fn activate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn deactivate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_press(&mut self, _project: &mut Project, x: f32, y: f32) -> ToolResult {
        self.is_creating = true;
        self.x = x;
        self.y = y;
        self.text_content.clear();
        Ok(vec![])
    }
    
    fn handle_drag(&mut self, _project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        // Text creation doesn't typically support dragging
        Ok(vec![])
    }
    
    fn handle_release(&mut self, project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        if self.is_creating {
            self.is_creating = false;
            
            // Create a new text layer
            let mut layer = Layer::new("Text Layer".to_string(), 200, 50, LayerType::Text);
            layer.bounds.x = self.x;
            layer.bounds.y = self.y;
            
            // In a real implementation, this would open a text editor UI
            println!("Creating text layer at ({}, {})", self.x, self.y);
            
            // Add the layer to the project
            project.add_layer(layer);
        }
        Ok(vec![])
    }
    
    fn handle_key_press(&mut self, _project: &mut Project, _key: &str) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_key_release(&mut self, _project: &mut Project, _key: &str) -> ToolResult {
        Ok(vec![])
    }
    
    fn cursor(&self) -> CursorType {
        CursorType::Crosshair
    }
}