use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Enhanced relationship models (Follow, Block, Mute)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub is_mutual: bool, // True if both users follow each other
    pub notification_enabled: bool, // Whether to notify about followed user's activity
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub id: Uuid,
    pub blocker_id: Uuid,
    pub blocked_id: Uuid,
    pub block_reason: Option<String>,
    pub is_permanent: bool,
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Mute {
    pub id: Uuid,
    pub muter_id: Uuid,
    pub muted_id: Uuid,
    pub mute_type: MuteType,
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "mute_type", rename_all = "UPPERCASE")]
pub enum MuteType {
    All,        // Mute all activity
    Posts,      // Mute only posts
    Comments,   // Mute only comments
    Mentions,   // Mute only mentions
}

// Notification models for social interactions
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: Uuid,
    pub recipient_id: Uuid,
    pub sender_id: Option<Uuid>, // None for system notifications
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub data: serde_json::Value, // Additional data specific to notification type
    pub is_read: bool,
    pub is_dismissed: bool,
    pub priority: NotificationPriority,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub read_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "notification_type", rename_all = "UPPERCASE")]
pub enum NotificationType {
    // Social notifications
    Follow,
    Unfollow,
    PostLike,
    CommentLike,
    PostComment,
    PostReply,
    PostShare,
    PostRepost,
    Mention,
    
    // Forum notifications
    ThreadReply,
    ThreadUpvote,
    ThreadDownvote,
    ThreadPin,
    ThreadLock,
    ForumMention,
    
    // Community notifications
    CommunityInvite,
    CommunityJoin,
    CommunityLeave,
    CommunityRoleChange,
    CommunityBan,
    
    // System notifications
    SystemAnnouncement,
    SecurityAlert,
    PolicyUpdate,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "notification_priority", rename_all = "UPPERCASE")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

// Feed generation models and algorithms
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feed_type: FeedType,
    pub algorithm: FeedAlgorithm,
    pub last_updated: DateTime<Utc>,
    pub items: Vec<FeedItem>,
    pub settings: FeedSettings,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "feed_type", rename_all = "UPPERCASE")]
pub enum FeedType {
    Home,           // Main feed with followed users
    Discover,       // Algorithmic discovery feed
    Community,      // Community-specific feed
    Forum,          // Forum-specific feed
    Trending,       // Trending content
    Recent,         // Most recent content
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "feed_algorithm", rename_all = "UPPERCASE")]
pub enum FeedAlgorithm {
    Chronological,  // Time-based ordering
    Engagement,     // Based on likes, comments, shares
    Relevance,      // Based on user interests and behavior
    Cooperative,    // Based on cooperative scores
    Mixed,          // Combination of algorithms
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FeedItem {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub content_type: FeedContentType,
    pub content_id: Uuid, // Post ID, Thread ID, etc.
    pub score: f64, // Relevance/ranking score
    pub position: i32,
    pub reason: String, // Why this item was included in feed
    #[serde(with = "crate::utils::datetime")]
    pub added_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "feed_content_type", rename_all = "UPPERCASE")]
pub enum FeedContentType {
    Post,
    Thread,
    Comment,
    Community,
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct FeedSettings {
    pub id: Uuid,
    pub user_id: Uuid,
    pub show_reposts: bool,
    pub show_likes: bool,
    pub show_follows: bool,
    pub nsfw_filter: bool,
    pub language_filter: Vec<String>,
    pub blocked_keywords: Vec<String>,
    pub preferred_content_types: Vec<FeedContentType>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

// Forum-specific interactions (upvote, downvote, pin, lock)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_type: VoteTargetType,
    pub target_id: Uuid, // Thread ID or Reply ID
    pub vote_type: VoteType,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "crate::utils::datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "vote_target_type", rename_all = "UPPERCASE")]
pub enum VoteTargetType {
    Thread,
    ThreadReply,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "vote_type", rename_all = "UPPERCASE")]
pub enum VoteType {
    Upvote,
    Downvote,
}

// Moderation actions (pin, lock, etc.)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ModerationAction {
    pub id: Uuid,
    pub moderator_id: Uuid,
    pub target_type: ModerationTargetType,
    pub target_id: Uuid,
    pub action_type: ModerationActionType,
    pub reason: Option<String>,
    pub duration: Option<i32>, // Duration in hours for temporary actions
    pub is_reversed: bool,
    pub reversed_by: Option<Uuid>,
    pub reversed_at: Option<DateTime<Utc>>,
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "moderation_target_type", rename_all = "UPPERCASE")]
pub enum ModerationTargetType {
    Post,
    Comment,
    Thread,
    ThreadReply,
    User,
    Community,
    Forum,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "moderation_action_type", rename_all = "UPPERCASE")]
pub enum ModerationActionType {
    Pin,
    Unpin,
    Lock,
    Unlock,
    Delete,
    Restore,
    Hide,
    Unhide,
    Ban,
    Unban,
    Mute,
    Unmute,
    Warn,
    Remove,
}

// User activity tracking for feed algorithms
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserActivity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub activity_type: ActivityType,
    pub target_type: String, // "post", "thread", "user", etc.
    pub target_id: Uuid,
    pub metadata: serde_json::Value, // Additional activity-specific data
    #[serde(with = "crate::utils::datetime")]
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "activity_type", rename_all = "UPPERCASE")]
pub enum ActivityType {
    View,
    Like,
    Comment,
    Share,
    Follow,
    Upvote,
    Downvote,
    Join,
    Leave,
    Create,
    Edit,
    Delete,
}

// Implementation methods for Follow
impl Follow {
    /// Creates a new follow relationship
    pub fn new(follower_id: Uuid, followed_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            follower_id,
            followed_id,
            is_mutual: false,
            notification_enabled: true,
            created_at: now,
            updated_at: now,
        }
    }

    /// Sets mutual follow status
    pub fn set_mutual(&mut self, is_mutual: bool) {
        self.is_mutual = is_mutual;
        self.touch();
    }

    /// Toggles notifications
    pub fn toggle_notifications(&mut self) {
        self.notification_enabled = !self.notification_enabled;
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for Block
impl Block {
    /// Creates a new block relationship
    pub fn new(blocker_id: Uuid, blocked_id: Uuid, reason: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            blocker_id,
            blocked_id,
            block_reason: reason,
            is_permanent: true,
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a temporary block
    pub fn new_temporary(blocker_id: Uuid, blocked_id: Uuid, duration_hours: i32, reason: Option<String>) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(duration_hours as i64);
        Self {
            id: Uuid::new_v4(),
            blocker_id,
            blocked_id,
            block_reason: reason,
            is_permanent: false,
            expires_at: Some(expires_at),
            created_at: now,
            updated_at: now,
        }
    }

    /// Checks if block has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Makes block permanent
    pub fn make_permanent(&mut self) {
        self.is_permanent = true;
        self.expires_at = None;
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for Mute
impl Mute {
    /// Creates a new mute relationship
    pub fn new(muter_id: Uuid, muted_id: Uuid, mute_type: MuteType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            muter_id,
            muted_id,
            mute_type,
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a temporary mute
    pub fn new_temporary(muter_id: Uuid, muted_id: Uuid, mute_type: MuteType, duration_hours: i32) -> Self {
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(duration_hours as i64);
        Self {
            id: Uuid::new_v4(),
            muter_id,
            muted_id,
            mute_type,
            expires_at: Some(expires_at),
            created_at: now,
            updated_at: now,
        }
    }

    /// Checks if mute has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
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

// Implementation methods for Notification
impl Notification {
    /// Creates a new notification
    pub fn new(
        recipient_id: Uuid,
        sender_id: Option<Uuid>,
        notification_type: NotificationType,
        title: String,
        message: String,
        data: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            recipient_id,
            sender_id,
            notification_type,
            title,
            message,
            data,
            is_read: false,
            is_dismissed: false,
            priority: NotificationPriority::Normal,
            created_at: Utc::now(),
            read_at: None,
        }
    }

    /// Marks notification as read
    pub fn mark_as_read(&mut self) {
        self.is_read = true;
        self.read_at = Some(Utc::now());
    }

    /// Dismisses notification
    pub fn dismiss(&mut self) {
        self.is_dismissed = true;
    }

    /// Sets notification priority
    pub fn set_priority(&mut self, priority: NotificationPriority) {
        self.priority = priority;
    }
}

// Implementation methods for Vote
impl Vote {
    /// Creates a new upvote
    pub fn new_upvote(user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            target_type,
            target_id,
            vote_type: VoteType::Upvote,
            created_at: now,
            updated_at: now,
        }
    }

    /// Creates a new downvote
    pub fn new_downvote(user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            target_type,
            target_id,
            vote_type: VoteType::Downvote,
            created_at: now,
            updated_at: now,
        }
    }

    /// Changes vote type
    pub fn change_vote_type(&mut self, new_vote_type: VoteType) {
        self.vote_type = new_vote_type;
        self.touch();
    }

    /// Checks if this is an upvote
    pub fn is_upvote(&self) -> bool {
        self.vote_type == VoteType::Upvote
    }

    /// Checks if this is a downvote
    pub fn is_downvote(&self) -> bool {
        self.vote_type == VoteType::Downvote
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

// Implementation methods for ModerationAction
impl ModerationAction {
    /// Creates a new moderation action
    pub fn new(
        moderator_id: Uuid,
        target_type: ModerationTargetType,
        target_id: Uuid,
        action_type: ModerationActionType,
        reason: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            moderator_id,
            target_type,
            target_id,
            action_type,
            reason,
            duration: None,
            is_reversed: false,
            reversed_by: None,
            reversed_at: None,
            created_at: Utc::now(),
        }
    }

    /// Creates a temporary moderation action
    pub fn new_temporary(
        moderator_id: Uuid,
        target_type: ModerationTargetType,
        target_id: Uuid,
        action_type: ModerationActionType,
        duration_hours: i32,
        reason: Option<String>,
    ) -> Self {
        let mut action = Self::new(moderator_id, target_type, target_id, action_type, reason);
        action.duration = Some(duration_hours);
        action
    }

    /// Reverses the moderation action
    pub fn reverse(&mut self, reversed_by: Uuid) {
        self.is_reversed = true;
        self.reversed_by = Some(reversed_by);
        self.reversed_at = Some(Utc::now());
    }

    /// Checks if action has expired
    pub fn is_expired(&self) -> bool {
        if let Some(duration) = self.duration {
            let expires_at = self.created_at + chrono::Duration::hours(duration as i64);
            Utc::now() > expires_at
        } else {
            false
        }
    }
}

// Implementation methods for UserActivity
impl UserActivity {
    /// Creates a new user activity record
    pub fn new(
        user_id: Uuid,
        activity_type: ActivityType,
        target_type: String,
        target_id: Uuid,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            activity_type,
            target_type,
            target_id,
            metadata,
            created_at: Utc::now(),
        }
    }
}

// Implementation methods for Feed
impl Feed {
    /// Creates a new feed
    pub fn new(user_id: Uuid, feed_type: FeedType, algorithm: FeedAlgorithm) -> Self {
        let settings = FeedSettings::new(user_id);
        Self {
            id: Uuid::new_v4(),
            user_id,
            feed_type,
            algorithm,
            last_updated: Utc::now(),
            items: Vec::new(),
            settings,
            created_at: Utc::now(),
        }
    }

    /// Adds an item to the feed
    pub fn add_item(&mut self, content_type: FeedContentType, content_id: Uuid, score: f64, reason: String) {
        let item = FeedItem {
            id: Uuid::new_v4(),
            feed_id: self.id,
            content_type,
            content_id,
            score,
            position: self.items.len() as i32,
            reason,
            added_at: Utc::now(),
        };
        self.items.push(item);
        self.last_updated = Utc::now();
    }

    /// Clears all items from the feed
    pub fn clear_items(&mut self) {
        self.items.clear();
        self.last_updated = Utc::now();
    }

    /// Sorts items by score (highest first)
    pub fn sort_by_score(&mut self) {
        self.items.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        // Update positions after sorting
        for (index, item) in self.items.iter_mut().enumerate() {
            item.position = index as i32;
        }
        self.last_updated = Utc::now();
    }
}

// Implementation methods for FeedSettings
impl FeedSettings {
    /// Creates new feed settings with defaults
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            show_reposts: true,
            show_likes: true,
            show_follows: true,
            nsfw_filter: true,
            language_filter: Vec::new(),
            blocked_keywords: Vec::new(),
            preferred_content_types: vec![FeedContentType::Post, FeedContentType::Thread],
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a blocked keyword
    pub fn add_blocked_keyword(&mut self, keyword: String) {
        if !self.blocked_keywords.contains(&keyword) {
            self.blocked_keywords.push(keyword);
            self.touch();
        }
    }

    /// Removes a blocked keyword
    pub fn remove_blocked_keyword(&mut self, keyword: &str) {
        self.blocked_keywords.retain(|k| k != keyword);
        self.touch();
    }

    /// Updates the updated_at timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}