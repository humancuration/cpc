//! Lasso selection tool for the Art application
//!
//! This module implements the lasso selection tool which allows users
//! to create freeform selections on the canvas.

use crate::tools::trait::{Tool, ToolResult, CursorType};
use crate::core::models::{Project, Action};
use crate::core::selection::SelectionService;
use uuid::Uuid;

/// Lasso selection tool
///
/// This tool allows users to create freeform selections by clicking and dragging
/// to draw a polygonal outline. The selection is created when the user releases
/// the mouse button. Points are simplified using the Ramer-Douglas-Peucker algorithm
/// to reduce complexity while maintaining shape fidelity.
pub struct LassoSelectionTool {
    /// Whether we're currently making a selection
    is_selecting: bool,
    /// Points in the selection
    points: Vec<(f32, f32)>,
}

impl LassoSelectionTool {
    /// Simplify points using the Ramer-Douglas-Peucker algorithm
    pub fn simplify_points(&self) -> Vec<(f32, f32)> {
        if self.points.len() <= 2 {
            return self.points.clone();
        }
        
        let epsilon = 1.0; // Simplification threshold
        self.ramer_douglas_peucker(&self.points, epsilon)
    }
    
    /// Ramer-Douglas-Peucker algorithm for line simplification
    pub fn ramer_douglas_peucker(&self, points: &[(f32, f32)], epsilon: f32) -> Vec<(f32, f32)> {
        if points.len() < 3 {
            return points.to_vec();
        }
        
        let mut max_distance = 0.0;
        let mut index = 0;
        
        // Find the point with maximum distance
        for i in 1..(points.len() - 1) {
            let distance = self.point_to_line_distance(points[i], points[0], points[points.len() - 1]);
            if distance > max_distance {
                max_distance = distance;
                index = i;
            }
        }
        
        // If max distance is greater than epsilon, recursively simplify
        if max_distance > epsilon {
            let mut result1 = self.ramer_douglas_peucker(&points[0..=index], epsilon);
            let result2 = self.ramer_douglas_peucker(&points[index..], epsilon);
            
            // Remove duplicate point at the join
            result1.pop();
            result1.extend(result2);
            result1
        } else {
            vec![points[0], points[points.len() - 1]]
        }
    }
    
    /// Calculate the distance from a point to a line segment
    pub fn point_to_line_distance(&self, point: (f32, f32), line_start: (f32, f32), line_end: (f32, f32)) -> f32 {
        let (px, py) = point;
        let (x1, y1) = line_start;
        let (x2, y2) = line_end;
        
        let line_magnitude = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        if line_magnitude < f32::EPSILON {
            return ((px - x1).powi(2) + (py - y1).powi(2)).sqrt();
        }
        
        let u = ((px - x1) * (x2 - x1) + (py - y1) * (y2 - y1)) / line_magnitude.powi(2);
        let (ix, iy) = if u < 0.0 {
            (x1, y1)
        } else if u > 1.0 {
            (x2, y2)
        } else {
            (x1 + u * (x2 - x1), y1 + u * (y2 - y1))
        };
        
        ((px - ix).powi(2) + (py - iy).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_simplification() {
        let tool = LassoSelectionTool {
            is_selecting: false,
            points: vec![(0.0, 0.0), (1.0, 1.0)],
        };
        
        // Test with few points - should return as is
        let simplified = tool.simplify_points();
        assert_eq!(simplified, vec![(0.0, 0.0), (1.0, 1.0)]);
        
        // Test with collinear points - should simplify to endpoints
        let tool_with_points = LassoSelectionTool {
            is_selecting: false,
            points: vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0)],
        };
        let simplified = tool_with_points.simplify_points();
        assert_eq!(simplified.len(), 2);
        assert_eq!(simplified[0], (0.0, 0.0));
        assert_eq!(simplified[1], (2.0, 2.0));
    }

    #[test]
    fn test_point_to_line_distance() {
        let tool = LassoSelectionTool::new();
        
        // Distance from point to line segment
        let distance = tool.point_to_line_distance((1.0, 1.0), (0.0, 0.0), (2.0, 0.0));
        assert!((distance - 1.0).abs() < 0.001);
        
        // Distance from point to endpoint
        let distance = tool.point_to_line_distance((3.0, 1.0), (0.0, 0.0), (2.0, 0.0));
        let expected = (1.0f32.powi(2) + 1.0f32.powi(2)).sqrt();
        assert!((distance - expected).abs() < 0.001);
    }
}

impl LassoSelectionTool {
    /// Create a new lasso selection tool
    pub fn new() -> Self {
        Self {
            is_selecting: false,
            points: Vec::new(),
        }
    }
}

impl Tool for LassoSelectionTool {
    fn name(&self) -> &str {
        "Lasso Selection"
    }
    
    fn activate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn deactivate(&mut self) -> ToolResult {
        Ok(vec![])
    }
    
    fn handle_press(&mut self, _project: &mut Project, x: f32, y: f32) -> ToolResult {
        self.is_selecting = true;
        self.points.clear();
        self.points.push((x, y));
        Ok(vec![])
    }
    
    fn handle_drag(&mut self, _project: &mut Project, x: f32, y: f32) -> ToolResult {
        if self.is_selecting {
            self.points.push((x, y));
        }
        Ok(vec![])
    fn handle_release(&mut self, project: &mut Project, x: f32, y: f32) -> ToolResult {
        if self.is_selecting {
            self.points.push((x, y));
            self.is_selecting = false;
            
            // Simplify points to reduce complexity
            let simplified_points = self.simplify_points();
            
            // Create lasso selection using SelectionService
            SelectionService::create_lasso_selection(project, simplified_points.clone(), None)?;
            
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