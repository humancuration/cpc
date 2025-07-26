use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::domain::models::automation::AutomationLane;

/// Represents an audio effect that can be applied to tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub id: Uuid,
    pub name: String,
    pub effect_type: EffectType,
    pub parameters: EffectParameters,
    pub enabled: bool,
    pub automation_lanes: Vec<AutomationLane>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Reverb,
    Delay,
    Compressor,
    Equalizer,
    Distortion,
    Chorus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectParameters {
    pub reverb_size: Option<f32>,
    pub delay_time: Option<f32>,
    pub delay_feedback: Option<f32>,
    pub compression_ratio: Option<f32>,
    pub eq_bands: Option<Vec<(f32, f32)>>,
    pub distortion_gain: Option<f32>,
    pub chorus_rate: Option<f32>,
    pub chorus_depth: Option<f32>,
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
            compression_ratio: Some(4.0),
            eq_bands: Some(vec![(1000.0, 0.0), (5000.0, 0.0), (10000.0, 0.0)]),
            distortion_gain: Some(0.5),
            chorus_rate: Some(0.5),
            chorus_depth: Some(0.3),
        }
    }
}