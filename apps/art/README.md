# Art App

A professional-grade digital creation tool similar to Photoshop, Krita, and Procreate, built with Rust and Bevy engine.

## Overview

The Art application provides a comprehensive set of digital creation tools for artists, including:

- Layer-based editing system
- Professional brush tools with pressure sensitivity
- Advanced selection and transformation tools
- Support for industry-standard file formats (PNG, WebP)
- Real-time collaboration capabilities
- Undo/Redo history management

## Features

### Core Functionality (Phase 1 - MVP)
- Canvas rendering with zoom and pan
- Basic brush tools with size and opacity controls
- Layer management (add, remove, reorder)
- File I/O (import/export PNG/WebP)
- Undo/Redo system

### Advanced Features (Phase 2)
- Selection tools (rectangle, lasso, magic wand)
- Transform operations (move, scale, rotate)
- Text tools
- Layer effects and blending modes
- Customizable brush presets

### Collaboration & Optimization (Phase 3)
- Real-time collaboration support
- Performance tuning for large canvases
- Advanced file formats (PSD, KRA)
- Plugin system for filters and tools

## Architecture

The application follows a hexagonal architecture with clear separation between core logic and adapters:

```
apps/art/
├── src/
│   ├── core/              # Domain models and business logic
│   ├── rendering/         # Bevy-based rendering pipeline
│   ├── tools/             # Editing tools implementation
│   ├── persistence/       # Sled storage implementation
│   └── main.rs            # Application entry point
```

## Dependencies

- [Bevy 0.16](https://bevyengine.org/) - Game engine used for rendering
- [ffmpeg-next 6.0](https://crates.io/crates/ffmpeg-next) - For AV1/WebP support
- [Sled 0.34](https://crates.io/crates/sled) - Embedded database for local storage
- [uuid](https://crates.io/crates/uuid) - UUID generation
- [serde](https://crates.io/crates/serde) - Serialization framework

## Shared Packages Integration

- `collaboration_engine` - Real-time collaboration capabilities
- `media` - Image processing and encoding/decoding
- `common_utils` - Shared utilities and helper functions

## Building and Running

To build and run the application:

```bash
cd apps/art
cargo run
```

## Testing

To run tests:

```bash
cd apps/art
cargo test
```

## License

This software is licensed under the CPC license - see the LICENSE file for details.