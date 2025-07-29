use crate::domain::models::{Track, Effect};
use crate::domain::models::automation::{AutomationLane, automation_utils};
use uuid::Uuid;
use std::collections::HashMap;

/// Service for managing and applying automation in real-time
pub struct AutomationService {
    /// Cache for optimized parameter lookups
    parameter_cache: HashMap<String, f32>,
}

impl AutomationService {
    /// Create a new automation service
    pub fn new() -> Self {
        Self {
            parameter_cache: HashMap::new(),
        }
    }

    /// Apply automation to track parameters for a given time
    pub fn apply_track_automation(&mut self, track: &mut Track, time: u64) {
        for lane in &track.automation_lanes {
            if let Some(track_id) = lane.track_id {
                if track_id == track.id {
                    let value = lane.evaluate_at(time);
                    
                    match lane.parameter_id.as_str() {
                        "volume" => {
                            track.volume = automation_utils::scale_value(value, 0.0, 2.0); // 0% to 200%
                        }
                        "pan" => {
                            track.pan = automation_utils::scale_value(value, -1.0, 1.0); // Full left to full right
                        }
                        _ => {
                            // Custom parameter handling
                            self.parameter_cache.insert(
                                format!("{}:{}", track.id, lane.parameter_id),
                                value
                            );
                        }
                    }
                }
            }
        }
    }

    /// Apply automation to effect parameters for a given time
    pub fn apply_effect_automation(&mut self, effect: &mut Effect, time: u64) {
        for lane in &effect.automation_lanes {
            if let Some(effect_id) = lane.effect_id {
                if effect_id == effect.id {
                    let value = lane.evaluate_at(time);
                    
                    match effect.effect_type {
                        crate::domain::models::EffectType::Reverb => {
                            if lane.parameter_id == "size" {
                                effect.parameters.reverb_size = Some(automation_utils::scale_value(value, 0.0, 1.0));
                            }
                        }
                        crate::domain::models::EffectType::Delay => {
                            match lane.parameter_id.as_str() {
                                "time" => {
                                    effect.parameters.delay_time = Some(automation_utils::scale_value(value, 0.0, 2.0));
                                }
                                "feedback" => {
                                    effect.parameters.delay_feedback = Some(automation_utils::scale_value(value, 0.0, 0.99));
                                }
                                "mix" => {
                                    effect.parameters.delay_mix = Some(automation_utils::scale_value(value, 0.0, 1.0));
                                }
                                _ => {}
                            }
                        }
                        crate::domain::models::EffectType::Compressor => {
                            if lane.parameter_id == "ratio" {
                                effect.parameters.compression_ratio = Some(automation_utils::scale_value(value, 1.0, 20.0));
                            }
                        }
                        crate::domain::models::EffectType::Equalizer => {
                            if lane.parameter_id.starts_with("eq_") {
                                // Handle EQ band automation
                                let band_index: usize = lane.parameter_id[3..].parse().unwrap_or(0);
                                if let Some(ref mut bands) = effect.parameters.eq_bands {
                                    if band_index < bands.len() {
                                        bands[band_index].1 = automation_utils::scale_value(value, -12.0, 12.0);
                                    }
                                }
                            }
                        }
                        crate::domain::models::EffectType::Distortion => {
                            match lane.parameter_id.as_str() {
                                "gain" => {
                                    effect.parameters.distortion_gain = Some(automation_utils::scale_value(value, 0.0, 10.0));
                                }
                                "tone" => {
                                    effect.parameters.distortion_tone = Some(automation_utils::scale_value(value, 0.0, 1.0));
                                }
                                _ => {}
                            }
                        }
                        crate::domain::models::EffectType::Chorus => {
                            match lane.parameter_id.as_str() {
                                "rate" => {
                                    effect.parameters.chorus_rate = Some(automation_utils::scale_value(value, 0.1, 5.0));
                                }
                                "depth" => {
                                    effect.parameters.chorus_depth = Some(automation_utils::scale_value(value, 0.0, 1.0));
                                }
                                "mix" => {
                                    effect.parameters.chorus_mix = Some(automation_utils::scale_value(value, 0.0, 1.0));
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            // Custom parameter handling
                            self.parameter_cache.insert(
                                format!("{}:{}", effect.id, lane.parameter_id),
                                value
                            );
                        }
                    }
                }
            }
        }
    }

    /// Get automation value for a specific parameter
    pub fn get_parameter_value(&self, id: &str, default: f32) -> f32 {
        self.parameter_cache.get(id).copied().unwrap_or(default)
    }

    /// Clear the parameter cache
    pub fn clear_cache(&mut self) {
        self.parameter_cache.clear();
    }

    /// Create default automation lanes for a track
    pub fn create_default_track_lanes(track_id: Uuid) -> Vec<AutomationLane> {
        vec![
            AutomationLane::for_track("volume".to_string(), track_id),
            AutomationLane::for_track("pan".to_string(), track_id),
        ]
    }

    /// Create default automation lanes for an effect
    pub fn create_default_effect_lanes(effect_id: Uuid, effect_type: crate::domain::models::EffectType) -> Vec<AutomationLane> {
        match effect_type {
            crate::domain::models::EffectType::Reverb => {
                vec![
                    AutomationLane::for_effect("size".to_string(), effect_id),
                ]
            }
            crate::domain::models::EffectType::Delay => {
                vec![
                    AutomationLane::for_effect("time".to_string(), effect_id),
                    AutomationLane::for_effect("feedback".to_string(), effect_id),
                    AutomationLane::for_effect("mix".to_string(), effect_id),
                ]
            }
            crate::domain::models::EffectType::Compressor => {
                vec![
                    AutomationLane::for_effect("ratio".to_string(), effect_id),
                ]
            }
            crate::domain::models::EffectType::Equalizer => {
                vec![
                    AutomationLane::for_effect("band0".to_string(), effect_id),
                    AutomationLane::for_effect("band1".to_string(), effect_id),
                    AutomationLane::for_effect("band2".to_string(), effect_id),
                ]
            }
            crate::domain::models::EffectType::Distortion => {
                vec![
                    AutomationLane::for_effect("gain".to_string(), effect_id),
                ]
            }
            crate::domain::models::EffectType::Chorus => {
                vec![
                    AutomationLane::for_effect("rate".to_string(), effect_id),
                    AutomationLane::for_effect("depth".to_string(), effect_id),
                    AutomationLane::for_effect("mix".to_string(), effect_id),
                ]
            }
        }
    }

    /// Process automation for multiple tracks and effects at a given time
    pub fn process_automation_frame(
        &mut self,
        tracks: &mut [Track],
        effects: &mut [Effect],
        time: u64,
    ) {
        // Process track automation
        for track in tracks.iter_mut() {
            self.apply_track_automation(track, time);
        }

        // Process effect automation
        for effect in effects.iter_mut() {
            self.apply_effect_automation(effect, time);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{Track, TrackType, Effect, EffectType};

    #[test]
    fn test_track_automation() {
        let mut service = AutomationService::new();
        let mut track = Track::new_audio("Test Track".to_string());
        
        // Add volume automation
        let mut lane = AutomationLane::for_track("volume".to_string(), track.id);
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(0, 0.5)).unwrap();
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(1000, 1.0)).unwrap();
        track.automation_lanes.push(lane);

        // Apply automation
        service.apply_track_automation(&mut track, 500);
        assert!((track.volume - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_reverb_automation() {
        let mut service = AutomationService::new();
        let mut effect = Effect::new("Test Reverb".to_string(), EffectType::Reverb);
        
        // Add size automation
        let mut lane = AutomationLane::for_effect("size".to_string(), effect.id);
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(0, 0.0)).unwrap();
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(1000, 1.0)).unwrap();
        effect.automation_lanes.push(lane);

        // Apply automation
        service.apply_effect_automation(&mut effect, 500);
        assert!((effect.parameters.reverb_size.unwrap() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_delay_automation() {
        let mut service = AutomationService::new();
        let mut effect = Effect::new("Test Delay".to_string(), EffectType::Delay);
        
        // Add time automation
        let mut lane = AutomationLane::for_effect("time".to_string(), effect.id);
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(0, 0.0)).unwrap();
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(1000, 1.0)).unwrap();
        effect.automation_lanes.push(lane);
        
        // Apply automation
        service.apply_effect_automation(&mut effect, 500);
        assert!((effect.parameters.delay_time.unwrap() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_distortion_automation() {
        let mut service = AutomationService::new();
        let mut effect = Effect::new("Test Distortion".to_string(), EffectType::Distortion);
        
        // Add gain automation
        let mut lane = AutomationLane::for_effect("gain".to_string(), effect.id);
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(0, 0.0)).unwrap();
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(1000, 1.0)).unwrap();
        effect.automation_lanes.push(lane);
        
        // Apply automation
        service.apply_effect_automation(&mut effect, 500);
        assert!((effect.parameters.distortion_gain.unwrap() - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_chorus_automation() {
        let mut service = AutomationService::new();
        let mut effect = Effect::new("Test Chorus".to_string(), EffectType::Chorus);
        
        // Add rate automation
        let mut lane = AutomationLane::for_effect("rate".to_string(), effect.id);
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(0, 0.0)).unwrap();
        lane.add_point(crate::domain::models::automation::AutomationPoint::new(1000, 1.0)).unwrap();
        effect.automation_lanes.push(lane);
        
        // Apply automation
        service.apply_effect_automation(&mut effect, 500);
        assert!((effect.parameters.chorus_rate.unwrap() - 2.55).abs() < 0.01);
    }

    #[test]
    fn test_parameter_cache() {
        let mut service = AutomationService::new();
        
        // Test cache operations
        service.parameter_cache.insert("test".to_string(), 0.75);
        assert_eq!(service.get_parameter_value("test", 0.5), 0.75);
        assert_eq!(service.get_parameter_value("missing", 0.5), 0.5);
        
        service.clear_cache();
        assert_eq!(service.get_parameter_value("test", 0.5), 0.5);
    }

    #[test]
    fn test_default_lanes() {
        let track_id = Uuid::new_v4();
        let effect_id = Uuid::new_v4();
        
        let track_lanes = AutomationService::create_default_track_lanes(track_id);
        assert_eq!(track_lanes.len(), 2);
        assert!(track_lanes.iter().any(|l| l.parameter_id == "volume"));
        
        let effect_lanes = AutomationService::create_default_effect_lanes(effect_id, EffectType::Delay);
        assert_eq!(effect_lanes.len(), 3);
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "time"));
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "mix"));

        let effect_lanes = AutomationService::create_default_effect_lanes(effect_id, EffectType::Chorus);
        assert_eq!(effect_lanes.len(), 3);
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "rate"));
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "mix"));

        let effect_lanes = AutomationService::create_default_effect_lanes(effect_id, EffectType::Distortion);
        assert_eq!(effect_lanes.len(), 1);
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "gain"));

        let effect_lanes = AutomationService::create_default_effect_lanes(effect_id, EffectType::Compressor);
        assert_eq!(effect_lanes.len(), 1);
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "ratio"));

        let effect_lanes = AutomationService::create_default_effect_lanes(effect_id, EffectType::Equalizer);
        assert_eq!(effect_lanes.len(), 3);
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "band0"));

        let effect_lanes = AutomationService::create_default_effect_lanes(effect_id, EffectType::Reverb);
        assert_eq!(effect_lanes.len(), 1);
        assert!(effect_lanes.iter().any(|l| l.parameter_id == "size"));
    }
}