# Backend Documentation

## Overview
The CPC backend provides core APIs for...

## Table of Contents
1. [Configuration Guide](configuration.md)
2. [Security Architecture](security_architecture.md)
3. [API Reference](api.md)
4. [Deployment Guide](deployment.md)

## Getting Started
To run locally:
```bash
cargo run
## Camera Service

The `CameraService` handles camera permission management and availability checks.

### Methods

#### `request_permission()`
Requests camera permission from the user.

**Returns**:  
`Result<CameraPermission, BarcodeError>`

**Permission States**:
- `Granted`: Camera access allowed
- `Denied`: Camera access blocked
- `Prompt`: Permission request pending

#### `check_availability()`
Checks if a camera is available on the device.

**Returns**:  
`Result<bool, BarcodeError>`  
- `true`: Camera available
- `false`: No camera available

#### `ensure_permission()`
Ensures camera permission is granted before proceeding.

**Returns**:  
`Result<(), BarcodeError>`  
- Success if permission granted
- Error with `CameraPermissionDenied` if blocked

### Usage Example

```rust
use crate::services::camera::CameraService;
use crate::types::product::BarcodeError;

async fn init_camera() -> Result<(), BarcodeError> {
    CameraService::ensure_permission().await?;
    // Additional camera initialization logic
    Ok(())
}
```

### Error Handling

The service returns specific errors for common camera issues:
- `CameraPermissionDenied`: User denied camera access
- `CameraNotAvailable`: No camera hardware detected
- `CameraError`: Generic camera operation failure

### JS-Rust Boundary

Camera operations use a clean JS-Rust interface:
```rust
#[wasm_bindgen(js_namespace = ["window", "camera"])]
async fn requestCameraPermission() -> JsValue;
```
This integrates with our JavaScript camera implementation while maintaining Rust type safety.