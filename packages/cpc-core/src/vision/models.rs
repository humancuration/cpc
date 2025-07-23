use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Types of machine learning models for image recognition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelType {
    ObjectDetection,
    Classification,
    FeatureExtraction,
    TextRecognition,
}

/// Metadata for a machine learning model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub model_type: ModelType,
    pub path: PathBuf,
    pub input_size: (u32, u32),
    pub confidence_threshold: f32,
    pub labels: Vec<String>,
}

/// Result of image recognition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    pub detections: Vec<Detection>,
    pub processing_time_ms: u64,
}

/// Individual detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub label: String,
    pub confidence: f32,
    pub bbox: BoundingBox,
}

/// Bounding box coordinates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Error types for vision operations
#[derive(Debug, thiserror::Error)]
pub enum RecognitionError {
    #[error("Model loading failed: {0}")]
    ModelLoadError(String),
    
    #[error("Image preprocessing failed: {0}")]
    PreprocessingError(String),
    
    #[error("Inference failed: {0}")]
    InferenceError(String),
    
    #[error("Postprocessing failed: {0}")]
    PostprocessingError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl From<tract::TractError> for RecognitionError {
    fn from(err: tract::TractError) -> Self {
        RecognitionError::InferenceError(err.to_string())
    }
}

impl From<image::ImageError> for RecognitionError {
    fn from(err: image::ImageError) -> Self {
        RecognitionError::PreprocessingError(err.to_string())
    }
}