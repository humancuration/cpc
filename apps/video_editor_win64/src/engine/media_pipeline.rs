// Media pipeline: FrameProvider and DecodedFrame
// This module defines the trait and types used by the engine to obtain decoded frames.

use std::sync::Arc;
use uuid::Uuid;

pub type ClipId = u64;

/// A decoded media frame supplied by the media pipeline.
/// Cpu buffers are RGBA8 interleaved.
/// Gpu textures are owned Arc references that can be cloned safely.
#[derive(Debug)]
pub enum DecodedFrame {
    /// CPU-side decoded frame in RGBA8 format
    Cpu { buffer: Vec<u8>, width: u32, height: u32 },
    /// GPU-side decoded frame as an owned texture reference
    Gpu { texture: Arc<wgpu::Texture> },
}

/// Trait for any frame provider implementation (e.g., proxy decoder, real decoder, generators).
/// Implementors should return frames as close to the requested time as possible.
pub trait FrameProvider: Send + Sync {
    /// Get a decoded frame for the given clip at the specified time (in seconds).
    /// Returns None if the frame is not available or cannot be decoded.
    fn get_frame(&self, clip_id: ClipId, time: f64) -> Option<DecodedFrame>;
}

/// A no-op provider that returns None for all requests.
pub struct NullFrameProvider;
impl FrameProvider for NullFrameProvider {
    fn get_frame(&self, _clip_id: ClipId, _time: f64) -> Option<DecodedFrame> {
        None
    }
}

/// A simple solid color CPU frame provider (useful for tests and scaffolding).
/// Produces RGBA8 frames of the given color and size irrespective of clip/time.
pub struct SolidColorFrameProvider {
    pub width: u32,
    pub height: u32,
    pub rgba: [u8; 4],
}

impl SolidColorFrameProvider {
    pub fn new(width: u32, height: u32, rgba: [u8; 4]) -> Self {
        Self { width, height, rgba }
    }
}

impl FrameProvider for SolidColorFrameProvider {
    fn get_frame(&self, _clip_id: ClipId, _time: f64) -> Option<DecodedFrame> {
        let px = self.rgba;
        let mut buffer = vec![0u8; (self.width * self.height * 4) as usize];
        for chunk in buffer.chunks_exact_mut(4) {
            chunk.copy_from_slice(&px);
        }
        Some(DecodedFrame::Cpu {
            buffer,
            width: self.width,
            height: self.height,
        })
    }
}

// WASM interface placeholder for ffmpeg.wasm render (AV1/Opus/WebM only).
// In desktop builds this will be stubbed; in wasm feature, call into JS/WASM.
pub struct Frame {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Render a composition frame at the given time via ffmpeg.wasm (stub).
/// Only AV1/Opus/WebM are supported. Unsupported codec paths should error in wasm builds.
pub fn render_frame(_composition_id: Uuid, _time_ms: u64) -> Frame {
    // TODO: Call ffmpeg.wasm in wasm target with AV1/WebM constraints.
    // For now, return a placeholder CPU frame (black).
    let width = 640;
    let height = 360;
    let buffer = vec![0u8; (width * height * 4) as usize];
    Frame { buffer, width, height }
}

/// Simple ring queue for frame request times to smooth scrubbing.
/// A host runner can push desired times; media backend can prefetch.
pub struct FrameQueue {
    times_ms: std::collections::VecDeque<u64>,
    capacity: usize,
}

impl FrameQueue {
    pub fn new(capacity: usize) -> Self {
        Self { times_ms: std::collections::VecDeque::with_capacity(capacity), capacity }
    }
    pub fn push(&mut self, t: u64) {
        if self.times_ms.len() == self.capacity {
            self.times_ms.pop_front();
        }
        self.times_ms.push_back(t);
    }
    pub fn drain(&mut self) -> impl Iterator<Item = u64> + '_ {
        self.times_ms.drain(..)
    }
    pub fn is_empty(&self) -> bool { self.times_ms.is_empty() }
    pub fn len(&self) -> usize { self.times_ms.len() }
}

// ------------ Frame Cache (LRU) ------------
use lru::LruCache;

/// Unique frame key for cache lookups
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FrameKey {
    pub composition_id: Uuid,
    pub time_ms: u64,
}

/// Cached frame payload
#[derive(Debug)]
pub struct CachedFrame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// LRU frame cache with a byte-size budget and metrics.
/// Used for timeline scrubbing previews to avoid re-decoding frames.
pub struct FrameCache {
   pub cache: LruCache<FrameKey, CachedFrame>,
   pub max_size_bytes: usize,
   pub current_size_bytes: usize,
   // Metrics
   pub hits: u64,
   pub misses: u64,
   pub evictions: u64,
}

impl FrameCache {
   /// Create a new frame cache with the specified maximum size in bytes.
   pub fn new(max_size_bytes: usize) -> Self {
       Self {
           cache: LruCache::unbounded(),
           max_size_bytes,
           current_size_bytes: 0,
           hits: 0,
           misses: 0,
           evictions: 0,
       }
   }

   /// Calculate the cache hit rate (hits / total accesses)
   pub fn hit_rate(&self) -> f32 {
       let total = self.hits + self.misses;
       if total == 0 { 0.0 } else { self.hits as f32 / total as f32 }
   }

   fn size_of(frame: &CachedFrame) -> usize {
       frame.pixels.len()
   }

   fn enforce_budget(&mut self) {
       while self.current_size_bytes > self.max_size_bytes {
           if let Some((_k, v)) = self.cache.pop_lru() {
               self.current_size_bytes = self.current_size_bytes.saturating_sub(Self::size_of(&v));
               self.evictions += 1;
           } else {
               break;
           }
       }
   }

   /// Retrieve a cached frame by key, updating hit/miss metrics
   pub fn get(&mut self, key: &FrameKey) -> Option<&CachedFrame> {
       let got = self.cache.get(key);
       if got.is_some() { self.hits += 1; } else { self.misses += 1; }
       got
   }

   /// Store a frame in the cache, evicting entries if necessary to stay within budget
   pub fn put(&mut self, key: FrameKey, frame: CachedFrame) {
       let sz = Self::size_of(&frame);
       if let Some(old) = self.cache.put(key, frame) {
           self.current_size_bytes = self.current_size_bytes.saturating_sub(Self::size_of(&old));
       }
       self.current_size_bytes = self.current_size_bytes.saturating_add(sz);
       self.enforce_budget();
   }
}