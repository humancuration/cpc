use uuid::Uuid;
use chrono::{DateTime, Utc};

pub enum ContentType {
    SocialPost,
    Photo,
    Video,
    Story,
}

pub enum Visibility {
    Public,
    Friends,
    Group,
    Private,
}

pub struct ContentItem {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub content_type: ContentType,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub visibility: Visibility,
    pub relevance_score: f32,
}

// Add other domain models as needed