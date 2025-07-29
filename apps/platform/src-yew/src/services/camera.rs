pps/cpc-platform/src-yew/src/services/camera.rs</path>
<content lines="1-90">
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use crate::types::product::{BarcodeError, BarcodeErrorCode};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "camera"])]
    async fn requestCameraPermission() -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "camera"])]
    async fn checkCameraAvailability() -> JsValue;
}

pub enum CameraPermission {
    Granted,
    Denied,
    Prompt,
}

pub struct CameraService;

impl CameraService {
    pub async fn request_permission() -> Result<CameraPermission, BarcodeError> {
        let result = requestCameraPermission().await;
        let permission_str = result.as_string().unwrap_or_else(|| "prompt".to_string());
        
        match permission_str.as_str() {
            "granted" => Ok(CameraPermission::Granted),
            "denied" => Ok(CameraPermission::Denied),
            _ => Ok(CameraPermission::Prompt),
        }
    }

    pub async fn check_availability() -> Result<bool, BarcodeError> {
        let result = checkCameraAvailability().await;
        match result.as_bool() {
            Some(available) => Ok(available),
            None => Err(BarcodeError {
                code: BarcodeErrorCode::CameraNotAvailable,
                message: "Failed to check camera availability".to_string(),
            }),
        }
    }
    
    pub async fn ensure_permission() -> Result<(), BarcodeError> {
        match Self::request_permission().await? {
            CameraPermission::Granted => Ok(()),
            CameraPermission::Denied => Err(BarcodeError {
                code: BarcodeErrorCode::CameraPermissionDenied,
                message: "Camera permission denied".to_string(),
            }),
            CameraPermission::Prompt => {
                // In a real app, we would trigger the permission prompt here
                // For now, we'll treat prompt as denied to be safe
                Err(BarcodeError {
                    code: BarcodeErrorCode::CameraPermissionDenied,
                    message: "Camera permission not granted".to_string(),
                })
            }
        }
    }
}