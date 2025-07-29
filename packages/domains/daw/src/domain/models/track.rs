use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use crate::domain::models::automation::AutomationLane;
use crate::domain::models::effect::EffectInstance;

/// Represents the type of track in the DAW
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TrackType {
    Audio,
    Midi,
    Bus,
}

/// Represents a fade curve type for audio clips
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FadeCurve {
    Linear,
    Exponential(f32), // Exponential factor
    Logarithmic(f32), // Logarithmic factor
    SCurve(f32),      // S-curve factor
}

/// Represents fade in/out parameters for clips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fade {
    pub curve: FadeCurve,
    pub duration: u64, // Duration in samples
}

impl Fade {
    pub fn new(curve: FadeCurve, duration: u64) -> Self {
        Self { curve, duration }
    }

    pub fn none() -> Self {
        Self {
            curve: FadeCurve::Linear,
            duration: 0,
        }
    }
}

/// Represents a MIDI note
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MidiNote {
    pub pitch: u8,      // MIDI note number (0-127)
    pub velocity: u8,   // MIDI velocity (0-127)
    pub start_time: u64, // Start time in samples
    pub duration: u64,   // Duration in samples
    pub channel: u8,     // MIDI channel (0-15)
}

/// Represents a MIDI clip containing note data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiClipData {
    pub notes: Vec<MidiNote>,
    pub instrument_plugin_id: Option<Uuid>, // Reference to instrument plugin
    pub transpose: i8,      // Transposition in semitones
    pub velocity_scale: f32, // Velocity scaling factor
}

impl MidiClipData {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            instrument_plugin_id: None,
            transpose: 0,
            velocity_scale: 1.0,
        }
    }
}

/// Represents an audio clip with file reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioClipData {
    pub file_path: PathBuf,
    pub sample_rate: u32,
    pub channels: u16,
    pub format: String, // e.g., "wav", "flac", "mp3"
    pub cached_samples: Option<Arc<Vec<f32>>>, // Cached audio data for performance
}

impl AudioClipData {
    pub fn new(file_path: PathBuf, sample_rate: u32, channels: u16, format: String) -> Self {
        Self {
            file_path,
            sample_rate,
            channels,
            format,
            cached_samples: None,
        }
    }
}

/// Represents a single clip on a track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: Uuid,
    pub name: String,
    pub start_position: u64, // Start position in samples from track beginning
    pub duration: u64,       // Duration in samples
    pub fade_in: Fade,
    pub fade_out: Fade,
    pub clip_type: ClipType,
    pub muted: bool,
    pub color: Option<String>, // Optional color for visual organization
}

/// Enum representing different types of clips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipType {
    Audio(AudioClipData),
    Midi(MidiClipData),
}

impl Clip {
    pub fn new_audio(
        name: String,
        start_position: u64,
        duration: u64,
        file_path: PathBuf,
        sample_rate: u32,
        channels: u16,
        format: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            start_position,
            duration,
            fade_in: Fade::none(),
            fade_out: Fade::none(),
            clip_type: ClipType::Audio(AudioClipData::new(file_path, sample_rate, channels, format)),
            muted: false,
            color: None,
        }
    }

    pub fn new_midi(name: String, start_position: u64, duration: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            start_position,
            duration,
            fade_in: Fade::none(),
            fade_out: Fade::none(),
            clip_type: ClipType::Midi(MidiClipData::new()),
            muted: false,
            color: None,
        }
    }

    pub fn end_position(&self) -> u64 {
        self.start_position + self.duration
    }

    pub fn overlaps_with(&self, other_start: u64, other_end: u64) -> bool {
        let self_end = self.end_position();
        self.start_position < other_end && self_end > other_start
    }

    pub fn is_audio(&self) -> bool {
        matches!(self.clip_type, ClipType::Audio(_))
    }

    pub fn is_midi(&self) -> bool {
        matches!(self.clip_type, ClipType::Midi(_))
    }
}

/// Represents a single track in a DAW project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub name: String,
    pub track_type: TrackType,
    pub clips: Vec<Clip>,
    pub volume: f32,
    pub pan: f32,
    pub muted: bool,
    pub solo: bool,
    pub effects: Vec<EffectInstance>,
    pub input_channels: Vec<String>, // Input channel configuration
    pub output_channels: Vec<String>, // Output channel configuration
    pub automation_lanes: Vec<AutomationLane>,
    pub audio_data: Vec<f32>, // Audio data for the track
}

impl Track {
    pub fn new(name: String, track_type: TrackType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            track_type,
            clips: Vec::new(),
            volume: 1.0,
            pan: 0.0,
            muted: false,
            solo: false,
            effects: Vec::new(),
            input_channels: Vec::new(),
            output_channels: Vec::new(),
            automation_lanes: Vec::new(),
            audio_data: Vec::new(),
        }
    }

    pub fn new_audio(name: String) -> Self {
        Self::new(name, TrackType::Audio)
    }

    pub fn new_midi(name: String) -> Self {
        Self::new(name, TrackType::Midi)
    }

    pub fn new_bus(name: String) -> Self {
        Self::new(name, TrackType::Bus)
    }

    /// Add a clip to the track
    pub fn add_clip(&mut self, clip: Clip) -> Result<(), String> {
        // Validate clip type matches track type
        match (self.track_type, &clip.clip_type) {
            (TrackType::Audio, ClipType::Audio(_)) => {},
            (TrackType::Midi, ClipType::Midi(_)) => {},
            (TrackType::Bus, _) => return Err("Bus tracks cannot contain clips".to_string()),
            _ => return Err("Clip type does not match track type".to_string()),
        }

        self.clips.push(clip);
        self.sort_clips();
        Ok(())
    }

    /// Remove a clip by ID
    pub fn remove_clip(&mut self, clip_id: Uuid) -> Option<Clip> {
        if let Some(pos) = self.clips.iter().position(|c| c.id == clip_id) {
            Some(self.clips.remove(pos))
        } else {
            None
        }
    }

    /// Get a clip by ID
    pub fn get_clip(&self, clip_id: Uuid) -> Option<&Clip> {
        self.clips.iter().find(|c| c.id == clip_id)
    }

    /// Get a mutable clip by ID
    pub fn get_clip_mut(&mut self, clip_id: Uuid) -> Option<&mut Clip> {
        self.clips.iter_mut().find(|c| c.id == clip_id)
    }

    /// Move a clip to a new position
    pub fn move_clip(&mut self, clip_id: Uuid, new_position: u64) -> Result<(), String> {
        if let Some(clip) = self.get_clip_mut(clip_id) {
            clip.start_position = new_position;
            self.sort_clips();
            Ok(())
        } else {
            Err("Clip not found".to_string())
        }
    }

    /// Split a clip at the specified position
    pub fn split_clip(&mut self, clip_id: Uuid, split_position: u64) -> Result<(), String> {
        if let Some(pos) = self.clips.iter().position(|c| c.id == clip_id) {
            let clip = &self.clips[pos];
            
            // Validate split position
            if split_position <= clip.start_position || split_position >= clip.end_position() {
                return Err("Invalid split position".to_string());
            }

            let mut first_clip = clip.clone();
            let mut second_clip = clip.clone();
            
            let first_duration = split_position - clip.start_position;
            let second_duration = clip.end_position() - split_position;
            
            first_clip.duration = first_duration;
            second_clip.start_position = split_position;
            second_clip.duration = second_duration;
            second_clip.id = Uuid::new_v4();
            
            // Replace original with first part
            self.clips[pos] = first_clip;
            
            // Insert second part
            self.clips.insert(pos + 1, second_clip);
            self.sort_clips();
            
            Ok(())
        } else {
            Err("Clip not found".to_string())
        }
    }

    /// Trim the start of a clip
    pub fn trim_clip_start(&mut self, clip_id: Uuid, new_start: u64) -> Result<(), String> {
        if let Some(clip) = self.get_clip_mut(clip_id) {
            if new_start >= clip.end_position() {
                return Err("New start position exceeds clip end".to_string());
            }
            
            let delta = new_start - clip.start_position;
            clip.start_position = new_start;
            clip.duration = clip.duration.saturating_sub(delta);
            Ok(())
        } else {
            Err("Clip not found".to_string())
        }
    }

    /// Trim the end of a clip
    pub fn trim_clip_end(&mut self, clip_id: Uuid, new_end: u64) -> Result<(), String> {
        if let Some(clip) = self.get_clip_mut(clip_id) {
            if new_end <= clip.start_position {
                return Err("New end position is before clip start".to_string());
            }
            
            clip.duration = new_end - clip.start_position;
            Ok(())
        } else {
            Err("Clip not found".to_string())
        }
    }

    /// Get all clips within a time range
    pub fn get_clips_in_range(&self, start: u64, end: u64) -> Vec<&Clip> {
        self.clips
            .iter()
            .filter(|clip| clip.overlaps_with(start, end))
            .collect()
    }

    /// Get the total duration of the track (based on last clip end)
    pub fn total_duration(&self) -> u64 {
        self.clips
            .iter()
            .map(|clip| clip.end_position())
            .max()
            .unwrap_or(0)
    }

    /// Sort clips by start position
    fn sort_clips(&mut self) {
        self.clips.sort_by_key(|clip| clip.start_position);
    }

    /// Get clips at a specific time position
    pub fn get_clips_at_position(&self, position: u64) -> Vec<&Clip> {
        self.clips
            .iter()
            .filter(|clip| position >= clip.start_position && position < clip.end_position())
            .collect()
    }

    /// Check if any clips overlap at a given position
    pub fn has_overlapping_clips(&self, start: u64, duration: u64) -> bool {
        let end = start + duration;
        self.clips.iter().any(|clip| clip.overlaps_with(start, end))
    }

    /// Duplicate a clip
    pub fn duplicate_clip(&mut self, clip_id: Uuid, new_position: u64) -> Result<(), String> {
        if let Some(clip) = self.get_clip(clip_id) {
            let mut new_clip = clip.clone();
            new_clip.id = Uuid::new_v4();
            new_clip.start_position = new_position;
            self.add_clip(new_clip)
        } else {
            Err("Clip not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_creation() {
        let clip = Clip::new_audio(
            "Test Audio".to_string(),
            0,
            44100,
            PathBuf::from("test.wav"),
            44100,
            2,
            "wav".to_string(),
        );
        
        assert_eq!(clip.name, "Test Audio");
        assert_eq!(clip.start_position, 0);
        assert_eq!(clip.duration, 44100);
        assert!(clip.is_audio());
    }

#[test]
    fn test_midi_clip_creation() {
        let clip = Clip::new_midi("Test MIDI".to_string(), 0, 44100);
        
        assert_eq!(clip.name, "Test MIDI");
        assert_eq!(clip.start_position, 0);
        assert_eq!(clip.duration, 44100);
        assert!(clip.is_midi());
    }

    #[test]
    fn test_clip_overlap() {
        let clip1 = Clip::new_midi("Clip 1".to_string(), 0, 1000);
        let clip2 = Clip::new_midi("Clip 2".to_string(), 500, 1000);
        let clip3 = Clip::new_midi("Clip 3".to_string(), 2000, 1000);
        
        assert!(clip1.overlaps_with(500, 1500));
        assert!(clip2.overlaps_with(0, 500));
        assert!(!clip1.overlaps_with(2000, 3000));
        assert!(!clip3.overlaps_with(0, 1000));
    }

    #[test]
    fn test_track_creation() {
        let track = Track::new_audio("Test Track".to_string());
        
        assert_eq!(track.name, "Test Track");
        assert_eq!(track.track_type, TrackType::Audio);
        assert!(track.clips.is_empty());
    }

    #[test]
    fn test_add_remove_clips() {
        let mut track = Track::new_audio("Test Track".to_string());
        let clip = Clip::new_midi("Test".to_string(), 0, 1000);
        
        // MIDI clip should fail on audio track
        let result = track.add_clip(clip.clone());
        assert!(result.is_err());
        
        // Remove audio_data from clip for audio track test
        let audio_clip = Clip::new_audio(
            "Test".to_string(),
            0,
            1000,
            PathBuf::from("test.wav"),
            44100,
            2,
            "wav".to_string(),
        );
        
        let result = track.add_clip(audio_clip);
        assert!(result.is_ok());
        assert_eq!(track.clips.len(), 1);
        
        let clip_id = track.clips[0].id;
        let removed = track.remove_clip(clip_id);
        assert!(removed.is_some());
        assert!(track.clips.is_empty());
    }

    #[test]
    fn test_move_clip() {
        let mut track = Track::new_midi("MIDI Track".to_string());
        let clip = Clip::new_midi("Test".to_string(), 0, 1000);
        
        track.add_clip(clip).unwrap();
        let clip_id = track.clips[0].id;
        
        track.move_clip(clip_id, 2000).unwrap();
        assert_eq!(track.clips[0].start_position, 2000);
    }

    #[test]
    fn test_split_clip() {
        let mut track = Track::new_midi("MIDI Track".to_string());
        let clip = Clip::new_midi("Test".to_string(), 0, 2000);
        
        track.add_clip(clip).unwrap();
        let clip_id = track.clips[0].id;
        
        track.split_clip(clip_id, 1000).unwrap();
        
        assert_eq!(track.clips.len(), 2);
        assert_eq!(track.clips[0].duration, 1000);
        assert_eq!(track.clips[1].start_position, 1000);
        assert_eq!(track.clips[1].duration, 1000);
    }

    #[test]
    fn test_trim_clip() {
        let mut track = Track::new_midi("MIDI Track".to_string());
        let clip = Clip::new_midi("Test".to_string(), 0, 2000);
        
        track.add_clip(clip).unwrap();
        let clip_id = track.clips[0].id;
        
        track.trim_clip_start(clip_id, 500).unwrap();
        assert_eq!(track.clips[0].start_position, 500);
        assert_eq!(track.clips[0].duration, 1500);
        
        track.trim_clip_end(clip_id, 1500).unwrap();
        assert_eq!(track.clips[0].duration, 1000);
    }

    #[test]
    fn test_get_clips_in_range() {
        let mut track = Track::new_midi("MIDI Track".to_string());
        
        track.add_clip(Clip::new_midi("Clip 1".to_string(), 0, 1000)).unwrap();
        track.add_clip(Clip::new_midi("Clip 2".to_string(), 500, 1000)).unwrap();
        track.add_clip(Clip::new_midi("Clip 3".to_string(), 2000, 1000)).unwrap();
        
        let clips = track.get_clips_in_range(400, 600);
        assert_eq!(clips.len(), 2);
        
        let clips = track.get_clips_in_range(1500, 2500);
        assert_eq!(clips.len(), 1);
        
        let clips = track.get_clips_in_range(3000, 4000);
        assert_eq!(clips.len(), 0);
    }

    #[test]
    fn test_duplicate_clip() {
        let mut track = Track::new_midi("MIDI Track".to_string());
        let clip = Clip::new_midi("Test".to_string(), 0, 1000);
        
        track.add_clip(clip).unwrap();
        let clip_id = track.clips[0].id;
        
        track.duplicate_clip(clip_id, 2000).unwrap();
        
        assert_eq!(track.clips.len(), 2);
        assert_eq!(track.clips[0].name, track.clips[1].name);
        assert_eq!(track.clips[1].start_position, 2000);
        assert_ne!(track.clips[0].id, track.clips[1].id);
    }

    #[test]
    fn test_track_type_validation() {
        let mut audio_track = Track::new_audio("Audio Track".to_string());
        let mut midi_track = Track::new_midi("MIDI Track".to_string());
        let mut bus_track = Track::new_bus("Bus Track".to_string());
        
        let audio_clip = Clip::new_audio(
            "Test".to_string(),
            0,
            1000,
            PathBuf::from("test.wav"),
            44100,
            2,
            "wav".to_string(),
        );
        
        let midi_clip = Clip::new_midi("Test".to_string(), 0, 1000);
        
        // Audio clip to audio track - should work
        assert!(audio_track.add_clip(audio_clip.clone()).is_ok());
        
        // MIDI clip to MIDI track - should work
        assert!(midi_track.add_clip(midi_clip.clone()).is_ok());
        
        // Audio clip to MIDI track - should fail
        assert!(midi_track.add_clip(audio_clip).is_err());
        
        // MIDI clip to audio track - should fail
        assert!(audio_track.add_clip(midi_clip).is_err());
        
        // Any clip to bus track - should fail
        assert!(bus_track.add_clip(Clip::new_midi("Test".to_string(), 0, 1000)).is_err());
    }
}
}