//! Selection system for the Art application
//!
//! This module provides functionality for managing selections in the art application,
//! including different selection types and operations.

use crate::core::models::{Project, Layer, Rect};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Different types of selections that can be made
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SelectionType {
    /// Rectangular selection
    Rectangle,
    /// Freeform (lasso) selection
    Lasso,
    /// Magic wand selection based on color similarity
    MagicWand,
}

/// Represents a selection area in the canvas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionArea {
    /// Type of selection
    pub selection_type: SelectionType,
    /// Bounding rectangle of the selection
    pub bounds: Rect,
    /// Raw pixel data representing the selection mask (0 = unselected, 255 = selected)
    pub mask: Vec<u8>,
    /// Layer this selection is applied to (None for all layers)
    pub layer_id: Option<Uuid>,
}

impl SelectionArea {
    /// Create a new rectangular selection
    pub fn new_rectangle(x: f32, y: f32, width: f32, height: f32, layer_id: Option<Uuid>) -> Self {
        let bounds = Rect { x, y, width, height };
        let mask = vec![255; (width as usize) * (height as usize)];
        
        Self {
            selection_type: SelectionType::Rectangle,
            bounds,
            mask,
            layer_id,
        }
    }
    
    /// Create a new lasso selection (stub implementation)
    pub fn new_lasso(points: &[(f32, f32)], layer_id: Option<Uuid>) -> Self {
        // Calculate bounds from points
        let mut min_x = points[0].0;
        let mut max_x = points[0].0;
        let mut min_y = points[0].1;
        let mut max_y = points[0].1;
        
        for &(x, y) in points {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        
        let bounds = Rect {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        };
        
        // For now, create a simple mask (in a real implementation, this would rasterize the polygon)
        let mask = vec![255; (bounds.width as usize) * (bounds.height as usize)];
        
        Self {
            selection_type: SelectionType::Lasso,
            bounds,
            mask,
            layer_id,
        }
    }
    
    /// Create a new magic wand selection (stub implementation)
    pub fn new_magic_wand(
        layer: &Layer,
        start_x: u32,
        start_y: u32,
        tolerance: f32,
        layer_id: Option<Uuid>,
    ) -> Self {
        let bounds = layer.bounds.clone();
        // For now, create a simple mask (in a real implementation, this would perform flood fill)
        let mask = vec![255; (bounds.width as usize) * (bounds.height as usize)];
        
        Self {
            selection_type: SelectionType::MagicWand,
            bounds,
            mask,
            layer_id,
        }
    }
    
    /// Check if a point is within the selection
    pub fn contains(&self, x: f32, y: f32) -> bool {
        if x < self.bounds.x || x >= self.bounds.x + self.bounds.width ||
           y < self.bounds.y || y >= self.bounds.y + self.bounds.height {
            return false;
        }
        
        // Convert to mask coordinates
        let mask_x = (x - self.bounds.x) as usize;
        let mask_y = (y - self.bounds.y) as usize;
        let index = mask_y * (self.bounds.width as usize) + mask_x;
        
        if index < self.mask.len() {
            self.mask[index] > 0
        } else {
            false
        }
    }
    
    /// Get the mask value at a specific point
    pub fn mask_value(&self, x: f32, y: f32) -> u8 {
        if x < self.bounds.x || x >= self.bounds.x + self.bounds.width ||
           y < self.bounds.y || y >= self.bounds.y + self.bounds.height {
            return 0;
        }
        
        // Convert to mask coordinates
        let mask_x = (x - self.bounds.x) as usize;
        let mask_y = (y - self.bounds.y) as usize;
        let index = mask_y * (self.bounds.width as usize) + mask_x;
        
        if index < self.mask.len() {
            self.mask[index]
        } else {
            0
        }
    }
}

/// Current selection state for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionState {
    /// Active selections
    pub selections: Vec<SelectionArea>,
    /// Whether the selection is currently being modified
    pub is_modifying: bool,
}

impl SelectionState {
    /// Create a new empty selection state
    pub fn new() -> Self {
        Self {
            selections: Vec::new(),
            is_modifying: false,
        }
    }
    
    /// Add a new selection
    pub fn add_selection(&mut self, selection: SelectionArea) {
        self.selections.push(selection);
    }
    
    /// Clear all selections
    pub fn clear(&mut self) {
        self.selections.clear();
    }
    
    /// Check if there are any active selections
    pub fn is_empty(&self) -> bool {
        self.selections.is_empty()
    }
    
    /// Check if a point is selected in any selection
    pub fn is_selected(&self, x: f32, y: f32) -> bool {
        self.selections.iter().any(|selection| selection.contains(x, y))
    }
}

/// Service for managing selections
pub struct SelectionService;

impl SelectionService {
    /// Create a rectangular selection
    pub fn create_rectangle_selection(
        project: &mut Project,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        layer_id: Option<Uuid>,
    ) -> Result<(), String> {
        let selection = SelectionArea::new_rectangle(x, y, width, height, layer_id);
        project.selection_state.add_selection(selection);
        Ok(())
    }
    
    /// Create a lasso selection
    pub fn create_lasso_selection(
        project: &mut Project,
        points: Vec<(f32, f32)>,
        layer_id: Option<Uuid>,
    ) -> Result<(), String> {
        if points.is_empty() {
            return Err("Lasso selection requires at least one point".to_string());
        }
        
        let selection = SelectionArea::new_lasso(&points, layer_id);
        project.selection_state.add_selection(selection);
        Ok(())
    }
    
    /// Create a magic wand selection
    pub fn create_magic_wand_selection(
        project: &mut Project,
        layer_id: Uuid,
        x: u32,
        y: u32,
        tolerance: f32,
    ) -> Result<(), String> {
        let layer = project.get_layer(layer_id)
            .ok_or("Layer not found")?;
        
        let selection = SelectionArea::new_magic_wand(layer, x, y, tolerance, Some(layer_id));
        project.selection_state.add_selection(selection);
        Ok(())
    }
    
    /// Clear all selections
    pub fn clear_selections(project: &mut Project) {
        project.selection_state.clear();
    }
    
    /// Invert the current selection
    pub fn invert_selection(project: &mut Project) -> Result<(), String> {
        // In a real implementation, this would invert all selection masks
        // For now, we'll just clear the selections
        project.selection_state.clear();
        Ok(())
    }
}