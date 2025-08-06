# OpenGL Renderer

This package provides an OpenGL backend implementation for the CPC rendering system.

## Overview

The `opengl_renderer` package implements the `Renderer` trait from `rendering_core` using the `glow` crate for OpenGL rendering.

## Key Components

### OpenGLRenderer

The main renderer implementation:

```rust
pub struct OpenGLRenderer {
    gl: Context,
    program: NativeProgram,
    vao: NativeVertexArray,
    vbo: NativeBuffer,
}
```

### OpenGLTextureHandle

Backend-specific texture handle:

```rust
pub struct OpenGLTextureHandle {
    pub texture: NativeTexture,
}
```

## Usage

To use this renderer in your application:

```rust
use opengl_renderer::OpenGLRenderer;
use rendering_core::Renderer;
use glow::Context;

// Create OpenGL context (platform-specific)
let gl = unsafe { Context::from_loader_function(|s| /* loader function */) };

// Create renderer
let mut renderer = OpenGLRenderer::new(gl);

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

- `src/shaders/vertex.glsl` - Vertex shader
- `src/shaders/fragment.glsl` - Fragment shader

## Dependencies

- `rendering_core` - Core rendering abstraction
- `glow` - OpenGL bindings
- `wry` - Windowing library

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.