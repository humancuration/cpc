//! Undo/Redo functionality for the Art application
//!
//! This module implements the command pattern for undo/redo functionality,
//! allowing users to reverse and reapply their actions.

use crate::core::models::{Project, Action, Layer};
use std::collections::VecDeque;

/// Error types for undo/redo operations
#[derive(Debug)]
pub enum UndoError {
    NothingToUndo,
    NothingToRedo,
    InvalidAction,
}

/// History manager for undo/redo functionality
pub struct HistoryManager {
    /// Completed actions that can be undone
    undo_stack: Vec<Action>,
    /// Undone actions that can be redone
    redo_stack: Vec<Action>,
    /// Maximum number of actions to keep in history
    max_history: usize,
}

impl HistoryManager {
    /// Create a new history manager
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history,
        }
    }
    
    /// Add an action to the history
    pub fn add_action(&mut self, action: Action) {
        self.undo_stack.push(action);
        self.redo_stack.clear(); // Clear redo stack when new action is added
        
        // Limit history size
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.remove(0);
        }
    }
    
    /// Undo the last action
    pub fn undo(&mut self, project: &mut Project) -> Result<(), UndoError> {
        if let Some(action) = self.undo_stack.pop() {
            self.redo_stack.push(action.clone());
            self.execute_undo_action(project, action)?;
            Ok(())
        } else {
            Err(UndoError::NothingToUndo)
        }
    }
    
    /// Redo the last undone action
    pub fn redo(&mut self, project: &mut Project) -> Result<(), UndoError> {
        if let Some(action) = self.redo_stack.pop() {
            self.undo_stack.push(action.clone());
            self.execute_redo_action(project, action)?;
            Ok(())
        } else {
            Err(UndoError::NothingToRedo)
        }
    }
    
    /// Execute an undo action on the project
    fn execute_undo_action(&self, project: &mut Project, action: Action) -> Result<(), UndoError> {
        match action {
            Action::BrushStroke { layer_id, .. } => {
                // In a real implementation, we would restore the layer state before the stroke
                // For now, we'll just log that the action was undone
                println!("Undid brush stroke on layer {:?}", layer_id);
                Ok(())
            }
            Action::LayerAdded { layer } => {
                // Remove the layer that was added
                project.remove_layer(layer.id);
                Ok(())
            }
            Action::LayerRemoved { layer_id } => {
                // In a real implementation, we would restore the removed layer
                // For now, we'll just log that the action was undone
                println!("Undid layer removal: {:?}", layer_id);
                Ok(())
            }
            Action::LayerReordered { layer_id, from_index, to_index } => {
                // Reorder back to original position
                project.reorder_layer(layer_id, from_index);
                Ok(())
            }
        }
    }
    
    /// Execute a redo action on the project
    fn execute_redo_action(&self, project: &mut Project, action: Action) -> Result<(), UndoError> {
        match action {
            Action::BrushStroke { layer_id, points, brush } => {
                // In a real implementation, we would reapply the brush stroke
                // For now, we'll just log that the action was redone
                println!("Redid brush stroke on layer {:?} with {} points", layer_id, points.len());
                Ok(())
            }
            Action::LayerAdded { layer } => {
                // Re-add the layer
                project.add_layer(layer);
                Ok(())
            }
            Action::LayerRemoved { layer_id } => {
                // Remove the layer again
                project.remove_layer(layer_id);
                Ok(())
            }
            Action::LayerReordered { layer_id, from_index: _, to_index } => {
                // Reorder to the new position
                project.reorder_layer(layer_id, to_index);
                Ok(())
            }
        }
    }
    
    /// Check if there are actions that can be undone
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    
    /// Check if there are actions that can be redone
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    
    /// Clear all history
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{LayerType};
    
    #[test]
    fn test_history_manager() {
        let mut history = HistoryManager::new(10);
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        
        // Initially, nothing to undo or redo
        assert!(!history.can_undo());
        assert!(!history.can_redo());
        
        // Add a layer
        let layer = Layer::new("Test Layer".to_string(), 800, 600, LayerType::Raster);
        let layer_id = layer.id;
        let action = Action::LayerAdded { layer: layer.clone() };
        history.add_action(action);
        
        // Now we can undo
        assert!(history.can_undo());
        assert!(!history.can_redo());
        
        // Execute undo
        let result = history.undo(&mut project);
        assert!(result.is_ok());
        assert!(!history.can_undo());
        assert!(history.can_redo());
        assert_eq!(project.layers.len(), 0);
        
        // Execute redo
        let result = history.redo(&mut project);
        assert!(result.is_ok());
        assert!(history.can_undo());
        assert!(!history.can_redo());
        assert_eq!(project.layers.len(), 1);
        assert_eq!(project.layers[0].id, layer_id);
    }
    
    #[test]
    fn test_history_limit() {
        let mut history = HistoryManager::new(3);
        
        // Add more actions than the limit
        for i in 0..5 {
            let action = Action::LayerAdded {
                layer: Layer::new(format!("Layer {}", i), 100, 100, LayerType::Raster),
            };
            history.add_action(action);
        }
        
        // Check that only the last 3 actions are kept
        assert_eq!(history.undo_stack.len(), 3);
    }
    
    #[test]
    fn test_undo_error() {
        let mut history = HistoryManager::new(10);
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        
        // Try to undo with empty history
        let result = history.undo(&mut project);
        assert!(result.is_err());
        match result.unwrap_err() {
            UndoError::NothingToUndo => {},
            _ => panic!("Expected NothingToUndo error"),
        }
    }
    
    #[test]
    fn test_redo_error() {
        let mut history = HistoryManager::new(10);
        let mut project = Project::new("Test Project".to_string(), 800, 600);
        
        // Try to redo with empty redo stack
        let result = history.redo(&mut project);
        assert!(result.is_err());
        match result.unwrap_err() {
            UndoError::NothingToRedo => {},
            _ => panic!("Expected NothingToRedo error"),
        }
    }
}