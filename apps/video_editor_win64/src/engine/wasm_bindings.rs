#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use uuid::Uuid;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmFrame {
    #[wasm_bindgen(getter_with_clone)]
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "wasm")]
impl From<crate::engine::media_pipeline::Frame> for WasmFrame {
    fn from(f: crate::engine::media_pipeline::Frame) -> Self {
        WasmFrame { buffer: f.buffer, width: f.width, height: f.height }
    }
}

/// Validate codec/container constraints.
/// Only WebM container with AV1 video and Opus audio are allowed.
#[cfg(feature = "wasm")]
fn validate_codec(container: &str, video_codec: &str, audio_codec: &str) -> Result<(), JsError> {
    let container_ok = matches!(container.to_ascii_lowercase().as_str(), "webm");
    let video_ok = matches!(video_codec.to_ascii_lowercase().as_str(), "av1" | "av01");
    let audio_ok = matches!(audio_codec.to_ascii_lowercase().as_str(), "opus");
    if container_ok && video_ok && audio_ok {
        Ok(())
    } else {
        Err(JsError::new("Unsupported codec/container. Only WebM container with AV1 video and Opus audio are allowed."))
    }
}

/// Decode AV1 bitstream using ffmpeg.wasm (placeholder).
/// In a real build, this calls into JS glue that wraps ffmpeg.wasm.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn decode_av1(_data: &[u8]) -> Result<WasmFrame, JsError> {
    // TODO: Actual ffmpeg.wasm call; return 1x1 transparent pixel for now.
    let frame = crate::engine::media_pipeline::Frame { buffer: vec![0,0,0,0], width: 1, height: 1 };
    Ok(frame.into())
}

/// Render composition via ffmpeg.wasm (stub).
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn render_composition_av1(comp_id_hi: u64, comp_id_lo: u64, time_ms: u64) -> Result<WasmFrame, JsError> {
    let bytes_hi = comp_id_hi.to_be_bytes();
    let bytes_lo = comp_id_lo.to_be_bytes();
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&bytes_hi);
    bytes[8..].copy_from_slice(&bytes_lo);
    let comp_id = Uuid::from_bytes(bytes);
    let f = crate::engine::media_pipeline::render_frame(comp_id, time_ms);
    Ok(f.into())
}