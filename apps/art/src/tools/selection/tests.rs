// DEPRECATED - This file is no longer used. See test.rs instead.
//! Tests for selection tools algorithms

use super::lasso::LassoSelectionTool;

#[test]
fn test_point_simplification() {
    let tool = LassoSelectionTool::new();
    
    // Test with few points - should return as is
    let points = vec![(0.0, 0.0), (1.0, 1.0)];
    let simplified = tool.simplify_points();
    assert_eq!(simplified, points);
    
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
    assert!((distance - (1.0f32.powi(2) + 1.0f32.powi(2)).sqrt()).abs() < 0.001);
}