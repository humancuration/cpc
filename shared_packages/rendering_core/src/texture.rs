//! Texture conversion traits and descriptors for the rendering core

use crate::TextureHandle;
use std::fmt;

/// Error types for rendering operations
#[derive(Debug)]
pub enum RenderError {
    /// Invalid parameter provided
    InvalidParameter(String),
    /// Texture creation failed
    TextureCreationFailed(String),
    /// Texture conversion failed
    TextureConversionFailed(String),
    /// Memory allocation failed
    OutOfMemory(String),
    /// Other rendering error
    Other(String),
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            RenderError::TextureCreationFailed(msg) => write!(f, "Texture creation failed: {}", msg),
            RenderError::TextureConversionFailed(msg) => write!(f, "Texture conversion failed: {}", msg),
            RenderError::OutOfMemory(msg) => write!(f, "Out of memory: {}", msg),
            RenderError::Other(msg) => write!(f, "Rendering error: {}", msg),
        }
    }
}

impl std::error::Error for RenderError {}
//! Texture conversion traits and descriptors for the rendering core

use crate::TextureHandle;

/// Texture descriptor containing metadata about a texture
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub usage: TextureUsage,
}

/// Texture formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8Unorm,
    Rgba8UnormSrgb,
    Bgra8Unorm,
    Bgra8UnormSrgb,
}

/// Texture usage flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextureUsage {
    pub texture_binding: bool,
    pub copy_src: bool,
    pub copy_dst: bool,
}

impl Default for TextureUsage {
    fn default() -> Self {
        Self {
            texture_binding: true,
            copy_src: false,
            copy_dst: true,
        }
    }
}

/// Trait for converting between different texture representations
pub trait TextureConverter {
    /// Convert a texture from one backend to another
    fn convert_texture(
        &self,
        source: &dyn TextureHandle,
        target_format: TextureFormat,
    ) -> Box<dyn TextureHandle>;
}