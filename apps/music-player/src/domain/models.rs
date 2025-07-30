//! Core business models for the music player module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// A musical track with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub title: String,
    pub duration_ms: u64,
    pub media_cid: String,  // Content ID for p2p storage
    pub waveform_data_cid: Option<String>, // For visualizer
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Track {
    /// Create a new track
    pub fn new(
        artist_id: Uuid,
        title: String,
        duration_ms: u64,
        media_cid: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            artist_id,
            album_id: None,
            title,
            duration_ms,
            media_cid,
            waveform_data_cid: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the track's album
    pub fn set_album(&mut self, album_id: Uuid) {
        self.album_id = Some(album_id);
        self.updated_at = Utc::now();
    }

    /// Update the waveform data CID
    pub fn set_waveform_data(&mut self, waveform_data_cid: String) {
        self.waveform_data_cid = Some(waveform_data_cid);
        self.updated_at = Utc::now();
    }
}

/// A timestamped comment on a track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampedComment {
    pub id: Uuid,
    pub track_id: Uuid,
    pub user_id: Uuid,
    pub timestamp_ms: u64,  // Position in track where comment applies
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TimestampedComment {
    /// Create a new timestamped comment
    pub fn new(
        track_id: Uuid,
        user_id: Uuid,
        timestamp_ms: u64,
        content: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            track_id,
            user_id,
            timestamp_ms,
            content,
            created_at: now,
            updated_at: now,
        }
    }
}

/// A user's interaction with a track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrackInteraction {
    Like { 
        track_id: Uuid, 
        user_id: Uuid,
        created_at: DateTime<Utc> 
    },
    Repost { 
        track_id: Uuid, 
        user_id: Uuid,
        comment: Option<String>, 
        created_at: DateTime<Utc> 
    },
    FollowArtist { 
        artist_id: Uuid, 
        follower_id: Uuid,
        created_at: DateTime<Utc> 
    },
}

impl TrackInteraction {
    /// Create a new like interaction
    pub fn new_like(track_id: Uuid, user_id: Uuid) -> Self {
        Self::Like {
            track_id,
            user_id,
            created_at: Utc::now(),
        }
    }

    /// Create a new repost interaction
    pub fn new_repost(track_id: Uuid, user_id: Uuid, comment: Option<String>) -> Self {
        Self::Repost {
            track_id,
            user_id,
            comment,
            created_at: Utc::now(),
        }
    }

    /// Create a new follow artist interaction
    pub fn new_follow_artist(artist_id: Uuid, follower_id: Uuid) -> Self {
        Self::FollowArtist {
            artist_id,
            follower_id,
            created_at: Utc::now(),
        }
    }
}

/// Visualizer configuration and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizerPreset {
    pub id: Uuid,
    pub name: String,
    pub config: serde_json::Value,  // Custom visualizer settings
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

impl VisualizerPreset {
    /// Create a new visualizer preset
    pub fn new(name: String, config: serde_json::Value, is_default: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            config,
            is_default,
            created_at: Utc::now(),
        }
    }
}

/// Playlist containing tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub track_positions: Vec<PlaylistTrack>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Playlist {
    /// Create a new playlist
    pub fn new(
        owner_id: Uuid,
        title: String,
        description: Option<String>,
        is_public: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id,
            title,
            description,
            is_public,
            track_positions: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a track to the playlist
    pub fn add_track(&mut self, track_id: Uuid) {
        let position = self.track_positions.len() as u32;
        self.track_positions.push(PlaylistTrack { track_id, position });
        self.updated_at = Utc::now();
    }

    /// Remove a track from the playlist
    pub fn remove_track(&mut self, track_id: Uuid) {
        self.track_positions.retain(|pt| pt.track_id != track_id);
        // Re-index positions
        for (index, pt) in self.track_positions.iter_mut().enumerate() {
            pt.position = index as u32;
        }
        self.updated_at = Utc::now();
    }
}

/// Position of a track within a playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistTrack {
    pub track_id: Uuid,
    pub position: u32,
}

/// Data structure for visualizer waveform data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveformData {
    pub sample_rate: u32,
    pub duration_ms: u64,
    pub amplitudes: Vec<f32>,
}

/// Data structure for frequency analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyData {
    pub sample_rate: u32,
    pub bands: Vec<String>,
    pub frames: Vec<FrequencyFrame>,
}

/// A single frame of frequency data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyFrame {
    pub timestamp_ms: u64,
    pub values: Vec<f32>,
}

/// Download status for offline content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    InProgress { progress: f32 },
    Completed,
    Failed { error: String },
}

/// Download manifest for offline content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadManifest {
    pub track_id: Uuid,
    pub track_cid: String,
    pub media_cid: String,
    pub waveform_cid: Option<String>,
    pub metadata: serde_json::Value,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
}