use std::path::Path;
use std::time::Instant;
use image::{DynamicImage, ImageBuffer, Rgb};
use tract::prelude::*;
use crate::vision::{Model, ModelType, RecognitionResult, Detection, BoundingBox, RecognitionError};

/// Main image recognition struct
pub struct ImageRecognizer {
    model: Model,
    tract_model: RunnableModel<TypedFact, Box<dyn TypedOp>>,
}

impl ImageRecognizer {
    /// Create a new ImageRecognizer with the specified model
    pub fn new(model: Model) -> Result<Self, RecognitionError> {
        let model_path = model.path.clone();
        let tract_model = tract_onnx::onnx()
            .model_for_path(model_path)?
            .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 3, model.input_size.1 as usize, model.input_size.0 as usize]))?
            .into_optimized()?
            .into_runnable()?;
        
        Ok(Self { model, tract_model })
    }

    /// Recognize objects in an image
    pub fn recognize(&self, image: &DynamicImage) -> Result<RecognitionResult, RecognitionError> {
        let start_time = Instant::now();
        
        // Preprocess image
        let tensor = self.preprocess_image(image)?;
        
        // Run inference
        let results = self.tract_model.run(tvec![tensor])?;
        
        // Postprocess results
        let detections = self.postprocess_results(&results)?;
        
        let processing_time = start_time.elapsed().as_millis() as u64;
        
        Ok(RecognitionResult {
            detections,
            processing_time_ms: processing_time,
        })
    }

    /// Preprocess image for model input
    fn preprocess_image(&self, image: &DynamicImage) -> Result<Tensor, RecognitionError> {
        let (width, height) = self.model.input_size;
        
        // Resize image to model input size
        let resized = image.resize_exact(width, height, image::imageops::FilterType::Nearest);
        
        // Convert to RGB and normalize to [0, 1]
        let mut data = Vec::with_capacity((width * height * 3) as usize);
        
        for pixel in resized.pixels() {
            let rgb = pixel.0;
            data.push(rgb[0] as f32 / 255.0);
            data.push(rgb[1] as f32 / 255.0);
            data.push(rgb[2] as f32 / 255.0);
        }
        
        Tensor::from_shape(&[1, 3, height as usize, width as usize], &data)
            .map_err(|e| RecognitionError::PreprocessingError(e.to_string()))
    }

    /// Postprocess model outputs into detection results
    fn postprocess_results(&self, outputs: &[TValue]) -> Result<Vec<Detection>, RecognitionError> {
        let mut detections = Vec::new();
        
        match self.model.model_type {
            ModelType::ObjectDetection => {
                // Handle object detection outputs
                // Format: [batch, num_boxes, 4 + num_classes]
                if let Some(output) = outputs.get(0) {
                    let data = output.to_array_view::<f32>()
                        .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                    
                    // Process detection boxes
                    for box_idx in 0..data.dim().get(1).unwrap_or(0) {
                        let mut max_confidence = 0.0;
                        let mut best_class = 0;
                        
                        // Find class with highest confidence
                        for class_idx in 4..data.dim().get(2).unwrap_or(0) {
                            let confidence = data.get([0, box_idx, class_idx])
                                .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                            
                            if *confidence > max_confidence {
                                max_confidence = *confidence;
                                best_class = class_idx - 4;
                            }
                        }
                        
                        if max_confidence > self.model.confidence_threshold {
                            let x = data.get([0, box_idx, 0])
                                .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                            let y = data.get([0, box_idx, 1])
                                .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                            let w = data.get([0, box_idx, 2])
                                .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                            let h = data.get([0, box_idx, 3])
                                .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                            
                            let label = self.model.labels
                                .get(best_class)
                                .unwrap_or(&"unknown".to_string())
                                .clone();
                            
                            detections.push(Detection {
                                label,
                                confidence: max_confidence,
                                bbox: BoundingBox {
                                    x: *x,
                                    y: *y,
                                    width: *w,
                                    height: *h,
                                },
                            });
                        }
                    }
                }
            }
            ModelType::Classification => {
                // Handle classification outputs
                // Format: [batch, num_classes]
                if let Some(output) = outputs.get(0) {
                    let data = output.to_array_view::<f32>()
                        .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                    
                    let mut max_confidence = 0.0;
                    let mut best_class = 0;
                    
                    for class_idx in 0..data.dim().get(1).unwrap_or(0) {
                        let confidence = data.get([0, class_idx])
                            .map_err(|e| RecognitionError::PostprocessingError(e.to_string()))?;
                        
                        if *confidence > max_confidence {
                            max_confidence = *confidence;
                            best_class = class_idx;
                        }
                    }
                    
                    if max_confidence > self.model.confidence_threshold {
                        let label = self.model.labels
                            .get(best_class)
                            .unwrap_or(&"unknown".to_string())
                            .clone();
                        
                        detections.push(Detection {
                            label,
                            confidence: max_confidence,
                            bbox: BoundingBox {
                                x: 0.0,
                                y: 0.0,
                                width: 1.0,
                                height: 1.0,
                            },
                        });
                    }
                }
            }
            _ => {
                return Err(RecognitionError::PostprocessingError(
                    "Unsupported model type".to_string()
                ));
            }
        }
        
        Ok(detections)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;
    use std::path::PathBuf;

    #[test]
    fn test_preprocessing() {
        let model = Model {
            model_type: ModelType::Classification,
            path: PathBuf::from("test.onnx"),
            input_size: (224, 224),
            confidence_threshold: 0.5,
            labels: vec!["cat".to_string(), "dog".to_string()],
        };
        
        // Note: This test would need a real ONNX model file
        // For now, just test preprocessing without loading model
        let img = RgbImage::new(100, 100);
        let dynamic_img = DynamicImage::ImageRgb8(img);
        
        // This would fail without a real model, but shows the structure
        // let recognizer = ImageRecognizer::new(model).unwrap();
        // let tensor = recognizer.preprocess_image(&dynamic_img).unwrap();
        // assert_eq!(tensor.shape(), &[1, 3, 224, 224]);
    }
}