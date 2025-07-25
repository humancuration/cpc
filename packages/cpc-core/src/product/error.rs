use thiserror::Error;

#[derive(Error, Debug)]
pub enum BarcodeError {
    #[error("Barcode not found or unreadable: {0}")]
    Unreadable(String),
    #[error("FFmpeg initialization failed: {0}")]
    FfmpegInit(String),
    #[error("Unsupported codec: {0}")]
    UnsupportedCodec(String),
    #[error("WASM runtime error: {0}")]
    WasmRuntime(String),
    #[error("Image processing error: {0}")]
    ImageProcessing(String),
    #[error("Unsupported barcode format: {0}")]
    UnsupportedFormat(String),
}