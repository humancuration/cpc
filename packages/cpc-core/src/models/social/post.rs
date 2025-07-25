use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// The fully-featured Post model for Phase 2
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub feed_position: Option<i32>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
    #[graphql(skip)]
    #[sqlx(default)]
    pub media_items: Vec<MediaItem>,
    // Metadata fields
    pub tags: Vec<String>,
    pub mentions: Vec<Uuid>, // User IDs mentioned in the post
    pub reply_to_post_id: Option<Uuid>, // For threading support
    pub repost_of_post_id: Option<Uuid>, // For reposts
    pub edit_history: Vec<PostEdit>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MediaItem {
    pub id: Uuid,
    #[graphql(skip)]
    pub post_id: Uuid,
    pub url: String,
    pub media_type: MediaType,
    pub processing_status: ProcessingStatus,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    // Additional metadata
    pub file_size: Option<i64>,
    pub duration: Option<i32>, // For video/audio in seconds
    pub width: Option<i32>,    // For images/videos
    pub height: Option<i32>,   // For images/videos
    pub thumbnail_url: Option<String>,
    pub alt_text: Option<String>, // Accessibility
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "processing_status", rename_all = "UPPERCASE")]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

// Comment model with threading support
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>, // For threading/nested replies
    pub thread_depth: i32, // How deep in the thread this comment is
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
    pub mentions: Vec<Uuid>, // User IDs mentioned in the comment
    pub edit_history: Vec<CommentEdit>,
}

// Reply model (specialized comment for direct replies to posts)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Reply {
    pub id: Uuid,
    pub original_post_id: Uuid,
    pub reply_post_id: Uuid, // The post that serves as the reply
    pub author_id: Uuid,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

// Like model for posts and comments
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_type: LikeTargetType,
    pub target_id: Uuid, // Post ID or Comment ID
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "like_target_type", rename_all = "UPPERCASE")]
pub enum LikeTargetType {
    Post,
    Comment,
}

// Share model for sharing posts
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub share_message: Option<String>, // Optional message when sharing
    pub share_type: ShareType,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "share_type", rename_all = "UPPERCASE")]
pub enum ShareType {
    Direct,     // Direct share to feed
    Message,    // Share via private message
    External,   // Share outside platform
}

// Repost model for reposting content
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Repost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_post_id: Uuid,
    pub repost_message: Option<String>, // Optional message when reposting
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

// Edit history for posts
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PostEdit {
    pub id: Uuid,
    pub post_id: Uuid,
    pub previous_content: String,
    pub edit_reason: Option<String>,
    #[serde(with = "crate::utils::datetime")]
    pub edited_at: DateTime<Utc>,
}

// Edit history for comments
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CommentEdit {
    pub id: Uuid,
    pub comment_id: Uuid,
    pub previous_content: String,
    pub edit_reason: Option<String>,
    #[serde(with = "crate::utils::datetime")]
    pub edited_at: DateTime<Utc>,
}
//
 Implementation methods for Post
impl Post {
    /// Creates a new post
    pub fn new(author_id: Uuid, content: String, visibility: Visibility) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            author_id,
            content,
            visibility,
            cooperative_id: None,
            feed_position: None,
            created_at: now,
            updated_at: now,
            media_items: Vec::new(),
            tags: Vec::new(),
            mentions: Vec::new(),
            reply_to_post_id: None,
            repost_of_post_id: None,
            edit_history: Vec::new(),
        }
    }

    /// Creates a reply post
    pub fn new_reply(author_id: Uuid, content: String, reply_to_post_id: Uuid, visibility: Visibility) -> Self {
        let mut post = Self::new(author_id, content, visibility);
        post.reply_to_post_id = Some(reply_to_post_id);
        post
    }

    /// Creates a repost
    pub fn new_repost(author_id: Uuid, original_post_id: Uuid, repost_message: Option<String>, visibility: Visibility) -> Self {
        let content = repost_message.unwrap_or_default();
        let mut post = Self::new(author_id, content, visibility);
        post.repost_of_post_id = Some(original_post_id);
        post
    }

    /// Checks if this is a reply
    pub fn is_reply(&self) -> bool {
        self.reply_to_post_id.is_some()
    }

    /// Checks if this is a repost
    pub fn is_repost(&self) -> bool {
        self.repost_of_post_id.is_some()
    }

    /// Adds a media item to the post
    pub fn add_media_item(&mut self, media_item: MediaItem) {
        self.media_items.push(media_item);
        self.touch();
    }

    /// Adds a tag to the post
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.touch();
        }
    }

    /// Adds a mention to the post
    pub fn add_mention(&mut self, user_id: Uuid) {
        if !self.mentions.contains(&user_id) {
            self.mentions.push(user_id);
            self.touch();
        }
    }

    /// Edits the post content
    pub fn edit_content(&mut self, new_content: String, edit_reason: Option<String>) {
        let edit = PostEdit {
            id: Uuid::new_v4(),
            post_id: self.id,
            previous_content: self.content.clone(),
            edit_reason,
            edited_at: Utc::now(),
        };
        self.edit_history.push(edit);
        self.content = new_content;
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for Comment
impl Comment {
    /// Creates a new comment
    pub fn new(post_id: Uuid, author_id: Uuid, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            post_id,
            author_id,
            content,
            parent_comment_id: None,
            thread_depth: 0,
            created_at: now,
            updated_at: now,
            mentions: Vec::new(),
            edit_history: Vec::new(),
        }
    }

    /// Creates a reply to another comment
    pub fn new_reply(post_id: Uuid, author_id: Uuid, content: String, parent_comment_id: Uuid, thread_depth: i32) -> Self {
        let mut comment = Self::new(post_id, author_id, content);
        comment.parent_comment_id = Some(parent_comment_id);
        comment.thread_depth = thread_depth;
        comment
    }

    /// Checks if this is a reply to another comment
    pub fn is_reply(&self) -> bool {
        self.parent_comment_id.is_some()
    }

    /// Adds a mention to the comment
    pub fn add_mention(&mut self, user_id: Uuid) {
        if !self.mentions.contains(&user_id) {
            self.mentions.push(user_id);
            self.touch();
        }
    }

    /// Edits the comment content
    pub fn edit_content(&mut self, new_content: String, edit_reason: Option<String>) {
        let edit = CommentEdit {
            id: Uuid::new_v4(),
            comment_id: self.id,
            previous_content: self.content.clone(),
            edit_reason,
            edited_at: Utc::now(),
        };
        self.edit_history.push(edit);
        self.content = new_content;
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for MediaItem
impl MediaItem {
    /// Creates a new media item
    pub fn new(post_id: Uuid, url: String, media_type: MediaType) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id,
            url,
            media_type,
            processing_status: ProcessingStatus::Pending,
            created_at: Utc::now(),
            file_size: None,
            duration: None,
            width: None,
            height: None,
            thumbnail_url: None,
            alt_text: None,
        }
    }

    /// Updates processing status
    pub fn set_processing_status(&mut self, status: ProcessingStatus) {
        self.processing_status = status;
    }

    /// Sets media dimensions
    pub fn set_dimensions(&mut self, width: i32, height: i32) {
        self.width = Some(width);
        self.height = Some(height);
    }

    /// Sets media duration (for video/audio)
    pub fn set_duration(&mut self, duration: i32) {
        self.duration = Some(duration);
    }

    /// Sets file size
    pub fn set_file_size(&mut self, size: i64) {
        self.file_size = Some(size);
    }

    /// Sets thumbnail URL
    pub fn set_thumbnail_url(&mut self, url: String) {
        self.thumbnail_url = Some(url);
    }

    /// Sets alt text for accessibility
    pub fn set_alt_text(&mut self, alt_text: String) {
        self.alt_text = Some(alt_text);
    }
}

// Implementation methods for Like
impl Like {
    /// Creates a new like for a post
    pub fn new_post_like(user_id: Uuid, post_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            target_type: LikeTargetType::Post,
            target_id: post_id,
            created_at: Utc::now(),
        }
    }

    /// Creates a new like for a comment
    pub fn new_comment_like(user_id: Uuid, comment_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            target_type: LikeTargetType::Comment,
            target_id: comment_id,
            created_at: Utc::now(),
        }
    }

    /// Checks if this is a post like
    pub fn is_post_like(&self) -> bool {
        self.target_type == LikeTargetType::Post
    }

    /// Checks if this is a comment like
    pub fn is_comment_like(&self) -> bool {
        self.target_type == LikeTargetType::Comment
    }
}

// Implementation methods for Share
impl Share {
    /// Creates a new share
    pub fn new(user_id: Uuid, post_id: Uuid, share_type: ShareType, share_message: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            post_id,
            share_message,
            share_type,
            created_at: Utc::now(),
        }
    }

    /// Creates a direct share to feed
    pub fn new_direct_share(user_id: Uuid, post_id: Uuid, message: Option<String>) -> Self {
        Self::new(user_id, post_id, ShareType::Direct, message)
    }

    /// Creates a message share
    pub fn new_message_share(user_id: Uuid, post_id: Uuid, message: Option<String>) -> Self {
        Self::new(user_id, post_id, ShareType::Message, message)
    }

    /// Creates an external share
    pub fn new_external_share(user_id: Uuid, post_id: Uuid, message: Option<String>) -> Self {
        Self::new(user_id, post_id, ShareType::External, message)
    }
}

// Implementation methods for Repost
impl Repost {
    /// Creates a new repost
    pub fn new(user_id: Uuid, original_post_id: Uuid, repost_message: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            original_post_id,
            repost_message,
            created_at: Utc::now(),
        }
    }
}

// Implementation methods for Reply
impl Reply {
    /// Creates a new reply
    pub fn new(original_post_id: Uuid, reply_post_id: Uuid, author_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            original_post_id,
            reply_post_id,
            author_id,
            created_at: Utc::now(),
        }
    }
}