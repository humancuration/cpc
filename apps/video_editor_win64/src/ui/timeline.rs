//! Timeline UI Module
//!
//! Handles multi-track timeline with keyframe animation
use crate::engine::composition::{Composition, Keyframe, InterpolationType, PropertyType, interpolate};
use bevy_ecs::prelude::*;
use uuid::Uuid;
use std::collections::{HashMap, BTreeMap};
use rstar::{RTree, RTreeObject, AABB, PointDistance};

#[derive(Clone, Debug)]
pub enum ClipKind {
   Video,
   Audio,
   Image,
   Title,
}
#[derive(Default)]
/// A video track containing clips, effects, and spatial indexing for efficient lookup.
/// Tracks are ordered by layer (Z-index) with 0 being the bottom layer.
pub struct VideoTrack {
  pub id: u64,
  pub clips: Vec<Clip>,
  // Replace placeholder with actual effect nodes reference ids if desired.
  // For now, store effect ids (by string) to resolve via registry during render.
  pub effects: Vec<&'static str>,
  pub layer: u32, // Z-index (0 bottom)
  pub locked: bool,
  pub muted: bool,
  /// Spatial index for O(log n) clip lookup by time
  pub spatial_index: RTree<ClipRegion>,
}

#[derive(Default)]
/// An audio track containing audio clips, volume/pan automation, and effects.
pub struct AudioTrack {
   pub id: u64,
   pub clips: Vec<AudioClip>,
   pub volume: f32, // dB
   pub pan: f32, // -1..1
   pub effects: Vec<u64>, // placeholder
}

#[derive(Default)]
/// The main timeline model containing compositions, cursor position, and frame cache.
/// This is the core data structure that drives the timeline UI and rendering.
pub struct TimelineModel {
   pub compositions: BTreeMap<Uuid, Composition>,
   pub current_composition: Option<Uuid>,
   pub cursor_position: u64, // ms
   pub scale_factor: f32,    // px per second
   // Frame cache for preview frames
   pub frame_cache: Option<crate::engine::media_pipeline::FrameCache>,
}

impl TimelineModel {
   /// Create a new timeline model with default settings and a 256MB frame cache.
   pub fn new() -> Self {
       Self {
           compositions: BTreeMap::new(),
           current_composition: None,
           cursor_position: 0,
           scale_factor: 100.0,
           frame_cache: Some(crate::engine::media_pipeline::FrameCache::new(256 * 1024 * 1024)), // 256MB default
       }
   }

   fn current_mut(&mut self) -> Option<&mut Composition> {
       self.current_composition.and_then(|id| self.compositions.get_mut(&id))
   }

   /// Set the currently active composition by its UUID.
   pub fn set_current_composition(&mut self, id: Uuid) {
       self.current_composition = Some(id);
   }

   /// Add a new composition to the timeline and set it as current if none exists.
   /// Returns the UUID of the added composition.
   pub fn add_composition(&mut self, comp: Composition) -> Uuid {
       let id = comp.id;
       self.compositions.insert(id, comp);
       if self.current_composition.is_none() {
           self.current_composition = Some(id);
       }
       id
   }

   /// Add a new video track to the current composition.
   /// The track will be assigned the next available ID and layer index.
   pub fn add_video_track(&mut self) {
       if let Some(comp) = self.current_mut() {
           let next_id = comp.video_tracks.len() as u64 + 1;
           comp.video_tracks.push(VideoTrack {
              id: next_id,
              layer: next_id as u32 - 1,
              clips: Vec::new(),
              effects: Vec::new(), // store effect ids like "builtin.blur"
              locked: false,
              muted: false,
              spatial_index: RTree::new(),
          });
           comp.video_tracks.sort_by_key(|t| t.layer);
       }
   }

   pub fn add_audio_track(&mut self) {
       if let Some(comp) = self.current_mut() {
           let next_id = comp.audio_tracks.len() as u64 + 1;
           comp.audio_tracks.push(AudioTrack { id: next_id, volume: 0.0, pan: 0.0, ..Default::default() });
       }
   }

   pub fn add_clip(&mut self, track_id: u64, mut clip: Clip) {
       if let Some(comp) = self.current_mut() {
           if let Some(track) = comp.video_tracks.iter_mut().find(|t| t.id == track_id) {
               clip.track_id = track_id;
               let region = ClipRegion::from_clip(&clip);
               track.clips.push(clip);
               track.clips.sort_by_key(|c| c.start_ms);
               // Rebuild/update spatial index incrementally
               track.spatial_index.insert(region);
           }
       }
   }

   /// Add a keyframe to a clip's property at the specified time.
   /// Returns the UUID of the newly created keyframe, or None if the clip was not found.
   pub fn add_keyframe(&mut self, clip_id: u64, property: PropertyType, value: f32, time: u64, interpolation: InterpolationType) -> Option<Uuid> {
       let mut out = None;
       if let Some(comp) = self.current_mut() {
           for t in comp.video_tracks.iter_mut() {
               if let Some(clip) = t.clips.iter_mut().find(|c| c.id == clip_id) {
                   let kf = Keyframe { id: Uuid::new_v4(), property: property.clone(), value, time, interpolation };
                   clip.keyframes.entry(property).or_default().push(kf);
                   if let Some(list) = clip.keyframes.get_mut(&property) {
                       list.sort_by_key(|k| k.time);
                       out = list.last().map(|k| k.id);
                   }
                   break;
               }
           }
       }
       out
   }

   /// Remove a keyframe from a clip's property by its UUID.
   /// Returns true if the keyframe was found and removed, false otherwise.
   pub fn remove_keyframe(&mut self, clip_id: u64, property: PropertyType, keyframe_id: Uuid) -> bool {
       if let Some(comp) = self.current_mut() {
           for t in comp.video_tracks.iter_mut() {
               if let Some(clip) = t.clips.iter_mut().find(|c| c.id == clip_id) {
                   if let Some(list) = clip.keyframes.get_mut(&property) {
                       let before = list.len();
                       list.retain(|k| k.id != keyframe_id);
                       return list.len() != before;
                   }
               }
           }
       }
       false
   }

   pub fn update_keyframe(&mut self, keyframe_id: Uuid, new_value: f32) -> bool {
       if let Some(comp) = self.current_mut() {
           for t in comp.video_tracks.iter_mut() {
               for clip in &mut t.clips {
                   for list in clip.keyframes.values_mut() {
                       if let Some(k) = list.iter_mut().find(|k| k.id == keyframe_id) {
                           k.value = new_value;
                           return true;
                       }
                   }
               }
           }
       }
       false
   }

   pub fn set_keyframe_interpolation(&mut self, keyframe_id: Uuid, interpolation: InterpolationType) -> bool {
       if let Some(comp) = self.current_mut() {
           for t in comp.video_tracks.iter_mut() {
               for clip in &mut t.clips {
                   for list in clip.keyframes.values_mut() {
                       if let Some(k) = list.iter_mut().find(|k| k.id == keyframe_id) {
                           k.interpolation = interpolation;
                           return true;
                       }
                   }
               }
           }
       }
       false
   }

   /// Get the interpolated value of a property at a specific time for a clip.
   /// Uses spatial indexing for efficient clip lookup and keyframe interpolation.
   pub fn get_value_at_time(&self, clip_id: u64, property: PropertyType, time: u64) -> Option<f32> {
       let comp = self.current_composition.and_then(|id| self.compositions.get(&id))?;
       for t in &comp.video_tracks {
           // Use spatial index to limit candidate clips at given time
           let point = [time as f64, 0.0];
           let mut candidates = t.spatial_index.locate_at_point(&point).peekable();
           if candidates.peek().is_none() {
               // Fallback: query by envelope intersection in case of not exact point hit due to discrete bounds
               for region in t.spatial_index.locate_in_envelope_intersecting(&AABB::from_corners([time as f64, 0.0], [time as f64, 0.0])) {
                   if let Some(clip) = t.clips.iter().find(|c| c.id == region.clip_id) {
                       if let Some(v) = clip.get_value_at_time(property.clone(), time) {
                           return Some(v);
                       }
                   }
               }
           } else {
               for region in candidates {
                   if let Some(clip) = t.clips.iter().find(|c| c.id == region.clip_id) {
                       if let Some(v) = clip.get_value_at_time(property.clone(), time) {
                           return Some(v);
                       }
                   }
               }
           }
           // Also handle case where clip_id directly matches
           if let Some(clip) = t.clips.iter().find(|c| c.id == clip_id) {
               return clip.get_value_at_time(property, time);
           }
       }
       None
   }
}

/// Video clip with keyframed properties
/// Represents a media clip on the timeline with position, duration, and animated properties.
pub struct Clip {
   pub id: u64,
   pub kind: ClipKind,
   pub start_ms: u64,
   pub duration_ms: u64,
   pub track_id: u64,
   pub keyframes: HashMap<PropertyType, Vec<Keyframe>>,
}

impl Clip {
   /// Get the interpolated value of a property at a specific time.
   /// Uses keyframe interpolation (linear, bezier, or hold) to calculate the value.
   pub fn get_value_at_time(&self, property: PropertyType, time: u64) -> Option<f32> {
       let list = self.keyframes.get(&property)?;
       if list.is_empty() { return None; }
       if let Some(exact) = list.iter().find(|k| k.time == time) {
           return Some(exact.value);
       }
       // find surrounding keyframes
       let mut prev: Option<&Keyframe> = None;
       let mut next: Option<&Keyframe> = None;
       for k in list {
           if k.time <= time { prev = Some(k); } else { next = Some(k); break; }
       }
       match (prev, next) {
           (Some(a), Some(b)) => Some(interpolate(a, b, time)),
           (Some(a), None) => Some(a.value),
           (None, Some(b)) => Some(b.value),
           _ => None,
       }
   }
}

/// Audio clip with volume/pan keyframes
pub struct AudioClip {
    pub id: u64,
    pub start_ms: u64,
    pub duration_ms: u64,
    pub track_id: u64,
    pub volume_keyframes: Vec<Keyframe>,
    pub pan_keyframes: Vec<Keyframe>,
}

/// Spatial indexing region for clips along the time axis.
/// Represented as a 2D rectangle with Y fixed to 0..1 so we can use rstar AABB queries.
#[derive(Clone, Copy, Debug)]
pub struct ClipRegion {
    pub start_ms: u64,
    pub end_ms: u64,
    pub clip_id: u64,
}

impl ClipRegion {
    pub fn from_clip(c: &Clip) -> Self {
        Self {
            start_ms: c.start_ms,
            end_ms: c.start_ms + c.duration_ms,
            clip_id: c.id,
        }
    }
}

impl RTreeObject for ClipRegion {
    type Envelope = AABB<[f64; 2]>;
    fn envelope(&self) -> Self::Envelope {
        let min = [self.start_ms as f64, 0.0];
        let max = [self.end_ms as f64, 1.0];
        AABB::from_corners(min, max)
    }
}

impl PointDistance for ClipRegion {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let aabb = self.envelope();
        aabb.distance_2(point)
    }
}

// ECS integration
/// Bevy ECS resource wrapper for the timeline model.
#[derive(Resource)]
pub struct TimelineResource(pub TimelineModel);

pub struct TimelineEvent {
   pub event_type: TimelineEventType,
}

pub enum TimelineEventType {
   TrackAdded,
   ClipAdded,
   KeyframeAdded,
   KeyframeUpdated,
   KeyframeRemoved,
}

/// Timeline UI plugin for Bevy
/// Registers the timeline resource, events, and systems with the Bevy app.
pub struct TimelinePlugin;

impl Plugin for TimelinePlugin {
   fn build(&self, app: &mut App) {
       app.insert_resource(TimelineResource(TimelineModel::new()))
           .add_event::<TimelineEvent>()
           // Hook preview update system so FrameUpdateEvent consumers run.
           .add_event::<crate::plugins::FrameUpdateEvent>()
           .add_systems(Update, (timeline_update_system, crate::plugins::frame_update_system));
   }
}

fn timeline_update_system(
   mut _events: EventReader<TimelineEvent>,
   mut _timeline: ResMut<TimelineResource>,
) {
   // Handle timeline events to keep ECS in sync
}