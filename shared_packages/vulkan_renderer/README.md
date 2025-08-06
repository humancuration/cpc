# Vulkan Renderer

This package provides a Vulkan backend implementation for the CPC rendering system.

## Overview

The `vulkan_renderer` package implements the `Renderer` trait from `rendering_core` using the `vulkano` crate for Vulkan rendering.

## Key Components

### VulkanRenderer

The main renderer implementation:

```rust
pub struct VulkanRenderer {
    device: Arc<Device>,
    queue: Arc<Queue>,
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    command_buffer_builder: Option<AutoCommandBufferBuilder<PrimaryCommandBufferAbstract>>,
}
```

### VulkanTextureHandle

Backend-specific texture handle:

```rust
pub struct VulkanTextureHandle {
    pub image_view: Arc<ImageView<StorageImage>>,
}
```

## Usage

To use this renderer in your application:

```rust
use vulkan_renderer::VulkanRenderer;
use rendering_core::Renderer;

// Create renderer
let mut renderer = VulkanRenderer::new();

// Initialize with window
renderer.init(window);

// Render loop
loop {
    renderer.begin_frame();
    renderer.render_layer(&layer, &texture_handle);
    renderer.end_frame();
}
```

## Shaders

The package includes basic shaders for rendering:

- `src/shaders/vertex.vert` - Vertex shader
- `src/shaders/fragment.frag` - Fragment shader

## Dependencies

- `rendering_core` - Core rendering abstraction
- `vulkano` - Vulkan bindings
- `vulkano-shaders` - Shader compilation utilities
- `wry` - Windowing library

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.