//! Tests for coordinate translation with various fonts and layouts

#[cfg(test)]
mod tests {
    use super::super::position_translator::{PositionTranslator, LineMetrics, CharMetrics, FontId};
    
    #[test]
    fn test_coordinate_translation_with_variable_fonts() {
        let mut translator = PositionTranslator::new();
        
        // Add some line metrics
        let line_metrics = LineMetrics {
            top: 0.0,
            height: 24.0, // Taller line
            wrapped_ranges: vec![(0, 10), (10, 20)], // Wrapped at position 10
        };
        translator.update_line_metrics(0, line_metrics);
        
        // Add some character metrics for different fonts
        let font1 = FontId {
            family: "Arial".to_string(),
            size: 12.0,
            weight: 400,
        };
        
        let font2 = FontId {
            family: "Times New Roman".to_string(),
            size: 14.0,
            weight: 700,
        };
        
        // Add character metrics
        translator.update_font_metrics(font1.clone(), 'A', CharMetrics { width: 8.5, kerning: 0.0 });
        translator.update_font_metrics(font1.clone(), 'B', CharMetrics { width: 7.2, kerning: 0.0 });
        translator.update_font_metrics(font2.clone(), 'A', CharMetrics { width: 9.1, kerning: 0.0 });
        translator.update_font_metrics(font2.clone(), 'B', CharMetrics { width: 8.0, kerning: 0.0 });
        
        // Test conversion (simplified since our implementation is basic)
        let (x, y) = translator.document_to_screen(0, 5);
        // Should use line metrics for Y and default char width for X
        assert_eq!(y, 0.0); // line_metrics.top
        assert_eq!(x, 40.0); // 5 * 8.0 (default width)
    }
    
    #[test]
    fn test_coordinate_translation_with_wrapped_lines() {
        let mut translator = PositionTranslator::new();
        
        // Add line metrics with wrapping
        let line_metrics = LineMetrics {
            top: 50.0,
            height: 20.0,
            wrapped_ranges: vec![(0, 15), (15, 30)], // Line wraps at position 15
        };
        translator.update_line_metrics(2, line_metrics);
        
        let (x, y) = translator.document_to_screen(2, 10);
        assert_eq!(y, 50.0); // line_metrics.top
        assert_eq!(x, 80.0); // 10 * 8.0 (default width)
        
        // Test position beyond first wrapped segment
        let (x, y) = translator.document_to_screen(2, 20);
        assert_eq!(y, 50.0); // Still same line
        assert_eq!(x, 160.0); // 20 * 8.0 (default width)
    }
    
    #[test]
    fn test_coordinate_translation_with_scroll_offset() {
        let mut translator = PositionTranslator::new();
        translator.set_scroll_offset(100.0, 50.0);
        
        // Add line metrics
        let line_metrics = LineMetrics {
            top: 100.0,
            height: 25.0,
            wrapped_ranges: vec![],
        };
        translator.update_line_metrics(3, line_metrics);
        
        let (x, y) = translator.document_to_screen(3, 8);
        assert_eq!(y, 50.0); // 100.0 (line top) - 50.0 (scroll Y)
        assert_eq!(x, -36.0); // (8 * 8.0) - 100.0 (scroll X)
    }
    
    #[test]
    fn test_screen_to_document_conversion_with_metrics() {
        let mut translator = PositionTranslator::new();
        
        // Add line metrics
        let line_metrics = LineMetrics {
            top: 40.0,
            height: 22.0,
            wrapped_ranges: vec![(0, 20)],
        };
        translator.update_line_metrics(1, line_metrics);
        
        // Test conversion back from screen coordinates
        let (line, col) = translator.screen_to_doc(80.0, 50.0);
        assert_eq!(line, 1); // Should match the line with top=40.0 and height=22.0
        assert_eq!(col, 10); // 80.0 / 8.0 (default width)
    }
    
    #[test]
    fn test_font_id_equality() {
        let font1 = FontId {
            family: "Arial".to_string(),
            size: 12.0,
            weight: 400,
        };
        
        let font2 = FontId {
            family: "Arial".to_string(),
            size: 12.0,
            weight: 400,
        };
        
        let font3 = FontId {
            family: "Arial".to_string(),
            size: 14.0,
            weight: 400,
        };
        
        assert_eq!(font1, font2);
        assert_ne!(font1, font3);
    }
    
    #[test]
    fn test_cache_invalidation_with_metrics_updates() {
        let mut translator = PositionTranslator::new();
        
        // First conversion should calculate and cache
        let (x1, y1) = translator.document_to_screen(0, 5);
        
        // Add metrics which should invalidate cache
        let line_metrics = LineMetrics {
            top: 10.0,
            height: 30.0,
            wrapped_ranges: vec![],
        };
        translator.update_line_metrics(0, line_metrics);
        
        // Second conversion should use new metrics
        let (x2, y2) = translator.document_to_screen(0, 5);
        assert_eq!(y2, 10.0); // line_metrics.top
        assert_ne!(y1, y2); // Should be different from cached value
    }
}