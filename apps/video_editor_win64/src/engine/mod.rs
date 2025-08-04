/*
 Screaming architecture: engine — composition ECS, render systems, effect trait.
*/
pub mod systems;
pub mod composition;
pub mod effects;
pub mod test_utils;
pub mod wasm_bindings; // new: ffmpeg.wasm interface (gated internally)
pub const GPU_INTERPOLATION_WGSL: &str = include_str!("gpu_interpolation.wgsl");

/// Add tracing spans to hot paths for performance profiling
pub mod tracing_spans {
    use tracing::instrument;

    #[instrument(name = "engine.composition.render", skip_all, fields(time_ms = time_ms))]
    pub fn composition_render(time_ms: u64) {
        // Placeholder for actual composition rendering logic
    }

    #[instrument(name = "engine.frame_cache.acquire", skip_all)]
    pub fn frame_cache_acquire() {
        // Placeholder for frame cache acquisition logic
    }

    #[instrument(name = "engine.lru_ops", skip_all)]
    pub fn lru_operations() {
        // Placeholder for LRU operations logic
    }

    #[instrument(name = "engine.timeline.get_value", skip_all, fields(time_ms = time_ms))]
    pub fn timeline_get_value(time_ms: u64) {
        // Placeholder for timeline value retrieval logic
    }

    #[instrument(name = "engine.transition.process", skip_all)]
    pub fn transition_process() {
        // Placeholder for transition processing logic
    }
}

#[cfg(feature = "wasm")]
pub use wasm_bindings::*;

use std::sync::Arc;
use tracing::{info, instrument, trace};

// --- Core Engine facade (kept lightweight for now) ---
pub struct Engine {
    started: bool,
}

impl Engine {
    pub fn new() -> Self {
        info!("Engine initialized (bevy stub)");
        Self { started: false }
    }

    pub fn start(&mut self) {
        self.started = true;
        info!("Engine started (stub)");
    }

    #[instrument(name = "engine.render_frame", skip_all)]
    pub fn render_frame(&mut self) {
        if !self.started {
            self.start();
        }
        // In future: drive Bevy schedules here.
        trace!("render_frame tick");
    }

    /// Stub preview: produce a simple animated RGBA checkerboard at 640x360.
    /// Returns (pixels, width, height)
    #[instrument(name = "preview.request_frame", skip_all, fields(time_ms = time_ms))]
    pub fn preview_frame_at(&mut self, time_ms: f32) -> (Arc<[u8]>, u32, u32) {
        let w: u32 = 640;
        let h: u32 = 360;
        let mut buf = vec![0u8; (w * h * 4) as usize];
        let t = (time_ms / 1000.0) as f32;
        for y in 0..h {
            for x in 0..w {
                let idx = ((y * w + x) * 4) as usize;
                let xf = x as f32 / w as f32;
                let yf = y as f32 / h as f32;
                let r = (((xf + t).sin() * 0.5 + 0.5) * 255.0) as u8;
                let g = (((yf + t * 0.7).cos() * 0.5 + 0.5) * 255.0) as u8;
                let checker = (((x / 16) + (y / 16)) % 2) as u8;
                let b = if checker == 0 { 32 } else { 200 };
                buf[idx] = r;
                buf[idx + 1] = g;
                buf[idx + 2] = b;
                buf[idx + 3] = 255;
            }
        }
        (Arc::from(buf.into_boxed_slice()), w, h)
    }
}