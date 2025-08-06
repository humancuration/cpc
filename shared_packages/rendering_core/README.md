# Rendering Core

This package provides the core abstraction for multi-backend rendering support in CPC applications.

## Overview

The `rendering_core` package defines the `Renderer` trait that all rendering backends must implement. This allows applications to support multiple graphics APIs (OpenGL, Vulkan, etc.) through a consistent interface.

## Key Components

### Renderer Trait

The main interface that all backends must implement:

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

### Common Types

- `RenderSettings` - Configuration for rendering quality
- `TextureHandle` - Trait for backend-specific texture handles
- `TextureFiltering` - Texture filtering options
- `AntiAliasingMode` - Anti-aliasing modes
- `Layer` - Layer data structure
- `Tile` - Tile data structure

## Usage

To use this package, implement the `Renderer` trait for your specific backend:

```rust
use rendering_core::{Renderer, RenderSettings, TextureHandle, Layer, Tile};
use wry::application::window::Window;

pub struct MyRenderer {
    // Backend-specific fields
}

impl Renderer for MyRenderer {
    fn init(&mut self, window: &Window) {
        // Initialize backend with window
    }
    
    fn begin_frame(&mut self) {
        // Begin frame rendering
    }
    
    fn render_layer(&mut self, layer: &Layer, texture: &dyn TextureHandle) {
        // Render a layer
    }
    
    fn render_tile(&mut self, tile: &Tile, texture: &dyn TextureHandle) {
        // Render a tile
    }
    
    fn end_frame(&mut self) {
        // End frame rendering
    }
    
    fn resize(&mut self, width: u32, height: u32) {
        // Handle window resize
    }
    
    fn apply_quality_settings(&mut self, settings: &RenderSettings) {
        // Apply quality settings
    }
}
```

## Related Packages

- `opengl_renderer` - OpenGL backend implementation
- `vulkan_renderer` - Vulkan backend implementation

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.