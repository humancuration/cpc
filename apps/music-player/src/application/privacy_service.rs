//! Privacy service for handling user consent and data minimization

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::domain::errors::{Result, MusicPlayerError};
use crate::infrastructure::database::consent_repository::ConsentRepository;

/// Consent types for different operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsentType {
    Playback,
    Recommendations,
    Social,
    Following,
    OfflineDownload,
}

impl std::fmt::Display for ConsentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConsentType::Playback => write!(f, "playback"),
            ConsentType::Recommendations => write!(f, "recommendations"),
            ConsentType::Social => write!(f, "social"),
            ConsentType::Following => write!(f, "following"),
            ConsentType::OfflineDownload => write!(f, "offline_download"),
        }
    }
}

impl std::str::FromStr for ConsentType {
    type Err = MusicPlayerError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "playback" => Ok(ConsentType::Playback),
            "recommendations" => Ok(ConsentType::Recommendations),
            "social" => Ok(ConsentType::Social),
            "following" => Ok(ConsentType::Following),
            "offline_download" => Ok(ConsentType::OfflineDownload),
            _ => Err(MusicPlayerError::InvalidConsentType),
        }
    }
}

/// Status of user consent
#[derive(Debug, Clone)]
pub struct ConsentStatus {
    pub granted: bool,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Service for handling privacy consent and data minimization
pub struct PrivacyService {
    consent_repository: Arc<dyn ConsentRepository>,
    default_expiration: Duration,
}

impl PrivacyService {
    /// Create a new privacy service with repository dependency
    pub fn new(consent_repository: Arc<dyn ConsentRepository>) -> Self {
        Self {
            consent_repository,
            default_expiration: Duration::days(365), // 1 year default expiration
        }
    }
    
    /// Verify that a user has given valid consent for a specific operation
    pub async fn verify_consent(&self, user_id: Uuid, consent_type: ConsentType) -> Result<()> {
        let consent_status = self.consent_repository
            .get_consent(user_id, consent_type.clone())
            .await?
            .ok_or(MusicPlayerError::PermissionDenied { message: format!("Consent required for {}", consent_type) })?;
            
        if !consent_status.granted {
            return Err(MusicPlayerError::PermissionDenied { message: format!("Consent denied for {}", consent_type) });
        }
        
        if let Some(expires_at) = consent_status.expires_at {
            if expires_at < Utc::now() {
                return Err(MusicPlayerError::PermissionDenied { message: format!("Consent expired for {}", consent_type) });
            }
        }
        
        Ok(())
    }
    
    /// Record or update user consent
    pub async fn record_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType,
        granted: bool,
        expires_at: Option<DateTime<Utc>>
    ) -> Result<()> {
        let expires_at = expires_at.unwrap_or_else(|| Utc::now() + self.default_expiration);
        
        self.consent_repository
            .store_consent(user_id, consent_type, granted, Some(expires_at))
            .await?;
            
        Ok(())
    }
    
    /// Get all consents for a user
    pub async fn get_all_consents(&self, user_id: Uuid) -> Result<Vec<(ConsentType, ConsentStatus)>> {
        self.consent_repository.get_all_consents(user_id).await
    }
    
    /// Revoke all consents for a user (for account deletion)
    pub async fn revoke_all_consents(&self, user_id: Uuid) -> Result<()> {
        self.consent_repository.revoke_all_consents(user_id).await
    }
    
    /// Check if consent is required for a specific operation
    pub fn is_consent_required(&self, consent_type: ConsentType) -> bool {
        match consent_type {
            ConsentType::Playback => true,
            ConsentType::Recommendations => true,
            ConsentType::Social => true,
            ConsentType::Following => true,
            ConsentType::OfflineDownload => true,
        }
    }
    
    /// Apply data minimization to a list of tracks for anonymous users
    pub fn apply_data_minimization(&self, tracks: Vec<crate::domain::models::Track>) -> Vec<crate::domain::models::Track> {
        // For anonymous users, return only basic track info
        tracks.into_iter()
            .map(|track| {
                crate::domain::models::Track {
                    id: track.id,
                    title: track.title,
                    artist_id: track.artist_id,
                    duration_ms: track.duration_ms,
                    // Clear any personally identifiable information
                    album_id: None,
                    media_cid: track.media_cid,
                    waveform_data_cid: None,
                    created_at: track.created_at,
                    updated_at: track.updated_at,
                }
            })
            .collect()
    }
}