//! Custom error types for the music player module

use thiserror::Error;
use crate::application::privacy_service::ConsentType;

/// Custom error type for music player operations
#[derive(Error, Debug)]
pub enum MusicPlayerError {
    #[error("Track not found: {id}")]
    TrackNotFound { id: String },

    #[error("User not found: {id}")]
    UserNotFound { id: String },

    #[error("Invalid timestamp: {timestamp_ms}")]
    InvalidTimestamp { timestamp_ms: u64 },

    #[error("Database error: {source}")]
    DatabaseError {
        #[from]
        source: sqlx::Error,
    },

    #[error("P2P network error: {message}")]
    P2PError { message: String },

    #[error("Audio processing error: {message}")]
    AudioProcessingError { message: String },

    #[error("Invalid input: {message}")]
    InvalidInput { message: String },

    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },

    #[error("Consent required for {0}")]
    ConsentRequired(ConsentType),

    #[error("Consent expired for {0}")]
    ConsentExpired(ConsentType),

    #[error("Invalid consent type")]
    InvalidConsentType,

    #[error("Download limit exceeded")]
    DownloadLimitExceeded,

    #[error("Storage limit exceeded")]
    StorageLimitExceeded,

    #[error("Visualizer data not available")]
    VisualizerDataNotAvailable,

    #[error("Playlist not found: {id}")]
    PlaylistNotFound { id: String },

    #[error("Content not available: {cid}")]
    ContentNotAvailable { cid: String },
}

/// Result type alias for music player operations
pub type Result<T> = std::result::Result<T, MusicPlayerError>;