//! Integration tests for all domain models
//! This file tests that all domain models work together correctly

#[cfg(test)]
mod tests {
    use crate::domain::models::*;
    use crate::domain::models::automation::*;
    
    #[test]
    fn test_project_with_all_track_types() {
        let mut project = Project::new("Integration Test".to_string(), 44100);
        
        // Create all track types
        let audio_track = project.add_audio_track("Audio Track".to_string());
        let midi_track = project.add_midi_track("MIDI Track".to_string());
        let bus_track = project.add_bus_track("Bus Track".to_string());
        
        // Add clips to audio track
        let audio_clip = Clip::new_audio(
            "Test Audio".to_string(),
            0,
            44100,
            std::path::PathBuf::from("test.wav"),
            44100,
            2,
            "wav".to_string(),
        );
        assert!(audio_track.add_clip(audio_clip).is_ok());
        
        // Add clips to MIDI track
        let midi_clip = Clip::new_midi("Test MIDI".to_string(), 0, 44100);
        assert!(midi_track.add_clip(midi_clip).is_ok());
        
        // Add effects to tracks
        let reverb = EffectInstance::new(EffectType::Reverb);
        audio_track.effects.push(reverb);
        
        let delay = EffectInstance::new(EffectType::Delay);
        midi_track.effects.push(delay);
        
        // Add automation lanes
        let volume_lane = AutomationLane::for_track("volume".to_string(), audio_track.id);
        audio_track.automation_lanes.push(volume_lane);
        
        // Verify project structure
        assert_eq!(project.tracks.len(), 3);
        assert_eq!(project.total_duration(), 44100);
    }
    
    #[test]
    fn test_serialization_roundtrip() {
        // Test Project serialization
        let mut project = Project::new("Serialization Test".to_string(), 48000);
        let track = project.add_audio_track("Test Track".to_string());
        let clip = Clip::new_audio(
            "Test Clip".to_string(),
            0,
            24000,
            std::path::PathBuf::from("test.wav"),
            48000,
            2,
            "wav".to_string(),
        );
        let _ = track.add_clip(clip);
        
        let serialized = serde_json::to_string(&project).unwrap();
        let deserialized: Project = serde_json::from_str(&serialized).unwrap();
        assert_eq!(project.name, deserialized.name);
        assert_eq!(project.tracks.len(), deserialized.tracks.len());
        
        // Test Track serialization
        let track = Track::new_audio("Test Track".to_string());
        let serialized = serde_json::to_string(&track).unwrap();
        let deserialized: Track = serde_json::from_str(&serialized).unwrap();
        assert_eq!(track.name, deserialized.name);
        assert_eq!(track.track_type, deserialized.track_type);
        
        // Test Effect serialization
        let effect = EffectInstance::new(EffectType::Reverb);
        let serialized = serde_json::to_string(&effect).unwrap();
        let deserialized: EffectInstance = serde_json::from_str(&serialized).unwrap();
        assert_eq!(effect.effect_type, deserialized.effect_type);
        assert_eq!(effect.parameters.len(), deserialized.parameters.len());
        
        // Test Automation serialization
        let point = AutomationPoint::new(1000, 0.5);
        let serialized = serde_json::to_string(&point).unwrap();
        let deserialized: AutomationPoint = serde_json::from_str(&serialized).unwrap();
        assert_eq!(point.position, deserialized.position);
        assert_eq!(point.value, deserialized.value);
    }
    
    #[test]
    fn test_mix_creation() {
        let mix = Mix::new(44100);
        assert_eq!(mix.sample_rate, 44100);
        assert_eq!(mix.bit_depth, 24);
    }
}