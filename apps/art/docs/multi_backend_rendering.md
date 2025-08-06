# Multi-Backend Rendering Support

This document describes the multi-backend rendering system implemented for the Art application.

## Overview

The Art application now supports multiple rendering backends through an abstract interface. This allows the application to use different graphics APIs (Bevy, OpenGL, Vulkan) while maintaining a consistent interface.

## Architecture

### Core Components

1. **`rendering_core`** - Shared package containing the abstract `Renderer` trait and common types
2. **`opengl_renderer`** - Shared package implementing OpenGL rendering
3. **`vulkan_renderer`** - Shared package implementing Vulkan rendering
4. **`abstract_pipeline`** - Application-level module that delegates to backend implementations

### Renderer Trait

The core of the system is the `Renderer` trait defined in `rendering_core`:

```rust
pub trait Renderer {
    fn init(&mut self, window: &Window);
    fn begin_frame(&mut self);
    fn render_layer(&mut self, layer: &Layer, texture: &dyn TextureHandle);
    fn render_tile(&mut self, tile: &Tile, texture: &dyn TextureHandle);
    fn end_frame(&mut self);
    fn resize(&mut self, width: u32, height: u32);
    fn apply_quality_settings(&mut self, settings: &RenderSettings);
}
```

### Backend Implementations

Each backend implements the `Renderer` trait:

- **Bevy**: Uses Bevy's existing rendering pipeline
- **OpenGL**: Uses `glow` crate for OpenGL rendering
- **Vulkan**: Uses `vulkano` crate for Vulkan rendering

## Integration Points

### Backend Selection

The application can select a rendering backend at startup:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum RenderBackend {
    Bevy,
    OpenGL,
    Vulkan,
}
```

### Quality Management

The `QualityManager` can configure backend-specific settings:

```rust
impl QualityManager {
    pub fn configure_backend(&self, renderer: &mut dyn Renderer) {
        let core_settings = rendering_core::RenderSettings {
            resolution_scale: 1.0,
            max_anisotropy: 1,
            shadow_quality: 1,
            texture_filtering: rendering_core::TextureFiltering::Linear,
            anti_aliasing: rendering_core::AntiAliasingMode::None,
        };
        
        renderer.apply_quality_settings(&core_settings);
    }
}
```

### Texture Conversion

The `TextureCache` provides methods to convert Bevy textures to backend-specific formats:

```rust
impl TextureCache {
    pub fn as_opengl_texture(&self, handle: &Handle<Image>) -> opengl_renderer::OpenGLTextureHandle {
        // Conversion implementation
    }
    
    pub fn as_vulkan_texture(&self, handle: &Handle<Image>) -> vulkan_renderer::VulkanTextureHandle {
        // Conversion implementation
    }
}
```

## Performance Considerations

| Backend | Startup Time | Memory | GPU Utilization | Best For |
|---------|--------------|--------|-----------------|----------|
| Bevy    | Medium       | High   | Good            | Rapid development |
| OpenGL  | Fast         | Medium | Moderate        | Older hardware |
| Vulkan  | Slow         | Low    | Excellent       | Modern systems |

## Future Work

1. Complete OpenGL and Vulkan implementations
2. Add runtime backend switching
3. Implement proper texture conversion between backends
4. Add more quality settings options
5. Implement proper error handling for backend initialization

## Files Created

- `shared_packages/rendering_core/`
- `shared_packages/opengl_renderer/`
- `shared_packages/vulkan_renderer/`
- `apps/art/src/rendering/abstract_pipeline.rs`