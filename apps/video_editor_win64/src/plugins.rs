//! Plugin API for effects and transitions
//!
//! Follows architecture.md (Effects Pipeline) and exposes:
//! - Plugin base trait
//! - EffectParams with keyframe support
//! - VideoEffect trait with GPU context
//! - Transition types and registry
//! - Registration API
//!
//! Preview sync events are defined here to avoid UI coupling.

use std::collections::HashMap;
use uuid::Uuid;
use bevy_ecs::prelude::*;

// Reuse keyframe primitives from engine::composition to keep a single source of truth.
use crate::engine::composition::{Keyframe, PropertyType};

/// Compute pipeline hook for GPU keyframe interpolation (stub).
/// The actual compute shader is provided in engine/gpu_interpolation.wgsl.
/// We expose a Bevy plugin and a system function so the app can .add_plugins(ComputePlugin)
/// and .add_systems(Update, run_interpolation_compute).
pub struct ComputePlugin;

impl Plugin for ComputePlugin {
   fn build(&self, app: &mut App) {
       app.add_systems(Update, run_interpolation_compute);
   }
}

/// System that would dispatch the compute pipeline for keyframe interpolation.
/// For now, this is a no-op stub wiring that will later:
/// - create/load the shader module from WGSL
/// - create bind groups with keyframe buffers
/// - dispatch workgroups to interpolate into a parameter buffer per property
pub fn run_interpolation_compute() {
   // Stub; real implementation will live in the desktop/web runner that owns wgpu device/queue.
}

/// Base plugin trait for all plugin kinds
/// Plugins must implement this trait to be registered in the plugin registry.
pub trait Plugin: Send + Sync {
    /// Unique identifier for the plugin (e.g., "builtin.blur")
    fn id(&self) -> &'static str;
    /// Human-readable name for UI display
    fn display_name(&self) -> &'static str;
    /// Optional initialization hook
    fn init(&self) {}
}

/// Parameters passed to effects and transitions, with keyframe support.
/// values provides quick access for scalar parameters at current time.
/// keyframes allows time-varying parameters per property.
#[derive(Clone, Default)]
pub struct EffectParams {
    /// Scalar parameter values (e.g., "radius" => 2.0)
    pub values: HashMap<String, f32>,
    /// Keyframe animations for time-varying parameters
    pub keyframes: Vec<Keyframe>,
}

impl EffectParams {
    /// Convenience: fetch the latest value for a named parameter, falling back to default.
    pub fn get(&self, key: &str, default: f32) -> f32 {
        *self.values.get(key).unwrap_or(&default)
    }

    /// Group keyframes by property for faster lookups in engines.
    pub fn keyframes_by_property(&self) -> HashMap<PropertyType, Vec<Keyframe>> {
        let mut map: HashMap<PropertyType, Vec<Keyframe>> = HashMap::new();
        for k in &self.keyframes {
            map.entry(k.property.clone()).or_default().push(k.clone());
        }
        for list in map.values_mut() {
            list.sort_by_key(|k| k.time);
        }
        map
    }
}

/// Minimal GPU context abstraction for effect processing.
/// This wraps what we need from wgpu without binding to a specific renderer setup.
pub struct GpuContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    // In a fuller implementation we could expose caches, bind group layouts, shader modules, etc.
}

/// A texture view wrapper for input/output frames.
/// In practice, frames will live as wgpu::TextureView handles.
pub struct FrameView<'a> {
    pub view: &'a wgpu::TextureView,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
    // In future we can associate a pool handle to auto-release.
}

/// VideoEffect trait for GPU-accelerated processing.
/// Implementations should be pure w.r.t. inputs and write into out_frame.
pub trait VideoEffect: Plugin {
    /// Process video frames using GPU acceleration
    ///
    /// # Parameters
    /// * `gpu` - GPU context with device and queue
    /// * `in_frame_a` - Primary input frame
    /// * `in_frame_b` - Optional secondary input frame (e.g., for transitions)
    /// * `out_frame` - Output frame to write results to
    /// * `params` - Effect parameters with keyframe support
    /// * `time` - Current time in seconds
    fn process(
        &self,
        gpu: &GpuContext,
        in_frame_a: &FrameView,       // primary source
        in_frame_b: Option<&FrameView>, // optional secondary source (e.g., for transitions)
        out_frame: &FrameView,
        params: &EffectParams,
        time: f64,
    );
}

/// Transition types per architecture.md
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TransitionType {
    Crossfade,
    Slide,
    Wipe,
}

/// Transition data model
#[derive(Clone, Debug)]
pub struct Transition {
    pub transition_type: TransitionType,
    pub duration: f32, // seconds
    pub clip_in: u64,
    pub clip_out: u64,
}

/// Registry to hold available effects and transitions.
/// For now, we store boxed trait objects by id.
#[derive(Default)]
pub struct PluginRegistry {
    effects: HashMap<&'static str, Box<dyn VideoEffect>>,
}

impl PluginRegistry {
    /// Create a new empty plugin registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a video effect in the registry
    pub fn register_effect<E: VideoEffect + 'static>(&mut self, effect: E) {
        self.effects.insert(effect.id(), Box::new(effect));
    }

    /// Retrieve a video effect by its ID
    pub fn get_effect(&self, id: &str) -> Option<&Box<dyn VideoEffect>> {
        self.effects.get(id)
    }

    /// Iterate over all registered effects
    pub fn iter_effects(&self) -> impl Iterator<Item = (&'static str, &Box<dyn VideoEffect>)> {
        self.effects.iter().map(|(k, v)| (*k, v))
    }
}

use crate::engine::effects::{BlurEffect, ColorCorrectionEffect, TransformEffect};

// -------- Reference built-in transitions (GPU accelerated stubs) --------

/// Crossfade: out = mix(A, B, t)
pub struct CrossfadeEffect;
impl Plugin for CrossfadeEffect {
    fn id(&self) -> &'static str { "builtin.crossfade" }
    fn display_name(&self) -> &'static str { "Crossfade" }
}
impl VideoEffect for CrossfadeEffect {
    fn process(
        &self,
        gpu: &GpuContext,
        in_a: &FrameView,
        in_b: Option<&FrameView>,
        out: &FrameView,
        params: &EffectParams,
        _time: f64,
    ) {
        let t = params.get("t", 0.0).clamp(0.0, 1.0);
        // GPU pipeline stub: use a simple shader that mixes two textures by t.
        // For now, we set up a render pass that binds inputs and writes to out.
        run_fullscreen_mix_pass(gpu, in_a, in_b.expect("Crossfade requires B"), out, t);
    }
}

/// Slide: out = sample A and B with UV offsets driven by t
pub struct SlideEffect;
impl Plugin for SlideEffect {
    fn id(&self) -> &'static str { "builtin.slide" }
    fn display_name(&self) -> &'static str { "Slide" }
}
impl VideoEffect for SlideEffect {
    fn process(
        &self,
        gpu: &GpuContext,
        in_a: &FrameView,
        in_b: Option<&FrameView>,
        out: &FrameView,
        params: &EffectParams,
        _time: f64,
    ) {
        let t = params.get("t", 0.0).clamp(0.0, 1.0);
        run_fullscreen_slide_pass(gpu, in_a, in_b.expect("Slide requires B"), out, t);
    }
}

/// Wipe: out = select(A,B) via threshold mask by t
pub struct WipeEffect;
impl Plugin for WipeEffect {
    fn id(&self) -> &'static str { "builtin.wipe" }
    fn display_name(&self) -> &'static str { "Wipe" }
}
impl VideoEffect for WipeEffect {
    fn process(
        &self,
        gpu: &GpuContext,
        in_a: &FrameView,
        in_b: Option<&FrameView>,
        out: &FrameView,
        params: &EffectParams,
        _time: f64,
    ) {
        let t = params.get("t", 0.0).clamp(0.0, 1.0);
        run_fullscreen_wipe_pass(gpu, in_a, in_b.expect("Wipe requires B"), out, t);
    }
}

// ---------------- GPU helper stubs (wgpu) ----------------

fn run_fullscreen_mix_pass(
    gpu: &GpuContext,
    a: &FrameView,
    b: &FrameView,
    out: &FrameView,
    t: f32,
) {
    // NOTE: These are stubs to illustrate architecture. Real implementation will:
    // - create/bind pipeline with vertex/fragment shaders
    // - bind texture views/samplers for a and b
    // - push constant/buffer for parameter t
    // - render a fullscreen triangle to out.view
    let _ = (gpu.device, gpu.queue, a.view, b.view, out.view, t, a.format, out.format);
}

fn run_fullscreen_slide_pass(
    gpu: &GpuContext,
    a: &FrameView,
    b: &FrameView,
    out: &FrameView,
    t: f32,
) {
    let _ = (gpu.device, gpu.queue, a.view, b.view, out.view, t);
}

fn run_fullscreen_wipe_pass(
    gpu: &GpuContext,
    a: &FrameView,
    b: &FrameView,
    out: &FrameView,
    t: f32,
) {
    let _ = (gpu.device, gpu.queue, a.view, b.view, out.view, t);
}

/// Register a default set of effects into a given registry.
/// Includes transitions and common per-clip effects.
pub fn register_default_effects(reg: &mut PluginRegistry) {
   reg.register_effect(CrossfadeEffect);
   reg.register_effect(SlideEffect);
   reg.register_effect(WipeEffect);
   reg.register_effect(BlurEffect);
   reg.register_effect(ColorCorrectionEffect);
   reg.register_effect(TransformEffect);
}

// ---------------- Preview Sync ----------------

/// Event carrying a CPU image buffer to update the preview texture.
/// In a real integration, this would carry a handle to a GPU texture or an image asset id.
#[derive(Event)]
pub struct FrameUpdateEvent {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

/// Bevy system to update preview texture from FrameUpdateEvent.
/// For now, this is a stub that would write into an Assets<Image> in a full app.
pub fn frame_update_system(
    mut events: EventReader<FrameUpdateEvent>,
) {
    for _e in events.read() {
        // TODO: integrate with renderer/preview panel texture.
        // e.g., textures.set(handle, Image::from_buffer(&e.pixels, e.width, e.height));
    }
}

// Helper to dispatch a frame update from timeline cursor changes (temporary)
pub fn dispatch_preview_update(app_world: &mut bevy_ecs::world::World, composition_id: uuid::Uuid, time_ms: u64) {
    // Try cache first
    let mut ev = None;
    if let Some(mut res) = app_world.get_resource_mut::<crate::ui::timeline::TimelineResource>() {
        if let Some(cache) = res.0.frame_cache.as_mut() {
            let key = crate::engine::media_pipeline::FrameKey { composition_id, time_ms };
            if let Some(cf) = cache.get(&key) {
                ev = Some(FrameUpdateEvent { pixels: cf.pixels.clone(), width: cf.width, height: cf.height });
            } else {
                let f = crate::engine::media_pipeline::render_frame(composition_id, time_ms);
                let cached = crate::engine::media_pipeline::CachedFrame { pixels: f.buffer.clone(), width: f.width, height: f.height };
                cache.put(key, cached);
                ev = Some(FrameUpdateEvent { pixels: f.buffer, width: f.width, height: f.height });
            }
        }
    }
    if let Some(event) = ev {
        if let Some(mut writer) = app_world.get_resource_mut::<bevy_ecs::event::Events<FrameUpdateEvent>>() {
            writer.send(event);
        }
    }
}

// ---------------- Tests ----------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn effect_params_get_default() {
        let params = EffectParams::default();
        assert!((params.get("missing", 0.42) - 0.42).abs() < 1e-6);
    }

    #[test]
    fn keyframes_grouping_sorted() {
        // Build 3 keyframes for the same property with out-of-order times
        let mut kfs = Vec::new();
        kfs.push(Keyframe {
            id: Uuid::new_v4(),
            property: PropertyType::Opacity,
            value: 0.0,
            time: 100,
            interpolation: crate::engine::composition::InterpolationType::Linear,
        });
        kfs.push(Keyframe {
            id: Uuid::new_v4(),
            property: PropertyType::Opacity,
            value: 1.0,
            time: 50,
            interpolation: crate::engine::composition::InterpolationType::Linear,
        });
        kfs.push(Keyframe {
            id: Uuid::new_v4(),
            property: PropertyType::Opacity,
            value: 0.5,
            time: 75,
            interpolation: crate::engine::composition::InterpolationType::Linear,
        });

        let params = EffectParams {
            values: HashMap::new(),
            keyframes: kfs,
        };

        let grouped = params.keyframes_by_property();
        let list = grouped.get(&PropertyType::Opacity).expect("missing opacity group");
        assert_eq!(list.len(), 3);
        assert!(list[0].time <= list[1].time && list[1].time <= list[2].time);
    }
}