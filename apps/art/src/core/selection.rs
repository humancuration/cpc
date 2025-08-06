//! Selection system for the Art application
//!
//! This module provides functionality for managing selections in the art application,
//! including different selection types and operations.

use crate::core::models::{Project, Layer, Rect};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use rayon::prelude::*;

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
///
/// A SelectionArea defines a region of the canvas that has been selected by the user.
/// It contains information about the shape of the selection, its boundaries, and which
/// layer it applies to (if any).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionArea {
    /// Unique identifier for the selection
    pub id: Uuid,
    /// Type of selection (Rectangle, Lasso, MagicWand)
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
            id: Uuid::new_v4(),
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
            id: Uuid::new_v4(),
            selection_type: SelectionType::Lasso,
            bounds,
            mask,
            layer_id,
        }
    }
    
    /// Create a new magic wand selection using scanline flood fill algorithm
    ///
    /// This function implements two different selection modes:
    /// 1. Contiguous selection: Uses scanline flood fill to select connected pixels
    /// 2. Non-contiguous selection: Selects all pixels in the layer matching the criteria
    ///
    /// For contiguous selection, we use the scanline flood fill algorithm which is more
    /// efficient than a simple 4-way or 8-way flood fill. The algorithm works by:
    /// 1. Adding the starting point to a queue
    /// 2. While the queue is not empty:
    ///    a. Dequeue a point (x, y)
    ///    b. Find the leftmost and rightmost points on the same scanline that match
    ///    c. Mark all points between left and right as selected
    ///    d. Check pixels above and below the scanline for matches and enqueue them
    ///
    /// For non-contiguous selection, we check all pixels in the layer and select those
    /// that match the color criteria within the specified tolerance.
    pub fn new_magic_wand(
        layer: &Layer,
        start_x: u32,
        start_y: u32,
        tolerance: f32,
        contiguous: bool,
        layer_id: Option<Uuid>,
    ) -> Self {
        let bounds = layer.bounds.clone();
        let width = bounds.width as u32;
        let height = bounds.height as u32;
        
        // Validate start coordinates
        if start_x >= width || start_y >= height {
            // Return empty selection if start point is out of bounds
            let mask = vec![0; (width as usize) * (height as usize)];
            return Self {
                id: Uuid::new_v4(),
                selection_type: SelectionType::MagicWand,
                bounds,
                mask,
                layer_id,
            };
        }
        
        // Get the target color at the starting point
        let target_color = get_pixel_color(&layer.pixels, width, start_x, start_y);
        
        // Create mask and visited array
        // For visited tracking, we use a Vec<bool> instead of a HashSet for performance.
        // Vec<bool> provides O(1) access with no hashing overhead, and for dense
        // flood fill operations, we're likely to visit a large portion of pixels
        // anyway, making the memory trade-off favorable.
        let mut mask = vec![0u8; (width as usize) * (height as usize)];
        let mut visited = vec![false; (width as usize) * (height as usize)];
        
        if contiguous {
            // Scanline flood fill algorithm for contiguous selection
            let mut queue = VecDeque::new();
            queue.push_back((start_x, start_y));
            visited[(start_y as usize) * (width as usize) + (start_x as usize)] = true;
            
            while let Some((x, y)) = queue.pop_front() {
                // Find left and right bounds of the scanline
                let left_x = find_left_bound(&layer.pixels, width, x, y, &target_color, tolerance);
                let right_x = find_right_bound(&layer.pixels, width, x, y, &target_color, tolerance);
                
                // Process the scanline
                for current_x in left_x..=right_x {
                    let index = (y as usize) * (width as usize) + (current_x as usize);
                    mask[index] = 255;
                    visited[index] = true;
                    
                    // Check neighbors above and below
                    check_neighbor(&mut queue, &mut visited, &layer.pixels, width, height,
                                  current_x, y.wrapping_sub(1), &target_color, tolerance);
                    check_neighbor(&mut queue, &mut visited, &layer.pixels, width, height,
                                  current_x, y.wrapping_add(1), &target_color, tolerance);
                }
            }
        } else {
            // Non-contiguous selection - check all pixels in the layer
            // Use Rayon for parallel processing of large images
            // We use a threshold of 1,000,000 pixels (1000x1000) to decide when to use
            // parallel processing. Below this threshold, sequential processing is more
            // efficient due to the overhead of thread management.
            if width * height > 1000000 {  // Only use parallel processing for large images
                mask.par_chunks_mut(width as usize)
                    .enumerate()
                    .for_each(|(y, chunk)| {
                        for x in 0..chunk.len() {
                            let pixel_color = get_pixel_color(&layer.pixels, width, x as u32, y as u32);
                            if color_match(&target_color, &pixel_color, tolerance) {
                                chunk[x] = 255;
                            }
                        }
                    });
            } else {
                // For smaller images, use sequential processing
                for y in 0..height {
                    for x in 0..width {
                        let pixel_color = get_pixel_color(&layer.pixels, width, x, y);
                        if color_match(&target_color, &pixel_color, tolerance) {
                            let index = (y as usize) * (width as usize) + (x as usize);
                            mask[index] = 255;
                        }
                    }
                }
            }
        }
        
        Self {
            id: Uuid::new_v4(),
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

/// Get the color of a pixel at the specified coordinates
fn get_pixel_color(pixels: &[u8], width: u32, x: u32, y: u32) -> [u8; 4] {
    let index = ((y * width + x) * 4) as usize;
    if index + 3 < pixels.len() {
        [pixels[index], pixels[index + 1], pixels[index + 2], pixels[index + 3]]
    } else {
        [0, 0, 0, 0]
    }
}
/// Check if two colors match within the specified tolerance
///
/// We use Euclidean distance in normalized RGB space to determine color similarity.
/// This approach treats RGB colors as 3D points and calculates the straight-line
/// distance between them. The tolerance value represents the maximum allowed
/// distance for colors to be considered a match.
///
/// The RGB values are normalized to [0,1] range to ensure equal weighting of
/// all color channels. This is important because RGB values in [0,255] range
/// would give blue channel 16x more weight than red/green due to the square
/// in the distance calculation.
fn color_match(target: &[u8; 4], candidate: &[u8; 4], tolerance: f32) -> bool {
    // Convert to f32 in [0,1] range
    let t_r = target[0] as f32 / 255.0;
    let t_g = target[1] as f32 / 255.0;
    let t_b = target[2] as f32 / 255.0;
    
    let c_r = candidate[0] as f32 / 255.0;
    let c_g = candidate[1] as f32 / 255.0;
    let c_b = candidate[2] as f32 / 255.0;

    // Calculate Euclidean distance in normalized RGB space
    let distance = ((t_r - c_r).powi(2) + (t_g - c_g).powi(2) + (t_b - c_b).powi(2)).sqrt();
    
    distance <= tolerance
}

/// Find the left bound of a scanline
fn find_left_bound(pixels: &[u8], width: u32, x: u32, y: u32, target_color: &[u8; 4], tolerance: f32) -> u32 {
    let mut left_x = x;
    while left_x > 0 {
        let test_x = left_x - 1;
        let pixel_color = get_pixel_color(pixels, width, test_x, y);
        if !color_match(target_color, &pixel_color, tolerance) {
            break;
        }
        left_x = test_x;
    }
    left_x
}

/// Find the right bound of a scanline
fn find_right_bound(pixels: &[u8], width: u32, x: u32, y: u32, target_color: &[u8; 4], tolerance: f32) -> u32 {
    let width_u32 = width as u32;
    let mut right_x = x;
    while right_x < width_u32 - 1 {
        let test_x = right_x + 1;
        let pixel_color = get_pixel_color(pixels, width, test_x, y);
        if !color_match(target_color, &pixel_color, tolerance) {
            break;
        }
        right_x = test_x;
    }
    right_x
}

/// Check and enqueue a neighbor pixel if it matches criteria
fn check_neighbor(
    queue: &mut VecDeque<(u32, u32)>,
    visited: &mut [bool],
    pixels: &[u8],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    target_color: &[u8; 4],
    tolerance: f32,
) {
    // Check bounds
    if x >= width || y >= height {
        return;
    }
    
    let index = (y as usize) * (width as usize) + (x as usize);
    
    // Check if already visited
    if visited[index] {
        return;
    }
    
    // Check color match
    let pixel_color = get_pixel_color(pixels, width, x, y);
    if color_match(target_color, &pixel_color, tolerance) {
        queue.push_back((x, y));
        visited[index] = true;
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
        }
    }
    
    /// Create a magic wand selection
    pub fn create_magic_wand_selection(
        project: &mut Project,
        layer_id: Uuid,
        x: u32,
        y: u32,
        tolerance: f32,
        contiguous: bool,
    ) -> Result<(), String> {
        let layer = project.get_layer(layer_id)
            .ok_or("Layer not found")?;
        
        let selection = SelectionArea::new_magic_wand(layer, x, y, tolerance, contiguous, Some(layer_id));
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Layer, LayerType, BlendMode, Rect};
    use serde_json::Value;
    use std::time::Instant;
    
    #[test]
    fn test_color_match() {
        let target = [255, 0, 0, 255]; // Red
        let candidate1 = [255, 0, 0, 255]; // Red
        let candidate2 = [200, 0, 0, 255]; // Darker red
        let candidate3 = [0, 255, 0, 255]; // Green
        
        // Exact match should always pass
        assert!(color_match(&target, &candidate1, 0.0));
        
        // Similar colors with tolerance
        assert!(color_match(&target, &candidate2, 0.3));
        
        // Different colors without tolerance
        assert!(!color_match(&target, &candidate3, 0.1));
    }
    
    #[test]
    fn test_get_pixel_color() {
        // Create a simple 2x2 image with known pixel values
        let pixels = vec![
            255, 0, 0, 255,   // Red pixel at (0,0)
            0, 255, 0, 255,   // Green pixel at (1,0)
            0, 0, 255, 255,   // Blue pixel at (0,1)
            255, 255, 0, 255, // Yellow pixel at (1,1)
        ];
        
        let color00 = get_pixel_color(&pixels, 2, 0, 0);
        assert_eq!(color00, [255, 0, 0, 255]);
        
        let color10 = get_pixel_color(&pixels, 2, 1, 0);
        assert_eq!(color10, [0, 255, 0, 255]);
        
        let color01 = get_pixel_color(&pixels, 2, 0, 1);
        assert_eq!(color01, [0, 0, 255, 255]);
        
        let color11 = get_pixel_color(&pixels, 2, 1, 1);
        assert_eq!(color11, [255, 255, 0, 255]);
    }
    
    #[test]
    fn test_find_bounds() {
        // Create a simple 3x3 image with a horizontal red line in the middle
        let pixels = vec![
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,  // Black row
            255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255,  // Red row
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,  // Black row
        ];
        
        let target_color = [255, 0, 0, 255]; // Red
        
        // Test finding bounds of the red line at the middle row
        let left_bound = find_left_bound(&pixels, 3, 1, 1, &target_color, 0.0);
        assert_eq!(left_bound, 0);
        
        let right_bound = find_right_bound(&pixels, 3, 1, 1, &target_color, 0.0);
        assert_eq!(right_bound, 2);
    }
    
    #[test]
    fn test_new_magic_wand_contiguous() {
        // Create a simple 3x3 layer with a red square in the middle
        let mut pixels = vec![0; 3 * 3 * 4]; // 3x3 RGBA
        
        // Set the center pixel to red
        let center_index = (1 * 3 + 1) * 4;
        pixels[center_index] = 255;     // R
        pixels[center_index + 1] = 0;  // G
        pixels[center_index + 2] = 0;  // B
        pixels[center_index + 3] = 255; // A
        
        let layer = Layer {
            id: Uuid::new_v4(),
            name: "Test Layer".to_string(),
            kind: LayerType::Raster,
            opacity: 1.0,
            visible: true,
            blend_mode: BlendMode::Normal,
            metadata: Value::Null,
            pixels,
            bounds: Rect { x: 0.0, y: 0.0, width: 3.0, height: 3.0 },
            effects: Vec::new(),
        };
        
        // Create a contiguous selection starting from the red pixel
        let selection = SelectionArea::new_magic_wand(&layer, 1, 1, 0.1, true, None);
        
        // Only the center pixel should be selected
        assert_eq!(selection.mask[4], 255); // Center pixel
        assert_eq!(selection.mask.iter().filter(|&&x| x == 255).count(), 1);
    }
    
    #[test]
    fn test_new_magic_wand_non_contiguous() {
        // Create a simple 3x3 layer with red pixels at (0,0) and (2,2)
        let mut pixels = vec![0; 3 * 3 * 4]; // 3x3 RGBA
        
        // Set pixel at (0,0) to red
        let index00 = (0 * 3 + 0) * 4;
        pixels[index00] = 255;     // R
        pixels[index00 + 1] = 0;  // G
        pixels[index00 + 2] = 0;  // B
        pixels[index00 + 3] = 255; // A
        
        // Set pixel at (2,2) to red
        let index22 = (2 * 3 + 2) * 4;
        pixels[index22] = 255;     // R
        pixels[index22 + 1] = 0;  // G
        pixels[index22 + 2] = 0;  // B
        pixels[index22 + 3] = 255; // A
        
        let layer = Layer {
            id: Uuid::new_v4(),
            name: "Test Layer".to_string(),
            kind: LayerType::Raster,
            opacity: 1.0,
            visible: true,
            blend_mode: BlendMode::Normal,
            metadata: Value::Null,
            pixels,
            bounds: Rect { x: 0.0, y: 0.0, width: 3.0, height: 3.0 },
            effects: Vec::new(),
        };
        
        // Create a non-contiguous selection starting from one red pixel
        let selection = SelectionArea::new_magic_wand(&layer, 0, 0, 0.1, false, None);
        
        // Both red pixels should be selected
        assert_eq!(selection.mask[0], 255); // Pixel (0,0)
        assert_eq!(selection.mask[8], 255); // Pixel (2,2)
        assert_eq!(selection.mask.iter().filter(|&&x| x == 255).count(), 2);
    }
    
    mod benchmarks {
        use super::*;
        use std::time::Instant;
        
        #[test]
        fn benchmark_contiguous_selection() {
            // Create a larger test image
            let width = 1000;
            let height = 1000;
            let mut pixels = vec![0; width * height * 4];
            
            // Fill with a uniform color
            for i in 0..pixels.len() / 4 {
                pixels[i * 4] = 255;     // R
                pixels[i * 4 + 1] = 0;   // G
                pixels[i * 4 + 2] = 0;   // B
                pixels[i * 4 + 3] = 255; // A
            }
            
            let layer = Layer {
                id: Uuid::new_v4(),
                name: "Benchmark Layer".to_string(),
                kind: LayerType::Raster,
                opacity: 1.0,
                visible: true,
                blend_mode: BlendMode::Normal,
                metadata: Value::Null,
                pixels,
                bounds: Rect { x: 0.0, y: 0.0, width: width as f32, height: height as f32 },
                effects: Vec::new(),
            };
            
            let start = Instant::now();
            let _selection = SelectionArea::new_magic_wand(&layer, 0, 0, 0.0, true, None);
            let duration = start.elapsed();
            
            println!("Contiguous selection benchmark: {:?}", duration);
            assert!(duration.as_millis() < 1000); // Should complete within 1 second
        }
        
        #[test]
        fn benchmark_non_contiguous_selection() {
            // Create a larger test image
            let width = 500;
            let height = 500;
            let mut pixels = vec![0; width * height * 4];
            
            // Fill with a uniform color
            for i in 0..pixels.len() / 4 {
                pixels[i * 4] = 255;     // R
                pixels[i * 4 + 1] = 0;   // G
                pixels[i * 4 + 2] = 0;   // B
                pixels[i * 4 + 3] = 255; // A
            }
            
            let layer = Layer {
                id: Uuid::new_v4(),
                name: "Benchmark Layer".to_string(),
                kind: LayerType::Raster,
                opacity: 1.0,
                visible: true,
                blend_mode: BlendMode::Normal,
                metadata: Value::Null,
                pixels,
                bounds: Rect { x: 0.0, y: 0.0, width: width as f32, height: height as f32 },
                effects: Vec::new(),
            };
            
            let start = Instant::now();
            let _selection = SelectionArea::new_magic_wand(&layer, 0, 0, 0.0, false, None);
            let duration = start.elapsed();
            
            println!("Non-contiguous selection benchmark: {:?}", duration);
            assert!(duration.as_millis() < 2000); // Should complete within 2 seconds
        }
        
        #[test]
        fn benchmark_worst_case_selection() {
            // Create a large test image with alternating colors to simulate worst case
            let width = 500;
            let height = 500;
            let mut pixels = vec![0; width * height * 4];
            
            // Fill with alternating red and blue pixels
            for y in 0..height {
                for x in 0..width {
                    let i = ((y * width + x) * 4) as usize;
                    if (x + y) % 2 == 0 {
                        pixels[i] = 255;     // R
                        pixels[i + 1] = 0;   // G
                        pixels[i + 2] = 0;   // B
                    } else {
                        pixels[i] = 0;       // R
                        pixels[i + 1] = 0;   // G
                        pixels[i + 2] = 255; // B
                    }
                    pixels[i + 3] = 255; // A
                }
            }
            
            let layer = Layer {
                id: Uuid::new_v4(),
                name: "Worst Case Benchmark Layer".to_string(),
                kind: LayerType::Raster,
                opacity: 1.0,
                visible: true,
                blend_mode: BlendMode::Normal,
                metadata: Value::Null,
                pixels,
                bounds: Rect { x: 0.0, y: 0.0, width: width as f32, height: height as f32 },
                effects: Vec::new(),
            };
            
            let start = Instant::now();
            let _selection = SelectionArea::new_magic_wand(&layer, 0, 0, 0.2, true, None);
            let duration = start.elapsed();
            
            println!("Worst case contiguous selection benchmark: {:?}", duration);
            assert!(duration.as_millis() < 5000); // Should complete within 5 seconds
        }
    }
}