//! GraphQL input/output types for the music player module

use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Input type for creating a track
#[derive(InputObject, Serialize, Deserialize)]
pub struct CreateTrackInput {
    pub artist_id: Uuid,
    pub title: String,
    pub duration_ms: u64,
    pub media_cid: String,
    pub album_id: Option<Uuid>,
}

/// Input type for updating a track
#[derive(InputObject, Serialize, Deserialize)]
pub struct UpdateTrackInput {
    pub title: Option<String>,
    pub album_id: Option<Uuid>,
}

/// Input type for adding a timestamped comment
#[derive(InputObject, Serialize, Deserialize)]
pub struct AddCommentInput {
    pub track_id: Uuid,
    pub timestamp_ms: u64,
    pub content: String,
}

/// Input type for creating a playlist
#[derive(InputObject, Serialize, Deserialize)]
pub struct CreatePlaylistInput {
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
}

/// Input type for adding tracks to a playlist
#[derive(InputObject, Serialize, Deserialize)]
pub struct AddTrackToPlaylistInput {
    pub playlist_id: Uuid,
    pub track_id: Uuid,
}

/// Input type for visualizer preset
#[derive(InputObject, Serialize, Deserialize)]
pub struct CreateVisualizerPresetInput {
    pub name: String,
    pub config: serde_json::Value,
    pub is_default: bool,
}

/// Input type for updating visualizer preset
#[derive(InputObject, Serialize, Deserialize)]
pub struct UpdateVisualizerPresetInput {
    pub name: Option<String>,
    pub config: Option<serde_json::Value>,
    pub is_default: Option<bool>,
}

/// Output type for track with additional metadata
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct TrackWithMetadata {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub title: String,
    pub duration_ms: u64,
    pub media_cid: String,
    pub waveform_data_cid: Option<String>,
    pub like_count: i32,
    pub play_count: i32,
    pub comment_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Output type for artist information
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ArtistInfo {
    pub id: Uuid,
    pub name: String,
    pub follower_count: i32,
    pub track_count: i32,
    pub is_followed: bool,
}

/// Output type for playlist with track count
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct PlaylistWithTrackCount {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub track_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Output type for download status
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct DownloadStatusOutput {
    pub track_id: Uuid,
    pub status: String,
    pub progress: Option<f32>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// Input type for search parameters
#[derive(InputObject, Serialize, Deserialize)]
pub struct SearchInput {
    pub query: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Output type for search results
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SearchResults {
    pub tracks: Vec<TrackWithMetadata>,
    pub artists: Vec<ArtistInfo>,
    pub playlists: Vec<PlaylistWithTrackCount>,
    pub total_tracks: i32,
    pub total_artists: i32,
    pub total_playlists: i32,
}

/// Input type for time range filter
#[derive(InputObject, Serialize, Deserialize)]
pub struct TimeRangeFilter {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Output type for social feed item
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SocialFeedItem {
    pub id: Uuid,
    pub track: TrackWithMetadata,
    pub user_id: Uuid,
    pub user_name: String,
    pub action: String, // "like", "repost", "comment"
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Output type for user profile
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub follower_count: i32,
    pub following_count: i32,
    pub track_count: i32,
    pub playlist_count: i32,
    pub is_following: bool,
}

/// Input type for user preferences
#[derive(InputObject, Serialize, Deserialize)]
pub struct UserPreferencesInput {
    pub preferred_visualizer_preset: Option<Uuid>,
    pub offline_storage_limit_mb: Option<i32>,
    pub auto_download_on_wifi: Option<bool>,
    pub enable_social_features: Option<bool>,
}

/// Output type for user preferences
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct UserPreferences {
    pub preferred_visualizer_preset: Option<Uuid>,
    pub offline_storage_limit_bytes: i64,
    pub auto_download_on_wifi: bool,
    pub enable_social_features: bool,
    pub privacy_settings: serde_json::Value,
}

/// Input type for privacy settings
#[derive(InputObject, Serialize, Deserialize)]
pub struct PrivacySettingsInput {
    pub share_listening_history: Option<bool>,
    pub share_playlists: Option<bool>,
    pub allow_comments: Option<bool>,
    pub allow_reposts: Option<bool>,
}

/// Output type for analytics data
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct AnalyticsData {
    pub total_plays: i32,
    pub unique_listeners: i32,
    pub completion_rate: f32,
    pub skip_rate: f32,
    pub average_listen_time_ms: i64,
    pub top_regions: Vec<RegionPlayCount>,
    pub play_history: Vec<PlayHistoryItem>,
}

/// Output type for region play count
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct RegionPlayCount {
    pub region: String,
    pub play_count: i32,
}

/// Output type for play history item
#[derive(SimpleObject, Serialize, Deserialize)]
pub struct PlayHistoryItem {
    pub track_id: Uuid,
    pub played_at: DateTime<Utc>,
    pub duration_ms: u64,
    pub completed: bool,
}