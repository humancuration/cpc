// Audio module: Rodio mixer scaffold with planned volume/keyframe controls and shared effect trait.
//
// This is a native-only stub. Actual streaming from timeline will push decoded buffers into tracks,
// apply automation (keyframes), and mix down to the output stream.

use std::sync::{Arc, Mutex};
use tracing::info;

/// Per-track volume automation point (time in ms, gain in dB).
#[derive(Clone, Copy, Debug)]
pub struct VolumeKeyframe {
    pub t_ms: u64,
    pub gain_db: f32,
}

#[derive(Default)]
pub struct TrackAutomation {
    pub volume_keys: Vec<VolumeKeyframe>,
}

impl TrackAutomation {
    pub fn volume_at(&self, _t_ms: u64) -> f32 {
        // TODO: interpolate between keyframes; for now return 0 dB
        0.0
    }
}

pub struct AudioTrack {
    pub id: u32,
    pub automation: TrackAutomation,
    // Later: ring buffer for decoded audio, channel layout, sample rate
}

pub struct AudioMixer {
    tracks: Vec<AudioTrack>,
    master_gain_db: f32,
    // Later: rodio OutputStream and Sink, or cpal directly for lower-level control
}

impl AudioMixer {
    pub fn new() -> Self {
        info!("Audio mixer initialized (rodio stub)");
        Self {
            tracks: Vec::new(),
            master_gain_db: 0.0,
        }
    }

    pub fn add_track(&mut self) -> u32 {
        let id = (self.tracks.len() as u32) + 1;
        self.tracks.push(AudioTrack { id, automation: TrackAutomation::default() });
        id
    }

    pub fn set_master_volume(&mut self, gain_db: f32) {
        self.master_gain_db = gain_db;
    }

    pub fn tracks(&self) -> &[AudioTrack] {
        &self.tracks
    }

    pub fn mix_block(&self, _t_start_ms: u64, _duration_ms: u64) {
        // TODO: Pull from each track buffer, apply per-track automation gain,
        // then sum and apply master gain. Feed to output stream.
    }
}

/// Shared trait for audio effects compatible with the Audio app ecosystem.
pub trait AudioEffect {
    fn id(&self) -> &'static str;
    fn process(&self /*, buffer: &mut [f32], sample_rate: u32, channels: u16 */);
}

/// Example effect: simple gain (stub)
pub struct GainEffect {
    pub gain_db: f32,
}

impl AudioEffect for GainEffect {
    fn id(&self) -> &'static str { "gain" }
    fn process(&self /*, buffer: &mut [f32], sample_rate: u32, channels: u16 */) {
        let _linear = 10f32.powf(self.gain_db / 20.0);
        // for sample in buffer { *sample *= _linear; }
    }
}