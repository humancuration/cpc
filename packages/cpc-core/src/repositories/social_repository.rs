use crate::models::social::{
    Post, Comment, Like, Share, Repost, MediaItem, Reply, PostEdit, CommentEdit,
    Follow, Block, Mute, Notification, Vote, UserActivity, Feed, FeedItem,
    Visibility, LikeTargetType, ShareType, MuteType, NotificationType,
    VoteTargetType, VoteType, ActivityType, FeedType, FeedAlgorithm, FeedContentType
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Data structure for creating posts
pub struct CreatePostData {
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub mentions: Vec<Uuid>,
    pub reply_to_post_id: Option<Uuid>,
    pub repost_of_post_id: Option<Uuid>,
}

/// Data structure for creating comments
pub struct CreateCommentData {
    pub post_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub parent_comment_id: Option<Uuid>,
    pub mentions: Vec<Uuid>,
}

#[async_trait]
pub trait SocialRepository: Send + Sync {
    // Post operations
    async fn create_post(&self, data: CreatePostData) -> Result<Post>;
    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>>;
    async fn update_post(&self, post: &Post) -> Result<()>;
    async fn delete_post(&self, id: Uuid) -> Result<()>;
    async fn get_user_posts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>>;
    async fn get_cooperative_posts(&self, cooperative_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>>;
    async fn search_posts(&self, query: &str, limit: i32, offset: i32) -> Result<Vec<Post>>;
    
    // Comment operations
    async fn create_comment(&self, data: CreateCommentData) -> Result<Comment>;
    async fn find_comment_by_id(&self, id: Uuid) -> Result<Option<Comment>>;
    async fn update_comment(&self, comment: &Comment) -> Result<()>;
    async fn delete_comment(&self, id: Uuid) -> Result<()>;
    async fn get_post_comments(&self, post_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Comment>>;
    async fn get_comment_replies(&self, parent_comment_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Comment>>;
    
    // Like operations
    async fn create_like(&self, like: &Like) -> Result<()>;
    async fn remove_like(&self, user_id: Uuid, target_type: LikeTargetType, target_id: Uuid) -> Result<()>;
    async fn get_post_likes(&self, post_id: Uuid) -> Result<Vec<Like>>;
    async fn get_comment_likes(&self, comment_id: Uuid) -> Result<Vec<Like>>;
    async fn check_user_liked(&self, user_id: Uuid, target_type: LikeTargetType, target_id: Uuid) -> Result<bool>;
    
    // Share operations
    async fn create_share(&self, share: &Share) -> Result<()>;
    async fn get_post_shares(&self, post_id: Uuid) -> Result<Vec<Share>>;
    async fn get_user_shares(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Share>>;
    
    // Repost operations
    async fn create_repost(&self, repost: &Repost) -> Result<()>;
    async fn get_post_reposts(&self, post_id: Uuid) -> Result<Vec<Repost>>;
    async fn get_user_reposts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Repost>>;
    
    // Follow operations
    async fn create_follow(&self, follow: &Follow) -> Result<()>;
    async fn remove_follow(&self, follower_id: Uuid, followed_id: Uuid) -> Result<()>;
    async fn get_user_followers(&self, user_id: Uuid) -> Result<Vec<Follow>>;
    async fn get_user_following(&self, user_id: Uuid) -> Result<Vec<Follow>>;
    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool>;
    
    // Block operations
    async fn create_block(&self, block: &Block) -> Result<()>;
    async fn remove_block(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<()>;
    async fn get_user_blocks(&self, user_id: Uuid) -> Result<Vec<Block>>;
    async fn check_is_blocked(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<bool>;
    
    // Mute operations
    async fn create_mute(&self, mute: &Mute) -> Result<()>;
    async fn remove_mute(&self, muter_id: Uuid, muted_id: Uuid) -> Result<()>;
    async fn get_user_mutes(&self, user_id: Uuid) -> Result<Vec<Mute>>;
    async fn check_is_muted(&self, muter_id: Uuid, muted_id: Uuid) -> Result<bool>;
    
    // Feed generation
    async fn generate_user_feed(&self, user_id: Uuid, feed_type: FeedType, limit: i32, offset: i32) -> Result<Vec<Post>>;
    async fn get_trending_posts(&self, limit: i32, offset: i32) -> Result<Vec<Post>>;
    async fn get_recent_posts(&self, limit: i32, offset: i32) -> Result<Vec<Post>>;
    
    // Activity tracking
    async fn record_user_activity(&self, activity: &UserActivity) -> Result<()>;
    async fn get_user_activity(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<UserActivity>>;
}

/// SQLite implementation of SocialRepository
pub struct SqliteSocialRepository {
    pool: SqlitePool,
}

impl SqliteSocialRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SocialRepository for SqliteSocialRepository {
    async fn create_post(&self, data: CreatePostData) -> Result<Post> {
        let mut tx = self.pool.begin().await?;
        
        let post_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Insert the post
        sqlx::query!(
            "INSERT INTO posts (
                id, author_id, content, visibility, cooperative_id, 
                created_at, updated_at, reply_to_post_id, repost_of_post_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            post_id,
            data.author_id,
            data.content,
            data.visibility,
            data.cooperative_id,
            now,
            now,
            data.reply_to_post_id,
            data.repost_of_post_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert tags
        for tag in &data.tags {
            sqlx::query!(
                "INSERT INTO post_tags (post_id, tag) VALUES (?, ?)",
                post_id,
                tag
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Insert mentions
        for mention in &data.mentions {
            sqlx::query!(
                "INSERT INTO post_mentions (post_id, user_id) VALUES (?, ?)",
                post_id,
                mention
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        
        // Return the created post
        self.find_post_by_id(post_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created post"))
    }
    
    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>> {
        let row = sqlx::query!(
            "SELECT * FROM posts WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let mut post = Post {
                    id: row.id,
                    author_id: row.author_id,
                    content: row.content,
                    visibility: row.visibility,
                    cooperative_id: row.cooperative_id,
                    feed_position: row.feed_position,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    media_items: Vec::new(),
                    tags: Vec::new(),
                    mentions: Vec::new(),
                    reply_to_post_id: row.reply_to_post_id,
                    repost_of_post_id: row.repost_of_post_id,
                    edit_history: Vec::new(),
                };
                
                // Load tags
                let tags = sqlx::query!(
                    "SELECT tag FROM post_tags WHERE post_id = ?",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                post.tags = tags.into_iter().map(|t| t.tag).collect();
                
                // Load mentions
                let mentions = sqlx::query!(
                    "SELECT user_id FROM post_mentions WHERE post_id = ?",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                post.mentions = mentions.into_iter().map(|m| m.user_id).collect();
                
                Ok(Some(post))
            }
            None => Ok(None),
        }
    }
    
    async fn update_post(&self, post: &Post) -> Result<()> {
        sqlx::query!(
            "UPDATE posts SET 
                content = ?, visibility = ?, cooperative_id = ?, 
                updated_at = ?, feed_position = ?
            WHERE id = ?",
            post.content,
            post.visibility,
            post.cooperative_id,
            post.updated_at,
            post.feed_position,
            post.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete_post(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Delete related data first
        sqlx::query!("DELETE FROM post_tags WHERE post_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM post_mentions WHERE post_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM likes WHERE target_type = 'POST' AND target_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM comments WHERE post_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the post
        sqlx::query!("DELETE FROM posts WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_user_posts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>> {
        let rows = sqlx::query!(
            "SELECT * FROM posts WHERE author_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        for row in rows {
            if let Some(post) = self.find_post_by_id(row.id).await? {
                posts.push(post);
            }
        }
        
        Ok(posts)
    }
    
    async fn get_cooperative_posts(&self, cooperative_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Post>> {
        let rows = sqlx::query!(
            "SELECT * FROM posts WHERE cooperative_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            cooperative_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        for row in rows {
            if let Some(post) = self.find_post_by_id(row.id).await? {
                posts.push(post);
            }
        }
        
        Ok(posts)
    }
    
    async fn search_posts(&self, query: &str, limit: i32, offset: i32) -> Result<Vec<Post>> {
        let search_term = format!("%{}%", query);
        let rows = sqlx::query!(
            "SELECT * FROM posts WHERE content LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            search_term,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        for row in rows {
            if let Some(post) = self.find_post_by_id(row.id).await? {
                posts.push(post);
            }
        }
        
        Ok(posts)
    }
    
    async fn create_comment(&self, data: CreateCommentData) -> Result<Comment> {
        let mut tx = self.pool.begin().await?;
        
        let comment_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Calculate thread depth
        let thread_depth = if let Some(parent_id) = data.parent_comment_id {
            let parent_depth = sqlx::query!(
                "SELECT thread_depth FROM comments WHERE id = ?",
                parent_id
            )
            .fetch_one(&mut *tx)
            .await?
            .thread_depth;
            parent_depth + 1
        } else {
            0
        };
        
        // Insert the comment
        sqlx::query!(
            "INSERT INTO comments (
                id, post_id, author_id, content, parent_comment_id, 
                thread_depth, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            comment_id,
            data.post_id,
            data.author_id,
            data.content,
            data.parent_comment_id,
            thread_depth,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert mentions
        for mention in &data.mentions {
            sqlx::query!(
                "INSERT INTO comment_mentions (comment_id, user_id) VALUES (?, ?)",
                comment_id,
                mention
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        
        // Return the created comment
        self.find_comment_by_id(comment_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created comment"))
    }
    
    async fn find_comment_by_id(&self, id: Uuid) -> Result<Option<Comment>> {
        let row = sqlx::query!(
            "SELECT * FROM comments WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let mut comment = Comment {
                    id: row.id,
                    post_id: row.post_id,
                    author_id: row.author_id,
                    content: row.content,
                    parent_comment_id: row.parent_comment_id,
                    thread_depth: row.thread_depth,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                    mentions: Vec::new(),
                    edit_history: Vec::new(),
                };
                
                // Load mentions
                let mentions = sqlx::query!(
                    "SELECT user_id FROM comment_mentions WHERE comment_id = ?",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                comment.mentions = mentions.into_iter().map(|m| m.user_id).collect();
                
                Ok(Some(comment))
            }
            None => Ok(None),
        }
    }
    
    async fn update_comment(&self, comment: &Comment) -> Result<()> {
        sqlx::query!(
            "UPDATE comments SET content = ?, updated_at = ? WHERE id = ?",
            comment.content,
            comment.updated_at,
            comment.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete_comment(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Delete related data first
        sqlx::query!("DELETE FROM comment_mentions WHERE comment_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM likes WHERE target_type = 'COMMENT' AND target_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the comment
        sqlx::query!("DELETE FROM comments WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_post_comments(&self, post_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Comment>> {
        let rows = sqlx::query!(
            "SELECT * FROM comments WHERE post_id = ? ORDER BY created_at ASC LIMIT ? OFFSET ?",
            post_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut comments = Vec::new();
        for row in rows {
            if let Some(comment) = self.find_comment_by_id(row.id).await? {
                comments.push(comment);
            }
        }
        
        Ok(comments)
    }
    
    async fn get_comment_replies(&self, parent_comment_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Comment>> {
        let rows = sqlx::query!(
            "SELECT * FROM comments WHERE parent_comment_id = ? ORDER BY created_at ASC LIMIT ? OFFSET ?",
            parent_comment_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut comments = Vec::new();
        for row in rows {
            if let Some(comment) = self.find_comment_by_id(row.id).await? {
                comments.push(comment);
            }
        }
        
        Ok(comments)
    }
    
    async fn create_like(&self, like: &Like) -> Result<()> {
        sqlx::query!(
            "INSERT INTO likes (id, user_id, target_type, target_id, created_at) VALUES (?, ?, ?, ?, ?)",
            like.id,
            like.user_id,
            like.target_type,
            like.target_id,
            like.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_like(&self, user_id: Uuid, target_type: LikeTargetType, target_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM likes WHERE user_id = ? AND target_type = ? AND target_id = ?",
            user_id,
            target_type,
            target_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_post_likes(&self, post_id: Uuid) -> Result<Vec<Like>> {
        let rows = sqlx::query_as!(
            Like,
            "SELECT * FROM likes WHERE target_type = 'POST' AND target_id = ?",
            post_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn get_comment_likes(&self, comment_id: Uuid) -> Result<Vec<Like>> {
        let rows = sqlx::query_as!(
            Like,
            "SELECT * FROM likes WHERE target_type = 'COMMENT' AND target_id = ?",
            comment_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn check_user_liked(&self, user_id: Uuid, target_type: LikeTargetType, target_id: Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM likes WHERE user_id = ? AND target_type = ? AND target_id = ?",
            user_id,
            target_type,
            target_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        Ok(count > 0)
    }
    
    async fn create_share(&self, share: &Share) -> Result<()> {
        sqlx::query!(
            "INSERT INTO shares (id, user_id, post_id, share_message, share_type, created_at) VALUES (?, ?, ?, ?, ?, ?)",
            share.id,
            share.user_id,
            share.post_id,
            share.share_message,
            share.share_type,
            share.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_post_shares(&self, post_id: Uuid) -> Result<Vec<Share>> {
        let rows = sqlx::query_as!(
            Share,
            "SELECT * FROM shares WHERE post_id = ?",
            post_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn get_user_shares(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Share>> {
        let rows = sqlx::query_as!(
            Share,
            "SELECT * FROM shares WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn create_repost(&self, repost: &Repost) -> Result<()> {
        sqlx::query!(
            "INSERT INTO reposts (id, user_id, original_post_id, repost_message, created_at) VALUES (?, ?, ?, ?, ?)",
            repost.id,
            repost.user_id,
            repost.original_post_id,
            repost.repost_message,
            repost.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_post_reposts(&self, post_id: Uuid) -> Result<Vec<Repost>> {
        let rows = sqlx::query_as!(
            Repost,
            "SELECT * FROM reposts WHERE original_post_id = ?",
            post_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn get_user_reposts(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Repost>> {
        let rows = sqlx::query_as!(
            Repost,
            "SELECT * FROM reposts WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn create_follow(&self, follow: &Follow) -> Result<()> {
        sqlx::query!(
            "INSERT INTO follows (id, follower_id, followed_id, is_mutual, notification_enabled, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            follow.id,
            follow.follower_id,
            follow.followed_id,
            follow.is_mutual,
            follow.notification_enabled,
            follow.created_at,
            follow.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_follow(&self, follower_id: Uuid, followed_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM follows WHERE follower_id = ? AND followed_id = ?",
            follower_id,
            followed_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_user_followers(&self, user_id: Uuid) -> Result<Vec<Follow>> {
        let rows = sqlx::query_as!(
            Follow,
            "SELECT * FROM follows WHERE followed_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn get_user_following(&self, user_id: Uuid) -> Result<Vec<Follow>> {
        let rows = sqlx::query_as!(
            Follow,
            "SELECT * FROM follows WHERE follower_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM follows WHERE follower_id = ? AND followed_id = ?",
            follower_id,
            followed_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        Ok(count > 0)
    }
    
    async fn create_block(&self, block: &Block) -> Result<()> {
        sqlx::query!(
            "INSERT INTO blocks (id, blocker_id, blocked_id, block_reason, is_permanent, expires_at, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            block.id,
            block.blocker_id,
            block.blocked_id,
            block.block_reason,
            block.is_permanent,
            block.expires_at,
            block.created_at,
            block.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_block(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM blocks WHERE blocker_id = ? AND blocked_id = ?",
            blocker_id,
            blocked_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_user_blocks(&self, user_id: Uuid) -> Result<Vec<Block>> {
        let rows = sqlx::query_as!(
            Block,
            "SELECT * FROM blocks WHERE blocker_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn check_is_blocked(&self, blocker_id: Uuid, blocked_id: Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM blocks WHERE blocker_id = ? AND blocked_id = ?",
            blocker_id,
            blocked_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        Ok(count > 0)
    }
    
    async fn create_mute(&self, mute: &Mute) -> Result<()> {
        sqlx::query!(
            "INSERT INTO mutes (id, muter_id, muted_id, mute_type, expires_at, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            mute.id,
            mute.muter_id,
            mute.muted_id,
            mute.mute_type,
            mute.expires_at,
            mute.created_at,
            mute.updated_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn remove_mute(&self, muter_id: Uuid, muted_id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM mutes WHERE muter_id = ? AND muted_id = ?",
            muter_id,
            muted_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_user_mutes(&self, user_id: Uuid) -> Result<Vec<Mute>> {
        let rows = sqlx::query_as!(
            Mute,
            "SELECT * FROM mutes WHERE muter_id = ?",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn check_is_muted(&self, muter_id: Uuid, muted_id: Uuid) -> Result<bool> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM mutes WHERE muter_id = ? AND muted_id = ?",
            muter_id,
            muted_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        Ok(count > 0)
    }
    
    async fn generate_user_feed(&self, user_id: Uuid, feed_type: FeedType, limit: i32, offset: i32) -> Result<Vec<Post>> {
        match feed_type {
            FeedType::Home => {
                // Get posts from followed users
                let rows = sqlx::query!(
                    "SELECT p.* FROM posts p 
                     INNER JOIN follows f ON p.author_id = f.followed_id 
                     WHERE f.follower_id = ? AND p.visibility IN ('PUBLIC', 'COOPERATIVE')
                     ORDER BY p.created_at DESC 
                     LIMIT ? OFFSET ?",
                    user_id,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await?;
                
                let mut posts = Vec::new();
                for row in rows {
                    if let Some(post) = self.find_post_by_id(row.id).await? {
                        posts.push(post);
                    }
                }
                Ok(posts)
            }
            FeedType::Recent => self.get_recent_posts(limit, offset).await,
            FeedType::Trending => self.get_trending_posts(limit, offset).await,
            _ => {
                // For other feed types, return recent posts for now
                self.get_recent_posts(limit, offset).await
            }
        }
    }
    
    async fn get_trending_posts(&self, limit: i32, offset: i32) -> Result<Vec<Post>> {
        // Simple trending algorithm based on recent likes and comments
        let rows = sqlx::query!(
            "SELECT p.*, 
                    (SELECT COUNT(*) FROM likes l WHERE l.target_type = 'POST' AND l.target_id = p.id AND l.created_at > datetime('now', '-24 hours')) as recent_likes,
                    (SELECT COUNT(*) FROM comments c WHERE c.post_id = p.id AND c.created_at > datetime('now', '-24 hours')) as recent_comments
             FROM posts p 
             WHERE p.visibility = 'PUBLIC' AND p.created_at > datetime('now', '-7 days')
             ORDER BY (recent_likes + recent_comments * 2) DESC, p.created_at DESC
             LIMIT ? OFFSET ?",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        for row in rows {
            if let Some(post) = self.find_post_by_id(row.id).await? {
                posts.push(post);
            }
        }
        
        Ok(posts)
    }
    
    async fn get_recent_posts(&self, limit: i32, offset: i32) -> Result<Vec<Post>> {
        let rows = sqlx::query!(
            "SELECT * FROM posts WHERE visibility = 'PUBLIC' ORDER BY created_at DESC LIMIT ? OFFSET ?",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut posts = Vec::new();
        for row in rows {
            if let Some(post) = self.find_post_by_id(row.id).await? {
                posts.push(post);
            }
        }
        
        Ok(posts)
    }
    
    async fn record_user_activity(&self, activity: &UserActivity) -> Result<()> {
        sqlx::query!(
            "INSERT INTO user_activities (id, user_id, activity_type, target_type, target_id, metadata, created_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            activity.id,
            activity.user_id,
            activity.activity_type,
            activity.target_type,
            activity.target_id,
            activity.metadata,
            activity.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_user_activity(&self, user_id: Uuid, limit: i32, offset: i32) -> Result<Vec<UserActivity>> {
        let rows = sqlx::query_as!(
            UserActivity,
            "SELECT * FROM user_activities WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
}