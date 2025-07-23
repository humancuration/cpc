# Vision Recognition Setup for CPC Desktop Client

## Overview
The CPC Desktop Client now includes vision recognition functionality using ONNX models. This enables image classification and object detection capabilities.

## Setup Instructions

### 1. Model Files
Place ONNX model files in the following locations:
- `apps/pds/src-tauri/models/` - Development models
- App data directory: `%APPDATA%/CPC/models/` (Windows) or `~/Library/Application Support/CPC/models/` (macOS)

### 2. Label Files
For each model file (e.g., `model.onnx`), create a corresponding label file `model.txt` with one label per line.

### 3. Testing the Setup

#### Available Tauri Commands:
- `start_camera_capture()` - Start camera capture
- `stop_camera_capture()` - Stop camera capture
- `get_latest_frame()` - Get latest camera frame as JPEG
- `recognize_image(path)` - Recognize objects in image file
- `process_camera_image(data)` - Process camera image data
- `get_available_models()` - List available models

#### Example Usage:
```javascript
// Start camera
await invoke('start_camera_capture');

// Get frame
const frame = await invoke('get_latest_frame');
if (frame) {
    const results = await invoke('process_camera_image', { imageData: frame });
    console.log('Recognition results:', results);
}

// Stop camera
await invoke('stop_camera_capture');
```

### 4. Model Requirements
- ONNX format (`.onnx` extension)
- Input: 224x224 RGB image (for classification)
- Output: Classification scores or detection boxes

### 5. Default Labels
The system includes default ImageNet labels for basic testing. For custom models, provide your own label files.

## Troubleshooting

### Common Issues:
1. **Model not found**: Check model file location and permissions
2. **Camera not working**: Ensure camera permissions are granted
3. **Recognition failing**: Verify model compatibility with ONNX Runtime

### Debug Steps:
1. Check console for error messages
2. Verify model files exist in expected locations
3. Test with sample images first
4. Check camera permissions in system settings

## Development
For development, place model files in `apps/pds/src-tauri/models/` directory.