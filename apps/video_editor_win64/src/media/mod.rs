use tracing::{info, trace};
use lru::LruCache;
use std::num::NonZeroUsize;

#[derive(Clone, Debug)]
pub struct Frame {
    pub pts_ms: u64,
    pub width: u32,
    pub height: u32,
}

impl Frame {
    #[inline]
    pub fn estimate_memory(&self) -> usize {
        // Assume RGBA 8-bit for preview estimation
        (self.width as usize) * (self.height as usize) * 4
    }
}

#[derive(Clone, Debug)]
pub struct AudioPacket {
    pub pts_ms: u64,
    pub samples: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
}

#[derive(Clone, Debug)]
pub struct FrameCacheConfig {
    pub max_bytes: usize,
}

impl Default for FrameCacheConfig {
    fn default() -> Self {
        Self { max_bytes: 2 * 1024 * 1024 * 1024 }
    }
}

pub struct FrameCache {
    inner: LruCache<u64, Frame>,
    current_mem: usize,
    max_mem: usize,
}

impl FrameCache {
    pub fn new(cfg: FrameCacheConfig) -> Self {
        // Capacity is a hint for entry count; eviction will be by bytes.
        // Use at least 128 entries to avoid tiny caches on low limits.
        let cap = (cfg.max_bytes / 1_000_000).max(128);
        Self {
            inner: LruCache::new(NonZeroUsize::new(cap).unwrap()),
            current_mem: 0,
            max_mem: cfg.max_bytes,
        }
    }

    pub fn get(&mut self, pts_ms: u64) -> Option<&Frame> {
        self.inner.get(&pts_ms)
    }

    pub fn put(&mut self, f: Frame) {
        let frame_size = f.estimate_memory();
        // Evict until we have room
        while self.current_mem + frame_size > self.max_mem {
            if let Some((_k, evicted)) = self.inner.pop_lru() {
                let sz = evicted.estimate_memory();
                self.current_mem = self.current_mem.saturating_sub(sz);
                trace!("FrameCache evicted frame pts={} size={}B current_mem={}B", evicted.pts_ms, sz, self.current_mem);
            } else {
                break;
            }
        }
        self.current_mem += frame_size;
        trace!("FrameCache insert pts={} size={}B current_mem={}B", f.pts_ms, frame_size, self.current_mem);
        self.inner.put(f.pts_ms, f);
    }

    pub fn current_bytes(&self) -> usize { self.current_mem }
    pub fn max_bytes(&self) -> usize { self.max_mem }
}

// Proxy management API
#[derive(Clone, Debug)]
pub struct ProxyRequest {
    pub input_path: String,
    pub target_height: u32,
    pub bitrate_kbps: u32,
}

pub trait ProxyManager: Send + Sync {
    fn enqueue(&self, req: ProxyRequest);
}

pub struct MediaPipeline {
    cache: FrameCache,
}

impl MediaPipeline {
    pub fn new() -> Self {
        info!("Media pipeline initialized (stub)");
        Self {
            cache: FrameCache::new(FrameCacheConfig::default()),
        }
    }

    pub fn import_media(&mut self, _path: &str) {
        // Later: start background proxy generation
    }

    pub fn decode_frame(&mut self, _t_ms: u64) -> Option<&Frame> {
        None
    }

    pub fn cache(&mut self) -> &mut FrameCache {
        &mut self.cache
    }
}

// WASM-specific ffmpeg.wasm entry points (signatures only here)
#[cfg(feature = "wasm")]
pub mod wasm {
    use anyhow::Result;
    use tracing::instrument;

    pub fn init_ffmpeg() {
        // Load ffmpeg.wasm core, configure codecs (AV1/Opus) and workers
    }

    /// Transcode to a proxy WebM with AV1 video and Opus audio.
    /// Returns the encoded bytes on success.
    #[instrument(name = "media.wasm.transcode_to_proxy", skip_all, fields(input_len = input.len(), target_height = target_height, bitrate_kbps = bitrate_kbps))]
    pub async fn transcode_to_proxy(
        input: &[u8],
        target_height: u32,
        bitrate_kbps: u32
    ) -> Result<Vec<u8>, anyhow::Error> {
        // Placeholder async API; implemented in the web runner with wasm_bindgen and ffmpeg.wasm.
        let _ = (input, target_height, bitrate_kbps);
        Ok(Vec::new())
    }
}