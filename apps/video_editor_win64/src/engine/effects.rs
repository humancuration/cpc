//! Effects and Transition Systems (Engine-side wiring)
//!
//! Implements:
//! - EffectParams model alignment with plugins
//! - Transition scheduling (Crossfade, Slide, Wipe) as Bevy systems
//! - GPU-accelerated processing hooks via wgpu
//! - Plugin registration API integration
//!
//! Reference: docs/architecture.md (Effects Pipeline)

use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::plugins::{
   EffectParams as PluginEffectParams, GpuContext, FrameView, PluginRegistry, VideoEffect,
   TransitionType, Transition as TransitionModel, Plugin,
};
use crate::engine::composition::{Frame, Composition};
use crate::ui::timeline::{TimelineResource, Clip, VideoTrack};

/// Built-in effect: Gaussian-like blur (stub).
pub struct BlurEffect;
impl Plugin for BlurEffect {
   fn id(&self) -> &'static str { "builtin.blur" }
   fn display_name(&self) -> &'static str { "Blur" }
}
impl VideoEffect for BlurEffect {
   fn process(&self, gpu: &GpuContext, in_a: &FrameView, _in_b: Option<&FrameView>, out: &FrameView, params: &PluginEffectParams, _time: f64) {
       let radius = params.get("radius", 2.0);
       let _ = (gpu.device, gpu.queue, in_a.view, out.view, radius);
       // TODO: dispatch separable blur passes; stubbed for now.
   }
}

/// Built-in effect: Color correction (stub).
pub struct ColorCorrectionEffect;
impl Plugin for ColorCorrectionEffect {
   fn id(&self) -> &'static str { "builtin.color_correction" }
   fn display_name(&self) -> &'static str { "Color Correction" }
}
impl VideoEffect for ColorCorrectionEffect {
   fn process(&self, gpu: &GpuContext, in_a: &FrameView, _in_b: Option<&FrameView>, out: &FrameView, params: &PluginEffectParams, _time: f64) {
       let exposure = params.get("exposure", 0.0);
       let contrast = params.get("contrast", 1.0);
       let saturation = params.get("saturation", 1.0);
       let _ = (gpu.device, gpu.queue, in_a.view, out.view, exposure, contrast, saturation);
       // TODO: fullscreen color grading pass; stubbed.
   }
}

/// Built-in effect: 2D Transform (stub).
pub struct TransformEffect;
impl Plugin for TransformEffect {
   fn id(&self) -> &'static str { "builtin.transform" }
   fn display_name(&self) -> &'static str { "Transform" }
}
impl VideoEffect for TransformEffect {
   fn process(&self, gpu: &GpuContext, in_a: &FrameView, _in_b: Option<&FrameView>, out: &FrameView, params: &PluginEffectParams, _time: f64) {
       let tx = params.get("tx", 0.0);
       let ty = params.get("ty", 0.0);
       let scale = params.get("scale", 1.0);
       let rot = params.get("rot", 0.0);
       let _ = (gpu.device, gpu.queue, in_a.view, out.view, tx, ty, scale, rot);
       // TODO: use vertex transform or compute to warp; stubbed.
   }
}

/// Engine-local wrapper that associates an effect instance with params.
#[derive(Clone)]
pub struct EffectNode {
    pub effect_id: &'static str,
    pub params: PluginEffectParams,
}

/// A scheduled transition between two clips.
#[derive(Clone)]
pub struct TransitionInstance {
    pub model: TransitionModel,
    /// Computed timeline: start/end in ms within the composition
    pub start_ms: u64,
    pub end_ms: u64,
}

/// Effects graph resource for current composition timeline.
/// In a later iteration this can become a DAG with nodes/edges.
#[derive(Resource, Default)]
pub struct EffectsGraph {
    /// Track -> list of effect nodes applied post clip sampling
    pub track_effects: HashMap<u64, Vec<EffectNode>>,
    /// Active transitions keyed by overlapping clip ids
    pub transitions: Vec<TransitionInstance>,
}

/// Resource that owns the plugin registry.
#[derive(Resource, Default)]
pub struct EffectsRegistry(pub PluginRegistry);

/// GPU device/queue shared resource to run wgpu passes.
/// In a runner integration this would be created during renderer init.
#[derive(Resource)]
pub struct WgpuDevice {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl WgpuDevice {
    pub fn context(&self) -> GpuContext {
        GpuContext { device: &self.device, queue: &self.queue }
    }
}

/// Bevy plugin to register systems for effects and transitions
pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EffectsGraph>()
           .init_resource::<EffectsRegistry>()
           .init_resource::<crate::engine::composition::FrameCache>()
           .init_resource::<crate::engine::composition::OutputTargets>()
           .add_systems(Update, (
               schedule_transitions_system,
               process_transitions_system,
               clear_framecache_scratch_system,
           ).chain());
    }
}

/// Scan the current composition timeline and schedule transitions.
/// For now, we create a transition when two clips abut with specified metadata.
/// This is a stub; the UI would create Transition entries explicitly.
fn schedule_transitions_system(
    timeline: Option<Res<TimelineResource>>,
    mut graph: ResMut<EffectsGraph>,
) {
    let Some(timeline) = timeline else { return; };
    let Some(comp_id) = timeline.0.current_composition else { return; };
    let Some(comp) = timeline.0.compositions.get(&comp_id) else { return; };

    // Naive pass: if two consecutive clips on a track overlap N ms, create a Crossfade with duration = overlap.
    graph.transitions.clear();
    for track in &comp.video_tracks {
        let mut clips = track.clips.clone();
        clips.sort_by_key(|c| c.start_ms);
        for pair in clips.windows(2) {
            let a = &pair[0];
            let b = &pair[1];
            let a_end = a.start_ms + a.duration_ms;
            if b.start_ms < a_end {
                let overlap = (a_end - b.start_ms) as f32 / 1000.0;
                if overlap > 0.0 {
                    graph.transitions.push(TransitionInstance {
                        model: TransitionModel {
                            transition_type: TransitionType::Crossfade,
                            duration: overlap,
                            clip_in: a.id,
                            clip_out: b.id,
                        },
                        start_ms: b.start_ms,
                        end_ms: a_end,
                    });
                }
            }
        }
    /// Process active transitions at current timeline time.
    /// This invokes the appropriate GPU-accelerated effect implementation.
    fn process_transitions_system(
        timeline: Option<Res<TimelineResource>>,
        registry: Option<Res<EffectsRegistry>>,
        graph: Res<EffectsGraph>,
        wgpu: Option<Res<WgpuDevice>>,
        mut frame_cache: ResMut<crate::engine::composition::FrameCache>,
        mut outputs: ResMut<crate::engine::composition::OutputTargets>,
    ) {
        let (Some(timeline), Some(registry), Some(wgpu)) = (timeline, registry, wgpu) else { return; };

        // Ensure a frame provider is present; use NullFrameProvider by default.
        if frame_cache.frame_provider.is_none() {
            struct NullFrameProvider;
            impl crate::engine::media_pipeline::FrameProvider for NullFrameProvider {
                fn get_frame(&self, _clip_id: crate::engine::media_pipeline::ClipId, _time: f64) -> Option<crate::engine::media_pipeline::DecodedFrame> {
                    None
                }
            }
            frame_cache.frame_provider = Some(std::sync::Arc::new(NullFrameProvider));
        }

        let time_ms = timeline.0.cursor_position;
        let time_s = time_ms as f32 / 1000.0;
    
        for t in &graph.transitions {
            if time_ms < t.start_ms || time_ms > t.end_ms { continue; }
            let span = (t.end_ms - t.start_ms).max(1) as f32;
            let t_norm = (time_ms - t.start_ms) as f32 / span; // 0..1
    
            // Compute params
            let mut params = PluginEffectParams::default();
            params.values.insert("t".into(), t_norm);
    
            // Lookup effect by transition type
            let effect_id = match t.model.transition_type {
                TransitionType::Crossfade => "builtin.crossfade",
                TransitionType::Slide => "builtin.slide",
                TransitionType::Wipe => "builtin.wipe",
            };
    
            // Ensure output target exists
            outputs.ensure_initialized(&wgpu.device);
            let out_view_opt = outputs.main_view();
            if out_view_opt.is_none() { continue; }
            // Store output view inside FrameCache scratch to keep a reference alive during call
            let out_view = out_view_opt.unwrap();
            frame_cache.clear_scratch(); // start fresh for this iteration
            // keep output view alive by pushing into scratch
            frame_cache.scratch_views.push(out_view);
            let out_view_ref: &wgpu::TextureView = frame_cache.scratch_views.last().unwrap();
    
            if let Some(eff) = registry.0.get_effect(effect_id) {
                // Acquire input frames from cache (with handles to keep textures alive)
                let (in_a, _handle_a) = match frame_cache.get_gpu_frame_view(t.model.clip_in, time_s, &wgpu.device, &wgpu.queue) {
                    Some(v) => v,
                    None => continue, // no frame, skip
                };
                let (in_b, _handle_b) = match frame_cache.get_gpu_frame_view(t.model.clip_out, time_s, &wgpu.device, &wgpu.queue) {
                    Some(v) => v,
                    None => continue,
                };
                let out = FrameView {
                    view: out_view_ref,
                    format: outputs.format,
                    width: outputs.width,
                    height: outputs.height,
                };
    
                let gpu_ctx = wgpu.context();
                eff.process(&gpu_ctx, &in_a, Some(&in_b), &out, &params, time_ms as f64 / 1000.0);
            }
        }
    }
        }
    }
}

// Clear scratch views after processing so references don't dangle across frames.
fn clear_framecache_scratch_system(mut cache: ResMut<crate::engine::composition::FrameCache>) {
    cache.clear_scratch();
}

// ---------------- Unit Tests ----------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::{CrossfadeEffect, SlideEffect, WipeEffect};

    #[test]
    fn registry_registers_and_resolves() {
        let mut reg = PluginRegistry::default();
        reg.register_effect(CrossfadeEffect);
        reg.register_effect(SlideEffect);
        reg.register_effect(WipeEffect);

        assert!(reg.get_effect("builtin.crossfade").is_some());
        assert!(reg.get_effect("builtin.slide").is_some());
        assert!(reg.get_effect("builtin.wipe").is_some());
    }

    #[test]
    fn schedule_crossfade_on_overlap() {
        // Build a minimal composition in a TimelineResource
        let mut timeline = crate::ui::timeline::TimelineModel::new();
        let mut comp = Composition::new("Test");
        let mut track = crate::ui::timeline::VideoTrack::default();
        track.id = 1;
        track.clips.push(Clip { id: 1, kind: crate::ui::timeline::ClipKind::Video, start_ms: 0, duration_ms: 1000, track_id: 1, keyframes: Default::default() });
        track.clips.push(Clip { id: 2, kind: crate::ui::timeline::ClipKind::Video, start_ms: 800, duration_ms: 1000, track_id: 1, keyframes: Default::default() });
        comp.video_tracks.push(track);
        let id = timeline.add_composition(comp);
        timeline.set_current_composition(id);
    
        let mut world = World::new();
        world.insert_resource(TimelineResource(timeline));
        world.insert_resource(EffectsGraph::default());
        let mut sched = Schedule::default();
        let mut update = bevy_ecs::schedule::SystemConfigs::new();
        update.add(schedule_transitions_system);
        sched.add_systems(Update, update);
    
        sched.run(&mut world);
    
        let graph = world.resource::<EffectsGraph>();
        assert_eq!(graph.transitions.len(), 1);
        let t = &graph.transitions[0];
        assert_eq!(t.model.transition_type, TransitionType::Crossfade);
        assert_eq!(t.start_ms, 800);
        assert_eq!(t.end_ms, 1000);
    }