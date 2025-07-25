use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// PostType enum to distinguish between social posts and forum threads
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "post_type", rename_all = "UPPERCASE")]
pub enum PostType {
    Social,     // Regular social media post
    Thread,     // Forum thread
    Reply,      // Reply to a thread or post
}

// Community model for organizing forums by topic/interest
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub owner_id: Uuid,
    pub moderator_ids: Vec<Uuid>,
    pub member_count: i64,
    pub is_private: bool,
    pub is_nsfw: bool,
    pub rules: Vec<CommunityRule>,
    pub tags: Vec<String>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Community rules
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CommunityRule {
    pub id: Uuid,
    pub community_id: Uuid,
    pub title: String,
    pub description: String,
    pub order_index: i32,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

// Forum struct with categories, rules, and moderation settings
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Forum {
    pub id: Uuid,
    pub community_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: ForumCategory,
    pub is_locked: bool,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub thread_count: i64,
    pub post_count: i64,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub moderator_ids: Vec<Uuid>,
    pub moderation_settings: ModerationSettings,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Forum categories
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "forum_category", rename_all = "UPPERCASE")]
pub enum ForumCategory {
    General,
    Discussion,
    QAndA,
    Announcements,
    Support,
    Feedback,
    OffTopic,
    Custom,
}

// Moderation settings for forums
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ModerationSettings {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub require_approval: bool,
    pub auto_lock_after_days: Option<i32>,
    pub max_thread_length: Option<i32>,
    pub allow_anonymous: bool,
    pub rate_limit_posts: Option<i32>, // Posts per hour
    pub banned_words: Vec<String>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Thread model for forum-style discussions with nested replies
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    pub id: Uuid,
    pub forum_id: Uuid,
    pub community_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub is_archived: bool,
    pub is_deleted: bool,
    pub reply_count: i64,
    pub view_count: i64,
    pub upvote_count: i64,
    pub downvote_count: i64,
    pub last_reply_at: Option<DateTime<Utc>>,
    pub last_reply_by: Option<Uuid>,
    pub tags: Vec<String>,
    pub flair: Option<String>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Thread reply model (extends the Comment model for forum-specific features)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ThreadReply {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub parent_reply_id: Option<Uuid>, // For nested replies
    pub thread_depth: i32,
    pub upvote_count: i64,
    pub downvote_count: i64,
    pub is_deleted: bool,
    pub is_moderator_reply: bool,
    pub edit_history: Vec<ThreadReplyEdit>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Edit history for thread replies
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ThreadReplyEdit {
    pub id: Uuid,
    pub reply_id: Uuid,
    pub previous_content: String,
    pub edit_reason: Option<String>,
    #[serde(with = "crate::utils::datetime")]
    pub edited_at: DateTime<Utc>,
}

// Community membership
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMembership {
    pub id: Uuid,
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub role: CommunityRole,
    pub is_banned: bool,
    pub ban_reason: Option<String>,
    pub ban_expires_at: Option<DateTime<Utc>>,
    #[serde(with = "crate::utils::datetime")]
    pub joined_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Community roles
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "community_role", rename_all = "UPPERCASE")]
pub enum CommunityRole {
    Owner,
    Moderator,
    Member,
    Contributor,
    Viewer,
}

// Implementation methods for Community
impl Community {
    /// Creates a new community
    pub fn new(name: String, display_name: String, description: String, owner_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            display_name,
            description,
            icon_url: None,
            banner_url: None,
            owner_id,
            moderator_ids: Vec::new(),
            member_count: 1, // Owner is the first member
            is_private: false,
            is_nsfw: false,
            rules: Vec::new(),
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a moderator to the community
    pub fn add_moderator(&mut self, user_id: Uuid) {
        if !self.moderator_ids.contains(&user_id) && user_id != self.owner_id {
            self.moderator_ids.push(user_id);
            self.touch();
        }
    }

    /// Removes a moderator from the community
    pub fn remove_moderator(&mut self, user_id: Uuid) {
        self.moderator_ids.retain(|&id| id != user_id);
        self.touch();
    }

    /// Adds a rule to the community
    pub fn add_rule(&mut self, title: String, description: String) {
        let rule = CommunityRule {
            id: Uuid::new_v4(),
            community_id: self.id,
            title,
            description,
            order_index: self.rules.len() as i32,
            created_at: Utc::now(),
        };
        self.rules.push(rule);
        self.touch();
    }

    /// Adds a tag to the community
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.touch();
        }
    }

    /// Updates member count
    pub fn update_member_count(&mut self, count: i64) {
        self.member_count = count;
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for Forum
impl Forum {
    /// Creates a new forum
    pub fn new(community_id: Uuid, name: String, description: String, category: ForumCategory) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            community_id,
            name,
            description,
            category,
            is_locked: false,
            is_pinned: false,
            is_archived: false,
            thread_count: 0,
            post_count: 0,
            last_activity_at: None,
            moderator_ids: Vec::new(),
            moderation_settings: ModerationSettings::new(Uuid::new_v4()), // Will be updated with actual forum ID
            created_at: now,
            updated_at: now,
        }
    }

    /// Locks the forum
    pub fn lock(&mut self) {
        self.is_locked = true;
        self.touch();
    }

    /// Unlocks the forum
    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.touch();
    }

    /// Pins the forum
    pub fn pin(&mut self) {
        self.is_pinned = true;
        self.touch();
    }

    /// Unpins the forum
    pub fn unpin(&mut self) {
        self.is_pinned = false;
        self.touch();
    }

    /// Archives the forum
    pub fn archive(&mut self) {
        self.is_archived = true;
        self.touch();
    }

    /// Updates thread and post counts
    pub fn update_counts(&mut self, thread_count: i64, post_count: i64) {
        self.thread_count = thread_count;
        self.post_count = post_count;
        self.last_activity_at = Some(Utc::now());
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for ModerationSettings
impl ModerationSettings {
    /// Creates new moderation settings
    pub fn new(forum_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            forum_id,
            require_approval: false,
            auto_lock_after_days: None,
            max_thread_length: None,
            allow_anonymous: false,
            rate_limit_posts: None,
            banned_words: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a banned word
    pub fn add_banned_word(&mut self, word: String) {
        if !self.banned_words.contains(&word) {
            self.banned_words.push(word);
            self.touch();
        }
    }

    /// Removes a banned word
    pub fn remove_banned_word(&mut self, word: &str) {
        self.banned_words.retain(|w| w != word);
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for Thread
impl Thread {
    /// Creates a new thread
    pub fn new(forum_id: Uuid, community_id: Uuid, author_id: Uuid, title: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            forum_id,
            community_id,
            author_id,
            title,
            content,
            is_pinned: false,
            is_locked: false,
            is_archived: false,
            is_deleted: false,
            reply_count: 0,
            view_count: 0,
            upvote_count: 0,
            downvote_count: 0,
            last_reply_at: None,
            last_reply_by: None,
            tags: Vec::new(),
            flair: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Pins the thread
    pub fn pin(&mut self) {
        self.is_pinned = true;
        self.touch();
    }

    /// Unpins the thread
    pub fn unpin(&mut self) {
        self.is_pinned = false;
        self.touch();
    }

    /// Locks the thread
    pub fn lock(&mut self) {
        self.is_locked = true;
        self.touch();
    }

    /// Unlocks the thread
    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.touch();
    }

    /// Soft deletes the thread
    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }

    /// Restores a soft deleted thread
    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.touch();
    }

    /// Increments view count
    pub fn increment_view_count(&mut self) {
        self.view_count += 1;
        self.touch();
    }

    /// Updates reply information
    pub fn update_last_reply(&mut self, reply_by: Uuid) {
        self.last_reply_at = Some(Utc::now());
        self.last_reply_by = Some(reply_by);
        self.reply_count += 1;
        self.touch();
    }

    /// Adds a tag to the thread
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.touch();
        }
    }

    /// Sets thread flair
    pub fn set_flair(&mut self, flair: String) {
        self.flair = Some(flair);
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for ThreadReply
impl ThreadReply {
    /// Creates a new thread reply
    pub fn new(thread_id: Uuid, author_id: Uuid, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            thread_id,
            author_id,
            content,
            parent_reply_id: None,
            thread_depth: 0,
            upvote_count: 0,
            downvote_count: 0,
            is_deleted: false,
            is_moderator_reply: false,
            edit_history: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a nested reply
    pub fn new_nested_reply(thread_id: Uuid, author_id: Uuid, content: String, parent_reply_id: Uuid, depth: i32) -> Self {
        let mut reply = Self::new(thread_id, author_id, content);
        reply.parent_reply_id = Some(parent_reply_id);
        reply.thread_depth = depth;
        reply
    }

    /// Marks as moderator reply
    pub fn mark_as_moderator_reply(&mut self) {
        self.is_moderator_reply = true;
        self.touch();
    }

    /// Soft deletes the reply
    pub fn soft_delete(&mut self) {
        self.is_deleted = true;
        self.touch();
    }

    /// Restores a soft deleted reply
    pub fn restore(&mut self) {
        self.is_deleted = false;
        self.touch();
    }

    /// Edits the reply content
    pub fn edit_content(&mut self, new_content: String, edit_reason: Option<String>) {
        let edit = ThreadReplyEdit {
            id: Uuid::new_v4(),
            reply_id: self.id,
            previous_content: self.content.clone(),
            edit_reason,
            edited_at: Utc::now(),
        };
        self.edit_history.push(edit);
        self.content = new_content;
        self.touch();
    }

    /// Checks if this is a nested reply
    pub fn is_nested_reply(&self) -> bool {
        self.parent_reply_id.is_some()
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for CommunityMembership
impl CommunityMembership {
    /// Creates a new community membership
    pub fn new(community_id: Uuid, user_id: Uuid, role: CommunityRole) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            community_id,
            user_id,
            role,
            is_banned: false,
            ban_reason: None,
            ban_expires_at: None,
            joined_at: now,
            updated_at: now,
        }
    }

    /// Bans a member
    pub fn ban(&mut self, reason: String, expires_at: Option<DateTime<Utc>>) {
        self.is_banned = true;
        self.ban_reason = Some(reason);
        self.ban_expires_at = expires_at;
        self.touch();
    }

    /// Unbans a member
    pub fn unban(&mut self) {
        self.is_banned = false;
        self.ban_reason = None;
        self.ban_expires_at = None;
        self.touch();
    }

    /// Updates member role
    pub fn update_role(&mut self, new_role: CommunityRole) {
        self.role = new_role;
        self.touch();
    }

    /// Checks if ban has expired
    pub fn is_ban_expired(&self) -> bool {
        if let Some(expires_at) = self.ban_expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}