//! Serialization utilities for the CPC platform
//!
//! This module provides common serialization and deserialization functionality
//! for JSON and MessagePack formats.

pub mod json;
pub mod msgpack;

// Re-export the main serialization functions for convenience
pub use json::{to_json, from_json, to_json_vec};
pub use msgpack::{to_msgpack, from_msgpack, to_msgpack_vec};