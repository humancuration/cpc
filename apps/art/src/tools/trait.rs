//! Tool trait for the Art application
//!
//! This module defines the common interface that all tools must implement.

use crate::core::models::{Project, Action};
use uuid::Uuid;

/// Result type for tool operations
pub type ToolResult = Result<Vec<Action>, String>;

/// Common interface for all editing tools
pub trait Tool {
    /// Get the name of the tool
    fn name(&self) -> &str;
    
    /// Activate the tool
    fn activate(&mut self) -> ToolResult;
    
    /// Deactivate the tool
    fn deactivate(&mut self) -> ToolResult;
    
    /// Handle a mouse/touch press event
    fn handle_press(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult;
    
    /// Handle a mouse/touch drag event
    fn handle_drag(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult;
    
    /// Handle a mouse/touch release event
    fn handle_release(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult;
    
    /// Handle a key press event
    fn handle_key_press(&mut self, project: &mut Project, key: &str) -> ToolResult;
    
    /// Handle a key release event
    fn handle_key_release(&mut self, project: &mut Project, key: &str) -> ToolResult;
    
    /// Get the cursor type for this tool
    fn cursor(&self) -> CursorType;
}

/// Types of cursors that tools can use
#[derive(Debug, Clone, PartialEq)]
pub enum CursorType {
    /// Default arrow cursor
    Arrow,
    /// Crosshair cursor
    Crosshair,
    /// Move cursor
    Move,
    /// Resize cursor with direction
    Resize(ResizeDirection),
    /// Custom cursor with a specific icon
    Custom(String),
}

/// Directions for resize cursors
#[derive(Debug, Clone, PartialEq)]
pub enum ResizeDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}