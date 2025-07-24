use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a social media post in the application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub media_urls: Vec<String>,
    pub author: User,
    pub timestamp: DateTime<Utc>,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub like_count: i32,
    pub comment_count: i32,
    pub share_count: i32,
    pub is_liked: bool,
    pub is_shared: bool,
}

/// Defines who can see a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Visibility {
    Public,
    Cooperative,
    Private,
}

/// User information for post authors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// Timeline filtering options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimelineFilters {
    pub content_type: Option<ContentType>,
    pub author_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub visibility: Option<Visibility>,
    pub cooperative_only: bool,
}

/// Content type filtering
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContentType {
    Posts,
    Replies,
    Media,
}

/// Pagination parameters for timeline
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginationParams {
    pub limit: i32,
    pub offset: i32,
    pub after: Option<Uuid>,
}

/// Timeline response with posts and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineResponse {
    pub posts: Vec<Post>,
    pub has_more: bool,
    pub total_count: i64,
}

/// Social interaction types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocialInteraction {
    Like { post_id: Uuid },
    Unlike { post_id: Uuid },
    Comment { post_id: Uuid, content: String },
    Share { post_id: Uuid },
}

/// Post creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub content: String,
    pub media_urls: Vec<String>,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
}

/// Comment creation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub post_id: Uuid,
    pub content: String,
}

/// Post update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub content: Option<String>,
    pub visibility: Option<Visibility>,
}

/// Timeline cache entry for secure storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineCache {
    pub posts: Vec<Post>,
    pub last_updated: DateTime<Utc>,
    pub filters: TimelineFilters,
    pub pagination: PaginationParams,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

impl Post {
    /// Check if post matches the given filters
    pub fn matches_filters(&self, filters: &TimelineFilters) -> bool {
        if let Some(author_id) = filters.author_id {
            if self.author.id != author_id {
                return false;
            }
        }
        
        if let Some(date_from) = filters.date_from {
            if self.timestamp < date_from {
                return false;
            }
        }
        
        if let Some(date_to) = filters.date_to {
            if self.timestamp > date_to {
                return false;
            }
        }
        
        if let Some(visibility) = &filters.visibility {
            if &self.visibility != visibility {
                return false;
            }
        }
        
        if filters.cooperative_only && self.cooperative_id.is_none() {
            return false;
        }
        
        true
    }
}

impl TimelineFilters {
    /// Create a new filter with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set content type filter
    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }
    
    /// Set author filter
    pub fn with_author(mut self, author_id: Uuid) -> Self {
        self.author_id = Some(author_id);
        self
    }
    
    /// Set date range filter
    pub fn with_date_range(mut self, from: DateTime<Utc>, to: DateTime<Utc>) -> Self {
        self.date_from = Some(from);
        self.date_to = Some(to);
        self
    }
    
    /// Set visibility filter
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }
    
    /// Set cooperative only filter
    pub fn with_cooperative_only(mut self, cooperative_only: bool) -> Self {
        self.cooperative_only = cooperative_only;
        self
    }
}