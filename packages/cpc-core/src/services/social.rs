use crate::{
    models::social::{
        Post, Comment, Like, Share, Repost, Follow, Block, Mute, Notification,
        Visibility, LikeTargetType, ShareType, MuteType, NotificationType,
        FeedType, UserActivity, ActivityType, MediaItem, MediaType, FeedAlgorithm,
        ModerationAction, ModerationActionType, ModerationTargetType, Vote, VoteType, VoteTargetType
    },
    repositories::social_repository::{SocialRepository, CreatePostData, CreateCommentData},
    utils::datetime::now_utc,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
use url::Url;

/// Input data for creating a new post
#[derive(Debug, Clone)]
pub struct CreatePostInput {
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub media_urls: Vec<String>,
    pub tags: Vec<String>,
    pub mentions: Vec<Uuid>,
    pub reply_to_post_id: Option<Uuid>,
    pub repost_of_post_id: Option<Uuid>,
}

/// Input data for creating a comment
#[derive(Debug, Clone)]
pub struct CreateCommentInput {
    pub post_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub mentions: Vec<Uuid>,
}

/// Timeline result with pagination
#[derive(Debug, Clone)]
pub struct TimelineResult {
    pub posts: Vec<Post>,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// Feed generation parameters
#[derive(Debug, Clone)]
pub struct FeedParams {
    pub user_id: Uuid,
    pub feed_type: FeedType,
    pub algorithm: FeedAlgorithm,
    pub limit: i32,
    pub offset: i32,
    pub cooperative_only: bool,
}

/// Content moderation input
#[derive(Debug, Clone)]
pub struct ModerationInput {
    pub moderator_id: Uuid,
    pub target_type: ModerationTargetType,
    pub target_id: Uuid,
    pub action_type: ModerationActionType,
    pub reason: Option<String>,
    pub duration_hours: Option<i32>,
}

/// Content report input
#[derive(Debug, Clone)]
pub struct ReportInput {
    pub reporter_id: Uuid,
    pub target_type: ModerationTargetType,
    pub target_id: Uuid,
    pub reason: String,
    pub description: Option<String>,
}

/// Social interaction statistics
#[derive(Debug, Clone)]
pub struct InteractionStats {
    pub likes_count: i64,
    pub comments_count: i64,
    pub shares_count: i64,
    pub reposts_count: i64,
}

/// Content ranking factors
#[derive(Debug, Clone)]
pub struct RankingFactors {
    pub engagement_score: f64,
    pub recency_score: f64,
    pub relevance_score: f64,
    pub cooperative_score: f64,
    pub final_score: f64,
} 
/// 
SocialService handles all social interaction logic including posts, comments, 
/// feed generation, content ranking, and moderation
pub struct SocialService {
    repository: Box<dyn SocialRepository>,
    allowed_media_domains: HashSet<String>,
    allowed_media_extensions: HashSet<String>,
}

impl SocialService {
    /// Creates a new SocialService instance
    pub fn new(repository: Box<dyn SocialRepository>) -> Self {
        let allowed_media_domains = [
            "media.ourcoop.org",
            "secureusercontent.com",
            "localhost",
        ].iter().map(|s| s.to_string()).collect();

        let allowed_media_extensions = [
            "jpg", "jpeg", "png", "gif", "webp",  // Images
            "mp4", "mov", "webm", "avi",          // Videos
            "mp3", "wav", "ogg", "opus",          // Audio
        ].iter().map(|s| s.to_string()).collect();

        Self {
            repository,
            allowed_media_domains,
            allowed_media_extensions,
        }
    }

    // ===== POST MANAGEMENT =====

    /// Creates a new post with validation and media processing
    pub async fn create_post(&self, input: CreatePostInput, author_id: Uuid) -> Result<Post> {
        // Validate content
        self.validate_post_content(&input.content)?;
        
        // Validate media URLs
        self.validate_media_urls(&input.media_urls)?;
        
        // Extract mentions and tags from content
        let extracted_mentions = self.extract_mentions(&input.content);
        let extracted_tags = self.extract_hashtags(&input.content);
        
        // Combine with explicit mentions and tags
        let mut all_mentions = input.mentions;
        all_mentions.extend(extracted_mentions);
        all_mentions.sort();
        all_mentions.dedup();
        
        let mut all_tags = input.tags;
        all_tags.extend(extracted_tags);
        all_tags.sort();
        all_tags.dedup();

        let create_data = CreatePostData {
            author_id,
            content: input.content,
            visibility: input.visibility,
            cooperative_id: input.cooperative_id,
            tags: all_tags,
            mentions: all_mentions,
            reply_to_post_id: input.reply_to_post_id,
            repost_of_post_id: input.repost_of_post_id,
        };

        let post = self.repository.create_post(create_data).await?;
        
        // Record user activity
        self.record_activity(author_id, ActivityType::Create, "post", post.id).await?;
        
        // Send notifications for mentions
        self.send_mention_notifications(&post, &all_mentions).await?;
        
        Ok(post)
    }

    /// Gets a post by ID with permission checking
    pub async fn get_post(&self, post_id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Post>> {
        let post = self.repository.find_post_by_id(post_id).await?;
        
        match post {
            Some(post) => {
                if self.can_view_post(&post, viewer_id).await? {
                    Ok(Some(post))
                } else {
                    Ok(None) // Hide post if user can't view it
                }
            }
            None => Ok(None),
        }
    }

    /// Updates an existing post
    pub async fn update_post(&self, post_id: Uuid, content: String, editor_id: Uuid) -> Result<Post> {
        let mut post = self.repository.find_post_by_id(post_id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;
        
        // Check permissions
        if post.author_id != editor_id {
            return Err(anyhow!("Not authorized to edit this post"));
        }
        
        // Validate new content
        self.validate_post_content(&content)?;
        
        // Update post with edit history
        post.edit_content(content, Some("User edit".to_string()));
        
        self.repository.update_post(&post).await?;
        
        // Record activity
        self.record_activity(editor_id, ActivityType::Edit, "post", post_id).await?;
        
        Ok(post)
    }

    /// Deletes a post
    pub async fn delete_post(&self, post_id: Uuid, deleter_id: Uuid) -> Result<()> {
        let post = self.repository.find_post_by_id(post_id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;
        
        // Check permissions (author or moderator)
        if post.author_id != deleter_id && !self.is_moderator(deleter_id).await? {
            return Err(anyhow!("Not authorized to delete this post"));
        }
        
        self.repository.delete_post(post_id).await?;
        
        // Record activity
        self.record_activity(deleter_id, ActivityType::Delete, "post", post_id).await?;
        
        Ok(())
    }

    // ===== COMMENT MANAGEMENT =====

    /// Creates a new comment on a post
    pub async fn create_comment(&self, input: CreateCommentInput, author_id: Uuid) -> Result<Comment> {
        // Validate content
        self.validate_comment_content(&input.content)?;
        
        // Check if post exists and is accessible
        let post = self.repository.find_post_by_id(input.post_id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;
        
        if !self.can_view_post(&post, Some(author_id)).await? {
            return Err(anyhow!("Cannot comment on this post"));
        }
        
        // Extract mentions from content
        let extracted_mentions = self.extract_mentions(&input.content);
        let mut all_mentions = input.mentions;
        all_mentions.extend(extracted_mentions);
        all_mentions.sort();
        all_mentions.dedup();

        let create_data = CreateCommentData {
            post_id: input.post_id,
            author_id,
            content: input.content,
            parent_comment_id: input.parent_comment_id,
            mentions: all_mentions,
        };

        let comment = self.repository.create_comment(create_data).await?;
        
        // Record activity
        self.record_activity(author_id, ActivityType::Comment, "post", input.post_id).await?;
        
        // Send notifications
        self.send_comment_notifications(&comment, &post).await?;
        
        Ok(comment)
    }

    /// Gets comments for a post
    pub async fn get_post_comments(&self, post_id: Uuid, limit: i32, offset: i32, viewer_id: Option<Uuid>) -> Result<Vec<Comment>> {
        // Check if post is accessible
        let post = self.repository.find_post_by_id(post_id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;
        
        if !self.can_view_post(&post, viewer_id).await? {
            return Err(anyhow!("Cannot view comments on this post"));
        }
        
        self.repository.get_post_comments(post_id, limit, offset).await
    }

    // ===== SOCIAL INTERACTIONS =====

    /// Likes a post or comment
    pub async fn like_content(&self, user_id: Uuid, target_type: LikeTargetType, target_id: Uuid) -> Result<()> {
        // Check if already liked
        if self.repository.check_user_liked(user_id, target_type, target_id).await? {
            return Err(anyhow!("Content already liked"));
        }
        
        let like = Like::new(user_id, target_type, target_id);
        self.repository.create_like(&like).await?;
        
        // Record activity
        let target_type_str = match target_type {
            LikeTargetType::Post => "post",
            LikeTargetType::Comment => "comment",
        };
        self.record_activity(user_id, ActivityType::Like, target_type_str, target_id).await?;
        
        // Send notification to content author
        self.send_like_notification(&like).await?;
        
        Ok(())
    }

    /// Unlikes a post or comment
    pub async fn unlike_content(&self, user_id: Uuid, target_type: LikeTargetType, target_id: Uuid) -> Result<()> {
        self.repository.remove_like(user_id, target_type, target_id).await?;
        Ok(())
    }

    /// Shares a post
    pub async fn share_post(&self, user_id: Uuid, post_id: Uuid, share_type: ShareType, message: Option<String>) -> Result<()> {
        // Check if post exists and is shareable
        let post = self.repository.find_post_by_id(post_id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;
        
        if !self.can_view_post(&post, Some(user_id)).await? {
            return Err(anyhow!("Cannot share this post"));
        }
        
        let share = Share::new(user_id, post_id, share_type, message);
        self.repository.create_share(&share).await?;
        
        // Record activity
        self.record_activity(user_id, ActivityType::Share, "post", post_id).await?;
        
        Ok(())
    }

    /// Reposts a post
    pub async fn repost(&self, user_id: Uuid, original_post_id: Uuid, message: Option<String>) -> Result<()> {
        // Check if post exists and is repostable
        let post = self.repository.find_post_by_id(original_post_id).await?
            .ok_or_else(|| anyhow!("Post not found"))?;
        
        if !self.can_view_post(&post, Some(user_id)).await? {
            return Err(anyhow!("Cannot repost this post"));
        }
        
        let repost = Repost::new(user_id, original_post_id, message);
        self.repository.create_repost(&repost).await?;
        
        // Record activity
        self.record_activity(user_id, ActivityType::Share, "post", original_post_id).await?;
        
        Ok(())
    }

    // ===== RELATIONSHIP MANAGEMENT =====

    /// Follows a user
    pub async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<()> {
        if follower_id == followed_id {
            return Err(anyhow!("Cannot follow yourself"));
        }
        
        // Check if already following
        if self.repository.check_is_following(follower_id, followed_id).await? {
            return Err(anyhow!("Already following this user"));
        }
        
        let follow = Follow::new(follower_id, followed_id);
        self.repository.create_follow(&follow).await?;
        
        // Record activity
        self.record_activity(follower_id, ActivityType::Follow, "user", followed_id).await?;
        
        // Send notification
        self.send_follow_notification(follower_id, followed_id).await?;
        
        Ok(())
    }

    /// Unfollows a user
    pub async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<()> {
        self.repository.remove_follow(follower_id, followed_id).await?;
        Ok(())
    }

    /// Blocks a user
    pub async fn block_user(&self, blocker_id: Uuid, blocked_id: Uuid, reason: Option<String>) -> Result<()> {
        if blocker_id == blocked_id {
            return Err(anyhow!("Cannot block yourself"));
        }
        
        let block = Block::new(blocker_id, blocked_id, reason);
        self.repository.create_block(&block).await?;
        
        // Also remove any follow relationships
        let _ = self.repository.remove_follow(blocker_id, blocked_id).await;
        let _ = self.repository.remove_follow(blocked_id, blocker_id).await;
        
        Ok(())
    }

    /// Unblocks a user
    pub async fn unblock_user(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<()> {
        self.repository.remove_block(blocker_id, blocked_id).await?;
        Ok(())
    }

    /// Mutes a user
    pub async fn mute_user(&self, muter_id: Uuid, muted_id: Uuid, mute_type: MuteType) -> Result<()> {
        if muter_id == muted_id {
            return Err(anyhow!("Cannot mute yourself"));
        }
        
        let mute = Mute::new(muter_id, muted_id, mute_type);
        self.repository.create_mute(&mute).await?;
        
        Ok(())
    }

    /// Unmutes a user
    pub async fn unmute_user(&self, muter_id: Uuid, muted_id: Uuid) -> Result<()> {
        self.repository.remove_mute(muter_id, muted_id).await?;
        Ok(())
    }

    // ===== FEED GENERATION AND CONTENT RANKING =====

    /// Generates a personalized feed for a user
    pub async fn generate_feed(&self, params: FeedParams) -> Result<TimelineResult> {
        let posts = match params.feed_type {
            FeedType::Home => self.generate_home_feed(&params).await?,
            FeedType::Discover => self.generate_discover_feed(&params).await?,
            FeedType::Trending => self.generate_trending_feed(&params).await?,
            FeedType::Recent => self.generate_recent_feed(&params).await?,
            _ => self.repository.get_recent_posts(params.limit, params.offset).await?,
        };

        // Apply content ranking based on algorithm
        let ranked_posts = self.rank_content(posts, &params).await?;
        
        // Filter out blocked/muted content
        let filtered_posts = self.filter_content(ranked_posts, params.user_id).await?;
        
        let has_more = filtered_posts.len() == params.limit as usize;
        let next_cursor = if has_more {
            filtered_posts.last().map(|p| p.id.to_string())
        } else {
            None
        };

        Ok(TimelineResult {
            posts: filtered_posts,
            has_more,
            next_cursor,
        })
    }

    /// Generates home feed (posts from followed users)
    async fn generate_home_feed(&self, params: &FeedParams) -> Result<Vec<Post>> {
        self.repository.generate_user_feed(
            params.user_id,
            FeedType::Home,
            params.limit,
            params.offset,
        ).await
    }

    /// Generates discover feed (algorithmic content discovery)
    async fn generate_discover_feed(&self, params: &FeedParams) -> Result<Vec<Post>> {
        // Get trending posts and mix with some recent content
        let mut trending = self.repository.get_trending_posts(params.limit / 2, 0).await?;
        let recent = self.repository.get_recent_posts(params.limit / 2, 0).await?;
        
        trending.extend(recent);
        Ok(trending)
    }

    /// Generates trending feed
    async fn generate_trending_feed(&self, params: &FeedParams) -> Result<Vec<Post>> {
        self.repository.get_trending_posts(params.limit, params.offset).await
    }

    /// Generates recent feed
    async fn generate_recent_feed(&self, params: &FeedParams) -> Result<Vec<Post>> {
        self.repository.get_recent_posts(params.limit, params.offset).await
    }

    /// Ranks content based on various factors
    async fn rank_content(&self, mut posts: Vec<Post>, params: &FeedParams) -> Result<Vec<Post>> {
        match params.algorithm {
            FeedAlgorithm::Chronological => {
                // Already sorted by creation time
                Ok(posts)
            }
            FeedAlgorithm::Engagement => {
                // Sort by engagement metrics
                for post in &mut posts {
                    let stats = self.get_interaction_stats(post.id).await?;
                    let engagement_score = (stats.likes_count as f64 * 1.0) +
                                         (stats.comments_count as f64 * 2.0) +
                                         (stats.shares_count as f64 * 3.0) +
                                         (stats.reposts_count as f64 * 2.5);
                    
                    // Store score in feed_position for sorting (temporary hack)
                    post.feed_position = Some(engagement_score as i32);
                }
                
                posts.sort_by(|a, b| {
                    b.feed_position.unwrap_or(0).cmp(&a.feed_position.unwrap_or(0))
                });
                
                Ok(posts)
            }
            FeedAlgorithm::Relevance => {
                // TODO: Implement relevance-based ranking using user interests
                Ok(posts)
            }
            FeedAlgorithm::Cooperative => {
                // TODO: Implement cooperative score-based ranking
                Ok(posts)
            }
            FeedAlgorithm::Mixed => {
                // TODO: Implement mixed algorithm
                Ok(posts)
            }
        }
    }

    /// Filters content based on user preferences and blocks/mutes
    async fn filter_content(&self, posts: Vec<Post>, user_id: Uuid) -> Result<Vec<Post>> {
        let mut filtered_posts = Vec::new();
        
        // Get user's blocks and mutes
        let blocks = self.repository.get_user_blocks(user_id).await?;
        let mutes = self.repository.get_user_mutes(user_id).await?;
        
        let blocked_users: HashSet<Uuid> = blocks.iter().map(|b| b.blocked_id).collect();
        let muted_users: HashSet<Uuid> = mutes.iter().map(|m| m.muted_id).collect();
        
        for post in posts {
            // Skip posts from blocked users
            if blocked_users.contains(&post.author_id) {
                continue;
            }
            
            // Skip posts from muted users (depending on mute type)
            if muted_users.contains(&post.author_id) {
                // TODO: Check mute type and filter accordingly
                continue;
            }
            
            filtered_posts.push(post);
        }
        
        Ok(filtered_posts)
    }

    /// Gets interaction statistics for a post
    async fn get_interaction_stats(&self, post_id: Uuid) -> Result<InteractionStats> {
        let likes = self.repository.get_post_likes(post_id).await?;
        let comments = self.repository.get_post_comments(post_id, 1000, 0).await?; // Get all comments for count
        let shares = self.repository.get_post_shares(post_id).await?;
        let reposts = self.repository.get_post_reposts(post_id).await?;
        
        Ok(InteractionStats {
            likes_count: likes.len() as i64,
            comments_count: comments.len() as i64,
            shares_count: shares.len() as i64,
            reposts_count: reposts.len() as i64,
        })
    }

    // ===== CONTENT MODERATION =====

    /// Performs a moderation action
    pub async fn moderate_content(&self, input: ModerationInput) -> Result<()> {
        // Check if user is a moderator
        if !self.is_moderator(input.moderator_id).await? {
            return Err(anyhow!("Not authorized to perform moderation actions"));
        }
        
        let action = if let Some(duration) = input.duration_hours {
            ModerationAction::new_temporary(
                input.moderator_id,
                input.target_type,
                input.target_id,
                input.action_type,
                duration,
                input.reason,
            )
        } else {
            ModerationAction::new(
                input.moderator_id,
                input.target_type,
                input.target_id,
                input.action_type,
                input.reason,
            )
        };
        
        // Apply the moderation action
        match input.action_type {
            ModerationActionType::Delete => {
                match input.target_type {
                    ModerationTargetType::Post => {
                        self.repository.delete_post(input.target_id).await?;
                    }
                    ModerationTargetType::Comment => {
                        self.repository.delete_comment(input.target_id).await?;
                    }
                    _ => return Err(anyhow!("Unsupported moderation target for delete action")),
                }
            }
            ModerationActionType::Hide => {
                // TODO: Implement content hiding
            }
            ModerationActionType::Ban => {
                // TODO: Implement user banning
            }
            _ => {
                // TODO: Implement other moderation actions
            }
        }
        
        // Record the moderation action
        // TODO: Store moderation actions in database
        
        Ok(())
    }

    /// Reports content for moderation
    pub async fn report_content(&self, input: ReportInput) -> Result<()> {
        // TODO: Implement content reporting system
        // This would typically:
        // 1. Store the report in a moderation queue
        // 2. Notify moderators
        // 3. Apply automatic actions for severe violations
        
        Ok(())
    }

    // ===== HELPER METHODS =====

    /// Validates post content
    fn validate_post_content(&self, content: &str) -> Result<()> {
        if content.trim().is_empty() {
            return Err(anyhow!("Post content cannot be empty"));
        }
        
        if content.len() > 5000 {
            return Err(anyhow!("Post content too long (max 5000 characters)"));
        }
        
        // TODO: Add more content validation (spam detection, etc.)
        
        Ok(())
    }

    /// Validates comment content
    fn validate_comment_content(&self, content: &str) -> Result<()> {
        if content.trim().is_empty() {
            return Err(anyhow!("Comment content cannot be empty"));
        }
        
        if content.len() > 2000 {
            return Err(anyhow!("Comment content too long (max 2000 characters)"));
        }
        
        Ok(())
    }

    /// Validates media URLs
    fn validate_media_urls(&self, urls: &[String]) -> Result<()> {
        for url in urls {
            let parsed_url = Url::parse(url)
                .map_err(|_| anyhow!("Invalid media URL: {}", url))?;
            
            // Check domain
            if let Some(host) = parsed_url.host_str() {
                if !self.allowed_media_domains.contains(host) {
                    return Err(anyhow!("Media host not allowed: {}", host));
                }
            }
            
            // Check scheme
            if !matches!(parsed_url.scheme(), "http" | "https") {
                return Err(anyhow!("Invalid URL scheme: {}", parsed_url.scheme()));
            }
            
            // Check file extension
            let path = parsed_url.path();
            if let Some(extension) = path.split('.').last() {
                if !self.allowed_media_extensions.contains(&extension.to_lowercase()) {
                    return Err(anyhow!("Unsupported media type: {}", extension));
                }
            }
        }
        
        Ok(())
    }

    /// Extracts @mentions from content
    fn extract_mentions(&self, content: &str) -> Vec<Uuid> {
        // TODO: Implement mention extraction and user lookup
        // This would parse @username patterns and resolve them to UUIDs
        Vec::new()
    }

    /// Extracts #hashtags from content
    fn extract_hashtags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        let words: Vec<&str> = content.split_whitespace().collect();
        
        for word in words {
            if word.starts_with('#') && word.len() > 1 {
                let tag = word[1..].to_lowercase();
                if !tag.is_empty() {
                    tags.push(tag);
                }
            }
        }
        
        tags
    }

    /// Checks if a user can view a post based on visibility and relationships
    async fn can_view_post(&self, post: &Post, viewer_id: Option<Uuid>) -> Result<bool> {
        match post.visibility {
            Visibility::Public => Ok(true),
            Visibility::Private => {
                match viewer_id {
                    Some(viewer_id) => Ok(viewer_id == post.author_id),
                    None => Ok(false),
                }
            }
            Visibility::Cooperative => {
                match (viewer_id, post.cooperative_id) {
                    (Some(viewer_id), Some(_cooperative_id)) => {
                        // TODO: Check if viewer is member of the cooperative
                        Ok(viewer_id == post.author_id)
                    }
                    _ => Ok(false),
                }
            }
        }
    }

    /// Checks if a user is a moderator
    async fn is_moderator(&self, _user_id: Uuid) -> Result<bool> {
        // TODO: Implement moderator checking
        // This would check user roles/permissions
        Ok(false)
    }

    /// Records user activity for analytics and feed algorithms
    async fn record_activity(&self, user_id: Uuid, activity_type: ActivityType, target_type: &str, target_id: Uuid) -> Result<()> {
        let activity = UserActivity::new(
            user_id,
            activity_type,
            target_type.to_string(),
            target_id,
            serde_json::json!({}),
        );
        
        self.repository.record_user_activity(&activity).await?;
        Ok(())
    }

    /// Sends notifications for mentions in posts
    async fn send_mention_notifications(&self, _post: &Post, _mentions: &[Uuid]) -> Result<()> {
        // TODO: Implement mention notifications
        Ok(())
    }

    /// Sends notification when someone comments on a post
    async fn send_comment_notifications(&self, _comment: &Comment, _post: &Post) -> Result<()> {
        // TODO: Implement comment notifications
        Ok(())
    }

    /// Sends notification when someone likes content
    async fn send_like_notification(&self, _like: &Like) -> Result<()> {
        // TODO: Implement like notifications
        Ok(())
    }

    /// Sends notification when someone follows a user
    async fn send_follow_notification(&self, _follower_id: Uuid, _followed_id: Uuid) -> Result<()> {
        // TODO: Implement follow notifications
        Ok(())
    }
}