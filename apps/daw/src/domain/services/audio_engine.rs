use crate::domain::models::{Track, Project, Effect};
use crate::domain::ports::audio_processing::{AudioProcessingPort, AudioProcessingError};
use crate::domain::services::automation_service::AutomationService;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioEngineError {
    #[error("Audio processing failed: {0}")]
    ProcessingError(#[from] AudioProcessingError),
    
    #[error("Track not found: {0}")]
    TrackNotFound(Uuid),
    
    #[error("Invalid audio parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Buffer overflow: attempted to use {actual} bytes, maximum is {max}")]
    BufferOverflow {
        actual: usize,
        max: usize,
    },
    
    #[error("Real-time constraint violated: {0}")]
    RealTimeConstraint(String),
}

/// Coordinates audio processing operations and provides high-level audio management
///
/// The AudioEngine serves as the domain service for all audio-related operations,
/// abstracting low-level audio processing details from the application layer.
pub struct AudioEngine {
    audio_processor: Arc<dyn AudioProcessingPort>,
    buffer_pool: Mutex<Vec<Vec<f32>>>,
    max_buffer_size: usize,
    automation_service: AutomationService,
}

impl AudioEngine {
    /// Creates a new AudioEngine instance
    /// 
    /// # Arguments
    /// * `audio_processor` - The underlying audio processing implementation
    /// * `max_buffer_size` - Maximum size for audio buffers in bytes
    pub fn new(audio_processor: Arc<dyn AudioProcessingPort>, max_buffer_size: usize) -> Self {
        Self {
            audio_processor,
            buffer_pool: Mutex::new(Vec::new()),
            max_buffer_size,
            automation_service: AutomationService::new(),
        }
    }

    /// Acquires a buffer from the pool or creates a new one
    async fn acquire_buffer(&self, size: usize) -> Result<Vec<f32>, AudioEngineError> {
        if size > self.max_buffer_size {
            return Err(AudioEngineError::BufferOverflow {
                actual: size,
                max: self.max_buffer_size,
            });
        }

        let mut pool = self.buffer_pool.lock().await;
        if let Some(mut buffer) = pool.pop() {
            buffer.clear();
            buffer.reserve(size);
            Ok(buffer)
        } else {
            Ok(Vec::with_capacity(size))
        }
    }

    /// Returns a buffer to the pool for reuse
    async fn release_buffer(&self, mut buffer: Vec<f32>) {
        let mut pool = self.buffer_pool.lock().await;
        if pool.len() < 16 {
            buffer.clear();
            pool.push(buffer);
        }
    }

    /// Performs real-time audio streaming with buffer management
    /// 
    /// # Arguments
    /// * `input` - Input audio buffer
    /// * `process_real_time` - Whether to enforce real-time constraints
    /// 
    /// # Returns
    /// Processed audio data ready for playback
    pub async fn stream_audio(
        &self,
        input: &[f32],
        process_real_time: bool,
    ) -> Result<Vec<f32>, AudioEngineError> {
        if process_real_time {
            let max_processing_time = std::time::Duration::from_millis(5); // 5ms for real-time
            let start = std::time::Instant::now();
            
            let result = self.audio_processor.process_audio(input).await?;
            
            let elapsed = start.elapsed();
            if elapsed > max_processing_time {
                return Err(AudioEngineError::RealTimeConstraint(
                    format!("Processing took {:?}, exceeded {:?} limit", elapsed, max_processing_time)
                ));
            }
            
            Ok(result)
        } else {
            self.audio_processor.process_audio(input).await.map_err(Into::into)
        }
    }

    /// Applies effects to a single track with automation
    ///
    /// # Arguments
    /// * `track` - The track to process
    /// * `effects` - All available effects in the project
    /// * `time` - Current playback time in samples
    /// * `real_time` - Whether to enforce real-time constraints
    ///
    /// # Returns
    /// Processed track audio data with effects and automation applied
    pub async fn process_track_with_effects(
        &self,
        track: &mut Track,
        effects: &mut [Effect],
        time: u64,
        real_time: bool,
    ) -> Result<Vec<f32>, AudioEngineError> {
        if track.muted {
            return Ok(Vec::new());
        }

        // Apply automation to track parameters
        self.automation_service.apply_track_automation(track, time);

        // Apply automation to effect parameters
        for effect_id in &track.effects {
            if let Some(effect) = effects.iter_mut().find(|e| &e.id == effect_id) {
                self.automation_service.apply_effect_automation(effect, time);
            }
        }

        let mut processed = if real_time {
            let buffer_size = track.audio_data.len();
            let mut buffer = self.acquire_buffer(buffer_size).await?;
            
            // Apply volume and pan with automation
            for (i, sample) in track.audio_data.iter().enumerate() {
                let adjusted_sample = sample * track.volume;
                if i < buffer.capacity() {
                    buffer.push(adjusted_sample);
                }
            }
            
            buffer
        } else {
            track.audio_data.iter().map(|s| s * track.volume).collect()
        };

        // Apply effects processing with automation (placeholder for now)
        // TODO: Implement actual effect chain processing
        
        if real_time {
            let result = self.stream_audio(&processed, true).await?;
            self.release_buffer(processed).await;
            Ok(result)
        } else {
            self.audio_processor.process_audio(&processed).await.map_err(Into::into)
        }
    }

    /// Mixes multiple tracks into a single audio stream with automation
    ///
    /// # Arguments
    /// * `tracks` - Collection of tracks to mix
    /// * `effects` - All available effects in the project
    /// * `time` - Current playback time in samples
    /// * `real_time` - Whether to enforce real-time constraints
    ///
    /// # Returns
    /// Mixed audio data with automation applied
    pub async fn mix_tracks(
        &self,
        tracks: &mut [Track],
        effects: &mut [Effect],
        time: u64,
        real_time: bool,
    ) -> Result<Vec<f32>, AudioEngineError> {
        if tracks.is_empty() {
            return Ok(Vec::new());
        }

        // Find the longest track length
        let max_length = tracks
            .iter()
            .map(|track| track.audio_data.len())
            .max()
            .unwrap_or(0);

        if max_length == 0 {
            return Ok(Vec::new());
        }

        let mut mixed = self.acquire_buffer(max_length).await?;
        mixed.resize(max_length, 0.0);

        // Mix tracks with volume and panning (including automation)
        for track in tracks.iter_mut() {
            if track.muted {
                continue;
            }

            let track_data = self.process_track_with_effects(track, effects, time, real_time).await?;
            
            for (i, (mix_sample, track_sample)) in mixed.iter_mut()
                .zip(track_data.iter().chain(std::iter::repeat(&0.0f32)))
                .enumerate()
            {
                if i < max_length {
                    *mix_sample += track_sample * track.volume;
                }
            }

            if real_time {
                self.release_buffer(track_data).await;
            }
        }

        // Normalize to prevent clipping
        let max_amplitude = mixed.iter().fold(0.0f32, |max, &sample| max.max(sample.abs()));
        if max_amplitude > 1.0 {
            for sample in &mut mixed {
                *sample /= max_amplitude;
            }
        }

        let result = if real_time {
            let processed = self.stream_audio(&mixed, true).await?;
            self.release_buffer(mixed).await;
            processed
        } else {
            self.audio_processor.process_audio(&mixed).await?
        };

        Ok(result)
    }

    /// Processes an entire project (all tracks) with automation
    ///
    /// # Arguments
    /// * `project` - The project to process
    /// * `time` - Current playback time in samples
    /// * `real_time` - Whether to enforce real-time constraints
    ///
    /// # Returns
    /// Final mixed audio data for the entire project with automation
    pub async fn process_project(
        &self,
        project: &mut Project,
        time: u64,
        real_time: bool,
    ) -> Result<Vec<f32>, AudioEngineError> {
        let mut tracks: Vec<&mut Track> = project.tracks.iter_mut().collect();
        let mut effects: Vec<&mut Effect> = project.effects.iter_mut().collect();
        
        // Process automation for this frame
        self.automation_service.process_automation_frame(
            &mut tracks.iter_mut().map(|t| &mut **t).collect::<Vec<_>>(),
            &mut effects.iter_mut().map(|e| &mut **e).collect::<Vec<_>>(),
            time,
        );

        // Convert back to immutable references for mixing
        let track_refs: Vec<&Track> = project.tracks.iter().collect();
        let effect_refs: Vec<&mut Effect> = project.effects.iter_mut().collect();
        
        // Create temporary copies for processing
        let mut processed_tracks: Vec<Track> = track_refs.iter().map(|t| (*t).clone()).collect();
        let mut processed_effects: Vec<Effect> = effect_refs.iter().map(|e| (*e).clone()).collect();
        
        self.mix_tracks(&mut processed_tracks, &mut processed_effects, time, real_time).await
    }

    /// Get the automation service for direct parameter control
    pub fn automation_service(&mut self) -> &mut AutomationService {
        &mut self.automation_service
    }

    /// Gets the current sample rate
    pub fn get_sample_rate(&self) -> u32 {
        self.audio_processor.get_sample_rate()
    }

    /// Gets the current buffer size
    pub fn get_buffer_size(&self) -> usize {
        self.audio_processor.get_buffer_size()
    }

    /// Preallocates buffers for real-time performance
    pub async fn preallocate_buffers(&self, count: usize, size: usize) -> Result<(), AudioEngineError> {
        if size > self.max_buffer_size {
            return Err(AudioEngineError::InvalidParameters(
                format!("Buffer size {} exceeds maximum {}", size, self.max_buffer_size)
            ));
        }

        let mut pool = self.buffer_pool.lock().await;
        pool.clear();
        
        for _ in 0..count {
            pool.push(Vec::with_capacity(size));
        }

        Ok(())
    }

    /// Clears all cached buffers
    pub async fn clear_buffers(&self) {
        let mut pool = self.buffer_pool.lock().await;
        pool.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::Project;

    struct MockAudioProcessor;
    
    #[async_trait]
    impl AudioProcessingPort for MockAudioProcessor {
        async fn process_audio(&self, input: &[f32]) -> Result<Vec<f32>, AudioProcessingError> {
            Ok(input.to_vec())
        }
        
        async fn process_track(&self, _track: &Track) -> Result<Vec<f32>, AudioProcessingError> {
            Ok(vec![1.0, 2.0, 3.0])
        }
        
        async fn process_project(&self, _project: &Project) -> Result<Vec<f32>, AudioProcessingError> {
            Ok(vec![1.0, 2.0, 3.0])
        }
        
        fn get_sample_rate(&self) -> u32 {
            44100
        }
        
        fn get_buffer_size(&self) -> usize {
            512
        }
    }

    #[tokio::test]
    async fn test_audio_engine_creation() {
        let processor = Arc::new(MockAudioProcessor);
        let engine = AudioEngine::new(processor, 1024 * 1024);
        assert_eq!(engine.get_sample_rate(), 44100);
    }

    #[tokio::test]
    async fn test_stream_audio() {
        let processor = Arc::new(MockAudioProcessor);
        let engine = AudioEngine::new(processor, 1024 * 1024);
        
        let input = vec![1.0, 2.0, 3.0];
        let result = engine.stream_audio(&input, false).await.unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }
}