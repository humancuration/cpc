# Vision Models for CPC Desktop

This directory contains vision models for the CPC desktop application.

## Model Requirements

### MobileNet V2 (Classification)
- **File**: `mobilenet_v2.onnx`
- **Input**: 224x224 RGB images
- **Output**: 1000-class ImageNet classification
- **Download**: Available from ONNX Model Zoo

### Setup Instructions

1. **Download MobileNet V2**:
   ```bash
   # Option 1: From ONNX Model Zoo
   wget https://github.com/onnx/models/raw/main/vision/classification/mobilenet/model/mobilenetv2-7.onnx -O mobilenet_v2.onnx
   
   # Option 2: Using Python
   python -c "
   import torch
   import torchvision.models as models
   model = models.mobilenet_v2(pretrained=True)
   torch.onnx.export(model, torch.randn(1,3,224,224), 'mobilenet_v2.onnx')
   "
   ```

2. **Verify Model**:
   ```bash
   # Check model info
   python -c "
   import onnx
   model = onnx.load('mobilenet_v2.onnx')
   print(onnx.helper.printable_graph(model.graph))
   "
   ```

3. **Labels File**:
   - `imagenet_labels.txt` is already provided
   - Contains 1000 ImageNet class labels

## Usage

The vision system will automatically:
- Load models on demand
- Cache loaded models in memory
- Support multiple concurrent models
- Provide async processing

## API Endpoints

- `initialize_vision`: Initialize with default model
- `recognize_image`: Process single image file
- `process_camera_image`: Process camera frame
- `get_model_info`: Get available models
- `load_model`: Load specific model
- `unload_model`: Unload model from memory

## Performance Tips

- Models are cached in memory after first load
- Use `VisionOptions` to control confidence thresholds
- Batch processing available via `recognize_batch`
- CPU inference optimized for desktop use