use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub brand: Option<String>,
    pub sku: Option<String>,
    pub barcode: String,
    pub createdAt: String,
    pub updatedAt: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BarcodeErrorCode {
    CameraError,
    CameraPermissionDenied,
    CameraNotAvailable,
    NetworkError,
    NotFound,
    DecodingError,
    PermissionDenied,
    InvalidBarcodeFormat,
    ScanTimeout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarcodeError {
    pub code: BarcodeErrorCode,
    pub message: String,
}

impl fmt::Display for BarcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}