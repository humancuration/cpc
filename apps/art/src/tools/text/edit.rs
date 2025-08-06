//! Text editing tool for the Art application
//!
//! This module implements the text editing tool which allows users
//! to edit existing text layers on the canvas.

use crate::tools::trait::{Tool, ToolResult, CursorType};
use crate::core::models::Project;
use uuid::Uuid;

/// Text editing tool
pub struct TextEditTool {
    /// ID of the text layer being edited
    editing_layer_id: Option<Uuid>,
    /// Current text content
    text_content: String,
}

impl TextEditTool {
    /// Create a new text editing tool
    pub fn new() -> Self {
        Self {
            editing_layer_id: None,
            text_content: String::new(),
        }
    }
    
    /// Start editing a text layer
    pub fn start_editing(&mut self, layer_id: Uuid, initial_content: String) {
        self.editing_layer_id = Some(layer_id);
        self.text_content = initial_content;
    }
    
    /// Stop editing and apply changes
    pub fn stop_editing(&mut self, project: &mut Project) -> Result<(), String> {
        if let Some(layer_id) = self.editing_layer_id {
            // In a real implementation, this would update the text layer with the new content
            println!("Updating text layer {:?} with content: {}", layer_id, self.text_content);
            self.editing_layer_id = None;
            Ok(())
        } else {
            Err("Not currently editing a text layer".to_string())
        }
    }
}

impl Tool for TextEditTool {
    fn name(&self) -> &str {
        "Text Edit"
    }
    
    fn activate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn deactivate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_press(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult {
        // Check if we clicked on a text layer
        // In a real implementation, this would check for text layers at the click position
        println!("Checking for text layer at ({}, {})", x, y);
        Ok(vec![])
    }
    
    fn handle_drag(&mut self, _project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        // Text editing doesn't typically support dragging
        Ok(vec![])
    }
    
    fn handle_release(&mut self, _project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_key_press(&mut self, _project: &mut Project, key: &str) -> ToolResult {
        if self.editing_layer_id.is_some() {
            // Handle text input
            if key == "Backspace" {
                self.text_content.pop();
            } else if key.len() == 1 {
                self.text_content.push_str(key);
            }
        }
        Ok(vec![])
    }
    
    fn handle_key_release(&mut self, _project: &mut Project, _key: &str) -> ToolResult {
        Ok(vec![])
    }
    
    fn cursor(&self) -> CursorType {
        if self.editing_layer_id.is_some() {
            CursorType::Arrow
        } else {
            CursorType::Crosshair
        }
    }
}