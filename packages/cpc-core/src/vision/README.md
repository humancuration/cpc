# CPC Vision Module

This module provides core image recognition functionality using ONNX models through the `tract` library.

## Architecture

The vision module consists of:

1. **recognizer.rs** - Main `ImageRecognizer` struct for performing inference
2. **models.rs** - Data structures for models, results, and errors
3. **utils.rs** - Helper functions for creating common model configurations
4. **ffi/android/vision.rs** - Android JNI bindings for native usage

## Usage

### Basic Usage

```rust
use cpc_core::vision::{ImageRecognizer, Model, ModelType};

// Create model configuration
let model = Model {
    model_type: ModelType::ObjectDetection,
    path: "models/yolov5s.onnx".into(),
    input_size: (640, 640),
    confidence_threshold: 0.5,
    labels: vec!["person".to_string(), "car".to_string(), ...],
};

// Create recognizer
let recognizer = ImageRecognizer::new(model)?;

// Perform recognition
let result = recognizer.recognize(&image)?;
println!("Found {} detections in {}ms", 
    result.detections.len(), 
    result.processing_time_ms
);
```

### Android Integration

The module provides JNI bindings for Android:

```java
// Java/Kotlin usage
public class ImageRecognition {
    static {
        System.loadLibrary("cpc_core");
    }
    
    public native String nativeRecognize(Bitmap bitmap, String modelPath);
    public native long nativeInitRecognizer(String modelPath, int modelType, int width, int height, float confidence);
    public native String nativeRecognizeWithRecognizer(long recognizerPtr, Bitmap bitmap);
    public native void nativeDestroyRecognizer(long recognizerPtr);
}
```

## Model Support

- **Object Detection** - YOLO-style models (bounding boxes + classes)
- **Classification** - Image classification models
- **Feature Extraction** - Embedding generation (future)
- **Text Recognition** - OCR models (future)

## Dependencies

- `tract` - ONNX model inference
- `image` - Image processing and manipulation
- `serde` - Serialization for results
- `jni` - Android FFI bindings

## Example

Run the example:
```bash
cargo run --example vision_example
```

Note: Requires ONNX model files in the `models/` directory.