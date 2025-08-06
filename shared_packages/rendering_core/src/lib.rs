//! Core rendering abstraction for multiple backend support

pub mod texture;

use wry::application::window::Window;
use uuid::Uuid;

/// Settings for the renderer quality
#[derive(Clone, Debug)]
pub struct RenderSettings {
    pub resolution_scale: f32,
    pub max_anisotropy: u8,
    pub shadow_quality: u8,
    pub texture_filtering: TextureFiltering,
    pub anti_aliasing: AntiAliasingMode,
    // Quality parameters for tile rendering
    pub tile_rendering_enabled: bool,
    pub tile_size: u32,
    pub quality_preset: QualityPreset,
}

/// Texture filtering options
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextureFiltering {
    Nearest,
    Linear,
    Anisotropic,
}

/// Anti-aliasing modes
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AntiAliasingMode {
    None,
    MSAAx2,
    MSAAx4,
    MSAAx8,
}

/// Quality presets
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QualityPreset {
    Low,
    Medium,
    High,
    Ultra,
}

/// Handle to a texture (backend-specific implementation)
pub trait TextureHandle: Send + Sync {
    /// Convert texture to RGBA format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rendering_core::TextureHandle;
    ///
    /// // Assuming you have a texture handle
    /// // let texture: Box<dyn TextureHandle> = create_texture();
    /// // let rgba_data = texture.to_rgba().expect("Failed to convert to RGBA");
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// This method should:
    /// - Validate texture dimensions (non-zero width/height)
    /// - Create a framebuffer (for OpenGL) or staging buffer (for Vulkan) to read texture data
    /// - Read pixel data from the GPU
    /// - Return RGBA data as a Vec<u8> in row-major order
    /// - Properly clean up any temporary resources
    fn to_rgba(&self) -> Result<Vec<u8>, texture::RenderError>;
    
    /// Create texture from RGBA data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use rendering_core::TextureHandle;
    ///
    /// // Create a simple red 2x2 texture
    /// let rgba_data = vec![
    ///     255, 0, 0, 255,  // Red pixel
    ///     255, 0, 0, 255,  // Red pixel
    ///     255, 0, 0, 255,  // Red pixel
    ///     255, 0, 0, 255,  // Red pixel
    /// ];
    ///
    /// // For backend-specific implementations, use their constructors instead:
    /// // OpenGL: OpenGLTextureHandle::new_from_rgba(gl_context, &rgba_data, 2, 2)
    /// // Vulkan: VulkanTextureHandle::new_from_rgba(device, &rgba_data, 2, 2)
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// This method requires backend-specific context:
    /// - OpenGL: Requires an active OpenGL context
    /// - Vulkan: Requires a Vulkan device
    ///
    /// Backend implementations should provide their own constructors that accept
    /// the required context parameters.
    ///
    /// # Compile-time Backend Guards
    ///
    /// To ensure proper backend compilation, use feature flags:
    ///
    /// ```rust
    /// #[cfg(feature = "opengl")]
    /// use opengl_renderer::OpenGLTextureHandle;
    ///
    /// #[cfg(feature = "vulkan")]
    /// use vulkan_renderer::VulkanTextureHandle;
    /// ```
    fn from_rgba(data: &[u8], width: u32, height: u32) -> Result<Self, texture::RenderError> where Self: Sized;
}

/// Main renderer trait
pub trait Renderer {
    /// Initialize renderer with window context
    fn init(&mut self, window: &Window);
    
    /// Begin frame rendering
    fn begin_frame(&mut self);
    
    /// Render a layer
    fn render_layer(&mut self, layer: &Layer, texture: &dyn TextureHandle);
    
    /// Render a tile
    fn render_tile(&mut self, tile: &Tile, texture: &dyn TextureHandle);
    
    /// End frame rendering
    fn end_frame(&mut self);
    
    /// Handle window resize
    fn resize(&mut self, width: u32, height: u32);
    
    /// Apply quality settings
    fn apply_quality_settings(&mut self, settings: &RenderSettings);
    
    /// Get active textures from the renderer
    fn get_active_textures(&self) -> Vec<&dyn TextureHandle> {
        // Default implementation returns empty vector
        Vec::new()
    }
}

/// Layer structure (to be defined in art app)
#[derive(Debug)]
pub struct Layer;

/// Tile structure (to be defined in art app)
#[derive(Debug)]
pub struct Tile {
    pub layer_id: uuid::Uuid,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub dirty: bool,
}

impl Tile {
    /// Create a new tile
    pub fn new(layer_id: uuid::Uuid, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            layer_id,
            x,
            y,
            width,
            height,
            dirty: true, // New tiles are dirty by default
        }
    }
    
    /// Mark the tile as dirty
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }
    
    /// Clear the dirty flag
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}