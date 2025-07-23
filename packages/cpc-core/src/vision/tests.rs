#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgb};

    #[test]
    fn test_rect_creation() {
        let rect = Rect {
            x: 10.0,
            y: 20.0,
            width: 100.0,
            height: 200.0,
        };
        
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.width, 100.0);
        assert_eq!(rect.height, 200.0);
    }

    #[test]
    fn test_recognition_result_creation() {
        let result = RecognitionResult {
            label: "test".to_string(),
            confidence: 0.95,
            bounding_box: Some(Rect {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 200.0,
            }),
        };
        
        assert_eq!(result.label, "test");
        assert_eq!(result.confidence, 0.95);
        assert!(result.bounding_box.is_some());
    }

    #[test]
    fn test_image_preprocessing() {
        // Create a simple test image
        let mut img = DynamicImage::new_rgb8(800, 600);
        
        // Fill with test color
        for y in 0..600 {
            for x in 0..800 {
                img.put_pixel(x, y, Rgb([128, 128, 128]));
            }
        }
        
        // Note: This test would require a real model to test preprocessing
        // For now, we just verify the image creation works
        assert_eq!(img.width(), 800);
        assert_eq!(img.height(), 600);
    }

    #[test]
    fn test_serialization() {
        let result = RecognitionResult {
            label: "test".to_string(),
            confidence: 0.95,
            bounding_box: Some(Rect {
                x: 10.0,
                y: 20.0,
                width: 100.0,
                height: 200.0,
            }),
        };
        
        // Test JSON serialization
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("0.95"));
        
        // Test deserialization
        let deserialized: RecognitionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.label, "test");
        assert_eq!(deserialized.confidence, 0.95);
    }
}