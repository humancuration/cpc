use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::models::automation::AutomationLane;

/// Represents the type of audio effect
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EffectType {
    Reverb,
    Equalizer,
    Compressor,
    Delay,
    Distortion,
    Chorus,
}

/// Represents an effect instance with parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectInstance {
    pub effect_type: EffectType,
    pub parameters: HashMap<String, f32>,
}

impl EffectInstance {
    pub fn new(effect_type: EffectType) -> Self {
        let parameters = match effect_type {
            EffectType::Reverb => {
                let mut params = HashMap::new();
                params.insert("room_size".to_string(), 0.5);
                params.insert("damping".to_string(), 0.5);
                params.insert("wet_level".to_string(), 0.3);
                params.insert("dry_level".to_string(), 0.7);
                params
            }
            EffectType::Equalizer => {
                let mut params = HashMap::new();
                params.insert("low_gain".to_string(), 0.0);
                params.insert("mid_gain".to_string(), 0.0);
                params.insert("high_gain".to_string(), 0.0);
                params.insert("low_freq".to_string(), 80.0);
                params.insert("mid_freq".to_string(), 1000.0);
                params.insert("high_freq".to_string(), 8000.0);
                params
            }
            EffectType::Compressor => {
                let mut params = HashMap::new();
                params.insert("threshold".to_string(), -20.0);
                params.insert("ratio".to_string(), 4.0);
                params.insert("attack".to_string(), 10.0);
                params.insert("release".to_string(), 100.0);
                params.insert("makeup_gain".to_string(), 0.0);
                params
            }
            EffectType::Delay => {
                let mut params = HashMap::new();
                params.insert("time".to_string(), 0.3);
                params.insert("feedback".to_string(), 0.3);
                params.insert("mix".to_string(), 0.5);
                params
            }
            EffectType::Distortion => {
                let mut params = HashMap::new();
                params.insert("gain".to_string(), 0.5);
                params.insert("tone".to_string(), 0.5);
                params
            }
            EffectType::Chorus => {
                let mut params = HashMap::new();
                params.insert("rate".to_string(), 0.5);
                params.insert("depth".to_string(), 0.3);
                params.insert("mix".to_string(), 0.5);
                params
            }
        };
        
        Self {
            effect_type,
            parameters,
        }
    }
}

/// Represents a complete audio effect that can be applied to tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: Uuid,
    pub name: String,
    pub effect_type: EffectType,
    pub parameters: EffectParameters,
    pub enabled: bool,
    pub automation_lanes: Vec<AutomationLane>,
}

/// Extended effect parameters for the complete Effect struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectParameters {
    pub reverb_size: Option<f32>,
    pub delay_time: Option<f32>,
    pub delay_feedback: Option<f32>,
    pub delay_mix: Option<f32>,
    pub compression_ratio: Option<f32>,
    pub eq_bands: Option<Vec<(f32, f32)>>,
    pub distortion_gain: Option<f32>,
    pub distortion_tone: Option<f32>,
    pub chorus_rate: Option<f32>,
    pub chorus_depth: Option<f32>,
    pub chorus_mix: Option<f32>,
}

impl Effect {
    pub fn new(name: String, effect_type: EffectType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            effect_type,
            parameters: EffectParameters::default(),
            enabled: true,
            automation_lanes: Vec::new(),
        }
    }
}

impl Default for EffectParameters {
    fn default() -> Self {
        Self {
            reverb_size: Some(0.5),
            delay_time: Some(0.3),
            delay_feedback: Some(0.3),
            delay_mix: Some(0.5),
            compression_ratio: Some(4.0),
            eq_bands: Some(vec![(1000.0, 0.0), (5000.0, 0.0), (10000.0, 0.0)]),
            distortion_gain: Some(0.5),
            distortion_tone: Some(0.5),
            chorus_rate: Some(0.5),
            chorus_depth: Some(0.3),
            chorus_mix: Some(0.5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_type_variants() {
        let reverb = EffectType::Reverb;
        let eq = EffectType::Equalizer;
        let comp = EffectType::Compressor;
        
        assert_eq!(reverb, EffectType::Reverb);
        assert_eq!(eq, EffectType::Equalizer);
        assert_eq!(comp, EffectType::Compressor);
    }

    #[test]
    fn test_effect_instance_creation() {
        let reverb = EffectInstance::new(EffectType::Reverb);
        assert_eq!(reverb.effect_type, EffectType::Reverb);
        assert!(reverb.parameters.contains_key("room_size"));
        assert!(reverb.parameters.contains_key("damping"));
        assert!(reverb.parameters.contains_key("wet_level"));
        assert!(!reverb.parameters.contains_key("threshold")); // Shouldn't have compressor params

        let eq = EffectInstance::new(EffectType::Equalizer);
        assert_eq!(eq.effect_type, EffectType::Equalizer);
        assert!(eq.parameters.contains_key("low_gain"));
        assert!(eq.parameters.contains_key("mid_gain"));
        assert!(eq.parameters.contains_key("high_gain"));
        assert!(!eq.parameters.contains_key("room_size")); // Shouldn't have reverb params

        let comp = EffectInstance::new(EffectType::Compressor);
        assert_eq!(comp.effect_type, EffectType::Compressor);
        assert!(comp.parameters.contains_key("threshold"));
        assert!(comp.parameters.contains_key("ratio"));
        assert!(comp.parameters.contains_key("attack"));
        assert!(comp.parameters.contains_key("release"));

        let delay = EffectInstance::new(EffectType::Delay);
        assert_eq!(delay.effect_type, EffectType::Delay);
        assert!(delay.parameters.contains_key("time"));
        assert!(delay.parameters.contains_key("feedback"));
        assert!(delay.parameters.contains_key("mix"));
        assert!(!delay.parameters.contains_key("room_size")); // Shouldn't have reverb params

        let distortion = EffectInstance::new(EffectType::Distortion);
        assert_eq!(distortion.effect_type, EffectType::Distortion);
        assert!(distortion.parameters.contains_key("gain"));
        assert!(distortion.parameters.contains_key("tone"));
        assert!(!distortion.parameters.contains_key("time")); // Shouldn't have delay params

        let chorus = EffectInstance::new(EffectType::Chorus);
        assert_eq!(chorus.effect_type, EffectType::Chorus);
        assert!(chorus.parameters.contains_key("rate"));
        assert!(chorus.parameters.contains_key("depth"));
        assert!(chorus.parameters.contains_key("mix"));
        assert!(!chorus.parameters.contains_key("gain")); // Shouldn't have distortion params
    }

    #[test]
    fn test_effect_creation() {
        let effect = Effect::new("My Reverb".to_string(), EffectType::Reverb);
        assert_eq!(effect.name, "My Reverb");
        assert_eq!(effect.effect_type, EffectType::Reverb);
        assert!(effect.enabled);
        assert!(effect.automation_lanes.is_empty());
    }

    #[test]
    fn test_effect_parameters_default() {
        let params = EffectParameters::default();
        assert_eq!(params.reverb_size, Some(0.5));
        assert_eq!(params.delay_time, Some(0.3));
        assert_eq!(params.compression_ratio, Some(4.0));
        assert!(params.eq_bands.is_some());
    }

    #[test]
    fn test_serialization_deserialization() {
        let reverb = EffectInstance::new(EffectType::Reverb);
        let serialized = serde_json::to_string(&reverb).unwrap();
        let deserialized: EffectInstance = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(deserialized.effect_type, EffectType::Reverb);
        assert_eq!(deserialized.parameters.len(), reverb.parameters.len());
    }
}