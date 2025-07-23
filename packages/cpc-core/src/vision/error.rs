//! Error types for image recognition

use thiserror::Error;

/// Error types for image recognition operations
#[derive(Error, Debug)]
pub enum VisionError {
    #[error("Failed to load model: {0}")]
    ModelLoadError(String),
    
    #[error("Failed to preprocess image: {0}")]
    PreprocessingError(String),
    
    #[error("Model inference failed: {0}")]
    InferenceError(String),
    
    #[error("Failed to postprocess results: {0}")]
    PostprocessingError(String),
    
    #[error("Invalid model format: {0}")]
    InvalidModelFormat(String),
    
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("JNI error: {0}")]
    JniError(#[from] jni::errors::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Tensor processing error: {0}")]
    TensorError(#[from] tract::tract_core::anyhow::Error),
}

/// Result type for vision operations
pub type VisionResult<T> = Result<T, VisionError>;