//! Feed preferences and configuration for user feeds

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use async_trait::async_trait;

/// Types of feed algorithms available
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FeedAlgorithmType {
    /// Chronological feed (newest first)
    Chronological,
    /// Engagement-based feed (most engaged first)
    Engagement,
    /// Custom algorithm with specific name
    Custom(String),
}

/// User preferences for feed configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeedPreferences {
    /// Algorithm type to use for feed generation
    pub algorithm: FeedAlgorithmType,
    /// Maximum number of items to include in feed
    pub max_items: u32,
    /// Whether to include media attachments in feed
    pub include_media: bool,
    /// Whether to include posts from external sources
    pub include_external: bool,
}

impl Default for FeedPreferences {
    fn default() -> Self {
        Self {
            algorithm: FeedAlgorithmType::Chronological,
            max_items: 100,
            include_media: true,
            include_external: true,
        }
    }
}

/// Repository trait for accessing user feed preferences
#[async_trait]
pub trait FeedPreferencesRepository: Send + Sync {
    /// Get feed preferences for a specific user
    async fn get_preferences(
        &self, 
        user_id: Uuid
    ) -> Result<FeedPreferences, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Save feed preferences for a specific user
    async fn save_preferences(
        &self, 
        user_id: Uuid,
        preferences: FeedPreferences
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}