use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Shared recognition result structure matching Android implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    pub items: Vec<RecognitionItem>,
    pub processing_time_ms: u64,
    pub image_width: u32,
    pub image_height: u32,
}

/// Individual recognition item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionItem {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: Option<BoundingBox>,
}

/// Bounding box for object detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

/// Model configuration for vision tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionModelConfig {
    pub model_name: String,
    pub model_path: String,
    pub input_size: (u32, u32),
    pub confidence_threshold: f32,
    pub labels: Vec<String>,
    pub model_type: ModelType,
}

/// Types of vision models
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ModelType {
    Classification,
    ObjectDetection,
    FeatureExtraction,
}

/// Vision processing options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionOptions {
    pub confidence_threshold: Option<f32>,
    pub max_results: Option<usize>,
    pub model_name: Option<String>,
}

impl Default for VisionOptions {
    fn default() -> Self {
        Self {
            confidence_threshold: Some(0.1),
            max_results: Some(5),
            model_name: None,
        }
    }
}

/// Conversion from cpc-core vision Detection to shared RecognitionItem
impl From<crate::vision::Detection> for RecognitionItem {
    fn from(detection: crate::vision::Detection) -> Self {
        Self {
            label: detection.label,
            confidence: detection.confidence,
            bounding_box: Some(BoundingBox {
                left: detection.bbox.x,
                top: detection.bbox.y,
                right: detection.bbox.x + detection.bbox.width,
                bottom: detection.bbox.y + detection.bbox.height,
            }),
        }
    }
}

/// Conversion from cpc-core vision RecognitionResult to shared RecognitionResult
impl From<crate::vision::RecognitionResult> for RecognitionResult {
    fn from(result: crate::vision::RecognitionResult) -> Self {
        let items = result.detections.into_iter()
            .map(RecognitionItem::from)
            .collect();
        
        Self {
            items,
            processing_time_ms: result.processing_time_ms,
            image_width: 0, // Will be set by caller
            image_height: 0, // Will be set by caller
        }
    }
}

/// MobileNet model configuration
pub fn mobilenet_config() -> VisionModelConfig {
    VisionModelConfig {
        model_name: "mobilenet_v2".to_string(),
        model_path: "mobilenet_v2.onnx".to_string(),
        input_size: (224, 224),
        confidence_threshold: 0.1,
        labels: vec![
            "person", "bicycle", "car", "motorcycle", "airplane", "bus", "train", "truck",
            "boat", "traffic light", "fire hydrant", "stop sign", "parking meter", "bench",
            "bird", "cat", "dog", "horse", "sheep", "cow", "elephant", "bear", "zebra",
            "giraffe", "backpack", "umbrella", "handbag", "tie", "suitcase", "frisbee",
            "skis", "snowboard", "sports ball", "kite", "baseball bat", "baseball glove",
            "skateboard", "surfboard", "tennis racket", "bottle", "wine glass", "cup",
            "fork", "knife", "spoon", "bowl", "banana", "apple", "sandwich", "orange",
            "broccoli", "carrot", "hot dog", "pizza", "donut", "cake", "chair", "couch",
            "potted plant", "bed", "dining table", "toilet", "tv", "laptop", "mouse",
            "remote", "keyboard", "cell phone", "microwave", "oven", "toaster", "sink",
            "refrigerator", "book", "clock", "vase", "scissors", "teddy bear", "hair drier",
            "toothbrush"
        ],
        model_type: ModelType::Classification,
    }
}

/// Performance metrics for vision processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionMetrics {
    pub inference_time_ms: u64,
    pub preprocessing_time_ms: u64,
    pub postprocessing_time_ms: u64,
    pub total_time_ms: u64,
    pub memory_usage_mb: Option<f64>,
}

impl VisionMetrics {
    pub fn new() -> Self {
        Self {
            inference_time_ms: 0,
            preprocessing_time_ms: 0,
            postprocessing_time_ms: 0,
            total_time_ms: 0,
            memory_usage_mb: None,
        }
    }
}

/// Vision capabilities information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionCapabilities {
    pub supports_classification: bool,
    pub supports_object_detection: bool,
    pub supports_feature_extraction: bool,
    pub supported_formats: Vec<String>,
    pub max_image_size: (u32, u32),
    pub hardware_acceleration: bool,
}

impl Default for VisionCapabilities {
    fn default() -> Self {
        Self {
            supports_classification: true,
            supports_object_detection: false,
            supports_feature_extraction: false,
            supported_formats: vec![
                "JPEG".to_string(),
                "PNG".to_string(),
                "BMP".to_string(),
                "RGB".to_string(),
                "RGBA".to_string(),
            ],
            max_image_size: (4096, 4096),
            hardware_acceleration: false,
        }
    }
}