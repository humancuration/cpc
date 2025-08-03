//! Data structures for the CPC platform
//!
//! This module provides common data structures that can be used across applications.

pub mod lru_cache;
pub mod ring_buffer;

// Re-export the main data structures for convenience
pub use lru_cache::LruCache;
pub use ring_buffer::RingBuffer;