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