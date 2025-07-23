//! Vision utilities and helper functions

use std::path::Path;
use crate::vision::{Model, ModelType};

/// Create a default YOLO model configuration
pub fn create_yolo_model(model_path: impl AsRef<Path>) -> Model {
    Model {
        model_type: ModelType::ObjectDetection,
        path: model_path.as_ref().to_path_buf(),
        input_size: (640, 640),
        confidence_threshold: 0.5,
        labels: vec![
            "person".to_string(), "bicycle".to_string(), "car".to_string(),
            "motorcycle".to_string(), "airplane".to_string(), "bus".to_string(),
            "train".to_string(), "truck".to_string(), "boat".to_string(),
            "traffic light".to_string(), "fire hydrant".to_string(), "stop sign".to_string(),
            "parking meter".to_string(), "bench".to_string(), "bird".to_string(),
            "cat".to_string(), "dog".to_string(), "horse".to_string(),
            "sheep".to_string(), "cow".to_string(), "elephant".to_string(),
            "bear".to_string(), "zebra".to_string(), "giraffe".to_string(),
            "backpack".to_string(), "umbrella".to_string(), "handbag".to_string(),
            "tie".to_string(), "suitcase".to_string(), "frisbee".to_string(),
            "skis".to_string(), "snowboard".to_string(), "sports ball".to_string(),
            "kite".to_string(), "baseball bat".to_string(), "baseball glove".to_string(),
            "skateboard".to_string(), "surfboard".to_string(), "tennis racket".to_string(),
            "bottle".to_string(), "wine glass".to_string(), "cup".to_string(),
            "fork".to_string(), "knife".to_string(), "spoon".to_string(),
            "bowl".to_string(), "banana".to_string(), "apple".to_string(),
            "sandwich".to_string(), "orange".to_string(), "broccoli".to_string(),
            "carrot".to_string(), "hot dog".to_string(), "pizza".to_string(),
            "donut".to_string(), "cake".to_string(), "chair".to_string(),
            "couch".to_string(), "potted plant".to_string(), "bed".to_string(),
            "dining table".to_string(), "toilet".to_string(), "tv".to_string(),
            "laptop".to_string(), "mouse".to_string(), "remote".to_string(),
            "keyboard".to_string(), "cell phone".to_string(), "microwave".to_string(),
            "oven".to_string(), "toaster".to_string(), "sink".to_string(),
            "refrigerator".to_string(), "book".to_string(), "clock".to_string(),
            "vase".to_string(), "scissors".to_string(), "teddy bear".to_string(),
            "hair drier".to_string(), "toothbrush".to_string(),
        ],
    }
}

/// Create a simple classification model configuration
pub fn create_classification_model(model_path: impl AsRef<Path>, labels: Vec<String>) -> Model {
    Model {
        model_type: ModelType::Classification,
        path: model_path.as_ref().to_path_buf(),
        input_size: (224, 224),
        confidence_threshold: 0.7,
        labels,
    }
}