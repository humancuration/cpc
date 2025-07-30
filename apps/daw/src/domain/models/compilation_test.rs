//! Compilation test for all domain models
//! This file tests that all domain models can be imported and compiled together

#[cfg(test)]
mod tests {
    use crate::domain::models::*;
    use crate::domain::models::automation::*;
    
    #[test]
    fn test_all_models_can_be_imported() {
        // This test verifies that all models can be imported without conflicts
        let _project = Project::new("Test Project".to_string(), 44100);
        let _track = Track::new_audio("Test Track".to_string());
        let _effect_instance = EffectInstance::new(EffectType::Reverb);
        let _effect = Effect::new("Test Effect".to_string(), EffectType::Reverb);
        let _automation_point = AutomationPoint::new(0, 0.5);
        let _automation_lane = AutomationLane::new("test".to_string());
        let _mix = Mix::new(44100);
        
        // If we get here, all imports worked
        assert!(true);
    }
    
    #[test]
    fn test_project_with_effects() {
        let mut project = Project::new("Test Project".to_string(), 44100);
        let effect = Effect::new("Test Reverb".to_string(), EffectType::Reverb);
        project.effects.push(effect);
        
        assert_eq!(project.effects.len(), 1);
    }
    
    #[test]
    fn test_track_with_audio_data() {
        let mut track = Track::new_audio("Test Track".to_string());
        track.audio_data.push(0.5);
        track.audio_data.push(0.75);
        
        assert_eq!(track.audio_data.len(), 2);
    }
}