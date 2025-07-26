use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::models::Track;

/// Represents a complete DAW project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub tracks: Vec<Track>,
    pub tempo: f32,
    pub sample_rate: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Project {
    pub fn new(name: String, sample_rate: u32) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            tracks: Vec::new(),
            tempo: 120.0,
            sample_rate,
            created_at: now,
            updated_at: now,
        }
    }
/// Add a new audio track to the project
    pub fn add_audio_track(&mut self, name: String) -> &mut Track {
        let track = Track::new_audio(name);
        self.tracks.push(track);
        self.tracks.last_mut().unwrap()
    }

    /// Add a new MIDI track to the project
    pub fn add_midi_track(&mut self, name: String) -> &mut Track {
        let track = Track::new_midi(name);
        self.tracks.push(track);
        self.tracks.last_mut().unwrap()
    }

    /// Add a new bus track to the project
    pub fn add_bus_track(&mut self, name: String) -> &mut Track {
        let track = Track::new_bus(name);
        self.tracks.push(track);
        self.tracks.last_mut().unwrap()
    }

    /// Get the total duration of the project (based on longest track)
    pub fn total_duration(&self) -> u64 {
        self.tracks
            .iter()
            .map(|track| track.total_duration())
            .max()
            .unwrap_or(0)
    }

    /// Get all clips across all tracks within a time range
    pub fn get_all_clips_in_range(&self, start: u64, end: u64) -> Vec<(&Track, &Clip)> {
        self.tracks
            .iter()
            .flat_map(|track| {
                track.get_clips_in_range(start, end)
                    .into_iter()
                    .map(move |clip| (track, clip))
            })
            .collect()
    }

    /// Find a clip by ID across all tracks
    pub fn find_clip(&self, clip_id: Uuid) -> Option<(&Track, &Clip)> {
        self.tracks
            .iter()
            .find_map(|track| {
                track.get_clip(clip_id)
                    .map(|clip| (track, clip))
            })
    }

    /// Find a clip by ID across all tracks (mutable)
    pub fn find_clip_mut(&mut self, clip_id: Uuid) -> Option<(&mut Track, &mut Clip)> {
        self.tracks
            .iter_mut()
            .find_map(|track| {
                track.get_clip_mut(clip_id)
                    .map(|clip| (track, clip))
            })
    }
}
}