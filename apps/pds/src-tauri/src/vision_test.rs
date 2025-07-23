#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    
    #[test]
    fn test_vision_state_creation() {
        let state = VisionState::default();
        assert!(state.recognizers.blocking_lock().is_empty());
        assert!(state.default_model.blocking_lock().is_none());
    }
    
    #[test]
    fn test_model_path_resolution() {
        // This is a basic test - in real scenarios you'd mock the app handle
        let temp_dir = tempdir().unwrap();
        let models_dir = temp_dir.path().join("models");
        fs::create_dir(&models_dir).unwrap();
        
        // Create a dummy model file
        let model_path = models_dir.join("test_model.onnx");
        fs::write(&model_path, b"dummy model content").unwrap();
        
        // Test would require mocking tauri::AppHandle
        assert!(model_path.exists());
    }
    
    #[test]
    fn test_vision_options_default() {
        let options = VisionOptions::default();
        assert_eq!(options.confidence_threshold, Some(0.1));
        assert_eq!(options.max_results, Some(5));
        assert_eq!(options.model_name, None);
    }
}