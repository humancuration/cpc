/// Test runner to verify all domain models compile and tests pass
/// This file is for development verification only

#[cfg(test)]
mod integration_tests {
    use super::super::*;

    #[test]
    fn test_all_models_compile() {
        // This test verifies all models can be imported and used together
        let project = project::Project::new("Test Project");
        let track = track::Track::new_audio("Test Track");
        let effect = effect::EffectInstance::new(
            effect::EffectType::Reverb,
            "Test Effect".to_string(),
        );
        let lane = automation::AutomationLane::new("volume".to_string());
        
        assert_eq!(project.name, "Test Project");
        assert_eq!(track.name, "Test Track");
        assert_eq!(effect.name, "Test Effect");
        assert_eq!(lane.parameter_id, "volume");
    }

    #[test]
    fn test_serialization_roundtrip() {
        // Test project serialization
        let project = project::Project::new("Serialization Test");
        let serialized = serde_json::to_string(&project).unwrap();
        let deserialized: project::Project = serde_json::from_str(&serialized).unwrap();
        assert_eq!(project.name, deserialized.name);
        
        // Test automation point serialization
        let point = automation::AutomationPoint::new(1000, 0.5);
        let serialized = serde_json::to_string(&point).unwrap();
        let deserialized: automation::AutomationPoint = serde_json::from_str(&serialized).unwrap();
        assert_eq!(point.position, deserialized.position);
        assert_eq!(point.value, deserialized.value);
    }
}