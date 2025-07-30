//! Visualizer service for music player functionality

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::models::{VisualizerPreset, WaveformData, FrequencyData};
use crate::domain::errors::{Result, MusicPlayerError};
use crate::infrastructure::database::VisualizerRepository;
use crate::infrastructure::audio_processor::AudioProcessor;

/// Service for visualizer functionality
pub struct VisualizerService {
    visualizer_repository: Arc<VisualizerRepository>,
    audio_processor: Arc<AudioProcessor>,
}

impl VisualizerService {
    /// Create a new visualizer service
    pub fn new(
        visualizer_repository: Arc<VisualizerRepository>,
        audio_processor: Arc<AudioProcessor>,
    ) -> Self {
        Self {
            visualizer_repository,
            audio_processor,
        }
    }

    /// Generate waveform data for a track
    pub async fn generate_waveform_data(&self, track_media_cid: &str) -> Result<WaveformData> {
        self.audio_processor.extract_waveform_data(track_media_cid).await
    }

    /// Generate frequency analysis data for a track
    pub async fn generate_frequency_data(&self, track_media_cid: &str) -> Result<FrequencyData> {
        self.audio_processor.extract_frequency_data(track_media_cid).await
    }

    /// Get all visualizer presets
    pub async fn get_visualizer_presets(&self) -> Result<Vec<VisualizerPreset>> {
        self.visualizer_repository.find_all_presets().await
    }

    /// Get a specific visualizer preset by ID
    pub async fn get_visualizer_preset(&self, preset_id: Uuid) -> Result<VisualizerPreset> {
        self.visualizer_repository.find_preset_by_id(preset_id).await
    }

    /// Create a new visualizer preset
    pub async fn create_visualizer_preset(
        &self,
        name: String,
        config: serde_json::Value,
        is_default: bool,
    ) -> Result<VisualizerPreset> {
        let preset = VisualizerPreset::new(name, config, is_default);
        self.visualizer_repository.create_preset(&preset).await?;
        Ok(preset)
    }

    /// Update an existing visualizer preset
    pub async fn update_visualizer_preset(
        &self,
        preset_id: Uuid,
        name: Option<String>,
        config: Option<serde_json::Value>,
        is_default: Option<bool>,
    ) -> Result<VisualizerPreset> {
        self.visualizer_repository.update_preset(preset_id, name, config, is_default).await
    }

    /// Delete a visualizer preset
    pub async fn delete_visualizer_preset(&self, preset_id: Uuid) -> Result<()> {
        self.visualizer_repository.delete_preset(preset_id).await
    }

    /// Get the default visualizer preset
    pub async fn get_default_preset(&self) -> Result<VisualizerPreset> {
        self.visualizer_repository.find_default_preset().await
    }

    /// Apply a visualizer preset to a track
    pub async fn apply_visualizer_preset(&self, track_id: Uuid, preset_id: Uuid) -> Result<()> {
        // In a real implementation, this would store the association between track and preset
        // For now, we'll just validate that both exist
        let _preset = self.visualizer_repository.find_preset_by_id(preset_id).await?;
        // Track validation would happen in the repository or streaming service
        Ok(())
    }
}