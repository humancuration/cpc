//! Magic wand selection tool for the Art application
//!
//! This module implements the magic wand selection tool which allows users
//! to create selections based on color similarity.

use crate::tools::trait::{Tool, ToolResult, CursorType};
use crate::core::models::{Project, Action, Layer};
use crate::core::selection::SelectionService;
use uuid::Uuid;
use std::collections::VecDeque;

/// Magic wand selection tool
///
/// This tool allows users to create selections based on color similarity. When the
/// user clicks on a pixel, the tool selects all connected pixels that are within
/// the specified tolerance of the clicked pixel's color.
///
/// For implementation details of the selection algorithm, see `crate::core::selection::SelectionArea::new_magic_wand`.
pub struct MagicWandTool {
    /// Tolerance for color matching (0.0 to 1.0)
    tolerance: f32,
    /// Whether to use anti-aliasing
    anti_alias: bool,
    /// Whether to use contiguous selection
    contiguous: bool,
}

impl MagicWandTool {
    /// Create a new magic wand tool
    pub fn new() -> Self {
        Self {
            tolerance: 0.1,
            anti_alias: true,
            contiguous: true,
        }
    }
    
    /// Set the tolerance for color matching
    pub fn set_tolerance(&mut self, tolerance: f32) {
        self.tolerance = tolerance.clamp(0.0, 1.0);
    }
    
    /// Set whether to use anti-aliasing
    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        self.anti_alias = anti_alias;
    }
    
    /// Set whether to use contiguous selection
    pub fn set_contiguous(&mut self, contiguous: bool) {
        self.contiguous = contiguous;
    }
}

impl Tool for MagicWandTool {
    fn name(&self) -> &str {
        "Magic Wand"
    }
    
    fn activate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn deactivate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_press(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult {
        // For now, we'll just use the first layer as an example
        // In a real implementation, we would determine which layer was clicked
        if let Some(layer) = project.layers.first() {
            let layer_id = layer.id;
            
            // Convert float coordinates to integer pixel coordinates
            let px = x as u32;
            let py = y as u32;
            
            // Create magic wand selection using SelectionService
            SelectionService::create_magic_wand_selection(project, layer_id, px, py, self.tolerance, self.contiguous)?;
            
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
    
    fn handle_drag(&mut self, _project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        // Magic wand doesn't typically support dragging
        Ok(vec![])
    }
    
    fn handle_release(&mut self, _project: &mut Project, _x: f32, _y: f32) -> ToolResult {
        // Magic wand selection is created on press, not release
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