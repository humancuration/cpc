use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Represents the master mix output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mix {
    pub master_volume: f32,
    pub master_pan: f32,
    pub sample_rate: u32,
    pub bit_depth: u32,
}

impl Mix {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            master_volume: 1.0,
            master_pan: 0.0,
            sample_rate,
            bit_depth: 24,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_creation() {
        let mix = Mix::new(44100);
        assert_eq!(mix.master_volume, 1.0);
        assert_eq!(mix.master_pan, 0.0);
        assert_eq!(mix.sample_rate, 44100);
        assert_eq!(mix.bit_depth, 24);
    }

    #[test]
    fn test_serialization() {
        let mix = Mix::new(48000);
        let serialized = serde_json::to_string(&mix).unwrap();
        let deserialized: Mix = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(mix.master_volume, deserialized.master_volume);
        assert_eq!(mix.master_pan, deserialized.master_pan);
        assert_eq!(mix.sample_rate, deserialized.sample_rate);
        assert_eq!(mix.bit_depth, deserialized.bit_depth);
    }
}