# Video Editor Plugin Architecture (Draft)

This document outlines the initial plugin architecture for the CPC Video Editor. It targets both video (GPU) effects/transitions and audio effects, aligning with our cross-app goal to share effect traits with the Audio app.

Status: Prototype stage — stable API pending. Expect breaking changes.

## Goals

- Simple trait-based API to author custom effects and transitions.
- GPU-accelerated video effects via Bevy render pipeline and shaders.
- Audio effects compatible with the shared Audio app trait.
- Safe sandboxing: no unrestricted filesystem or network by default.
- Hot-reload in dev; cache manifests for release.

## Package Layout

Plugins can be:
1) In-tree (compiled together).
2) Out-of-tree dynamic libraries (native desktop only; optional).
3) WASM modules for webview contexts (planned).

## Core Traits (Rust)

Video effects:

```rust
pub trait Plugin: Send + Sync {
    fn id(&self) -> &'static str;
    fn display_name(&self) -> &'static str;
    fn init(&self) {}
}

pub trait VideoEffect: Plugin {
    fn process(&self /*, params, gpu_ctx */);
}
```

Audio effects:

```rust
pub trait AudioEffect: Plugin {
    fn process_audio(&self /*, params, buffer */);
}
```

Transitions:

```rust
pub trait Transition: Plugin {
    /// 0.0..=1.0 progress is provided by the timeline.
    fn blend(&self /*, gpu_ctx, from_tex, to_tex, progress */);
}
```

These traits live in `video_editor::plugins` and are shared with other CPC apps when appropriate.

## Registration

- In-tree: register in a plugin registry at startup.
- Dynamic: load from `.dll/.so/.dylib` using `libloading` (optional feature).
- WASM: load module in webview and register exported functions (planned).

Example in-tree registration:

```rust
fn register_builtin_plugins(reg: &mut PluginRegistry) {
    reg.register_video(Box::new(LutColorGrading { lut_name: "default.cube".into() }));
}
```

## Parameters

Parameters should be declarative to enable UI auto-generation:

```rust
#[derive(Clone, Debug)]
pub enum ParamValue {
    Bool(bool),
    IntRange { value: i32, min: i32, max: i32, step: i32 },
    FloatRange { value: f32, min: f32, max: f32, step: f32 },
    Color { r: f32, g: f32, b: f32 },
    LutFile(String),
}

pub trait Parametrized {
    fn params(&self) -> Vec<(String, ParamValue)>;
    fn set_param(&mut self, key: &str, value: ParamValue);
}
```

UI components can render controls directly from this metadata.

## GPU Effects and LUTs

- Video effects run as nodes in a Bevy-based render graph.
- LUT support: 3D LUT textures uploaded at `init()`, sampled in shader.
- Texture streaming is used for preview performance.
- Shaders must be portable (WGSL preferred).

## Audio Effects

- Process on small blocks (e.g., 128–1024 samples) with the engine schedule.
- Double-buffering and real-time safe code required (no allocations in audio thread).
- Parameters updated from UI through lock-free queues.

## Security

- No filesystem/network access by default.
- Whitelisted resource loading (e.g., LUTs from project assets).
- Dynamic plugins are disabled by default and require explicit user opt-in.

## Versioning

- Each plugin declares `api_version` and `plugin_version`.
- The host validates compatibility at load-time.

## Example: LUT Color Grading (Video)

Pseudocode:

```rust
pub struct LutColorGrading { pub lut_name: String }

impl Plugin for LutColorGrading {
    fn id(&self) -> &'static str { "lut_color_grading" }
    fn display_name(&self) -> &'static str { "LUT Color Grading" }
    fn init(&self) { /* upload 3D LUT texture */ }
}

impl VideoEffect for LutColorGrading {
    fn process(&self /*, params, gpu_ctx */) {
        // bind LUT texture, dispatch shader
    }
}
```

## Example: Gain (Audio)

```rust
pub struct GainEffect { pub gain_db: f32 }

impl Plugin for GainEffect {
    fn id(&self) -> &'static str { "gain" }
    fn display_name(&self) -> &'static str { "Gain" }
}

impl AudioEffect for GainEffect {
    fn process_audio(&self /*, params, buffer */) {
        let linear = 10f32.powf(self.gain_db / 20.0);
        // for s in buffer { *s *= linear; }
    }
}
```

## Roadmap

- Plugin manifest format (TOML) with metadata and parameters.
- Hot-reload loop and error reporting UI.
- Example gallery of effects/transitions.
- WASM plugin ABI prototype for webview contexts.