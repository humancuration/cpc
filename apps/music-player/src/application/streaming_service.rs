//! Streaming service for music player functionality

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::models::{Track, WaveformData, FrequencyData, DownloadManifest};
use crate::domain::errors::{Result, MusicPlayerError};
use crate::infrastructure::p2p::P2PStreamManager;
use crate::infrastructure::database::TrackRepository;
use crate::infrastructure::audio_processor::AudioProcessor;
use crate::application::privacy_service::{PrivacyService, ConsentType};
/// Service for streaming music tracks
pub struct StreamingService {
    track_repository: Arc<TrackRepository>,
    p2p_manager: Arc<P2PStreamManager>,
    audio_processor: Arc<AudioProcessor>,
    privacy_service: Arc<PrivacyService>,
}
}

impl StreamingService {
    /// Create a new streaming service
    pub fn new(
        track_repository: Arc<TrackRepository>,
        p2p_manager: Arc<P2PStreamManager>,
        audio_processor: Arc<AudioProcessor>,
        privacy_service: Arc<PrivacyService>,
    ) -> Self {
        Self {
            track_repository,
            p2p_manager,
            audio_processor,
            privacy_service,
        }
    }

    /// Get a streaming URL for a track
    pub async fn get_stream_url(&self, track_id: Uuid) -> Result<String> {
        let track = self.track_repository.find_by_id(track_id).await?;
        self.p2p_manager.get_stream_url(&track.media_cid).await
    }

    /// Get visualizer data for a track
    pub async fn get_visualizer_data(&self, track_id: Uuid) -> Result<WaveformData> {
        let track = self.track_repository.find_by_id(track_id).await?;
        
        if let Some(waveform_cid) = &track.waveform_data_cid {
            // Try to get from P2P network first
            match self.p2p_manager.get_visualizer_data(waveform_cid).await {
                Ok(data) => return Ok(data),
                Err(_) => {
                    // If P2P fails, fall back to generating it
                    return self.generate_visualizer_data(track_id).await;
                }
            }
        }
        
        // If no waveform data exists, generate it
        self.generate_visualizer_data(track_id).await
    }

    /// Generate visualizer data for a track
    pub async fn generate_visualizer_data(&self, track_id: Uuid) -> Result<WaveformData> {
        let track = self.track_repository.find_by_id(track_id).await?;
        let waveform_data = self.audio_processor.extract_waveform_data(&track.media_cid).await?;
        
        // Store the waveform data CID for future use
        let waveform_cid = self.p2p_manager.store_visualizer_data(&waveform_data).await?;
        self.track_repository.update_waveform_data(track_id, waveform_cid).await?;
        
        Ok(waveform_data)
    }

    /// Prepare content for offline download
    pub async fn prepare_offline_download(&self, user_id: Uuid, track_id: Uuid) -> Result<DownloadManifest> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        let track = self.track_repository.find_by_id(track_id).await?;
        
        // Get waveform data if available
        let waveform_cid = if let Some(waveform_data_cid) = &track.waveform_data_cid {
            Some(waveform_data_cid.clone())
        } else {
            None
        };
        
        let manifest = DownloadManifest {
            track_id: track.id,
            track_cid: track.id.to_string(),
            media_cid: track.media_cid.clone(),
            waveform_cid,
            metadata: serde_json::json!({
                "title": track.title,
                "duration_ms": track.duration_ms,
                "artist_id": track.artist_id,
            }),
            size_bytes: 0, // This would be calculated based on actual content size
            created_at: chrono::Utc::now(),
        };
        
        Ok(manifest)
    }

    /// Get recommended tracks for a user
    pub async fn get_recommended_tracks(&self, user_id: Option<Uuid>) -> Result<Vec<Track>> {
        // This is a simplified implementation
        // In a real system, this would use collaborative filtering or other recommendation algorithms
        if let Some(user_id) = user_id {
            // Verify explicit consent for recommendation data
            self.privacy_service
                .verify_consent(user_id, ConsentType::Recommendations)
                .await?;
                
            // Return tracks based on user preferences
            self.track_repository.find_popular_tracks(10).await
        } else {
            // Return popular tracks for anonymous users with data minimization
            let tracks = self.track_repository.find_popular_tracks(10).await?;
            Ok(self.privacy_service.apply_data_minimization(tracks))
        }
    }

    /// Get track by ID
    pub async fn get_track(&self, track_id: Uuid) -> Result<Track> {
        self.track_repository.find_by_id(track_id).await
    }

    /// Search tracks by title
    pub async fn search_tracks(&self, query: &str) -> Result<Vec<Track>> {
        self.track_repository.search_by_title(query).await
    }
}