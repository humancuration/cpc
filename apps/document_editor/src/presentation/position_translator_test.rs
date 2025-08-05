//! Unit tests for PositionTranslator

use super::position_translator::{PositionTranslator, SharedPositionTranslator, LineMetrics, CharMetrics, FontId, Rect};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_document_to_screen_conversion() {
        let mut translator = PositionTranslator::new();
        
        // Test basic conversion with default metrics
        let (x, y) = translator.document_to_screen(0, 0);
        assert_eq!(x, 0.0);
        assert_eq!(y, 0.0);
        
        let (x, y) = translator.document_to_screen(1, 5);
        assert_eq!(x, 40.0); // 5 * 8.0 (default char width)
        assert_eq!(y, 20.0); // 1 * 20.0 (default line height)
    }
    
    #[test]
    fn test_document_to_screen_with_scroll_offset() {
        let mut translator = PositionTranslator::new();
        translator.set_scroll_offset(10.0, 5.0);
        
        let (x, y) = translator.document_to_screen(1, 5);
        assert_eq!(x, 30.0); // (5 * 8.0) - 10.0
        assert_eq!(y, 15.0); // (1 * 20.0) - 5.0
    }
    
    #[test]
    fn test_screen_to_doc_conversion() {
        let mut translator = PositionTranslator::new();
        
        // Test basic conversion with default metrics
        let (line, col) = translator.screen_to_doc(40.0, 20.0);
        assert_eq!(line, 1);
        assert_eq!(col, 5);
    }
    
    #[test]
    fn test_screen_to_doc_with_scroll_offset() {
        let mut translator = PositionTranslator::new();
        translator.set_scroll_offset(10.0, 5.0);
        
        let (line, col) = translator.screen_to_doc(30.0, 15.0);
        assert_eq!(line, 1);
        assert_eq!(col, 5);
    }
    
    #[test]
    fn test_caching_behavior() {
        let mut translator = PositionTranslator::new();
        
        // First call should calculate and cache
        let (x1, y1) = translator.document_to_screen(2, 10);
        assert_eq!(x1, 80.0); // 10 * 8.0
        assert_eq!(y1, 40.0); // 2 * 20.0
        
        // Second call should use cache
        let (x2, y2) = translator.document_to_screen(2, 10);
        assert_eq!(x1, x2);
        assert_eq!(y1, y2);
    }
    
    #[test]
    fn test_cache_invalidation_on_scroll() {
        let mut translator = PositionTranslator::new();
        
        // First call should calculate and cache
        let (x1, y1) = translator.document_to_screen(2, 10);
        
        // Small scroll change shouldn't invalidate cache
        translator.set_scroll_offset(5.0, 2.0);
        let (x2, y2) = translator.document_to_screen(2, 10);
        assert_eq!(x1 - 5.0, x2);
        assert_eq!(y1 - 2.0, y2);
        
        // Large scroll change should invalidate cache
        translator.set_scroll_offset(100.0, 50.0);
        let (x3, y3) = translator.document_to_screen(2, 10);
        assert_eq!(x3, -20.0); // (10 * 8.0) - 100.0
        assert_eq!(y3, -10.0); // (2 * 20.0) - 50.0
    }
    
    #[test]
    fn test_shared_position_translator() {
        let shared_translator = SharedPositionTranslator::new();
        
        // Test basic conversion
        let (x, y) = shared_translator.document_to_screen(1, 5);
        assert_eq!(x, 40.0);
        assert_eq!(y, 20.0);
        
        // Test updating scroll offset
        shared_translator.set_scroll_offset(10.0, 5.0);
        let (x, y) = shared_translator.document_to_screen(1, 5);
        assert_eq!(x, 30.0);
        assert_eq!(y, 15.0);
    }
    
    #[test]
    fn test_viewport_functionality() {
        let mut translator = PositionTranslator::new();
        translator.set_viewport_size(1024.0, 768.0);
        translator.set_scroll_offset(50.0, 25.0);
        
        let viewport = translator.viewport();
        assert_eq!(viewport.x, 50.0);
        assert_eq!(viewport.y, 25.0);
        assert_eq!(viewport.width, 1024.0);
        assert_eq!(viewport.height, 768.0);
    }
    
    #[test]
    fn test_rect_contains() {
        let rect = Rect::new(10.0, 20.0, 100.0, 50.0);
        
        // Test points inside the rectangle
        assert!(rect.contains(15.0, 25.0));
        assert!(rect.contains(10.0, 20.0)); // Corner
        assert!(rect.contains(110.0, 70.0)); // Corner
        
        // Test points outside the rectangle
        assert!(!rect.contains(5.0, 25.0));
        assert!(!rect.contains(15.0, 15.0));
        assert!(!rect.contains(115.0, 25.0));
        assert!(!rect.contains(15.0, 75.0));
    }
}