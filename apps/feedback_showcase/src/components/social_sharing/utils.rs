//! Utility functions for social sharing components

use web_sys::HtmlElement;

/// Normalize DOM coordinates to values between 0 and 1
/// 
/// # Arguments
/// * `x` - X coordinate in pixels
/// * `y` - Y coordinate in pixels
/// * `container` - The HTML element to normalize against
/// 
/// # Returns
/// A tuple of normalized coordinates (x, y) where both values are between 0 and 1
pub fn normalize_coordinates(
    x: f32, 
    y: f32, 
    container: &HtmlElement
) -> (f32, f32) {
    let rect = container.get_bounding_client_rect();
    let norm_x = (x - rect.left() as f32) / rect.width() as f32;
    let norm_y = (y - rect.top() as f32) / rect.height() as f32;
    (norm_x, norm_y)
}