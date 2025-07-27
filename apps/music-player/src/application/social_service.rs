//! Social service for music player functionality

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::models::{TimestampedComment, TrackInteraction};
use crate::domain::errors::{Result, MusicPlayerError};
use crate::infrastructure::database::{CommentRepository, InteractionRepository};
use crate::application::privacy_service::{PrivacyService, ConsentType};

/// Service for social features
pub struct SocialService {
    comment_repository: Arc<CommentRepository>,
    interaction_repository: Arc<InteractionRepository>,
    privacy_service: Arc<PrivacyService>,
}

impl SocialService {
    /// Create a new social service
    pub fn new(
        comment_repository: Arc<CommentRepository>,
        interaction_repository: Arc<InteractionRepository>,
        privacy_service: Arc<PrivacyService>,
    ) -> Self {
        Self {
            comment_repository,
            interaction_repository,
            privacy_service,
        }
    }

    /// Add a timestamped comment to a track
    pub async fn add_timestamped_comment(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        timestamp_ms: u64,
        content: String,
    ) -> Result<TimestampedComment> {
        // Verify consent for social interactions
        self.privacy_service
            .verify_consent(user_id, ConsentType::Social)
            .await?;
            
        // Validate timestamp is within track duration
        // This would require fetching the track to check its duration
        
        let comment = TimestampedComment::new(track_id, user_id, timestamp_ms, content);
        self.comment_repository.create(&comment).await?;
        Ok(comment)
    }

    /// Like a track
    pub async fn like_track(&self, track_id: Uuid, user_id: Uuid) -> Result<()> {
        // Verify consent for social interactions
        self.privacy_service
            .verify_consent(user_id, ConsentType::Social)
            .await?;
            
        let interaction = TrackInteraction::new_like(track_id, user_id);
        self.interaction_repository.create_interaction(&interaction).await?;
        Ok(())
    }

    /// Repost a track
    pub async fn repost_track(
        &self,
        track_id: Uuid,
        user_id: Uuid,
        comment: Option<String>,
    ) -> Result<()> {
        // Verify consent for social interactions
        self.privacy_service
            .verify_consent(user_id, ConsentType::Social)
            .await?;
            
        let interaction = TrackInteraction::new_repost(track_id, user_id, comment);
        self.interaction_repository.create_interaction(&interaction).await?;
        Ok(())
    }

    /// Follow an artist
    pub async fn follow_artist(&self, artist_id: Uuid, follower_id: Uuid) -> Result<()> {
        // Verify consent for following interactions
        self.privacy_service
            .verify_consent(follower_id, ConsentType::Following)
            .await?;
            
        let interaction = TrackInteraction::new_follow_artist(artist_id, follower_id);
        self.interaction_repository.create_interaction(&interaction).await?;
        Ok(())
    }

    /// Get comments for a track within a timestamp range
    pub async fn get_comments_for_track(
        &self,
        track_id: Uuid,
        range: Option<(u64, u64)>,
    ) -> Result<Vec<TimestampedComment>> {
        if let Some((start, end)) = range {
            self.comment_repository.find_by_track_and_timestamp_range(track_id, start, end).await
        } else {
            self.comment_repository.find_by_track(track_id).await
        }
    }

    /// Get a user's social feed
    pub async fn get_social_feed(
        &self,
        user_id: Uuid,
        cursor: Option<DateTime<Utc>>,
    ) -> Result<Vec<TimestampedComment>> {
        // Verify consent for social interactions
        self.privacy_service
            .verify_consent(user_id, ConsentType::Social)
            .await?;
            
        self.comment_repository.find_by_followed_artists(user_id, cursor).await
    }

    /// Get likes for a track
    pub async fn get_track_likes(&self, track_id: Uuid) -> Result<Vec<Uuid>> {
        self.interaction_repository.find_track_likes(track_id).await
    }

    /// Check if a user has liked a track
    pub async fn has_user_liked_track(&self, track_id: Uuid, user_id: Uuid) -> Result<bool> {
        self.interaction_repository.user_has_liked_track(track_id, user_id).await
    }

    /// Get artists followed by a user
    pub async fn get_followed_artists(&self, user_id: Uuid) -> Result<Vec<Uuid>> {
        // Verify consent for following interactions
        self.privacy_service
            .verify_consent(user_id, ConsentType::Following)
            .await?;
            
        self.interaction_repository.find_followed_artists(user_id).await
    }

    /// Check if a user follows an artist
    pub async fn is_following_artist(&self, artist_id: Uuid, follower_id: Uuid) -> Result<bool> {
        self.interaction_repository.user_follows_artist(artist_id, follower_id).await
    }
}