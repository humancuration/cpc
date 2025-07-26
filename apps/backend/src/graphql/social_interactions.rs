use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use futures_util::{Stream, StreamExt};
use cpc_core::models::social::{
    Post, Comment, Reply, Like, Share, Repost, MediaItem, Visibility, MediaType,
    ProcessingStatus, LikeTargetType, ShareType, Follow, Block, Mute, MuteType,
    Notification, NotificationType, NotificationPriority, Feed, FeedType, FeedAlgorithm,
    FeedItem, FeedContentType, FeedSettings
};

/// GraphQL representation of a Post
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct PostType {
    pub id: ID,
    pub author_id: ID,
    pub content: String,
    pub visibility: VisibilityType,
    pub cooperative_id: Option<ID>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub like_count: i32,
    pub share_count: i32,
    pub comment_count: i32,
    pub is_edited: bool,
}

#[ComplexObject]
impl PostType {
    /// Get post author
    async fn author(&self, ctx: &Context<'_>) -> Result<Option<super::user_management::UserType>> {
        // TODO: Implement author loading via service
        Ok(None)
    }

    /// Get post media items
    async fn media(&self, ctx: &Context<'_>) -> Result<Vec<MediaItemType>> {
        // TODO: Implement media loading via service
        Ok(vec![])
    }

    /// Get post comments
    async fn comments(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<CommentType>> {
        // TODO: Implement comments loading via service
        Ok(vec![])
    }

    /// Get post likes
    async fn likes(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<LikeType>> {
        // TODO: Implement likes loading via service
        Ok(vec![])
    }

    /// Get post shares
    async fn shares(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<ShareType>> {
        // TODO: Implement shares loading via service
        Ok(vec![])
    }

    /// Check if current user has liked this post
    async fn is_liked_by_me(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement like check via service
        Ok(false)
    }

    /// Check if current user has shared this post
    async fn is_shared_by_me(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement share check via service
        Ok(false)
    }
}

/// GraphQL enum for Visibility
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VisibilityType {
    Public,
    Cooperative,
    Followers,
    Private,
}

/// GraphQL representation of MediaItem
#[derive(SimpleObject, Clone)]
pub struct MediaItemType {
    pub id: ID,
    pub post_id: ID,
    pub media_type: MediaTypeGraphQL,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub alt_text: Option<String>,
    pub processing_status: ProcessingStatusType,
    pub file_size: Option<i64>,
    pub duration: Option<f64>,
    pub created_at: DateTime<Utc>,
}

/// GraphQL enum for MediaType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum MediaTypeGraphQL {
    Image,
    Video,
    Audio,
    Document,
}

/// GraphQL enum for ProcessingStatus
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ProcessingStatusType {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// GraphQL representation of Comment
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct CommentType {
    pub id: ID,
    pub post_id: ID,
    pub author_id: ID,
    pub content: String,
    pub parent_comment_id: Option<ID>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub like_count: i32,
    pub reply_count: i32,
    pub is_edited: bool,
}

#[ComplexObject]
impl CommentType {
    /// Get comment author
    async fn author(&self, ctx: &Context<'_>) -> Result<Option<super::user_management::UserType>> {
        // TODO: Implement author loading via service
        Ok(None)
    }

    /// Get comment replies
    async fn replies(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<CommentType>> {
        // TODO: Implement replies loading via service
        Ok(vec![])
    }

    /// Get comment likes
    async fn likes(&self, ctx: &Context<'_>) -> Result<Vec<LikeType>> {
        // TODO: Implement likes loading via service
        Ok(vec![])
    }

    /// Check if current user has liked this comment
    async fn is_liked_by_me(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement like check via service
        Ok(false)
    }
}

/// GraphQL representation of Like
#[derive(SimpleObject, Clone)]
pub struct LikeType {
    pub id: ID,
    pub user_id: ID,
    pub target_id: ID,
    pub target_type: LikeTargetTypeGraphQL,
    pub created_at: DateTime<Utc>,
}

/// GraphQL enum for LikeTargetType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum LikeTargetTypeGraphQL {
    Post,
    Comment,
    Reply,
}

/// GraphQL representation of Share
#[derive(SimpleObject, Clone)]
pub struct ShareType {
    pub id: ID,
    pub user_id: ID,
    pub post_id: ID,
    pub share_type: ShareTypeGraphQL,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// GraphQL enum for ShareType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ShareTypeGraphQL {
    Repost,
    Quote,
    External,
}

/// GraphQL representation of Feed
#[derive(SimpleObject, Clone)]
pub struct FeedType {
    pub id: ID,
    pub user_id: ID,
    pub feed_type: FeedTypeGraphQL,
    pub algorithm: FeedAlgorithmType,
    pub settings: FeedSettingsType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// GraphQL enum for FeedType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FeedTypeGraphQL {
    Home,
    Following,
    Cooperative,
    Trending,
    Local,
}

/// GraphQL enum for FeedAlgorithm
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FeedAlgorithmType {
    Chronological,
    Algorithmic,
    Cooperative,
    Trending,
}

/// GraphQL representation of FeedSettings
#[derive(SimpleObject, Clone)]
pub struct FeedSettingsType {
    pub show_reposts: bool,
    pub show_replies: bool,
    pub content_filters: Vec<String>,
    pub cooperative_weight: f64,
    pub time_decay_factor: f64,
}

/// GraphQL representation of FeedItem
#[derive(SimpleObject, Clone)]
pub struct FeedItemType {
    pub id: ID,
    pub feed_id: ID,
    pub content_id: ID,
    pub content_type: FeedContentTypeGraphQL,
    pub score: f64,
    pub created_at: DateTime<Utc>,
    pub post: Option<PostType>,
}

/// GraphQL enum for FeedContentType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FeedContentTypeGraphQL {
    Post,
    Repost,
    Comment,
    Like,
}

/// GraphQL representation of Notification
#[derive(SimpleObject, Clone)]
pub struct NotificationType {
    pub id: ID,
    pub user_id: ID,
    pub notification_type: NotificationTypeGraphQL,
    pub title: String,
    pub message: String,
    pub priority: NotificationPriorityType,
    pub is_read: bool,
    pub related_id: Option<ID>,
    pub created_at: DateTime<Utc>,
}

/// GraphQL enum for NotificationType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum NotificationTypeGraphQL {
    Like,
    Comment,
    Follow,
    Mention,
    Repost,
    System,
}

/// GraphQL enum for NotificationPriority
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum NotificationPriorityType {
    Low,
    Normal,
    High,
    Urgent,
}

/// Input for creating a post
#[derive(InputObject)]
pub struct CreatePostInput {
    pub content: String,
    pub visibility: VisibilityType,
    pub cooperative_id: Option<ID>,
    pub media_ids: Vec<ID>,
}

/// Input for updating a post
#[derive(InputObject)]
pub struct UpdatePostInput {
    pub post_id: ID,
    pub content: String,
}

/// Input for creating a comment
#[derive(InputObject)]
pub struct CreateCommentInput {
    pub post_id: ID,
    pub content: String,
    pub parent_comment_id: Option<ID>,
}

/// Input for updating a comment
#[derive(InputObject)]
pub struct UpdateCommentInput {
    pub comment_id: ID,
    pub content: String,
}

/// Input for feed settings
#[derive(InputObject)]
pub struct FeedSettingsInput {
    pub show_reposts: Option<bool>,
    pub show_replies: Option<bool>,
    pub content_filters: Option<Vec<String>>,
    pub cooperative_weight: Option<f64>,
    pub time_decay_factor: Option<f64>,
}

/// Input for feed pagination
#[derive(InputObject)]
pub struct FeedPaginationInput {
    pub limit: i32,
    pub offset: Option<i32>,
    pub after: Option<ID>,
    pub before: Option<ID>,
}

/// Social interactions queries
#[derive(Default)]
pub struct SocialQuery;

#[Object]
impl SocialQuery {
    /// Get post by ID
    async fn post(&self, ctx: &Context<'_>, id: ID) -> Result<Option<PostType>> {
        let post_id = Uuid::parse_str(&id.to_string())?;
        let social_service = ctx.data::<std::sync::Arc<crate::services::social_service::SocialService>>()?;
        
        match social_service.get_post(post_id).await {
            Ok(post) => Ok(Some(post.into())),
            Err(crate::services::social_service::SocialServiceError::PostNotFound) => Ok(None),
            Err(e) => Err(format!("Failed to get post: {:?}", e).into()),
        }
    }

    /// Get posts by user
    async fn posts_by_user(
        &self,
        ctx: &Context<'_>,
        user_id: ID,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<PostType>> {
        let user_uuid = Uuid::parse_str(&user_id.to_string())?;
        let social_service = ctx.data::<std::sync::Arc<crate::services::social_service::SocialService>>()?;
        
        match social_service.get_posts_by_user(user_uuid).await {
            Ok(posts) => {
                let mut result: Vec<PostType> = posts.into_iter().map(Into::into).collect();
                
                // Apply pagination
                if let Some(offset) = offset {
                    if offset > 0 {
                        result = result.into_iter().skip(offset as usize).collect();
                    }
                }
                
                if let Some(limit) = limit {
                    if limit > 0 {
                        result.truncate(limit as usize);
                    }
                }
                
                Ok(result)
            },
            Err(e) => Err(format!("Failed to get user posts: {:?}", e).into()),
        }
    }

    /// Get user's feed
    async fn feed(
        &self,
        ctx: &Context<'_>,
        feed_type: FeedTypeGraphQL,
        pagination: Option<FeedPaginationInput>,
    ) -> Result<Vec<FeedItemType>> {
        // TODO: Implement feed retrieval
        Ok(vec![])
    }

    /// Get trending posts
    async fn trending_posts(
        &self,
        ctx: &Context<'_>,
        time_range: Option<String>,
        limit: Option<i32>,
    ) -> Result<Vec<PostType>> {
        // TODO: Implement trending posts retrieval
        Ok(vec![])
    }

    /// Search posts
    async fn search_posts(
        &self,
        ctx: &Context<'_>,
        query: String,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<PostType>> {
        // TODO: Implement post search
        Ok(vec![])
    }

    /// Get user's notifications
    async fn notifications(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        unread_only: Option<bool>,
    ) -> Result<Vec<NotificationType>> {
        // TODO: Implement notifications retrieval
        Ok(vec![])
    }

    /// Get notification count
    async fn notification_count(&self, ctx: &Context<'_>, unread_only: Option<bool>) -> Result<i32> {
        // TODO: Implement notification count
        Ok(0)
    }

    /// Get user's feed settings
    async fn feed_settings(&self, ctx: &Context<'_>) -> Result<Option<FeedSettingsType>> {
        // TODO: Implement feed settings retrieval
        Ok(None)
    }
}

/// Social interactions mutations
#[derive(Default)]
pub struct SocialMutation;

#[Object]
impl SocialMutation {
    /// Create a new post
    async fn create_post(&self, ctx: &Context<'_>, input: CreatePostInput) -> Result<PostType> {
        // Extract user ID from authentication context
        let auth_data = ctx.data::<crate::auth::AuthData>()?;
        let social_service = ctx.data::<std::sync::Arc<crate::services::social_service::SocialService>>()?;
        
        let cooperative_id = input.cooperative_id.map(|id| Uuid::parse_str(&id.to_string())).transpose()?;
        
        match social_service.create_post(
            auth_data.user_id,
            input.content,
            input.visibility.into(),
        ).await {
            Ok(post) => {
                let post_type: PostType = post.into();
                
                // Publish the new post event for subscriptions
                crate::graphql::subscription_events::SubscriptionEventPublisher::publish_post_updated(post_type.clone());
                
                Ok(post_type)
            },
            Err(e) => Err(format!("Failed to create post: {:?}", e).into()),
        }
    }

    /// Update a post
    async fn update_post(&self, ctx: &Context<'_>, input: UpdatePostInput) -> Result<PostType> {
        // TODO: Implement post update
        Err("Not implemented".into())
    }

    /// Delete a post
    async fn delete_post(&self, ctx: &Context<'_>, post_id: ID) -> Result<bool> {
        // TODO: Implement post deletion
        Err("Not implemented".into())
    }

    /// Like a post or comment
    async fn like(&self, ctx: &Context<'_>, target_id: ID, target_type: LikeTargetTypeGraphQL) -> Result<LikeType> {
        // TODO: Implement like functionality
        Err("Not implemented".into())
    }

    /// Unlike a post or comment
    async fn unlike(&self, ctx: &Context<'_>, target_id: ID, target_type: LikeTargetTypeGraphQL) -> Result<bool> {
        // TODO: Implement unlike functionality
        Err("Not implemented".into())
    }

    /// Share a post
    async fn share_post(
        &self,
        ctx: &Context<'_>,
        post_id: ID,
        share_type: ShareTypeGraphQL,
        comment: Option<String>,
    ) -> Result<ShareType> {
        // TODO: Implement post sharing
        Err("Not implemented".into())
    }

    /// Create a comment
    async fn create_comment(&self, ctx: &Context<'_>, input: CreateCommentInput) -> Result<CommentType> {
        // TODO: Implement comment creation
        Err("Not implemented".into())
    }

    /// Update a comment
    async fn update_comment(&self, ctx: &Context<'_>, input: UpdateCommentInput) -> Result<CommentType> {
        // TODO: Implement comment update
        Err("Not implemented".into())
    }

    /// Delete a comment
    async fn delete_comment(&self, ctx: &Context<'_>, comment_id: ID) -> Result<bool> {
        // TODO: Implement comment deletion
        Err("Not implemented".into())
    }

    /// Update feed settings
    async fn update_feed_settings(&self, ctx: &Context<'_>, input: FeedSettingsInput) -> Result<FeedSettingsType> {
        // TODO: Implement feed settings update
        Err("Not implemented".into())
    }

    /// Mark notification as read
    async fn mark_notification_read(&self, ctx: &Context<'_>, notification_id: ID) -> Result<bool> {
        // TODO: Implement notification marking
        Err("Not implemented".into())
    }

    /// Mark all notifications as read
    async fn mark_all_notifications_read(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement all notifications marking
        Err("Not implemented".into())
    }
}

/// Social interactions subscriptions
#[derive(Default)]
pub struct SocialSubscription;

#[Subscription]
impl SocialSubscription {
    /// Subscribe to new posts in feed
    async fn feed_updates(&self, ctx: &Context<'_>, feed_type: FeedTypeGraphQL) -> Result<impl Stream<Item = FeedItemType>> {
        // Create a subscription stream for feed updates
        Ok(async_graphql_simple_broker::SimpleBroker::<FeedItemType>::subscribe()
            .filter(move |feed_item| {
                // Filter feed items by feed type - would need to enhance FeedItemType to include feed_type
                async move { true } // Placeholder
            }))
    }

    /// Subscribe to post updates
    async fn post_updated(&self, ctx: &Context<'_>, post_id: ID) -> Result<impl Stream<Item = PostType>> {
        let post_uuid = Uuid::parse_str(&post_id.to_string())?;
        
        // Create a subscription stream for post updates
        Ok(async_graphql_simple_broker::SimpleBroker::<PostType>::subscribe()
            .filter(move |post| {
                let updated_post_id = Uuid::parse_str(&post.id.to_string()).unwrap_or_default();
                async move { updated_post_id == post_uuid }
            }))
    }

    /// Subscribe to new comments on a post
    async fn post_comments(&self, ctx: &Context<'_>, post_id: ID) -> Result<impl Stream<Item = CommentType>> {
        let post_uuid = Uuid::parse_str(&post_id.to_string())?;
        
        // Create a subscription stream for new comments
        Ok(async_graphql_simple_broker::SimpleBroker::<CommentType>::subscribe()
            .filter(move |comment| {
                let comment_post_id = Uuid::parse_str(&comment.post_id.to_string()).unwrap_or_default();
                async move { comment_post_id == post_uuid }
            }))
    }

    /// Subscribe to new likes on a post
    async fn post_likes(&self, ctx: &Context<'_>, post_id: ID) -> Result<impl Stream<Item = LikeType>> {
        let post_uuid = Uuid::parse_str(&post_id.to_string())?;
        
        // Create a subscription stream for new likes
        Ok(async_graphql_simple_broker::SimpleBroker::<LikeType>::subscribe()
            .filter(move |like| {
                let like_target_id = Uuid::parse_str(&like.target_id.to_string()).unwrap_or_default();
                async move { 
                    like_target_id == post_uuid && like.target_type == LikeTargetTypeGraphQL::Post
                }
            }))
    }

    /// Subscribe to user notifications
    async fn notifications(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = NotificationType>> {
        // Extract user ID from authentication context
        let auth_data = ctx.data::<crate::auth::AuthData>()?;
        let user_id = auth_data.user_id;
        
        // Create a subscription stream for user notifications
        Ok(async_graphql_simple_broker::SimpleBroker::<NotificationType>::subscribe()
            .filter(move |notification| {
                let notification_user_id = Uuid::parse_str(&notification.user_id.to_string()).unwrap_or_default();
                async move { notification_user_id == user_id }
            }))
    }
}

// Conversion implementations
impl From<Visibility> for VisibilityType {
    fn from(visibility: Visibility) -> Self {
        match visibility {
            Visibility::Public => VisibilityType::Public,
            Visibility::Cooperative => VisibilityType::Cooperative,
            Visibility::Followers => VisibilityType::Followers,
            Visibility::Private => VisibilityType::Private,
        }
    }
}

impl From<VisibilityType> for Visibility {
    fn from(visibility: VisibilityType) -> Self {
        match visibility {
            VisibilityType::Public => Visibility::Public,
            VisibilityType::Cooperative => Visibility::Cooperative,
            VisibilityType::Followers => Visibility::Followers,
            VisibilityType::Private => Visibility::Private,
        }
    }
}

impl From<MediaType> for MediaTypeGraphQL {
    fn from(media_type: MediaType) -> Self {
        match media_type {
            MediaType::Image => MediaTypeGraphQL::Image,
            MediaType::Video => MediaTypeGraphQL::Video,
            MediaType::Audio => MediaTypeGraphQL::Audio,
            MediaType::Document => MediaTypeGraphQL::Document,
        }
    }
}

impl From<ProcessingStatus> for ProcessingStatusType {
    fn from(status: ProcessingStatus) -> Self {
        match status {
            ProcessingStatus::Pending => ProcessingStatusType::Pending,
            ProcessingStatus::Processing => ProcessingStatusType::Processing,
            ProcessingStatus::Completed => ProcessingStatusType::Completed,
            ProcessingStatus::Failed => ProcessingStatusType::Failed,
        }
    }
}

impl From<LikeTargetType> for LikeTargetTypeGraphQL {
    fn from(target_type: LikeTargetType) -> Self {
        match target_type {
            LikeTargetType::Post => LikeTargetTypeGraphQL::Post,
            LikeTargetType::Comment => LikeTargetTypeGraphQL::Comment,
            LikeTargetType::Reply => LikeTargetTypeGraphQL::Reply,
        }
    }
}

impl From<ShareType> for ShareTypeGraphQL {
    fn from(share_type: ShareType) -> Self {
        match share_type {
            ShareType::Repost => ShareTypeGraphQL::Repost,
            ShareType::Quote => ShareTypeGraphQL::Quote,
            ShareType::External => ShareTypeGraphQL::External,
        }
    }
}

impl From<FeedType> for FeedTypeGraphQL {
    fn from(feed_type: FeedType) -> Self {
        match feed_type {
            FeedType::Home => FeedTypeGraphQL::Home,
            FeedType::Following => FeedTypeGraphQL::Following,
            FeedType::Cooperative => FeedTypeGraphQL::Cooperative,
            FeedType::Trending => FeedTypeGraphQL::Trending,
            FeedType::Local => FeedTypeGraphQL::Local,
        }
    }
}

impl From<FeedAlgorithm> for FeedAlgorithmType {
    fn from(algorithm: FeedAlgorithm) -> Self {
        match algorithm {
            FeedAlgorithm::Chronological => FeedAlgorithmType::Chronological,
            FeedAlgorithm::Algorithmic => FeedAlgorithmType::Algorithmic,
            FeedAlgorithm::Cooperative => FeedAlgorithmType::Cooperative,
            FeedAlgorithm::Trending => FeedAlgorithmType::Trending,
        }
    }
}

impl From<FeedContentType> for FeedContentTypeGraphQL {
    fn from(content_type: FeedContentType) -> Self {
        match content_type {
            FeedContentType::Post => FeedContentTypeGraphQL::Post,
            FeedContentType::Repost => FeedContentTypeGraphQL::Repost,
            FeedContentType::Comment => FeedContentTypeGraphQL::Comment,
            FeedContentType::Like => FeedContentTypeGraphQL::Like,
        }
    }
}

impl From<NotificationType> for NotificationTypeGraphQL {
    fn from(notification_type: NotificationType) -> Self {
        match notification_type {
            NotificationType::Like => NotificationTypeGraphQL::Like,
            NotificationType::Comment => NotificationTypeGraphQL::Comment,
            NotificationType::Follow => NotificationTypeGraphQL::Follow,
            NotificationType::Mention => NotificationTypeGraphQL::Mention,
            NotificationType::Repost => NotificationTypeGraphQL::Repost,
            NotificationType::System => NotificationTypeGraphQL::System,
        }
    }
}

impl From<NotificationPriority> for NotificationPriorityType {
    fn from(priority: NotificationPriority) -> Self {
        match priority {
            NotificationPriority::Low => NotificationPriorityType::Low,
            NotificationPriority::Normal => NotificationPriorityType::Normal,
            NotificationPriority::High => NotificationPriorityType::High,
            NotificationPriority::Urgent => NotificationPriorityType::Urgent,
        }
    }
}

// Conversion implementations for core types
impl From<Post> for PostType {
    fn from(post: Post) -> Self {
        Self {
            id: post.id.into(),
            author_id: post.author_id.into(),
            content: post.content,
            visibility: post.visibility.into(),
            cooperative_id: post.cooperative_id.map(Into::into),
            created_at: post.created_at,
            updated_at: post.updated_at,
            like_count: post.like_count,
            share_count: post.share_count,
            comment_count: post.comment_count,
            is_edited: post.is_edited,
        }
    }
}

impl From<Comment> for CommentType {
    fn from(comment: Comment) -> Self {
        Self {
            id: comment.id.into(),
            post_id: comment.post_id.into(),
            author_id: comment.author_id.into(),
            content: comment.content,
            parent_comment_id: comment.parent_comment_id.map(Into::into),
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            like_count: comment.like_count,
            reply_count: comment.reply_count,
            is_edited: comment.is_edited,
        }
    }
}

impl From<Like> for LikeType {
    fn from(like: Like) -> Self {
        Self {
            id: like.id.into(),
            user_id: like.user_id.into(),
            target_id: like.target_id.into(),
            target_type: like.target_type.into(),
            created_at: like.created_at,
        }
    }
}

impl From<Share> for ShareType {
    fn from(share: Share) -> Self {
        Self {
            id: share.id.into(),
            user_id: share.user_id.into(),
            post_id: share.post_id.into(),
            share_type: share.share_type.into(),
            comment: share.comment,
            created_at: share.created_at,
        }
    }
}

impl From<MediaItem> for MediaItemType {
    fn from(media: MediaItem) -> Self {
        Self {
            id: media.id.into(),
            post_id: media.post_id.into(),
            media_type: media.media_type.into(),
            url: media.url,
            thumbnail_url: media.thumbnail_url,
            alt_text: media.alt_text,
            processing_status: media.processing_status.into(),
            file_size: media.file_size,
            duration: media.duration,
            created_at: media.created_at,
        }
    }
}