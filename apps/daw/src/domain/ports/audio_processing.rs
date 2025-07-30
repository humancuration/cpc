use crate::domain::models::{Track, Project};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioProcessingError {
    #[error("Audio processing failed: {0}")]
    ProcessingFailed(String),
    
    #[error("Invalid audio format")]
    InvalidFormat,
    
    #[error("Device error: {0}")]
    DeviceError(String),
}

#[async_trait]
pub trait AudioProcessingPort: Send + Sync {
    async fn process_audio(&self, input: &[f32]) -> Result<Vec<f32>, AudioProcessingError>;
    
    async fn process_track(&self, track: &Track) -> Result<Vec<f32>, AudioProcessingError>;
    
    async fn process_project(&self, project: &Project) -> Result<Vec<f32>, AudioProcessingError>;
    
    async fn play(&self) -> Result<(), AudioProcessingError>;
    
    async fn pause(&self) -> Result<(), AudioProcessingError>;
    
    async fn stop(&self) -> Result<(), AudioProcessingError>;
    
    async fn set_volume(&self, volume: f32) -> Result<(), AudioProcessingError>;
    
    async fn is_playing(&self) -> Result<bool, AudioProcessingError>;
    
    fn get_sample_rate(&self) -> u32;
    
    fn get_buffer_size(&self) -> usize;
}