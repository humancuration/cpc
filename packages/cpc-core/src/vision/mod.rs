//! Image recognition functionality
//!
//! This module provides cross-platform image recognition capabilities using ONNX models.
//! It supports both desktop and Android platforms with consistent APIs.

mod models;
mod recognizer;
mod error;

#[cfg(test)]
mod tests;

pub use models::*;
pub use recognizer::*;
pub use error::*;