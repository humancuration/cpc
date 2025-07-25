use crate::models::social::{
    Community, CommunityRule, Forum, ForumCategory, ModerationSettings,
    Thread, ThreadReply, ThreadReplyEdit, CommunityMembership, CommunityRole,
    Vote, VoteTargetType, VoteType
};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Data structure for creating communities
pub struct CreateCommunityData {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub owner_id: Uuid,
    pub is_private: bool,
    pub is_nsfw: bool,
    pub tags: Vec<String>,
}

/// Data structure for creating forums
pub struct CreateForumData {
    pub community_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: ForumCategory,
}

/// Data structure for creating threads
pub struct CreateThreadData {
    pub forum_id: Uuid,
    pub community_id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub flair: Option<String>,
}

/// Data structure for creating thread replies
pub struct CreateThreadReplyData {
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
    pub is_moderator_reply: bool,
}

#[async_trait]
pub trait ForumRepository: Send + Sync {
    // Community operations
    async fn create_community(&self, data: CreateCommunityData) -> Result<Community>;
    async fn find_community_by_id(&self, id: Uuid) -> Result<Option<Community>>;
    async fn find_community_by_name(&self, name: &str) -> Result<Option<Community>>;
    async fn update_community(&self, community: &Community) -> Result<()>;
    async fn delete_community(&self, id: Uuid) -> Result<()>;
    async fn get_all_communities(&self, limit: i32, offset: i32) -> Result<Vec<Community>>;
    async fn search_communities(&self, query: &str, limit: i32, offset: i32) -> Result<Vec<Community>>;
    
    // Community membership operations
    async fn join_community(&self, community_id: Uuid, user_id: Uuid, role: CommunityRole) -> Result<CommunityMembership>;
    async fn leave_community(&self, community_id: Uuid, user_id: Uuid) -> Result<()>;
    async fn get_community_members(&self, community_id: Uuid, limit: i32, offset: i32) -> Result<Vec<CommunityMembership>>;
    async fn get_user_communities(&self, user_id: Uuid) -> Result<Vec<CommunityMembership>>;
    async fn update_member_role(&self, community_id: Uuid, user_id: Uuid, role: CommunityRole) -> Result<()>;
    async fn ban_member(&self, community_id: Uuid, user_id: Uuid, reason: String, expires_at: Option<DateTime<Utc>>) -> Result<()>;
    async fn unban_member(&self, community_id: Uuid, user_id: Uuid) -> Result<()>;
    
    // Forum operations
    async fn create_forum(&self, data: CreateForumData) -> Result<Forum>;
    async fn find_forum_by_id(&self, id: Uuid) -> Result<Option<Forum>>;
    async fn update_forum(&self, forum: &Forum) -> Result<()>;
    async fn delete_forum(&self, id: Uuid) -> Result<()>;
    async fn get_community_forums(&self, community_id: Uuid) -> Result<Vec<Forum>>;
    async fn lock_forum(&self, id: Uuid) -> Result<()>;
    async fn unlock_forum(&self, id: Uuid) -> Result<()>;
    async fn pin_forum(&self, id: Uuid) -> Result<()>;
    async fn unpin_forum(&self, id: Uuid) -> Result<()>;
    
    // Thread operations
    async fn create_thread(&self, data: CreateThreadData) -> Result<Thread>;
    async fn find_thread_by_id(&self, id: Uuid) -> Result<Option<Thread>>;
    async fn update_thread(&self, thread: &Thread) -> Result<()>;
    async fn delete_thread(&self, id: Uuid) -> Result<()>;
    async fn get_forum_threads(&self, forum_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Thread>>;
    async fn get_community_threads(&self, community_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Thread>>;
    async fn search_threads(&self, query: &str, community_id: Option<Uuid>, limit: i32, offset: i32) -> Result<Vec<Thread>>;
    async fn lock_thread(&self, id: Uuid) -> Result<()>;
    async fn unlock_thread(&self, id: Uuid) -> Result<()>;
    async fn pin_thread(&self, id: Uuid) -> Result<()>;
    async fn unpin_thread(&self, id: Uuid) -> Result<()>;
    async fn increment_thread_views(&self, id: Uuid) -> Result<()>;
    
    // Thread reply operations
    async fn create_thread_reply(&self, data: CreateThreadReplyData) -> Result<ThreadReply>;
    async fn find_thread_reply_by_id(&self, id: Uuid) -> Result<Option<ThreadReply>>;
    async fn update_thread_reply(&self, reply: &ThreadReply) -> Result<()>;
    async fn delete_thread_reply(&self, id: Uuid) -> Result<()>;
    async fn get_thread_replies(&self, thread_id: Uuid, limit: i32, offset: i32) -> Result<Vec<ThreadReply>>;
    async fn get_reply_children(&self, parent_reply_id: Uuid, limit: i32, offset: i32) -> Result<Vec<ThreadReply>>;
    
    // Voting operations
    async fn create_vote(&self, vote: &Vote) -> Result<()>;
    async fn remove_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<()>;
    async fn update_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid, vote_type: VoteType) -> Result<()>;
    async fn get_vote_counts(&self, target_type: VoteTargetType, target_id: Uuid) -> Result<(i64, i64)>; // (upvotes, downvotes)
    async fn get_user_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<Option<Vote>>;
    
    // Moderation operations
    async fn update_moderation_settings(&self, settings: &ModerationSettings) -> Result<()>;
    async fn get_moderation_settings(&self, forum_id: Uuid) -> Result<Option<ModerationSettings>>;
}

/// SQLite implementation of ForumRepository
pub struct SqliteForumRepository {
    pool: SqlitePool,
}

impl SqliteForumRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}#[async_tr
ait]
impl ForumRepository for SqliteForumRepository {
    async fn create_community(&self, data: CreateCommunityData) -> Result<Community> {
        let mut tx = self.pool.begin().await?;
        
        let community_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Insert the community
        sqlx::query!(
            "INSERT INTO communities (
                id, name, display_name, description, owner_id, 
                member_count, is_private, is_nsfw, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            community_id,
            data.name,
            data.display_name,
            data.description,
            data.owner_id,
            1, // Owner is the first member
            data.is_private,
            data.is_nsfw,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert tags
        for tag in &data.tags {
            sqlx::query!(
                "INSERT INTO community_tags (community_id, tag) VALUES (?, ?)",
                community_id,
                tag
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Add owner as member
        sqlx::query!(
            "INSERT INTO community_memberships (id, community_id, user_id, role, joined_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
            Uuid::new_v4(),
            community_id,
            data.owner_id,
            CommunityRole::Owner,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        // Return the created community
        self.find_community_by_id(community_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created community"))
    }
    
    async fn find_community_by_id(&self, id: Uuid) -> Result<Option<Community>> {
        let row = sqlx::query!(
            "SELECT * FROM communities WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let mut community = Community {
                    id: row.id,
                    name: row.name,
                    display_name: row.display_name,
                    description: row.description,
                    icon_url: row.icon_url,
                    banner_url: row.banner_url,
                    owner_id: row.owner_id,
                    moderator_ids: Vec::new(),
                    member_count: row.member_count,
                    is_private: row.is_private,
                    is_nsfw: row.is_nsfw,
                    rules: Vec::new(),
                    tags: Vec::new(),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                
                // Load moderators
                let moderators = sqlx::query!(
                    "SELECT user_id FROM community_memberships WHERE community_id = ? AND role = 'MODERATOR'",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                community.moderator_ids = moderators.into_iter().map(|m| m.user_id).collect();
                
                // Load rules
                let rules = sqlx::query_as!(
                    CommunityRule,
                    "SELECT * FROM community_rules WHERE community_id = ? ORDER BY order_index",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                community.rules = rules;
                
                // Load tags
                let tags = sqlx::query!(
                    "SELECT tag FROM community_tags WHERE community_id = ?",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                community.tags = tags.into_iter().map(|t| t.tag).collect();
                
                Ok(Some(community))
            }
            None => Ok(None),
        }
    }
    
    async fn find_community_by_name(&self, name: &str) -> Result<Option<Community>> {
        let row = sqlx::query!(
            "SELECT id FROM communities WHERE name = ?",
            name
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => self.find_community_by_id(row.id).await,
            None => Ok(None),
        }
    }
    
    async fn update_community(&self, community: &Community) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Update community
        sqlx::query!(
            "UPDATE communities SET 
                display_name = ?, description = ?, icon_url = ?, banner_url = ?,
                member_count = ?, is_private = ?, is_nsfw = ?, updated_at = ?
            WHERE id = ?",
            community.display_name,
            community.description,
            community.icon_url,
            community.banner_url,
            community.member_count,
            community.is_private,
            community.is_nsfw,
            community.updated_at,
            community.id
        )
        .execute(&mut *tx)
        .await?;
        
        // Update moderators - remove all and re-add
        sqlx::query!(
            "UPDATE community_memberships SET role = 'MEMBER' WHERE community_id = ? AND role = 'MODERATOR'",
            community.id
        )
        .execute(&mut *tx)
        .await?;
        
        for moderator_id in &community.moderator_ids {
            sqlx::query!(
                "UPDATE community_memberships SET role = 'MODERATOR' WHERE community_id = ? AND user_id = ?",
                community.id,
                moderator_id
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn delete_community(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Delete related data first
        sqlx::query!("DELETE FROM community_tags WHERE community_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM community_rules WHERE community_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM community_memberships WHERE community_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM forums WHERE community_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the community
        sqlx::query!("DELETE FROM communities WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_all_communities(&self, limit: i32, offset: i32) -> Result<Vec<Community>> {
        let rows = sqlx::query!(
            "SELECT id FROM communities ORDER BY member_count DESC, created_at DESC LIMIT ? OFFSET ?",
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut communities = Vec::new();
        for row in rows {
            if let Some(community) = self.find_community_by_id(row.id).await? {
                communities.push(community);
            }
        }
        
        Ok(communities)
    }
    
    async fn search_communities(&self, query: &str, limit: i32, offset: i32) -> Result<Vec<Community>> {
        let search_term = format!("%{}%", query);
        let rows = sqlx::query!(
            "SELECT id FROM communities 
             WHERE name LIKE ? OR display_name LIKE ? OR description LIKE ?
             ORDER BY member_count DESC LIMIT ? OFFSET ?",
            search_term,
            search_term,
            search_term,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut communities = Vec::new();
        for row in rows {
            if let Some(community) = self.find_community_by_id(row.id).await? {
                communities.push(community);
            }
        }
        
        Ok(communities)
    } 
   
    async fn join_community(&self, community_id: Uuid, user_id: Uuid, role: CommunityRole) -> Result<CommunityMembership> {
        let membership_id = Uuid::new_v4();
        let now = Utc::now();
        
        let mut tx = self.pool.begin().await?;
        
        // Insert membership
        sqlx::query!(
            "INSERT INTO community_memberships (id, community_id, user_id, role, joined_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?)",
            membership_id,
            community_id,
            user_id,
            role,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Update member count
        sqlx::query!(
            "UPDATE communities SET member_count = member_count + 1 WHERE id = ?",
            community_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        Ok(CommunityMembership {
            id: membership_id,
            community_id,
            user_id,
            role,
            is_banned: false,
            ban_reason: None,
            ban_expires_at: None,
            joined_at: now,
            updated_at: now,
        })
    }
    
    async fn leave_community(&self, community_id: Uuid, user_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Remove membership
        sqlx::query!(
            "DELETE FROM community_memberships WHERE community_id = ? AND user_id = ?",
            community_id,
            user_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Update member count
        sqlx::query!(
            "UPDATE communities SET member_count = member_count - 1 WHERE id = ?",
            community_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_community_members(&self, community_id: Uuid, limit: i32, offset: i32) -> Result<Vec<CommunityMembership>> {
        let rows = sqlx::query_as!(
            CommunityMembership,
            "SELECT * FROM community_memberships WHERE community_id = ? ORDER BY joined_at DESC LIMIT ? OFFSET ?",
            community_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn get_user_communities(&self, user_id: Uuid) -> Result<Vec<CommunityMembership>> {
        let rows = sqlx::query_as!(
            CommunityMembership,
            "SELECT * FROM community_memberships WHERE user_id = ? ORDER BY joined_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows)
    }
    
    async fn update_member_role(&self, community_id: Uuid, user_id: Uuid, role: CommunityRole) -> Result<()> {
        sqlx::query!(
            "UPDATE community_memberships SET role = ?, updated_at = ? WHERE community_id = ? AND user_id = ?",
            role,
            Utc::now(),
            community_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn ban_member(&self, community_id: Uuid, user_id: Uuid, reason: String, expires_at: Option<DateTime<Utc>>) -> Result<()> {
        sqlx::query!(
            "UPDATE community_memberships SET 
                is_banned = ?, ban_reason = ?, ban_expires_at = ?, updated_at = ?
             WHERE community_id = ? AND user_id = ?",
            true,
            reason,
            expires_at,
            Utc::now(),
            community_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn unban_member(&self, community_id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE community_memberships SET 
                is_banned = ?, ban_reason = ?, ban_expires_at = ?, updated_at = ?
             WHERE community_id = ? AND user_id = ?",
            false,
            None::<String>,
            None::<DateTime<Utc>>,
            Utc::now(),
            community_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn create_forum(&self, data: CreateForumData) -> Result<Forum> {
        let mut tx = self.pool.begin().await?;
        
        let forum_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Insert the forum
        sqlx::query!(
            "INSERT INTO forums (
                id, community_id, name, description, category,
                is_locked, is_pinned, is_archived, thread_count, post_count,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            forum_id,
            data.community_id,
            data.name,
            data.description,
            data.category,
            false,
            false,
            false,
            0,
            0,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Create default moderation settings
        let settings_id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO moderation_settings (
                id, forum_id, require_approval, allow_anonymous, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)",
            settings_id,
            forum_id,
            false,
            false,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        // Return the created forum
        self.find_forum_by_id(forum_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created forum"))
    }
    
    async fn find_forum_by_id(&self, id: Uuid) -> Result<Option<Forum>> {
        let row = sqlx::query!(
            "SELECT * FROM forums WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let mut forum = Forum {
                    id: row.id,
                    community_id: row.community_id,
                    name: row.name,
                    description: row.description,
                    category: row.category,
                    is_locked: row.is_locked,
                    is_pinned: row.is_pinned,
                    is_archived: row.is_archived,
                    thread_count: row.thread_count,
                    post_count: row.post_count,
                    last_activity_at: row.last_activity_at,
                    moderator_ids: Vec::new(),
                    moderation_settings: ModerationSettings::new(id), // Default, will be loaded
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                
                // Load moderation settings
                if let Some(settings) = self.get_moderation_settings(id).await? {
                    forum.moderation_settings = settings;
                }
                
                Ok(Some(forum))
            }
            None => Ok(None),
        }
    }    

    async fn update_forum(&self, forum: &Forum) -> Result<()> {
        sqlx::query!(
            "UPDATE forums SET 
                name = ?, description = ?, category = ?, is_locked = ?, 
                is_pinned = ?, is_archived = ?, thread_count = ?, post_count = ?,
                last_activity_at = ?, updated_at = ?
            WHERE id = ?",
            forum.name,
            forum.description,
            forum.category,
            forum.is_locked,
            forum.is_pinned,
            forum.is_archived,
            forum.thread_count,
            forum.post_count,
            forum.last_activity_at,
            forum.updated_at,
            forum.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete_forum(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Delete related data first
        sqlx::query!("DELETE FROM moderation_settings WHERE forum_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM threads WHERE forum_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the forum
        sqlx::query!("DELETE FROM forums WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_community_forums(&self, community_id: Uuid) -> Result<Vec<Forum>> {
        let rows = sqlx::query!(
            "SELECT id FROM forums WHERE community_id = ? ORDER BY is_pinned DESC, name ASC",
            community_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut forums = Vec::new();
        for row in rows {
            if let Some(forum) = self.find_forum_by_id(row.id).await? {
                forums.push(forum);
            }
        }
        
        Ok(forums)
    }
    
    async fn lock_forum(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE forums SET is_locked = ?, updated_at = ? WHERE id = ?",
            true,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn unlock_forum(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE forums SET is_locked = ?, updated_at = ? WHERE id = ?",
            false,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn pin_forum(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE forums SET is_pinned = ?, updated_at = ? WHERE id = ?",
            true,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn unpin_forum(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE forums SET is_pinned = ?, updated_at = ? WHERE id = ?",
            false,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn create_thread(&self, data: CreateThreadData) -> Result<Thread> {
        let mut tx = self.pool.begin().await?;
        
        let thread_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Insert the thread
        sqlx::query!(
            "INSERT INTO threads (
                id, forum_id, community_id, author_id, title, content,
                is_pinned, is_locked, is_archived, is_deleted,
                reply_count, view_count, upvote_count, downvote_count,
                flair, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            thread_id,
            data.forum_id,
            data.community_id,
            data.author_id,
            data.title,
            data.content,
            false,
            false,
            false,
            false,
            0,
            0,
            0,
            0,
            data.flair,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Insert tags
        for tag in &data.tags {
            sqlx::query!(
                "INSERT INTO thread_tags (thread_id, tag) VALUES (?, ?)",
                thread_id,
                tag
            )
            .execute(&mut *tx)
            .await?;
        }
        
        // Update forum thread count
        sqlx::query!(
            "UPDATE forums SET thread_count = thread_count + 1, last_activity_at = ? WHERE id = ?",
            now,
            data.forum_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        // Return the created thread
        self.find_thread_by_id(thread_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created thread"))
    }
    
    async fn find_thread_by_id(&self, id: Uuid) -> Result<Option<Thread>> {
        let row = sqlx::query!(
            "SELECT * FROM threads WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let mut thread = Thread {
                    id: row.id,
                    forum_id: row.forum_id,
                    community_id: row.community_id,
                    author_id: row.author_id,
                    title: row.title,
                    content: row.content,
                    is_pinned: row.is_pinned,
                    is_locked: row.is_locked,
                    is_archived: row.is_archived,
                    is_deleted: row.is_deleted,
                    reply_count: row.reply_count,
                    view_count: row.view_count,
                    upvote_count: row.upvote_count,
                    downvote_count: row.downvote_count,
                    last_reply_at: row.last_reply_at,
                    last_reply_by: row.last_reply_by,
                    tags: Vec::new(),
                    flair: row.flair,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                
                // Load tags
                let tags = sqlx::query!(
                    "SELECT tag FROM thread_tags WHERE thread_id = ?",
                    id
                )
                .fetch_all(&self.pool)
                .await?;
                thread.tags = tags.into_iter().map(|t| t.tag).collect();
                
                Ok(Some(thread))
            }
            None => Ok(None),
        }
    }    

    async fn update_thread(&self, thread: &Thread) -> Result<()> {
        sqlx::query!(
            "UPDATE threads SET 
                title = ?, content = ?, is_pinned = ?, is_locked = ?, 
                is_archived = ?, is_deleted = ?, reply_count = ?, view_count = ?,
                upvote_count = ?, downvote_count = ?, last_reply_at = ?, 
                last_reply_by = ?, flair = ?, updated_at = ?
            WHERE id = ?",
            thread.title,
            thread.content,
            thread.is_pinned,
            thread.is_locked,
            thread.is_archived,
            thread.is_deleted,
            thread.reply_count,
            thread.view_count,
            thread.upvote_count,
            thread.downvote_count,
            thread.last_reply_at,
            thread.last_reply_by,
            thread.flair,
            thread.updated_at,
            thread.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete_thread(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Get forum_id for updating counts
        let forum_id = sqlx::query!(
            "SELECT forum_id FROM threads WHERE id = ?",
            id
        )
        .fetch_one(&mut *tx)
        .await?
        .forum_id;
        
        // Delete related data first
        sqlx::query!("DELETE FROM thread_tags WHERE thread_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM thread_replies WHERE thread_id = ?", id)
            .execute(&mut *tx)
            .await?;
        sqlx::query!("DELETE FROM votes WHERE target_type = 'THREAD' AND target_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the thread
        sqlx::query!("DELETE FROM threads WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Update forum thread count
        sqlx::query!(
            "UPDATE forums SET thread_count = thread_count - 1 WHERE id = ?",
            forum_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_forum_threads(&self, forum_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Thread>> {
        let rows = sqlx::query!(
            "SELECT id FROM threads WHERE forum_id = ? AND is_deleted = false 
             ORDER BY is_pinned DESC, last_reply_at DESC, created_at DESC 
             LIMIT ? OFFSET ?",
            forum_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut threads = Vec::new();
        for row in rows {
            if let Some(thread) = self.find_thread_by_id(row.id).await? {
                threads.push(thread);
            }
        }
        
        Ok(threads)
    }
    
    async fn get_community_threads(&self, community_id: Uuid, limit: i32, offset: i32) -> Result<Vec<Thread>> {
        let rows = sqlx::query!(
            "SELECT id FROM threads WHERE community_id = ? AND is_deleted = false 
             ORDER BY is_pinned DESC, last_reply_at DESC, created_at DESC 
             LIMIT ? OFFSET ?",
            community_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut threads = Vec::new();
        for row in rows {
            if let Some(thread) = self.find_thread_by_id(row.id).await? {
                threads.push(thread);
            }
        }
        
        Ok(threads)
    }
    
    async fn search_threads(&self, query: &str, community_id: Option<Uuid>, limit: i32, offset: i32) -> Result<Vec<Thread>> {
        let search_term = format!("%{}%", query);
        
        let rows = if let Some(community_id) = community_id {
            sqlx::query!(
                "SELECT id FROM threads 
                 WHERE community_id = ? AND is_deleted = false AND (title LIKE ? OR content LIKE ?)
                 ORDER BY last_reply_at DESC, created_at DESC 
                 LIMIT ? OFFSET ?",
                community_id,
                search_term,
                search_term,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                "SELECT id FROM threads 
                 WHERE is_deleted = false AND (title LIKE ? OR content LIKE ?)
                 ORDER BY last_reply_at DESC, created_at DESC 
                 LIMIT ? OFFSET ?",
                search_term,
                search_term,
                limit,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        };
        
        let mut threads = Vec::new();
        for row in rows {
            if let Some(thread) = self.find_thread_by_id(row.id).await? {
                threads.push(thread);
            }
        }
        
        Ok(threads)
    }
    
    async fn lock_thread(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE threads SET is_locked = ?, updated_at = ? WHERE id = ?",
            true,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn unlock_thread(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE threads SET is_locked = ?, updated_at = ? WHERE id = ?",
            false,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn pin_thread(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE threads SET is_pinned = ?, updated_at = ? WHERE id = ?",
            true,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn unpin_thread(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE threads SET is_pinned = ?, updated_at = ? WHERE id = ?",
            false,
            Utc::now(),
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn increment_thread_views(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "UPDATE threads SET view_count = view_count + 1 WHERE id = ?",
            id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }   
 
    async fn create_thread_reply(&self, data: CreateThreadReplyData) -> Result<ThreadReply> {
        let mut tx = self.pool.begin().await?;
        
        let reply_id = Uuid::new_v4();
        let now = Utc::now();
        
        // Calculate thread depth
        let thread_depth = if let Some(parent_id) = data.parent_reply_id {
            let parent_depth = sqlx::query!(
                "SELECT thread_depth FROM thread_replies WHERE id = ?",
                parent_id
            )
            .fetch_one(&mut *tx)
            .await?
            .thread_depth;
            parent_depth + 1
        } else {
            0
        };
        
        // Insert the reply
        sqlx::query!(
            "INSERT INTO thread_replies (
                id, thread_id, author_id, content, parent_reply_id,
                thread_depth, upvote_count, downvote_count, is_deleted,
                is_moderator_reply, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            reply_id,
            data.thread_id,
            data.author_id,
            data.content,
            data.parent_reply_id,
            thread_depth,
            0,
            0,
            false,
            data.is_moderator_reply,
            now,
            now
        )
        .execute(&mut *tx)
        .await?;
        
        // Update thread reply count and last reply info
        sqlx::query!(
            "UPDATE threads SET 
                reply_count = reply_count + 1, 
                last_reply_at = ?, 
                last_reply_by = ?,
                updated_at = ?
             WHERE id = ?",
            now,
            data.author_id,
            now,
            data.thread_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        // Return the created reply
        self.find_thread_reply_by_id(reply_id).await?.ok_or_else(|| anyhow::anyhow!("Failed to retrieve created reply"))
    }
    
    async fn find_thread_reply_by_id(&self, id: Uuid) -> Result<Option<ThreadReply>> {
        let row = sqlx::query!(
            "SELECT * FROM thread_replies WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let reply = ThreadReply {
                    id: row.id,
                    thread_id: row.thread_id,
                    author_id: row.author_id,
                    content: row.content,
                    parent_reply_id: row.parent_reply_id,
                    thread_depth: row.thread_depth,
                    upvote_count: row.upvote_count,
                    downvote_count: row.downvote_count,
                    is_deleted: row.is_deleted,
                    is_moderator_reply: row.is_moderator_reply,
                    edit_history: Vec::new(), // TODO: Load edit history if needed
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                
                Ok(Some(reply))
            }
            None => Ok(None),
        }
    }
    
    async fn update_thread_reply(&self, reply: &ThreadReply) -> Result<()> {
        sqlx::query!(
            "UPDATE thread_replies SET 
                content = ?, upvote_count = ?, downvote_count = ?, 
                is_deleted = ?, is_moderator_reply = ?, updated_at = ?
            WHERE id = ?",
            reply.content,
            reply.upvote_count,
            reply.downvote_count,
            reply.is_deleted,
            reply.is_moderator_reply,
            reply.updated_at,
            reply.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn delete_thread_reply(&self, id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Get thread_id for updating counts
        let thread_id = sqlx::query!(
            "SELECT thread_id FROM thread_replies WHERE id = ?",
            id
        )
        .fetch_one(&mut *tx)
        .await?
        .thread_id;
        
        // Delete related data first
        sqlx::query!("DELETE FROM votes WHERE target_type = 'THREAD_REPLY' AND target_id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Delete the reply
        sqlx::query!("DELETE FROM thread_replies WHERE id = ?", id)
            .execute(&mut *tx)
            .await?;
        
        // Update thread reply count
        sqlx::query!(
            "UPDATE threads SET reply_count = reply_count - 1 WHERE id = ?",
            thread_id
        )
        .execute(&mut *tx)
        .await?;
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn get_thread_replies(&self, thread_id: Uuid, limit: i32, offset: i32) -> Result<Vec<ThreadReply>> {
        let rows = sqlx::query!(
            "SELECT id FROM thread_replies WHERE thread_id = ? AND is_deleted = false 
             ORDER BY created_at ASC LIMIT ? OFFSET ?",
            thread_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut replies = Vec::new();
        for row in rows {
            if let Some(reply) = self.find_thread_reply_by_id(row.id).await? {
                replies.push(reply);
            }
        }
        
        Ok(replies)
    }
    
    async fn get_reply_children(&self, parent_reply_id: Uuid, limit: i32, offset: i32) -> Result<Vec<ThreadReply>> {
        let rows = sqlx::query!(
            "SELECT id FROM thread_replies WHERE parent_reply_id = ? AND is_deleted = false 
             ORDER BY created_at ASC LIMIT ? OFFSET ?",
            parent_reply_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut replies = Vec::new();
        for row in rows {
            if let Some(reply) = self.find_thread_reply_by_id(row.id).await? {
                replies.push(reply);
            }
        }
        
        Ok(replies)
    }    
   
 async fn create_vote(&self, vote: &Vote) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Insert or update vote
        sqlx::query!(
            "INSERT OR REPLACE INTO votes (id, user_id, target_type, target_id, vote_type, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            vote.id,
            vote.user_id,
            vote.target_type,
            vote.target_id,
            vote.vote_type,
            vote.created_at,
            vote.updated_at
        )
        .execute(&mut *tx)
        .await?;
        
        // Update vote counts on target
        match vote.target_type {
            VoteTargetType::Thread => {
                let (upvotes, downvotes) = self.get_vote_counts(vote.target_type, vote.target_id).await?;
                sqlx::query!(
                    "UPDATE threads SET upvote_count = ?, downvote_count = ? WHERE id = ?",
                    upvotes,
                    downvotes,
                    vote.target_id
                )
                .execute(&mut *tx)
                .await?;
            }
            VoteTargetType::ThreadReply => {
                let (upvotes, downvotes) = self.get_vote_counts(vote.target_type, vote.target_id).await?;
                sqlx::query!(
                    "UPDATE thread_replies SET upvote_count = ?, downvote_count = ? WHERE id = ?",
                    upvotes,
                    downvotes,
                    vote.target_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn remove_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        
        // Remove vote
        sqlx::query!(
            "DELETE FROM votes WHERE user_id = ? AND target_type = ? AND target_id = ?",
            user_id,
            target_type,
            target_id
        )
        .execute(&mut *tx)
        .await?;
        
        // Update vote counts on target
        match target_type {
            VoteTargetType::Thread => {
                let (upvotes, downvotes) = self.get_vote_counts(target_type, target_id).await?;
                sqlx::query!(
                    "UPDATE threads SET upvote_count = ?, downvote_count = ? WHERE id = ?",
                    upvotes,
                    downvotes,
                    target_id
                )
                .execute(&mut *tx)
                .await?;
            }
            VoteTargetType::ThreadReply => {
                let (upvotes, downvotes) = self.get_vote_counts(target_type, target_id).await?;
                sqlx::query!(
                    "UPDATE thread_replies SET upvote_count = ?, downvote_count = ? WHERE id = ?",
                    upvotes,
                    downvotes,
                    target_id
                )
                .execute(&mut *tx)
                .await?;
            }
        }
        
        tx.commit().await?;
        Ok(())
    }
    
    async fn update_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid, vote_type: VoteType) -> Result<()> {
        sqlx::query!(
            "UPDATE votes SET vote_type = ?, updated_at = ? 
             WHERE user_id = ? AND target_type = ? AND target_id = ?",
            vote_type,
            Utc::now(),
            user_id,
            target_type,
            target_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_vote_counts(&self, target_type: VoteTargetType, target_id: Uuid) -> Result<(i64, i64)> {
        let upvotes = sqlx::query!(
            "SELECT COUNT(*) as count FROM votes 
             WHERE target_type = ? AND target_id = ? AND vote_type = 'UPVOTE'",
            target_type,
            target_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        let downvotes = sqlx::query!(
            "SELECT COUNT(*) as count FROM votes 
             WHERE target_type = ? AND target_id = ? AND vote_type = 'DOWNVOTE'",
            target_type,
            target_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        
        Ok((upvotes, downvotes))
    }
    
    async fn get_user_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<Option<Vote>> {
        let row = sqlx::query_as!(
            Vote,
            "SELECT * FROM votes WHERE user_id = ? AND target_type = ? AND target_id = ?",
            user_id,
            target_type,
            target_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(row)
    }
    
    async fn update_moderation_settings(&self, settings: &ModerationSettings) -> Result<()> {
        sqlx::query!(
            "UPDATE moderation_settings SET 
                require_approval = ?, auto_lock_after_days = ?, max_thread_length = ?,
                allow_anonymous = ?, rate_limit_posts = ?, updated_at = ?
            WHERE id = ?",
            settings.require_approval,
            settings.auto_lock_after_days,
            settings.max_thread_length,
            settings.allow_anonymous,
            settings.rate_limit_posts,
            settings.updated_at,
            settings.id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_moderation_settings(&self, forum_id: Uuid) -> Result<Option<ModerationSettings>> {
        let row = sqlx::query!(
            "SELECT * FROM moderation_settings WHERE forum_id = ?",
            forum_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        match row {
            Some(row) => {
                let settings = ModerationSettings {
                    id: row.id,
                    forum_id: row.forum_id,
                    require_approval: row.require_approval,
                    auto_lock_after_days: row.auto_lock_after_days,
                    max_thread_length: row.max_thread_length,
                    allow_anonymous: row.allow_anonymous,
                    rate_limit_posts: row.rate_limit_posts,
                    banned_words: Vec::new(), // TODO: Load banned words if needed
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                
                Ok(Some(settings))
            }
            None => Ok(None),
        }
    }
}