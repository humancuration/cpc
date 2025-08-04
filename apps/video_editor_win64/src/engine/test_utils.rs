// Test utilities for media frame processing and effects validation.
//
// Note: These helpers avoid real GPU initialization. They generate CPU-side frames
// and provide lightweight verification utilities for transitions.

use std::sync::Arc;

use crate::engine::media_pipeline::{FrameProvider, DecodedFrame, ClipId};

/// Stress test helpers for CPU FrameCache (LRU) behavior.
/// Builds many frames to exercise eviction.
pub mod cache_stress {
    use super::*;
    use crate::engine::media_pipeline::{FrameCache, FrameKey, CachedFrame};
    use uuid::Uuid;

    /// Fill a frame cache with frames until it reaches the target size in bytes
    pub fn fill_cache_bytes(cache: &mut FrameCache, composition_id: Uuid, target_bytes: usize, frame_size: (u32,u32)) {
        let (w,h) = frame_size;
        let bytes_per = (w * h * 4) as usize;
        let mut t = 0u64;
        while cache.current_size_bytes < target_bytes {
            let key = FrameKey { composition_id, time_ms: t };
            let frame = CachedFrame {
                pixels: vec![0u8; bytes_per],
                width: w,
                height: h,
            };
            cache.put(key, frame);
            t += 33;
            if t > 60_000 { break; }
        }
    }
}
 /// Guard: ensure WGPU_BACKEND is set before running GPU-dependent tests.
 /// Returns Ok(()) when present; otherwise Err with the exact docs message.
 /// This is essential for cross-platform CI testing.
 pub fn requires_wgpu_backend() -> Result<(), String> {
     match std::env::var("WGPU_BACKEND") {
         Ok(val) if !val.trim().is_empty() => Ok(()),
         _ => Err("Environment variable WGPU_BACKEND not set. Set it to one of: vulkan, dx12, metal, gl.".to_string()),
     }
 }
 }
 
 /// Headless GPU device helper for CI/integration tests.
 pub async fn create_headless_device() -> (wgpu::Device, wgpu::Queue) {
     let instance = wgpu::Instance::default();
     let adapter = instance
         .request_adapter(&wgpu::RequestAdapterOptions {
             power_preference: wgpu::PowerPreference::LowPower,
             compatible_surface: None,
             force_fallback_adapter: false,
         })
         .await
         .expect("No suitable GPU adapter found for headless tests");
     adapter
         .request_device(&wgpu::DeviceDescriptor::default(), None)
         .await
         .expect("Failed to create wgpu Device for headless tests")
 }

/// A solid color frame generator usable in tests.
/// Produces RGBA8 frames of the given size and color.
pub struct TestSolidColorProvider {
    pub w: u32,
    pub h: u32,
    pub rgba: [u8; 4],
}

/// Mock frame generator requested by spec.
/// Generates a solid color CPU frame.
pub struct SolidFrameGenerator {
    pub width: u32,
    pub height: u32,
    pub color: [u8; 4],
}

impl SolidFrameGenerator {
    pub fn new(width: u32, height: u32, color: [u8; 4]) -> Self {
        Self { width, height, color }
    }
}

impl FrameProvider for SolidFrameGenerator {
    fn get_frame(&self, _clip_id: ClipId, _time: f64) -> Option<DecodedFrame> {
        let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
        for c in buffer.chunks_exact_mut(4) {
            c.copy_from_slice(&self.color);
        }
        Some(DecodedFrame::Cpu {
            buffer,
            width: self.width,
            height: self.height,
        })
    }
}

impl TestSolidColorProvider {
    pub fn new(w: u32, h: u32, rgba: [u8; 4]) -> Self {
        Self { w, h, rgba }
    }
}

impl FrameProvider for TestSolidColorProvider {
    fn get_frame(&self, _clip_id: ClipId, _time: f64) -> Option<DecodedFrame> {
        let mut buf = vec![0u8; (self.w * self.h * 4) as usize];
        for chunk in buf.chunks_exact_mut(4) {
            chunk.copy_from_slice(&self.rgba);
        }
        Some(DecodedFrame::Cpu {
            buffer: buf,
            width: self.w,
            height: self.h,
        })
    }
/// Compute the expected RGBA mix for a crossfade between two solid colors.
/// t is 0..1.
/// This function is used in transition verification tests.
pub fn expected_crossfade_rgba(a: [u8; 4], b: [u8; 4], t: f32) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| -> u8 {
        let xf = x as f32;
        let yf = y as f32;
        (xf + (yf - xf) * t).round().clamp(0.0, 255.0) as u8
    };
    [
        lerp(a[0], b[0]),
        lerp(a[1], b[1]),
        lerp(a[2], b[2]),
        lerp(a[3], b[3]),
    ]
}
    ]
}

/// Verify a transition between two solid colors would yield the expected mixed color at t.
/// Returns true if the expected color matches.
pub fn verify_crossfade_solid(a: [u8; 4], b: [u8; 4], t: f32, sample: [u8; 4]) -> bool {
    expected_crossfade_rgba(a, b, t) == sample
}

/// Lightweight texture allocation metrics for assertions in tests.
/// Not tied to wgpu; the engine FrameCache exposes real metrics, but this helps CI-free checks.
#[derive(Default, Clone, Copy, Debug)]
pub struct TextureMetrics {
    pub allocations: usize,
    pub releases: usize,
    pub current_count: usize,
}

#[cfg(test)]
/// Mock GPU context for unit tests that don't need a real wgpu device.
pub struct MockGpuContext;

#[cfg(test)]
impl MockGpuContext {
    pub fn new() -> Self {
        MockGpuContext
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solid_provider_produces_correct_size() {
        let p = TestSolidColorProvider::new(64, 32, [10, 20, 30, 255]);
        match p.get_frame(1, 0.0) {
            Some(DecodedFrame::Cpu { buffer, width, height }) => {
                assert_eq!(width, 64);
                assert_eq!(height, 32);
                assert_eq!(buffer.len(), (64 * 32 * 4) as usize);
                assert_eq!(&buffer[0..4], &[10,20,30,255]);
            }
            _ => panic!("expected CPU frame"),
        }
    }

    #[test]
    fn expected_crossfade_color() {
        let a = [0, 0, 0, 255];
        let b = [255, 128, 64, 255];
        let mid = expected_crossfade_rgba(a, b, 0.5);
        assert_eq!(mid, [128, 64, 32, 255]);
    }

    #[test]
    fn crossfade_verification_helper() {
        let a = [10, 10, 10, 200];
        let b = [110, 210, 250, 220];
        let t = 0.25;
        let expected = expected_crossfade_rgba(a, b, t);
        assert!(verify_crossfade_solid(a, b, t, expected));
    }
}