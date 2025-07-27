//! Database repository implementations for music player module

use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::models::{Track, TimestampedComment, VisualizerPreset, DownloadStatus, DownloadManifest};
use crate::domain::errors::{Result, MusicPlayerError};

pub mod consent_repository;
pub mod pg_consent_repository;

/// Repository for track operations
pub struct TrackRepository {
    pool: Arc<PgPool>,
}

impl TrackRepository {
    /// Create a new track repository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Find a track by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Track> {
        let track = sqlx::query_as!(
            Track,
            r#"
            SELECT id, artist_id, album_id, title, duration_ms, media_cid, 
                   waveform_data_cid, created_at, updated_at
            FROM tracks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        track.ok_or_else(|| MusicPlayerError::TrackNotFound { id: id.to_string() })
    }

    /// Create a new track
    pub async fn create(&self, track: &Track) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO tracks (id, artist_id, album_id, title, duration_ms, media_cid, waveform_data_cid, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            track.id,
            track.artist_id,
            track.album_id,
            track.title,
            track.duration_ms as i64,
            track.media_cid,
            track.waveform_data_cid,
            track.created_at,
            track.updated_at
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Update waveform data CID for a track
    pub async fn update_waveform_data(&self, track_id: Uuid, waveform_cid: String) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE tracks
            SET waveform_data_cid = $1, updated_at = NOW()
            WHERE id = $2
            "#,
            waveform_cid,
            track_id
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Find popular tracks
    pub async fn find_popular_tracks(&self, limit: i64) -> Result<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT id, artist_id, album_id, title, duration_ms, media_cid, 
                   waveform_data_cid, created_at, updated_at
            FROM tracks
            ORDER BY created_at DESC
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(tracks)
    }

    /// Search tracks by title
    pub async fn search_by_title(&self, query: &str) -> Result<Vec<Track>> {
        let tracks = sqlx::query_as!(
            Track,
            r#"
            SELECT id, artist_id, album_id, title, duration_ms, media_cid, 
                   waveform_data_cid, created_at, updated_at
            FROM tracks
            WHERE title ILIKE $1
            ORDER BY created_at DESC
            "#,
            format!("%{}%", query)
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(tracks)
    }
}

/// Repository for comment operations
pub struct CommentRepository {
    pool: Arc<PgPool>,
}

impl CommentRepository {
    /// Create a new comment repository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Create a new comment
    pub async fn create(&self, comment: &TimestampedComment) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO track_comments (id, track_id, user_id, timestamp_ms, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            comment.id,
            comment.track_id,
            comment.user_id,
            comment.timestamp_ms as i64,
            comment.content,
            comment.created_at,
            comment.updated_at
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Find comments by track ID
    pub async fn find_by_track(&self, track_id: Uuid) -> Result<Vec<TimestampedComment>> {
        let comments = sqlx::query_as!(
            TimestampedComment,
            r#"
            SELECT id, track_id, user_id, timestamp_ms, content, created_at, updated_at
            FROM track_comments
            WHERE track_id = $1
            ORDER BY timestamp_ms ASC
            "#,
            track_id
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(comments)
    }

    /// Find comments by track ID and timestamp range
    pub async fn find_by_track_and_timestamp_range(
        &self,
        track_id: Uuid,
        start_ms: u64,
        end_ms: u64,
    ) -> Result<Vec<TimestampedComment>> {
        let comments = sqlx::query_as!(
            TimestampedComment,
            r#"
            SELECT id, track_id, user_id, timestamp_ms, content, created_at, updated_at
            FROM track_comments
            WHERE track_id = $1 AND timestamp_ms >= $2 AND timestamp_ms <= $3
            ORDER BY timestamp_ms ASC
            "#,
            track_id,
            start_ms as i64,
            end_ms as i64
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(comments)
    }

    /// Find comments by followed artists
    pub async fn find_by_followed_artists(
        &self,
        user_id: Uuid,
        cursor: Option<DateTime<Utc>>,
    ) -> Result<Vec<TimestampedComment>> {
        let comments = if let Some(cursor_time) = cursor {
            sqlx::query_as!(
                TimestampedComment,
                r#"
                SELECT tc.id, tc.track_id, tc.user_id, tc.timestamp_ms, tc.content, tc.created_at, tc.updated_at
                FROM track_comments tc
                JOIN tracks t ON tc.track_id = t.id
                JOIN artist_follows af ON t.artist_id = af.artist_id
                WHERE af.follower_id = $1 AND tc.created_at < $2
                ORDER BY tc.created_at DESC
                LIMIT 50
                "#,
                user_id,
                cursor_time
            )
            .fetch_all(&**self.pool)
            .await
            .map_err(MusicPlayerError::DatabaseError)?
        } else {
            sqlx::query_as!(
                TimestampedComment,
                r#"
                SELECT tc.id, tc.track_id, tc.user_id, tc.timestamp_ms, tc.content, tc.created_at, tc.updated_at
                FROM track_comments tc
                JOIN tracks t ON tc.track_id = t.id
                JOIN artist_follows af ON t.artist_id = af.artist_id
                WHERE af.follower_id = $1
                ORDER BY tc.created_at DESC
                LIMIT 50
                "#,
                user_id
            )
            .fetch_all(&**self.pool)
            .await
            .map_err(MusicPlayerError::DatabaseError)?
        };

        Ok(comments)
    }
}

/// Repository for interaction operations
pub struct InteractionRepository {
    pool: Arc<PgPool>,
}

impl InteractionRepository {
    /// Create a new interaction repository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Create a new interaction
    pub async fn create_interaction(&self, interaction: &crate::domain::models::TrackInteraction) -> Result<()> {
        match interaction {
            crate::domain::models::TrackInteraction::Like { track_id, user_id, created_at } => {
                sqlx::query!(
                    r#"
                    INSERT INTO track_likes (track_id, user_id, created_at)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (track_id, user_id) DO NOTHING
                    "#,
                    track_id,
                    user_id,
                    created_at
                )
                .execute(&**self.pool)
                .await
                .map_err(MusicPlayerError::DatabaseError)?;
            }
            crate::domain::models::TrackInteraction::Repost { track_id, user_id, comment, created_at } => {
                // For reposts, we might store them in a separate table or as a special type of comment
                // This is a simplified implementation
                sqlx::query!(
                    r#"
                    INSERT INTO track_likes (track_id, user_id, created_at)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (track_id, user_id) DO NOTHING
                    "#,
                    track_id,
                    user_id,
                    created_at
                )
                .execute(&**self.pool)
                .await
                .map_err(MusicPlayerError::DatabaseError)?;
            }
            crate::domain::models::TrackInteraction::FollowArtist { artist_id, follower_id, created_at } => {
                sqlx::query!(
                    r#"
                    INSERT INTO artist_follows (artist_id, follower_id, created_at)
                    VALUES ($1, $2, $3)
                    ON CONFLICT (artist_id, follower_id) DO NOTHING
                    "#,
                    artist_id,
                    follower_id,
                    created_at
                )
                .execute(&**self.pool)
                .await
                .map_err(MusicPlayerError::DatabaseError)?;
            }
        }

        Ok(())
    }

    /// Find track likes
    pub async fn find_track_likes(&self, track_id: Uuid) -> Result<Vec<Uuid>> {
        let rows = sqlx::query!(
            r#"
            SELECT user_id
            FROM track_likes
            WHERE track_id = $1
            "#,
            track_id
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        let user_ids = rows.into_iter().map(|row| row.user_id).collect();
        Ok(user_ids)
    }

    /// Check if user has liked a track
    pub async fn user_has_liked_track(&self, track_id: Uuid, user_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM track_likes
            WHERE track_id = $1 AND user_id = $2
            "#,
            track_id,
            user_id
        )
        .fetch_one(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(count > 0)
    }

    /// Find followed artists
    pub async fn find_followed_artists(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        let rows = sqlx::query!(
            r#"
            SELECT artist_id
            FROM artist_follows
            WHERE follower_id = $1
            "#,
            user_id
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        let artist_ids = rows.into_iter().map(|row| row.artist_id).collect();
        Ok(artist_ids)
    }

    /// Check if user follows artist
    pub async fn user_follows_artist(&self, artist_id: Uuid, follower_id: Uuid) -> Result<bool> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM artist_follows
            WHERE artist_id = $1 AND follower_id = $2
            "#,
            artist_id,
            follower_id
        )
        .fetch_one(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(count > 0)
    }
}

/// Repository for visualizer operations
pub struct VisualizerRepository {
    pool: Arc<PgPool>,
}

impl VisualizerRepository {
    /// Create a new visualizer repository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Find all presets
    pub async fn find_all_presets(&self) -> Result<Vec<VisualizerPreset>> {
        let presets = sqlx::query_as!(
            VisualizerPreset,
            r#"
            SELECT id, name, config, is_default, created_at
            FROM visualizer_presets
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(presets)
    }

    /// Find preset by ID
    pub async fn find_preset_by_id(&self, id: Uuid) -> Result<VisualizerPreset> {
        let preset = sqlx::query_as!(
            VisualizerPreset,
            r#"
            SELECT id, name, config, is_default, created_at
            FROM visualizer_presets
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        preset.ok_or_else(|| MusicPlayerError::InvalidInput { 
            message: format!("Visualizer preset not found: {}", id) 
        })
    }

    /// Find default preset
    pub async fn find_default_preset(&self) -> Result<VisualizerPreset> {
        let preset = sqlx::query_as!(
            VisualizerPreset,
            r#"
            SELECT id, name, config, is_default, created_at
            FROM visualizer_presets
            WHERE is_default = true
            ORDER BY created_at DESC
            LIMIT 1
            "#
        )
        .fetch_optional(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        preset.ok_or_else(|| MusicPlayerError::InvalidInput { 
            message: "No default visualizer preset found".to_string() 
        })
    }

    /// Create a new preset
    pub async fn create_preset(&self, preset: &VisualizerPreset) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO visualizer_presets (id, name, config, is_default, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            preset.id,
            preset.name,
            preset.config,
            preset.is_default,
            preset.created_at
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Update a preset
    pub async fn update_preset(
        &self,
        id: Uuid,
        name: Option<String>,
        config: Option<serde_json::Value>,
        is_default: Option<bool>,
    ) -> Result<VisualizerPreset> {
        if let Some(name) = name {
            sqlx::query!(
                r#"
                UPDATE visualizer_presets
                SET name = $1, updated_at = NOW()
                WHERE id = $2
                "#,
                name,
                id
            )
            .execute(&**self.pool)
            .await
            .map_err(MusicPlayerError::DatabaseError)?;
        }

        if let Some(config) = config {
            sqlx::query!(
                r#"
                UPDATE visualizer_presets
                SET config = $1, updated_at = NOW()
                WHERE id = $2
                "#,
                config,
                id
            )
            .execute(&**self.pool)
            .await
            .map_err(MusicPlayerError::DatabaseError)?;
        }

        if let Some(is_default) = is_default {
            // If setting this as default, unset others
            if is_default {
                sqlx::query!(
                    r#"
                    UPDATE visualizer_presets
                    SET is_default = false
                    WHERE is_default = true
                    "#
                )
                .execute(&**self.pool)
                .await
                .map_err(MusicPlayerError::DatabaseError)?;
            }

            sqlx::query!(
                r#"
                UPDATE visualizer_presets
                SET is_default = $1, updated_at = NOW()
                WHERE id = $2
                "#,
                is_default,
                id
            )
            .execute(&**self.pool)
            .await
            .map_err(MusicPlayerError::DatabaseError)?;
        }

        self.find_preset_by_id(id).await
    }

    /// Delete a preset
    pub async fn delete_preset(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM visualizer_presets
            WHERE id = $1
            "#,
            id
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }
}

/// Repository for cache operations
pub struct CacheRepository {
    pool: Arc<PgPool>,
}

impl CacheRepository {
    /// Create a new cache repository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Create a download record
    pub async fn create_download_record(&self, track_id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO offline_downloads (track_id, user_id, download_manifest, status, created_at)
            VALUES ($1, $2, $3, $4, NOW())
            ON CONFLICT (track_id, user_id) DO UPDATE
            SET status = $4, created_at = NOW(), completed_at = NULL
            "#,
            track_id,
            user_id,
            serde_json::json!({}),
            "pending"
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Get download status
    pub async fn get_download_status(&self, track_id: Uuid, user_id: Uuid) -> Result<DownloadStatus> {
        let row = sqlx::query!(
            r#"
            SELECT status, download_manifest
            FROM offline_downloads
            WHERE track_id = $1 AND user_id = $2
            "#,
            track_id,
            user_id
        )
        .fetch_optional(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        if let Some(row) = row {
            let status = match row.status.as_str() {
                "pending" => DownloadStatus::Pending,
                "in_progress" => {
                    // In a real implementation, we might store progress in the manifest
                    DownloadStatus::InProgress { progress: 0.0 }
                },
                "completed" => DownloadStatus::Completed,
                "failed" => {
                    let error = row.download_manifest
                        .get("error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error")
                        .to_string();
                    DownloadStatus::Failed { error }
                },
                _ => DownloadStatus::Pending,
            };
            Ok(status)
        } else {
            Ok(DownloadStatus::Pending)
        }
    }

    /// List user downloaded tracks
    pub async fn list_user_downloaded_tracks(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        let rows = sqlx::query!(
            r#"
            SELECT track_id
            FROM offline_downloads
            WHERE user_id = $1 AND status = 'completed'
            "#,
            user_id
        )
        .fetch_all(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        let track_ids = rows.into_iter().map(|row| row.track_id).collect();
        Ok(track_ids)
    }

    /// Purge old downloads
    pub async fn purge_old_downloads(&self, user_id: Uuid) -> Result<()> {
        // This is a simplified implementation
        // In a real system, this would implement LRU eviction based on usage
        sqlx::query!(
            r#"
            DELETE FROM offline_downloads
            WHERE user_id = $1 AND status = 'completed' 
            AND completed_at < NOW() - INTERVAL '30 days'
            "#,
            user_id
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Update download progress
    pub async fn update_download_progress(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        progress: f32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE offline_downloads
            SET status = 'in_progress', download_manifest = jsonb_set(download_manifest, '{progress}', $8::jsonb)
            WHERE track_id = $1 AND user_id = $2
            "#,
            track_id,
            user_id,
            serde_json::Value::Number(serde_json::Number::from_f64(progress as f64).unwrap_or(serde_json::Number::from(0)))
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Complete download
    pub async fn complete_download(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        manifest: DownloadManifest,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE offline_downloads
            SET status = 'completed', download_manifest = $3, completed_at = NOW()
            WHERE track_id = $1 AND user_id = $2
            "#,
            track_id,
            user_id,
            serde_json::to_value(manifest).map_err(|e| MusicPlayerError::InvalidInput { 
                message: format!("Failed to serialize manifest: {}", e) 
            })?
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Fail download
    pub async fn fail_download(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        error: String,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE offline_downloads
            SET status = 'failed', download_manifest = jsonb_set(download_manifest, '{error}', $3::jsonb)
            WHERE track_id = $1 AND user_id = $2
            "#,
            track_id,
            user_id,
            serde_json::Value::String(error)
        )
        .execute(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(())
    }

    /// Get user storage usage
    pub async fn get_user_storage_usage(&self, user_id: Uuid) -> Result<u64> {
        // This is a simplified implementation
        // In a real system, this would calculate actual storage usage
        let usage = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) * 1024 * 1024 as "usage!"  -- Assume 1MB per track
            FROM offline_downloads
            WHERE user_id = $1 AND status = 'completed'
            "#,
            user_id
        )
        .fetch_one(&**self.pool)
        .await
        .map_err(MusicPlayerError::DatabaseError)?;

        Ok(usage as u64)
    }

    /// Get user storage limit
    pub async fn get_user_storage_limit(&self, user_id: Uuid) -> Result<u64> {
        // This is a simplified implementation
        // In a real system, this would get the user's actual storage limit
        Ok(1024 * 1024 * 1024) // 1GB default
    }

    /// Set user storage limit
    pub async fn set_user_storage_limit(&self, user_id: Uuid, limit_bytes: u64) -> Result<()> {
        // This is a simplified implementation
        // In a real system, this would store the user's storage limit
        Ok(())
    }
}