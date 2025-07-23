use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// The fully-featured Post model for Phase 2
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[graphql(skip)]
    #[sqlx(default)]
    pub media_items: Vec<MediaItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
pub struct MediaItem {
    pub id: Uuid,
    #[graphql(skip)]
    pub post_id: Uuid,
    pub url: String,
    pub media_type: MediaType,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "visibility", rename_all = "UPPERCASE")]
pub enum Visibility {
    Public,
    Cooperative,
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "media_type", rename_all = "UPPERCASE")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Unknown,
}