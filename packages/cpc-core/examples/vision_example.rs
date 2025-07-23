//! Example demonstrating the vision recognition functionality
//!
//! This example shows how to use the ImageRecognizer to perform
//! object detection on an image.

use cpc_core::vision::{ImageRecognizer, Model, ModelType};
use image::DynamicImage;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CPC Vision Recognition Example");
    println!("==============================");
    
    // Example of creating a model configuration
    let model = Model {
        model_type: ModelType::ObjectDetection,
        path: PathBuf::from("models/yolov5s.onnx"),
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
    };
    
    println!("Model configuration:");
    println!("  Type: {:?}", model.model_type);
    println!("  Path: {:?}", model.path);
    println!("  Input size: {}x{}", model.input_size.0, model.input_size.1);
    println!("  Confidence threshold: {}", model.confidence_threshold);
    println!("  Number of labels: {}", model.labels.len());
    
    // Note: To actually run this example, you would need:
    // 1. An ONNX model file at the specified path
    // 2. An image to process
    // 3. Proper error handling for missing files
    
    println!("\nTo use this example:");
    println!("1. Place your ONNX model in the models/ directory");
    println!("2. Provide an image file to process");
    println!("3. Run: cargo run --example vision_example");
    
    Ok(())
}