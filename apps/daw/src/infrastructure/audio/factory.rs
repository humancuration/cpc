use super::RodioEngine;
use crate::domain::ports::audio_processing::{AudioProcessingPort, AudioProcessingError};

/// Factory for creating audio processing instances
pub struct AudioEngineFactory;

impl AudioEngineFactory {
    /// Creates a new Rodio audio engine with default settings
    pub fn create_rodio_engine() -> Result<Box<dyn AudioProcessingPort>, AudioProcessingError> {
        let engine = RodioEngine::new(44100, 512)?;
        Ok(Box::new(engine))
    }

    /// Creates a new Rodio audio engine with custom settings
    pub fn create_rodio_engine_with_settings(
        sample_rate: u32,
        buffer_size: usize,
    ) -> Result<Box<dyn AudioProcessingPort>, AudioProcessingError> {
        let engine = RodioEngine::new(sample_rate, buffer_size)?;
        Ok(Box::new(engine))
    }
}