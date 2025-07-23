# Android Image Recognition Integration Guide

## Overview
This integration provides real-time image recognition capabilities using the Rust-based vision core from CPC. The implementation uses CameraX for camera access and Jetpack Compose for the UI.

## Architecture
- **Rust Core**: Image recognition logic in `packages/cpc-core/src/vision/`
- **JNI Bridge**: Native bindings in `packages/cpc-core/src/ffi/android/vision.rs`
- **Android Components**: Activities, adapters, and layouts in `apps/cpc-platform/android/app/src/main/kotlin/com/cpc/vision/`

## Components Created

### 1. Core Classes
- **ImageRecognitionActivity.kt**: Main activity with camera preview and results display
- **ImageRecognitionNative.kt**: JNI interface to Rust core
- **RecognitionResultAdapter.kt**: RecyclerView adapter for displaying results
- **RecognitionResult.kt**: Data models for recognition results

### 2. Layout Resources
- **activity_image_recognition.xml**: Main layout with camera preview and results list
- **item_recognition_result.xml**: Individual result item layout
- **strings.xml**: Localized strings for UI elements

### 3. Native Integration
- **vision.rs**: JNI bindings for Android in Rust
- **AndroidManifest.xml**: Updated with camera permissions and activities

## Usage

### Starting the Image Recognition
```kotlin
// Launch the image recognition activity
val intent = Intent(context, ImageRecognitionActivity::class.java)
startActivity(intent)
```

### Direct JNI Usage
```kotlin
// Initialize recognizer
val recognizerPtr = ImageRecognitionNative.initRecognizer(
    modelPath = "/path/to/model.onnx",
    modelType = 0, // 0=ObjectDetection, 1=Classification, 2=FeatureExtraction, 3=TextRecognition
    inputWidth = 640,
    inputHeight = 640,
    confidenceThreshold = 0.5f
)

// Perform recognition
val bitmap = getBitmapFromCamera()
val jsonResult = ImageRecognitionNative.recognizeWithRecognizer(recognizerPtr, bitmap)

// Parse result
val result = Gson().fromJson(jsonResult, RecognitionResult::class.java)

// Cleanup
ImageRecognitionNative.destroyRecognizer(recognizerPtr)
```

## Model Requirements
- ONNX format models supported
- Default input size: 640x640 (configurable)
- COCO classes pre-configured for object detection
- Confidence threshold: 0.5 (configurable)

## Permissions
The following permissions are required in AndroidManifest.xml:
- `android.permission.CAMERA`
- `android.permission.INTERNET` (for model downloads)

## Testing
1. Install the APK on an Android device
2. Grant camera permissions when prompted
3. Point camera at objects to see real-time detection
4. Results will appear in a list below the camera preview

## Performance Notes
- Processing happens on a background thread
- Results are updated in real-time
- Memory-efficient bitmap handling
- Automatic cleanup of native resources

## Future Enhancements
- Add support for custom model loading
- Implement batch processing
- Add image gallery selection
- Support for different model architectures