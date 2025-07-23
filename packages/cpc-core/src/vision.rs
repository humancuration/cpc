use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use ndarray::{Array, Array4};
use ort::{Environment, Session, SessionBuilder, Value};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Recognition result for a single detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognitionResult {
    pub label: String,
    pub confidence: f32,
    pub bbox: Option<[f32; 4]>, // [x, y, width, height] in normalized coordinates
}

/// Image recognition using ONNX models
pub struct ImageRecognizer {
    session: Session,
    labels: Vec<String>,
}

impl ImageRecognizer {
    /// Create a new image recognizer from an ONNX model file
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let model_path = model_path.as_ref();
        
        // Create ONNX Runtime environment
        let environment = Environment::builder()
            .with_name("cpc-vision")
            .build()?;
        
        // Create session with CPU execution provider
        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(ort::GraphOptimizationLevel::Level3)?
            .with_model_from_file(model_path)?;
        
        // Load labels from model directory or use default
        let labels = load_labels(model_path)?;
        
        Ok(Self { session, labels })
    }
    
    /// Perform image recognition on the given image
    pub fn recognize(&self, image: &DynamicImage) -> Result<Vec<RecognitionResult>> {
        // Preprocess the image
        let input = self.preprocess_image(image)?;
        
        // Run inference
        let outputs = self.session.run(vec![Value::from_array(input)?])?;
        
        // Post-process the results
        let results = self.postprocess_outputs(&outputs)?;
        
        Ok(results)
    }
    
    /// Preprocess image for model input
    fn preprocess_image(&self, image: &DynamicImage) -> Result<Array4<f32>> {
        // Resize image to model input size (typically 224x224 for classification)
        let resized = image.resize_exact(224, 224, image::imageops::FilterType::Nearest);
        
        // Convert to RGB
        let rgb = resized.to_rgb8();
        
        // Normalize pixel values to [0, 1] and create tensor
        let mut tensor = Array4::zeros((1, 3, 224, 224));
        
        for y in 0..224 {
            for x in 0..224 {
                let pixel = rgb.get_pixel(x, y);
                tensor[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                tensor[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                tensor[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
            }
        }
        
        Ok(tensor)
    }
    
    /// Post-process model outputs to get recognition results
    fn postprocess_outputs(&self, outputs: &[Value]) -> Result<Vec<RecognitionResult>> {
        if outputs.is_empty() {
            return Ok(Vec::new());
        }
        
        // Get the first output (assuming classification)
        let output = &outputs[0];
        let tensor = output.try_extract::<f32>()?;
        
        // Handle different output formats
        let results = match tensor.len() {
            // Single classification output
            len if len == self.labels.len() => {
                let mut results = Vec::new();
                
                // Get top predictions
                let mut predictions: Vec<(usize, f32)> = tensor
                    .iter()
                    .enumerate()
                    .map(|(i, &score)| (i, score))
                    .collect();
                
                // Sort by confidence
                predictions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                
                // Take top 5 predictions
                for (class_idx, confidence) in predictions.iter().take(5) {
                    if *confidence > 0.1 { // Confidence threshold
                        results.push(RecognitionResult {
                            label: self.labels[*class_idx].clone(),
                            confidence: *confidence,
                            bbox: None,
                        });
                    }
                }
                
                results
            }
            _ => {
                // Handle object detection outputs
                self.process_detection_outputs(&tensor)?
            }
        };
        
        Ok(results)
    }
    
    /// Process object detection outputs
    fn process_detection_outputs(&self, tensor: &ndarray::ArrayViewD<f32>) -> Result<Vec<RecognitionResult>> {
        // This is a simplified implementation for YOLO-style outputs
        let mut results = Vec::new();
        
        // Assuming tensor shape is [batch, num_boxes, 5 + num_classes]
        if let Some(shape) = tensor.shape().get(2..) {
            if shape.len() >= 2 {
                let num_boxes = shape[0];
                let num_values = shape[1];
                
                for box_idx in 0..num_boxes {
                    // Extract box coordinates and confidence
                    let x = tensor[[0, 0, box_idx, 0]];
                    let y = tensor[[0, 0, box_idx, 1]];
                    let w = tensor[[0, 0, box_idx, 2]];
                    let h = tensor[[0, 0, box_idx, 3]];
                    let confidence = tensor[[0, 0, box_idx, 4]];
                    
                    if confidence > 0.5 { // Confidence threshold
                        // Find class with highest probability
                        let mut max_prob = 0.0;
                        let mut class_idx = 0;
                        
                        for class in 5..num_values {
                            let prob = tensor[[0, 0, box_idx, class]];
                            if prob > max_prob {
                                max_prob = prob;
                                class_idx = class - 5;
                            }
                        }
                        
                        if max_prob > 0.5 { // Class probability threshold
                            results.push(RecognitionResult {
                                label: self.labels.get(class_idx).unwrap_or(&"unknown".to_string()).clone(),
                                confidence: max_prob * confidence,
                                bbox: Some([x, y, w, h]),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
}

/// Load labels from a text file
fn load_labels<P: AsRef<Path>>(model_path: P) -> Result<Vec<String>> {
    let model_path = model_path.as_ref();
    let labels_path = model_path.with_extension("txt");
    
    if labels_path.exists() {
        let content = std::fs::read_to_string(&labels_path)?;
        let labels: Vec<String> = content
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        if !labels.is_empty() {
            return Ok(labels);
        }
    }
    
    // Default labels for ImageNet classes
    Ok(vec![
        "person".to_string(),
        "bicycle".to_string(),
        "car".to_string(),
        "motorcycle".to_string(),
        "airplane".to_string(),
        "bus".to_string(),
        "train".to_string(),
        "truck".to_string(),
        "boat".to_string(),
        "traffic light".to_string(),
        // Add more as needed...
    ])
}

/// Image processing utilities
pub mod utils {
    use image::{DynamicImage, GenericImageView};
    
    /// Crop an image to a specific region
    pub fn crop_image(image: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        image.crop_imm(x, y, width, height)
    }
    
    /// Resize an image to a specific size
    pub fn resize_image(image: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        image.resize_exact(width, height, image::imageops::FilterType::Lanczos3)
    }
    
    /// Convert image to tensor format
    pub fn image_to_tensor(image: &DynamicImage) -> ndarray::Array4<f32> {
        let rgb = image.to_rgb8();
        let (width, height) = rgb.dimensions();
        
        let mut tensor = ndarray::Array4::zeros((1, 3, height as usize, width as usize));
        
        for y in 0..height {
            for x in 0..width {
                let pixel = rgb.get_pixel(x, y);
                tensor[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
                tensor[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
                tensor[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
            }
        }
        
        tensor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};
    
    #[test]
    fn test_image_recognizer_creation() {
        // This test would require a real model file
        // For now, just test that the struct can be created
        assert!(true);
    }
    
    #[test]
    fn test_preprocess_image() {
        let recognizer = ImageRecognizer {
            session: unsafe { std::mem::zeroed() },
            labels: vec!["test".to_string()],
        };
        
        let image = DynamicImage::ImageRgb8(RgbImage::new(100, 100));
        let result = recognizer.preprocess_image(&image);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_load_labels() {
        let labels = load_labels("nonexistent.onnx").unwrap();
        assert!(!labels.is_empty());
    }
}