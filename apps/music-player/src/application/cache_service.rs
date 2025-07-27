//! Cache service for offline music player functionality

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::models::{DownloadStatus, DownloadManifest};
use crate::domain::errors::{Result, MusicPlayerError};
use crate::infrastructure::database::CacheRepository;
use crate::application::privacy_service::{PrivacyService, ConsentType};

/// Service for cache/offline functionality
pub struct CacheService {
    cache_repository: Arc<CacheRepository>,
    privacy_service: Arc<PrivacyService>,
}

impl CacheService {
    /// Create a new cache service
    pub fn new(
        cache_repository: Arc<CacheRepository>,
        privacy_service: Arc<PrivacyService>,
    ) -> Self {
        Self {
            cache_repository,
            privacy_service,
        }
    }

    /// Initiate download of a track for offline use
    pub async fn initiate_download(&self, track_id: Uuid, user_id: Uuid) -> Result<DownloadStatus> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        // Check if user has available storage
        let user_storage = self.cache_repository.get_user_storage_usage(user_id).await?;
        let user_limit = self.cache_repository.get_user_storage_limit(user_id).await?;
        
        if user_storage >= user_limit {
            return Err(MusicPlayerError::StorageLimitExceeded);
        }
        
        // Create download record
        self.cache_repository.create_download_record(track_id, user_id).await?;
        Ok(DownloadStatus::Pending)
    }

    /// Get download status for a track
    pub async fn get_download_status(&self, track_id: Uuid, user_id: Uuid) -> Result<DownloadStatus> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.get_download_status(track_id, user_id).await
    }

    /// List all available offline tracks for a user
    pub async fn list_available_offline_tracks(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.list_user_downloaded_tracks(user_id).await
    }

    /// Purge old downloads to free up storage space
    pub async fn purge_old_downloads(&self, user_id: Uuid) -> Result<()> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.purge_old_downloads(user_id).await
    }

    /// Update download progress
    pub async fn update_download_progress(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        progress: f32,
    ) -> Result<()> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.update_download_progress(track_id, user_id, progress).await
    }

    /// Mark download as completed
    pub async fn complete_download(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        manifest: DownloadManifest,
    ) -> Result<()> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.complete_download(track_id, user_id, manifest).await
    }

    /// Mark download as failed
    pub async fn fail_download(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        error: String,
    ) -> Result<()> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.fail_download(track_id, user_id, error).await
    }

    /// Get user's storage usage
    pub async fn get_storage_usage(&self, user_id: Uuid) -> Result<u64> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.get_user_storage_usage(user_id).await
    }

    /// Get user's storage limit
    pub async fn get_storage_limit(&self, user_id: Uuid) -> Result<u64> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.get_user_storage_limit(user_id).await
    }

    /// Set user's storage limit
    pub async fn set_storage_limit(&self, user_id: Uuid, limit_bytes: u64) -> Result<()> {
        // Verify consent for offline download
        self.privacy_service
            .verify_consent(user_id, ConsentType::OfflineDownload)
            .await?;
            
        self.cache_repository.set_user_storage_limit(user_id, limit_bytes).await
    }
}