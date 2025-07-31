//! Post domain models for social integration

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Source application for a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AppSource {
    Allat,
    Yapper,
}

/// Metadata for a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostMetadata {
    /// Timestamp when the post was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the post was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Engagement metrics
    pub engagement: EngagementMetrics,
    
    /// Optional media attachments
    pub media_attachments: Vec<MediaAttachment>,
    
    /// Optional hashtags
    pub hashtags: Vec<String>,
    
    /// Privacy settings
    pub privacy: PrivacySettings,
}

/// Engagement metrics for a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EngagementMetrics {
    /// Number of upvotes/likes
    pub upvotes: u64,
    
    /// Number of comments
    pub comments: u64,
    
    /// Number of shares
    pub shares: u64,
    
    /// Number of views
    pub views: u64,
}

impl EngagementMetrics {
    /// Create new engagement metrics with all values set to zero
    pub fn new() -> Self {
        Self {
            upvotes: 0,
            comments: 0,
            shares: 0,
            views: 0,
        }
    }
    
    /// Increment upvotes
    pub fn increment_upvotes(&mut self) {
        self.upvotes += 1;
    }
    
    /// Increment comments
    pub fn increment_comments(&mut self) {
        self.comments += 1;
    }
    
    /// Increment shares
    pub fn increment_shares(&mut self) {
        self.shares += 1;
    }
    
    /// Increment views
    pub fn increment_views(&mut self) {
        self.views += 1;
    }
}

impl Default for EngagementMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Media attachment for a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaAttachment {
    /// Unique identifier for the attachment
    pub id: Uuid,
    
    /// URL to the media file
    pub url: String,
    
    /// Media type (image, video, etc.)
    pub media_type: MediaType,
    
    /// Optional alt text for accessibility
    pub alt_text: Option<String>,
}

/// Media type for attachments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Image,
    Video,
    Audio,
}

/// Privacy settings for a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivacySettings {
    /// Whether the post is public
    pub is_public: bool,
    
    /// List of user IDs who can view the post (if not public)
    pub allowed_viewers: Vec<Uuid>,
    
    /// Whether the post can be shared
    pub shareable: bool,
}

/// A unified post from any social app
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedPost {
    /// Unique identifier for the post
    pub id: Uuid,
    
    /// Source application
    pub source: AppSource,
    
    /// Original post ID in the source application
    pub original_id: Uuid,
    
    /// Author of the post
    pub author_id: Uuid,
    
    /// Content of the post
    pub content: String,
    
    /// Metadata for the post
    pub metadata: PostMetadata,
    
    /// Additional properties specific to the source app
    pub properties: HashMap<String, String>,
}

impl UnifiedPost {
    /// Create a new unified post
    pub fn new(
        source: AppSource,
        original_id: Uuid,
        author_id: Uuid,
        content: String,
        metadata: PostMetadata,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source,
            original_id,
            author_id,
            content,
            metadata,
            properties: HashMap::new(),
        }
    }
    
    /// Add a property to the post
    pub fn add_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }
    
    /// Get a property from the post
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}