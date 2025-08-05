//! Transform service for the Art application
//!
//! This module provides functionality for transforming layers and selections,
//! including move, scale, and rotate operations.

use crate::core::models::{Project, Layer, Rect};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Types of transformations that can be applied
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransformType {
    /// Move/translate transformation
    Move(f32, f32),
    /// Scale transformation
    Scale(f32, f32),
    /// Rotate transformation (in degrees)
    Rotate(f32),
    /// Free transform (combination of scale, rotate, and move)
    Free(f32, f32, f32, f32), // x, y, scale_x, scale_y, rotation
}

/// Transform matrix for 2D transformations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformMatrix {
    /// 3x3 transformation matrix stored as [a, b, c, d, e, f] where:
    /// | a  b  c |
    /// | d  e  f |
    /// | 0  0  1 |
    pub matrix: [f32; 6],
}

impl TransformMatrix {
    /// Create a new identity matrix
    pub fn identity() -> Self {
        Self {
            matrix: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0],
        }
    }
    
    /// Create a translation matrix
    pub fn translation(tx: f32, ty: f32) -> Self {
        Self {
            matrix: [1.0, 0.0, tx, 0.0, 1.0, ty],
        }
    }
    
    /// Create a scaling matrix
    pub fn scaling(sx: f32, sy: f32) -> Self {
        Self {
            matrix: [sx, 0.0, 0.0, 0.0, sy, 0.0],
        }
    }
    
    /// Create a rotation matrix (angle in degrees)
    pub fn rotation(angle: f32) -> Self {
        let radians = angle.to_radians();
        let cos = radians.cos();
        let sin = radians.sin();
        
        Self {
            matrix: [cos, -sin, 0.0, sin, cos, 0.0],
        }
    }
    
    /// Combine this transform with another (this * other)
    pub fn combine(&self, other: &TransformMatrix) -> Self {
        let [a1, b1, c1, d1, e1, f1] = self.matrix;
        let [a2, b2, c2, d2, e2, f2] = other.matrix;
        
        Self {
            matrix: [
                a1 * a2 + b1 * d2,
                a1 * b2 + b1 * e2,
                a1 * c2 + b1 * f2 + c1,
                d1 * a2 + e1 * d2,
                d1 * b2 + e1 * e2,
                d1 * c2 + e1 * f2 + f1,
            ],
        }
    }
    
    /// Apply this transformation to a point
    pub fn apply_to_point(&self, x: f32, y: f32) -> (f32, f32) {
        let [a, b, c, d, e, f] = self.matrix;
        (a * x + b * y + c, d * x + e * y + f)
    }
    
    /// Get the inverse of this transformation matrix
    pub fn inverse(&self) -> Option<TransformMatrix> {
        let [a, b, c, d, e, f] = self.matrix;
        let det = a * e - b * d;
        
        if det.abs() < f32::EPSILON {
            return None; // Matrix is not invertible
        }
        
        let inv_det = 1.0 / det;
        
        Some(TransformMatrix {
            matrix: [
                inv_det * e,
                inv_det * -b,
                inv_det * (b * f - c * e),
                inv_det * -d,
                inv_det * a,
                inv_det * (c * d - a * f),
            ],
        })
    }
}

/// Service for managing transformations
pub struct TransformService;

impl TransformService {
    /// Move a layer by a specified offset
    pub fn move_layer(
        project: &mut Project,
        layer_id: Uuid,
        dx: f32,
        dy: f32,
    ) -> Result<(), String> {
        let layer = project.get_layer_mut(layer_id)
            .ok_or("Layer not found")?;
        
        layer.bounds.x += dx;
        layer.bounds.y += dy;
        
        Ok(())
    }
    
    /// Scale a layer by a specified factor
    pub fn scale_layer(
        project: &mut Project,
        layer_id: Uuid,
        scale_x: f32,
        scale_y: f32,
    ) -> Result<(), String> {
        let layer = project.get_layer_mut(layer_id)
            .ok_or("Layer not found")?;
        
        // Scale the bounds
        layer.bounds.width *= scale_x;
        layer.bounds.height *= scale_y;
        
        // In a real implementation, we would also scale the pixel data
        // For now, we'll just update the bounds
        
        Ok(())
    }
    
    /// Rotate a layer by a specified angle (in degrees)
    pub fn rotate_layer(
        project: &mut Project,
        layer_id: Uuid,
        angle: f32,
    ) -> Result<(), String> {
        // For now, we'll just log that a rotation was requested
        // In a real implementation, this would apply a rotation transformation
        println!("Rotating layer {:?} by {} degrees", layer_id, angle);
        Ok(())
    }
    
    /// Apply a free transform to a layer
    pub fn free_transform(
        project: &mut Project,
        layer_id: Uuid,
        transform: TransformMatrix,
    ) -> Result<(), String> {
        let layer = project.get_layer_mut(layer_id)
            .ok_or("Layer not found")?;
        
        // Apply transformation to bounds
        let (new_x, new_y) = transform.apply_to_point(layer.bounds.x, layer.bounds.y);
        let (new_width, new_height) = transform.apply_to_point(layer.bounds.width, layer.bounds.height);
        
        layer.bounds.x = new_x;
        layer.bounds.y = new_y;
        layer.bounds.width = new_width.abs();
        layer.bounds.height = new_height.abs();
        
        // In a real implementation, we would also transform the pixel data
        // For now, we'll just update the bounds
        
        Ok(())
    }
    
    /// Move a selection by a specified offset
    pub fn move_selection(
        project: &mut Project,
        dx: f32,
        dy: f32,
    ) -> Result<(), String> {
        for selection in &mut project.selection_state.selections {
            selection.bounds.x += dx;
            selection.bounds.y += dy;
        }
        
        Ok(())
    }
    
    /// Scale a selection by a specified factor
    pub fn scale_selection(
        project: &mut Project,
        scale_x: f32,
        scale_y: f32,
    ) -> Result<(), String> {
        for selection in &mut project.selection_state.selections {
            selection.bounds.width *= scale_x;
            selection.bounds.height *= scale_y;
            
            // Scale the mask data
            // In a real implementation, this would require resampling the mask
            // For now, we'll just recreate a simple mask
            selection.mask = vec![255; (selection.bounds.width as usize) * (selection.bounds.height as usize)];
        }
        
        Ok(())
    }
    
    /// Rotate a selection by a specified angle (in degrees)
    pub fn rotate_selection(
        project: &mut Project,
        angle: f32,
    ) -> Result<(), String> {
        // For now, we'll just log that a rotation was requested
        // In a real implementation, this would apply a rotation transformation to selections
        println!("Rotating selection by {} degrees", angle);
        Ok(())
    }
}