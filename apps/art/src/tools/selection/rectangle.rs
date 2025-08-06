//! Rectangle selection tool for the Art application
//!
//! This module implements the rectangle selection tool which allows users
//! to create rectangular selections on the canvas.

use crate::tools::trait::{Tool, ToolResult, CursorType};
use crate::core::models::{Project, Action, Rect};
use crate::core::selection::SelectionService;
use uuid::Uuid;

/// Rectangle selection tool
///
/// This tool allows users to create rectangular selections by clicking and dragging
/// on the canvas. The selection is created when the user releases the mouse button.
pub struct RectangleSelectionTool {
    /// Whether we're currently making a selection
    is_selecting: bool,
    /// Starting point of the selection
    start_x: f32,
    start_y: f32,
    /// Current end point of the selection
    end_x: f32,
    end_y: f32,
}

impl RectangleSelectionTool {
    /// Create a new rectangle selection tool
    pub fn new() -> Self {
        Self {
            is_selecting: false,
            start_x: 0.0,
            start_y: 0.0,
            end_x: 0.0,
            end_y: 0.0,
        }
    }
}

impl Tool for RectangleSelectionTool {
    fn name(&self) -> &str {
        "Rectangle Selection"
    }
    
    fn activate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn deactivate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_press(&mut self, _project: &mut Project, x: f32, y: f32) -> ToolResult {
        self.is_selecting = true;
        self.start_x = x;
        self.start_y = y;
        self.end_x = x;
        self.end_y = y;
        Ok(vec![])
    }
    
    fn handle_drag(&mut self, _project: &mut Project, x: f32, y: f32) -> ToolResult {
        if self.is_selecting {
            self.end_x = x;
            self.end_y = y;
        }
        Ok(vec![])
    fn handle_release(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult {
        if self.is_selecting {
            self.end_x = x;
            self.end_y = y;
            self.is_selecting = false;
            
            // Create the selection
            let min_x = self.start_x.min(self.end_x);
            let max_x = self.start_x.max(self.end_x);
            let min_y = self.start_y.min(self.end_y);
            let max_y = self.start_y.max(self.end_y);
            
            let width = max_x - min_x;
            let height = max_y - min_y;
            
            // Create rectangle selection using SelectionService
            SelectionService::create_rectangle_selection(project, min_x, min_y, width, height, None)?;
            
            // Get the selection ID (assuming it's the last one added)
            let selection_id = project.selection_state.selections.last()
                .map(|s| s.id)
                .unwrap_or_else(|| Uuid::nil());
            
            // Return action for undo/redo
            Ok(vec![Action::SelectionCreated { selection_id }])
        } else {
            Ok(vec![])
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
        CursorType::Crosshair
    }
}