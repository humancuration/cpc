use crate::domain::ports::audio_processing::{AudioProcessingPort, AudioProcessingError};
use crate::domain::models::{Track, Project};
use async_trait::async_trait;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

/// Basic audio effects implementation
mod effects {
    /// Simple reverb effect using comb filters
    pub struct Reverb {
        delay_buffer: Vec<f32>,
        delay_index: usize,
        decay: f32,
        delay_time: usize,
    }

    impl Reverb {
        pub fn new(sample_rate: u32, delay_ms: f32, decay: f32) -> Self {
            let delay_samples = (sample_rate as f32 * delay_ms / 1000.0) as usize;
            Self {
                delay_buffer: vec![0.0; delay_samples],
                delay_index: 0,
                decay,
                delay_time: delay_samples,
            }
        }

        pub fn process(&mut self, input: f32) -> f32 {
            let delayed_sample = self.delay_buffer[self.delay_index];
            let output = input + delayed_sample * self.decay;
            
            self.delay_buffer[self.delay_index] = output;
            self.delay_index = (self.delay_index + 1) % self.delay_time;
            
            output
        }
    }

    /// Simple delay effect
    pub struct Delay {
        delay_buffer: Vec<f32>,
        write_index: usize,
        delay_samples: usize,
        feedback: f32,
        mix: f32,
    }

    impl Delay {
        pub fn new(sample_rate: u32, delay_ms: f32, feedback: f32, mix: f32) -> Self {
            let delay_samples = (sample_rate as f32 * delay_ms / 1000.0) as usize;
            Self {
                delay_buffer: vec![0.0; delay_samples],
                write_index: 0,
                delay_samples,
                feedback,
                mix,
            }
        }

        pub fn process(&mut self, input: f32) -> f32 {
            let delayed_sample = self.delay_buffer[self.write_index];
            let output = input + delayed_sample * self.mix;
            
            let feedback_sample = input + delayed_sample * self.feedback;
            self.delay_buffer[self.write_index] = feedback_sample;
            
            self.write_index = (self.write_index + 1) % self.delay_samples;
            
            output
        }
    }

    /// Simple 3-band EQ (low, mid, high)
    pub struct Equalizer {
        pub low_gain: f32,
        pub mid_gain: f32,
        pub high_gain: f32,
        low_freq: f32,
        high_freq: f32,
        sample_rate: u32,
        y1_l: f32,
        y1_h: f32,
    }

    impl Equalizer {
        pub fn new(sample_rate: u32, low_gain: f32, mid_gain: f32, high_gain: f32) -> Self {
            Self {
                low_gain,
                mid_gain,
                high_gain,
                low_freq: 250.0,
                high_freq: 4000.0,
                sample_rate,
                y1_l: 0.0,
                y1_h: 0.0,
            }
        }

        pub fn process(&mut self, input: f32) -> f32 {
            let low = self.low_gain * self.low_pass_filter(input);
            let mid = self.mid_gain * self.band_pass_filter(input);
            let high = self.high_gain * self.high_pass_filter(input);
            
            low + mid + high
        }

        fn low_pass_filter(&mut self, input: f32) -> f32 {
            let alpha = 1.0 / (2.0 * std::f32::consts::PI * self.low_freq / self.sample_rate as f32 + 1.0);
            let output = alpha * input + (1.0 - alpha) * self.y1_l;
            self.y1_l = output;
            output
        }

        fn high_pass_filter(&mut self, input: f32) -> f32 {
            let alpha = 2.0 * std::f32::consts::PI * self.high_freq / self.sample_rate as f32;
            let output = alpha / (alpha + 1.0) * (input - self.y1_h) + alpha / (alpha + 1.0) * self.y1_h;
            self.y1_h = output;
            output
        }

        fn band_pass_filter(&mut self, input: f32) -> f32 {
            let low = self.low_pass_filter(input);
            let high = self.high_pass_filter(input);
            input - low - high
        }
    }
}

/// Real-time audio buffer for streaming
#[derive(Clone)]
pub struct AudioBuffer {
    buffer: Vec<f32>,
    sample_rate: u32,
    channels: u16,
    position: usize,
}

impl AudioBuffer {
    pub fn new(buffer: Vec<f32>, sample_rate: u32, channels: u16) -> Self {
        Self {
            buffer,
            sample_rate,
            channels,
            position: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.position >= self.buffer.len()
    }
}

impl Iterator for AudioBuffer {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.buffer.len() {
            let sample = self.buffer[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl rodio::Source for AudioBuffer {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.buffer.len().saturating_sub(self.position))
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        let samples_remaining = (self.buffer.len().saturating_sub(self.position)) as u64;
        let samples_per_second = self.sample_rate as u64 * self.channels as u64;
        if samples_per_second > 0 {
            Some(Duration::from_secs_f64(samples_remaining as f64 / samples_per_second as f64))
        } else {
            None
        }
    }
}

/// Rodio audio engine with real-time processing capabilities
pub struct RodioEngine {
    sample_rate: u32,
    buffer_size: usize,
    _stream: OutputStream,
    sink: Sink,
}

impl RodioEngine {
    pub fn new(sample_rate: u32, buffer_size: usize) -> Result<Self, AudioProcessingError> {
        let (_stream, stream_handle) = OutputStream::try_default()
            .map_err(|e| AudioProcessingError::DeviceError(e.to_string()))?;
        
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AudioProcessingError::DeviceError(e.to_string()))?;
        
        // Set target volume to prevent clipping
        sink.set_volume(0.8);
        
        Ok(Self {
            sample_rate,
            buffer_size,
            _stream,
            sink,
        })
    }

    /// Process a single track with effects
    fn process_single_track(&self, track: &Track) -> Result<Vec<f32>, AudioProcessingError> {
        let mut samples = track.audio_data.clone();
        
        // Ensure we have the right buffer size
        if samples.len() != self.buffer_size {
            samples.resize(self.buffer_size, 0.0);
        }
        
        // Apply volume
        for sample in &mut samples {
            *sample *= track.volume;
        }
        
        // Apply panning (simple stereo)
        let pan = track.pan.min(1.0).max(-1.0);
        let left_gain = (1.0 - pan).sqrt();
        let right_gain = (1.0 + pan).sqrt();
        
        // Interleave for stereo
        let mut stereo_samples = Vec::with_capacity(samples.len() * 2);
        for sample in samples {
            stereo_samples.push(sample * left_gain);
            stereo_samples.push(sample * right_gain);
        }
        
        Ok(stereo_samples)
    }

    /// Mix multiple track buffers together
    fn mix_track_buffers(&self, buffers: &[Vec<f32>]) -> Vec<f32> {
        if buffers.is_empty() {
            return vec![0.0; self.buffer_size * 2]; // Stereo silence
        }
        
        let max_length = buffers.iter().map(|b| b.len()).max().unwrap_or(0);
        let mut mixed = vec![0.0; max_length];
        
        for buffer in buffers {
            for (i, &sample) in buffer.iter().enumerate() {
                if i < mixed.len() {
                    mixed[i] += sample;
                }
            }
        }
        
        // Normalize to prevent clipping
        let max_amplitude = mixed.iter().map(|&s| s.abs()).fold(0.0, f32::max);
        if max_amplitude > 1.0 {
            let scale = 1.0 / max_amplitude;
            for sample in &mut mixed {
                *sample *= scale;
            }
        }
        
        mixed
    }
}

#[async_trait]
impl AudioProcessingPort for RodioEngine {
    async fn process_audio(&self, input: &[f32]) -> Result<Vec<f32>, AudioProcessingError> {
        let mut samples = input.to_vec();
        
        // Ensure we have the right buffer size
        if samples.len() != self.buffer_size {
            samples.resize(self.buffer_size, 0.0);
        }
        
        Ok(samples)
    }

    async fn process_track(&self, track: &Track) -> Result<Vec<f32>, AudioProcessingError> {
        self.process_single_track(track)
    }

    async fn process_project(&self, project: &Project) -> Result<Vec<f32>, AudioProcessingError> {
        let mut track_buffers = Vec::new();
        
        for track in &project.tracks {
            if !track.muted {
                let track_buffer = self.process_single_track(&track)?;
                track_buffers.push(track_buffer);
            }
        }
        
        let mixed_buffer = self.mix_track_buffers(&track_buffers);
        
        // Play the mixed audio
        let audio_buffer = AudioBuffer::new(mixed_buffer.clone(), self.sample_rate, 2);
        self.sink.append(audio_buffer);
        
        Ok(mixed_buffer)
    }

    async fn play(&self) -> Result<(), AudioProcessingError> {
        self.sink.play();
        Ok(())
    }

    async fn pause(&self) -> Result<(), AudioProcessingError> {
        self.sink.pause();
        Ok(())
    }

    async fn stop(&self) -> Result<(), AudioProcessingError> {
        self.sink.stop();
        Ok(())
    }

    async fn set_volume(&self, volume: f32) -> Result<(), AudioProcessingError> {
        self.sink.set_volume(volume.max(0.0).min(1.0));
        Ok(())
    }

    async fn is_playing(&self) -> Result<bool, AudioProcessingError> {
        Ok(!self.sink.is_paused() && !self.sink.empty())
    }

    fn get_sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn get_buffer_size(&self) -> usize {
        self.buffer_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{Track, Project};

    #[test]
    fn test_audio_buffer_creation() {
        let buffer = vec![1.0, 2.0, 3.0];
        let audio_buffer = AudioBuffer::new(buffer.clone(), 44100, 2);
        
        assert_eq!(audio_buffer.channels(), 2);
        assert_eq!(audio_buffer.sample_rate(), 44100);
    }

    #[test]
    fn test_mix_track_buffers_empty() {
        let engine = RodioEngine::new(44100, 512).unwrap();
        let mixed = engine.mix_track_buffers(&[]);
        assert_eq!(mixed.len(), 1024); // buffer_size * 2 (stereo)
    }

    #[test]
    fn test_mix_track_buffers_normalization() {
        let engine = RodioEngine::new(44100, 512).unwrap();
        let buffers = vec![vec![2.0, 2.0, 2.0]; 3];
        let mixed = engine.mix_track_buffers(&buffers);
        
        // Should be normalized to prevent clipping
        let max_amplitude = mixed.iter().fold(0.0, |max, &s| s.abs().max(max));
        assert!(max_amplitude <= 1.0);
    }

    #[test]
    fn test_process_single_track() {
        let engine = RodioEngine::new(44100, 512).unwrap();
        let track = Track {
            id: "test".to_string(),
            name: "Test Track".to_string(),
            audio_data: vec![0.5; 512],
            volume: 0.8,
            pan: 0.0,
            muted: false,
        };
        
        let processed = engine.process_single_track(&track).unwrap();
        assert_eq!(processed.len(), 1024); // Stereo output
    }

    #[tokio::test]
    async fn test_process_project() {
        let engine = RodioEngine::new(44100, 512).unwrap();
        let project = Project {
            id: "test".to_string(),
            name: "Test Project".to_string(),
            tracks: vec![
                Track {
                    id: "track1".to_string(),
                    name: "Track 1".to_string(),
                    audio_data: vec![0.5; 512],
                    volume: 0.8,
                    pan: -0.5,
                    muted: false,
                },
                Track {
                    id: "track2".to_string(),
                    name: "Track 2".to_string(),
                    audio_data: vec![0.3; 512],
                    volume: 0.6,
                    pan: 0.5,
                    muted: false,
                },
            ],
        };
        
        let result = engine.process_project(&project).await;
        assert!(result.is_ok());
    }
}