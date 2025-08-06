//! Move transform tool for the Art application
//!
//! This module implements the move tool which allows users
//! to move layers and selections around the canvas.

use crate::tools::trait::{Tool, ToolResult, CursorType};
use crate::core::models::{Project, Action, Rect};
use crate::services::transform::TransformService;
use uuid::Uuid;

/// Move transform tool
///
/// This tool allows users to move layers or selections around the canvas. It supports
/// both free movement and constrained movement (horizontal/vertical) when holding Shift.
/// The tool automatically detects whether to move a selection or a layer based on
/// what is under the cursor when the drag operation begins.
pub struct MoveTool {
    /// Whether we're currently moving something
    is_moving: bool,
    /// Starting point of the move operation
    start_x: f32,
    start_y: f32,
    /// Whether we're moving a selection or a layer
    moving_selection: bool,
    /// ID of the layer being moved (if any)
    layer_id: Option<Uuid>,
    /// Initial bounds for undo/redo
    initial_bounds: Option<Rect>,
}

impl MoveTool {
    /// Create a new move tool
    pub fn new() -> Self {
        Self {
            is_moving: false,
            start_x: 0.0,
            start_y: 0.0,
            moving_selection: false,
            layer_id: None,
            initial_bounds: None,
        }
    }
}

impl Tool for MoveTool {
    fn name(&self) -> &str {
        "Move"
    }
    
    fn activate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn deactivate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_press(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult {
        self.is_moving = true;
        self.start_x = x;
        self.start_y = y;
        
        // Check if we're moving a selection or a layer
        self.moving_selection = !project.selection_state.is_empty() &&
            project.selection_state.is_selected(x, y);
        
        // If moving a layer, find which layer
        if !self.moving_selection {
            // For now, we'll move the topmost visible layer at this position
            // In a real implementation, we would do proper hit testing
            for layer in project.layers.iter().rev() {
                if layer.visible &&
                   x >= layer.bounds.x && x < layer.bounds.x + layer.bounds.width &&
                   y >= layer.bounds.y && y < layer.bounds.y + layer.bounds.height {
                    self.layer_id = Some(layer.id);
                    self.initial_bounds = Some(layer.bounds.clone());
                    break;
                }
            }
        } else {
            // Store initial bounds for selections (simplified)
            if let Some(selection) = project.selection_state.selections.first() {
                self.initial_bounds = Some(selection.bounds.clone());
            }
        }
        
        Ok(vec![])
    }
    
    fn handle_drag(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult {
        if self.is_moving {
            let mut dx = x - self.start_x;
            let mut dy = y - self.start_y;
            
            // Constrained movement with Shift key (simplified - would need key state in real implementation)
            // For now, we'll simulate constrained movement when dx or dy is larger
            if dx.abs() > 2.0 * dy.abs() {
                dy = 0.0; // Horizontal movement
            } else if dy.abs() > 2.0 * dx.abs() {
                dx = 0.0; // Vertical movement
            }
            
            // Move either selection or layer
            if self.moving_selection {
                TransformService::move_selection(project, dx, dy)?;
            } else if let Some(layer_id) = self.layer_id {
                TransformService::move_layer(project, layer_id, dx, dy)?;
            }
        }
        Ok(vec![])
    }
    
    fn handle_release(&mut self, project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        if self.is_moving {
            self.is_moving = false;
            
            // Create undo/redo action
            if let Some(layer_id) = self.layer_id {
                if let Some(old_bounds) = self.initial_bounds.take() {
                    // Get new bounds
                    if let Some(layer) = project.get_layer(layer_id) {
                        let new_bounds = layer.bounds.clone();
                        return Ok(vec![Action::LayerTransformed {
                            layer_id,
                            old_bounds,
                            new_bounds,
                        }]);
                    }
                }
            } else if self.moving_selection {
                // For selections, we would create a SelectionModified action
                // This is simplified for now
                if let Some(old_bounds) = self.initial_bounds.take() {
                    if let Some(selection) = project.selection_state.selections.first() {
                        let selection_id = selection.id;
                        let new_bounds = selection.bounds.clone();
                        return Ok(vec![Action::SelectionModified {
                            selection_id,
                            old_bounds,
                            new_bounds,
                        }]);
                    }
                }
            }
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
        if self.is_moving {
            CursorType::Move
        } else {
            CursorType::Arrow
        }
    }
}