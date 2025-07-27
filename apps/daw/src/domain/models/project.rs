use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::models::Track;

/// Represents a complete DAW project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub tracks: Vec<Track>,
    pub effects: Vec<Effect>,
    pub bpm: u32,
    pub time_signature: (u8, u8),
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
            effects: Vec::new(),
            bpm: 120,
            time_signature: (4, 4),
            sample_rate,
            created_at: now,
            updated_at: now,
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        }
    
        #[test]
        fn test_project_creation() {
            let project = Project::new("Test Project".to_string(), 44100);
            assert_eq!(project.name, "Test Project");
            assert_eq!(project.bpm, 120);
            assert_eq!(project.time_signature, (4, 4));
            assert_eq!(project.sample_rate, 44100);
            assert!(project.tracks.is_empty());
        }
    
        #[test]
        fn test_add_tracks() {
            let mut project = Project::new("Test Project".to_string(), 44100);
            
            let audio_track = project.add_audio_track("Audio Track".to_string());
            assert_eq!(audio_track.name, "Audio Track");
            assert_eq!(audio_track.track_type, TrackType::Audio);
            
            let midi_track = project.add_midi_track("MIDI Track".to_string());
            assert_eq!(midi_track.name, "MIDI Track");
            assert_eq!(midi_track.track_type, TrackType::Midi);
            
            let bus_track = project.add_bus_track("Bus Track".to_string());
            assert_eq!(bus_track.name, "Bus Track");
            assert_eq!(bus_track.track_type, TrackType::Bus);
            
            assert_eq!(project.tracks.len(), 3);
        }
    
        #[test]
        fn test_total_duration() {
            let mut project = Project::new("Test Project".to_string(), 44100);
            
            // Empty project should have 0 duration
            assert_eq!(project.total_duration(), 0);
            
            // Add track with clips
            let track = project.add_midi_track("Test Track".to_string());
            let _ = track.add_clip(Clip::new_midi("Test".to_string(), 0, 44100));
            
            assert_eq!(project.total_duration(), 44100);
        }
    
        #[test]
        fn test_find_clip() {
            let mut project = Project::new("Test Project".to_string(), 44100);
            let track = project.add_midi_track("Test Track".to_string());
            
            let clip = Clip::new_midi("Test".to_string(), 0, 1000);
            let clip_id = clip.id;
            
            track.add_clip(clip).unwrap();
            
            let found = project.find_clip(clip_id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().1.name, "Test");
            
            let not_found = project.find_clip(Uuid::new_v4());
            assert!(not_found.is_none());
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