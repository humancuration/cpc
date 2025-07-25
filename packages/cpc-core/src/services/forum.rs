use crate::{
    models::social::{
        Community, CommunityRule, Forum, ForumCategory, ModerationSettings,
        Thread, ThreadReply, ThreadReplyEdit, CommunityMembership, CommunityRole,
        Vote, VoteTargetType, VoteType, ModerationAction, ModerationActionType, ModerationTargetType,
        Notification, NotificationType, NotificationPriority, UserActivity, ActivityType,
        FeedType, FeedAlgorithm, Feed, FeedItem, FeedContentType
    },
    repositories::forum_repository::{
        ForumRepository, CreateCommunityData, CreateForumData, CreateThreadData, CreateThreadReplyData
    },
    utils::datetime::now_utc,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc, Duration};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Input data for creating a new community
#[derive(Debug, Clone)]
pub struct CreateCommunityInput {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub is_private: bool,
    pub is_nsfw: bool,
    pub tags: Vec<String>,
}

/// Input data for creating a new forum
#[derive(Debug, Clone)]
pub struct CreateForumInput {
    pub community_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: ForumCategory,
}

/// Input data for creating a new thread
#[derive(Debug, Clone)]
pub struct CreateThreadInput {
    pub forum_id: Uuid,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub flair: Option<String>,
}

/// Input data for creating a thread reply
#[derive(Debug, Clone)]
pub struct CreateThreadReplyInput {
    pub thread_id: Uuid,
    pub content: String,
    pub parent_reply_id: Option<Uuid>,
}

/// Forum moderation input
#[derive(Debug, Clone)]
pub struct ForumModerationInput {
    pub moderator_id: Uuid,
    pub target_type: ModerationTargetType,
    pub target_id: Uuid,
    pub action_type: ModerationActionType,
    pub reason: Option<String>,
    pub duration_hours: Option<i32>,
}

/// Thread ranking factors for hot/trending algorithms
#[derive(Debug, Clone)]
pub struct ThreadRankingFactors {
    pub upvote_score: f64,
    pub engagement_score: f64,
    pub recency_score: f64,
    pub reply_velocity: f64,
    pub final_score: f64,
}

/// Community statistics
#[derive(Debug, Clone)]
pub struct CommunityStats {
    pub member_count: i64,
    pub thread_count: i64,
    pub post_count: i64,
    pub active_users_24h: i64,
    pub growth_rate: f64,
}

/// Thread statistics
#[derive(Debug, Clone)]
pub struct ThreadStats {
    pub reply_count: i64,
    pub view_count: i64,
    pub upvote_count: i64,
    pub downvote_count: i64,
    pub engagement_rate: f64,
}

/// Forum search parameters
#[derive(Debug, Clone)]
pub struct ForumSearchParams {
    pub query: String,
    pub community_id: Option<Uuid>,
    pub category: Option<ForumCategory>,
    pub tags: Vec<String>,
    pub sort_by: ForumSortBy,
    pub limit: i32,
    pub offset: i32,
}

/// Forum sorting options
#[derive(Debug, Clone, Copy)]
pub enum ForumSortBy {
    Hot,
    New,
    Top,
    Controversial,
    Rising,
}

/// ForumService handles all forum-related functionality including communities,
/// forums, threads, replies, voting, and moderation
pub struct ForumService {
    repository: Box<dyn ForumRepository>,
    max_thread_title_length: usize,
    max_thread_content_length: usize,
    max_reply_content_length: usize,
    max_community_name_length: usize,
    banned_words: HashSet<String>,
}

impl ForumService {
    /// Creates a new ForumService instance
    pub fn new(repository: Box<dyn ForumRepository>) -> Self {
        let banned_words = [
            "spam", "scam", "fake", "bot", "hack", "cheat"
        ].iter().map(|s| s.to_string()).collect();

        Self {
            repository,
            max_thread_title_length: 300,
            max_thread_content_length: 10000,
            max_reply_content_length: 5000,
            max_community_name_length: 50,
            banned_words,
        }
    }

    // ===== COMMUNITY MANAGEMENT =====

    /// Creates a new community
    pub async fn create_community(&self, input: CreateCommunityInput, owner_id: Uuid) -> Result<Community> {
        // Validate input
        self.validate_community_input(&input)?;
        
        // Check if community name is already taken
        if self.repository.find_community_by_name(&input.name).await?.is_some() {
            return Err(anyhow!("Community name already exists"));
        }

        let create_data = CreateCommunityData {
            name: input.name,
            display_name: input.display_name,
            description: input.description,
            owner_id,
            is_private: input.is_private,
            is_nsfw: input.is_nsfw,
            tags: input.tags,
        };

        let community = self.repository.create_community(create_data).await?;
        
        // Record activity
        self.record_activity(owner_id, ActivityType::Create, "community", community.id).await?;
        
        Ok(community)
    }

    /// Gets a community by ID
    pub async fn get_community(&self, community_id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Community>> {
        let community = self.repository.find_community_by_id(community_id).await?;
        
        match community {
            Some(community) => {
                if self.can_view_community(&community, viewer_id).await? {
                    Ok(Some(community))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Gets a community by name
    pub async fn get_community_by_name(&self, name: &str, viewer_id: Option<Uuid>) -> Result<Option<Community>> {
        let community = self.repository.find_community_by_name(name).await?;
        
        match community {
            Some(community) => {
                if self.can_view_community(&community, viewer_id).await? {
                    Ok(Some(community))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Updates a community
    pub async fn update_community(&self, community_id: Uuid, updates: CreateCommunityInput, editor_id: Uuid) -> Result<Community> {
        let mut community = self.repository.find_community_by_id(community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions
        if !self.can_moderate_community(&community, editor_id).await? {
            return Err(anyhow!("Not authorized to edit this community"));
        }
        
        // Validate updates
        self.validate_community_input(&updates)?;
        
        // Apply updates
        community.display_name = updates.display_name;
        community.description = updates.description;
        community.is_private = updates.is_private;
        community.is_nsfw = updates.is_nsfw;
        community.tags = updates.tags;
        community.touch();
        
        self.repository.update_community(&community).await?;
        
        // Record activity
        self.record_activity(editor_id, ActivityType::Edit, "community", community_id).await?;
        
        Ok(community)
    }

    /// Joins a community
    pub async fn join_community(&self, community_id: Uuid, user_id: Uuid) -> Result<CommunityMembership> {
        let community = self.repository.find_community_by_id(community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check if community is private
        if community.is_private {
            return Err(anyhow!("Cannot join private community without invitation"));
        }
        
        // Check if already a member
        let existing_memberships = self.repository.get_user_communities(user_id).await?;
        if existing_memberships.iter().any(|m| m.community_id == community_id) {
            return Err(anyhow!("Already a member of this community"));
        }
        
        let membership = self.repository.join_community(community_id, user_id, CommunityRole::Member).await?;
        
        // Record activity
        self.record_activity(user_id, ActivityType::Join, "community", community_id).await?;
        
        Ok(membership)
    }

    /// Leaves a community
    pub async fn leave_community(&self, community_id: Uuid, user_id: Uuid) -> Result<()> {
        let community = self.repository.find_community_by_id(community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Cannot leave if you're the owner
        if community.owner_id == user_id {
            return Err(anyhow!("Community owner cannot leave community"));
        }
        
        self.repository.leave_community(community_id, user_id).await?;
        
        // Record activity
        self.record_activity(user_id, ActivityType::Leave, "community", community_id).await?;
        
        Ok(())
    }

    /// Bans a member from a community
    pub async fn ban_member(&self, community_id: Uuid, user_id: Uuid, moderator_id: Uuid, reason: String, duration_hours: Option<i32>) -> Result<()> {
        let community = self.repository.find_community_by_id(community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check moderator permissions
        if !self.can_moderate_community(&community, moderator_id).await? {
            return Err(anyhow!("Not authorized to ban members"));
        }
        
        // Cannot ban the owner
        if community.owner_id == user_id {
            return Err(anyhow!("Cannot ban community owner"));
        }
        
        let expires_at = duration_hours.map(|hours| Utc::now() + Duration::hours(hours as i64));
        
        self.repository.ban_member(community_id, user_id, reason.clone(), expires_at).await?;
        
        // Record moderation action
        let action = ModerationAction::new(
            moderator_id,
            ModerationTargetType::User,
            user_id,
            ModerationActionType::Ban,
            Some(reason),
        );
        
        // Record activity
        self.record_activity(moderator_id, ActivityType::Delete, "user", user_id).await?;
        
        Ok(())
    }

    // ===== FORUM MANAGEMENT =====

    /// Creates a new forum within a community
    pub async fn create_forum(&self, input: CreateForumInput, creator_id: Uuid) -> Result<Forum> {
        let community = self.repository.find_community_by_id(input.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions
        if !self.can_moderate_community(&community, creator_id).await? {
            return Err(anyhow!("Not authorized to create forums in this community"));
        }
        
        // Validate input
        self.validate_forum_input(&input)?;

        let create_data = CreateForumData {
            community_id: input.community_id,
            name: input.name,
            description: input.description,
            category: input.category,
        };

        let forum = self.repository.create_forum(create_data).await?;
        
        // Record activity
        self.record_activity(creator_id, ActivityType::Create, "forum", forum.id).await?;
        
        Ok(forum)
    }

    /// Gets a forum by ID
    pub async fn get_forum(&self, forum_id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Forum>> {
        let forum = self.repository.find_forum_by_id(forum_id).await?;
        
        match forum {
            Some(forum) => {
                let community = self.repository.find_community_by_id(forum.community_id).await?
                    .ok_or_else(|| anyhow!("Community not found"))?;
                
                if self.can_view_community(&community, viewer_id).await? {
                    Ok(Some(forum))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Gets all forums in a community
    pub async fn get_community_forums(&self, community_id: Uuid, viewer_id: Option<Uuid>) -> Result<Vec<Forum>> {
        let community = self.repository.find_community_by_id(community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        if !self.can_view_community(&community, viewer_id).await? {
            return Err(anyhow!("Not authorized to view this community"));
        }
        
        self.repository.get_community_forums(community_id).await
    }

    // ===== THREAD MANAGEMENT =====

    /// Creates a new thread
    pub async fn create_thread(&self, input: CreateThreadInput, author_id: Uuid) -> Result<Thread> {
        let forum = self.repository.find_forum_by_id(input.forum_id).await?
            .ok_or_else(|| anyhow!("Forum not found"))?;
        
        let community = self.repository.find_community_by_id(forum.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions
        if !self.can_post_in_community(&community, author_id).await? {
            return Err(anyhow!("Not authorized to post in this community"));
        }
        
        // Check if forum is locked
        if forum.is_locked {
            return Err(anyhow!("Cannot create threads in locked forum"));
        }
        
        // Validate input
        self.validate_thread_input(&input)?;

        let create_data = CreateThreadData {
            forum_id: input.forum_id,
            community_id: forum.community_id,
            author_id,
            title: input.title,
            content: input.content,
            tags: input.tags,
            flair: input.flair,
        };

        let thread = self.repository.create_thread(create_data).await?;
        
        // Record activity
        self.record_activity(author_id, ActivityType::Create, "thread", thread.id).await?;
        
        Ok(thread)
    }

    /// Gets a thread by ID
    pub async fn get_thread(&self, thread_id: Uuid, viewer_id: Option<Uuid>) -> Result<Option<Thread>> {
        let thread = self.repository.find_thread_by_id(thread_id).await?;
        
        match thread {
            Some(mut thread) => {
                let community = self.repository.find_community_by_id(thread.community_id).await?
                    .ok_or_else(|| anyhow!("Community not found"))?;
                
                if self.can_view_community(&community, viewer_id).await? {
                    // Increment view count
                    self.repository.increment_thread_views(thread_id).await?;
                    thread.view_count += 1;
                    
                    Ok(Some(thread))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Gets threads in a forum with sorting and pagination
    pub async fn get_forum_threads(&self, forum_id: Uuid, sort_by: ForumSortBy, limit: i32, offset: i32, viewer_id: Option<Uuid>) -> Result<Vec<Thread>> {
        let forum = self.repository.find_forum_by_id(forum_id).await?
            .ok_or_else(|| anyhow!("Forum not found"))?;
        
        let community = self.repository.find_community_by_id(forum.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        if !self.can_view_community(&community, viewer_id).await? {
            return Err(anyhow!("Not authorized to view this forum"));
        }
        
        let mut threads = self.repository.get_forum_threads(forum_id, limit, offset).await?;
        
        // Apply sorting algorithm
        self.sort_threads(&mut threads, sort_by).await?;
        
        Ok(threads)
    }

    /// Pins a thread
    pub async fn pin_thread(&self, thread_id: Uuid, moderator_id: Uuid) -> Result<()> {
        let thread = self.repository.find_thread_by_id(thread_id).await?
            .ok_or_else(|| anyhow!("Thread not found"))?;
        
        let community = self.repository.find_community_by_id(thread.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions
        if !self.can_moderate_community(&community, moderator_id).await? {
            return Err(anyhow!("Not authorized to pin threads"));
        }
        
        self.repository.pin_thread(thread_id).await?;
        
        // Record moderation action
        self.record_activity(moderator_id, ActivityType::Edit, "thread", thread_id).await?;
        
        Ok(())
    }

    /// Locks a thread
    pub async fn lock_thread(&self, thread_id: Uuid, moderator_id: Uuid) -> Result<()> {
        let thread = self.repository.find_thread_by_id(thread_id).await?
            .ok_or_else(|| anyhow!("Thread not found"))?;
        
        let community = self.repository.find_community_by_id(thread.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions
        if !self.can_moderate_community(&community, moderator_id).await? {
            return Err(anyhow!("Not authorized to lock threads"));
        }
        
        self.repository.lock_thread(thread_id).await?;
        
        // Record moderation action
        self.record_activity(moderator_id, ActivityType::Edit, "thread", thread_id).await?;
        
        Ok(())
    }

    /// Deletes a thread
    pub async fn delete_thread(&self, thread_id: Uuid, deleter_id: Uuid) -> Result<()> {
        let thread = self.repository.find_thread_by_id(thread_id).await?
            .ok_or_else(|| anyhow!("Thread not found"))?;
        
        let community = self.repository.find_community_by_id(thread.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions (author or moderator)
        if thread.author_id != deleter_id && !self.can_moderate_community(&community, deleter_id).await? {
            return Err(anyhow!("Not authorized to delete this thread"));
        }
        
        self.repository.delete_thread(thread_id).await?;
        
        // Record activity
        self.record_activity(deleter_id, ActivityType::Delete, "thread", thread_id).await?;
        
        Ok(())
    }

    // ===== THREAD REPLY MANAGEMENT =====

    /// Creates a new thread reply
    pub async fn create_thread_reply(&self, input: CreateThreadReplyInput, author_id: Uuid) -> Result<ThreadReply> {
        let thread = self.repository.find_thread_by_id(input.thread_id).await?
            .ok_or_else(|| anyhow!("Thread not found"))?;
        
        let community = self.repository.find_community_by_id(thread.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        // Check permissions
        if !self.can_post_in_community(&community, author_id).await? {
            return Err(anyhow!("Not authorized to reply in this community"));
        }
        
        // Check if thread is locked
        if thread.is_locked {
            return Err(anyhow!("Cannot reply to locked thread"));
        }
        
        // Validate input
        self.validate_reply_input(&input)?;
        
        // Check if replying to a specific comment and calculate depth
        let depth = if let Some(parent_id) = input.parent_reply_id {
            let parent = self.repository.find_thread_reply_by_id(parent_id).await?
                .ok_or_else(|| anyhow!("Parent reply not found"))?;
            parent.thread_depth + 1
        } else {
            0
        };
        
        // Limit nesting depth
        if depth > 10 {
            return Err(anyhow!("Maximum reply depth exceeded"));
        }

        let create_data = CreateThreadReplyData {
            thread_id: input.thread_id,
            author_id,
            content: input.content,
            parent_reply_id: input.parent_reply_id,
            is_moderator_reply: self.can_moderate_community(&community, author_id).await?,
        };

        let reply = self.repository.create_thread_reply(create_data).await?;
        
        // Record activity
        self.record_activity(author_id, ActivityType::Comment, "thread", input.thread_id).await?;
        
        Ok(reply)
    }

    /// Gets replies for a thread
    pub async fn get_thread_replies(&self, thread_id: Uuid, limit: i32, offset: i32, viewer_id: Option<Uuid>) -> Result<Vec<ThreadReply>> {
        let thread = self.repository.find_thread_by_id(thread_id).await?
            .ok_or_else(|| anyhow!("Thread not found"))?;
        
        let community = self.repository.find_community_by_id(thread.community_id).await?
            .ok_or_else(|| anyhow!("Community not found"))?;
        
        if !self.can_view_community(&community, viewer_id).await? {
            return Err(anyhow!("Not authorized to view this thread"));
        }
        
        self.repository.get_thread_replies(thread_id, limit, offset).await
    }

    // ===== VOTING SYSTEM =====

    /// Upvotes a thread or reply
    pub async fn upvote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<()> {
        // Check if user already voted
        if let Some(existing_vote) = self.repository.get_user_vote(user_id, target_type, target_id).await? {
            if existing_vote.vote_type == VoteType::Upvote {
                return Err(anyhow!("Already upvoted"));
            } else {
                // Change from downvote to upvote
                self.repository.update_vote(user_id, target_type, target_id, VoteType::Upvote).await?;
            }
        } else {
            // Create new upvote
            let vote = Vote::new_upvote(user_id, target_type, target_id);
            self.repository.create_vote(&vote).await?;
        }
        
        // Update vote counts
        self.update_vote_counts(target_type, target_id).await?;
        
        // Record activity
        let target_type_str = match target_type {
            VoteTargetType::Thread => "thread",
            VoteTargetType::ThreadReply => "thread_reply",
        };
        self.record_activity(user_id, ActivityType::Upvote, target_type_str, target_id).await?;
        
        Ok(())
    }

    /// Downvotes a thread or reply
    pub async fn downvote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<()> {
        // Check if user already voted
        if let Some(existing_vote) = self.repository.get_user_vote(user_id, target_type, target_id).await? {
            if existing_vote.vote_type == VoteType::Downvote {
                return Err(anyhow!("Already downvoted"));
            } else {
                // Change from upvote to downvote
                self.repository.update_vote(user_id, target_type, target_id, VoteType::Downvote).await?;
            }
        } else {
            // Create new downvote
            let vote = Vote::new_downvote(user_id, target_type, target_id);
            self.repository.create_vote(&vote).await?;
        }
        
        // Update vote counts
        self.update_vote_counts(target_type, target_id).await?;
        
        // Record activity
        let target_type_str = match target_type {
            VoteTargetType::Thread => "thread",
            VoteTargetType::ThreadReply => "thread_reply",
        };
        self.record_activity(user_id, ActivityType::Downvote, target_type_str, target_id).await?;
        
        Ok(())
    }

    /// Removes a vote
    pub async fn remove_vote(&self, user_id: Uuid, target_type: VoteTargetType, target_id: Uuid) -> Result<()> {
        self.repository.remove_vote(user_id, target_type, target_id).await?;
        
        // Update vote counts
        self.update_vote_counts(target_type, target_id).await?;
        
        Ok(())
    }

    /// Gets vote counts for a target
    pub async fn get_vote_counts(&self, target_type: VoteTargetType, target_id: Uuid) -> Result<(i64, i64)> {
        self.repository.get_vote_counts(target_type, target_id).await
    }

    // ===== CONTENT RANKING AND ALGORITHMS =====

    /// Sorts threads based on the specified algorithm
    async fn sort_threads(&self, threads: &mut Vec<Thread>, sort_by: ForumSortBy) -> Result<()> {
        match sort_by {
            ForumSortBy::New => {
                // Already sorted by creation time in repository
            }
            ForumSortBy::Hot => {
                // Calculate hot score for each thread
                for thread in threads.iter_mut() {
                    let factors = self.calculate_thread_ranking_factors(thread).await?;
                    thread.view_count = factors.final_score as i64; // Temporary storage for sorting
                }
                
                threads.sort_by(|a, b| b.view_count.cmp(&a.view_count));
            }
            ForumSortBy::Top => {
                // Sort by upvote count
                threads.sort_by(|a, b| b.upvote_count.cmp(&a.upvote_count));
            }
            ForumSortBy::Controversial => {
                // Sort by controversy score (high engagement with mixed votes)
                for thread in threads.iter_mut() {
                    let controversy_score = self.calculate_controversy_score(thread.upvote_count, thread.downvote_count);
                    thread.view_count = controversy_score as i64; // Temporary storage
                }
                
                threads.sort_by(|a, b| b.view_count.cmp(&a.view_count));
            }
            ForumSortBy::Rising => {
                // Sort by rising score (recent threads with growing engagement)
                for thread in threads.iter_mut() {
                    let rising_score = self.calculate_rising_score(thread).await?;
                    thread.view_count = rising_score as i64; // Temporary storage
                }
                
                threads.sort_by(|a, b| b.view_count.cmp(&a.view_count));
            }
        }
        
        Ok(())
    }

    /// Calculates ranking factors for a thread
    async fn calculate_thread_ranking_factors(&self, thread: &Thread) -> Result<ThreadRankingFactors> {
        let now = Utc::now();
        let age_hours = (now - thread.created_at).num_hours() as f64;
        
        // Upvote score (with diminishing returns for downvotes)
        let upvote_score = (thread.upvote_count as f64) - (thread.downvote_count as f64 * 0.5);
        
        // Engagement score based on replies and views
        let engagement_score = (thread.reply_count as f64 * 2.0) + (thread.view_count as f64 * 0.1);
        
        // Recency score (decays over time)
        let recency_score = if age_hours > 0.0 {
            1.0 / (1.0 + age_hours / 24.0) // Decay over days
        } else {
            1.0
        };
        
        // Reply velocity (replies per hour)
        let reply_velocity = if age_hours > 0.0 {
            thread.reply_count as f64 / age_hours
        } else {
            thread.reply_count as f64
        };
        
        // Final hot score calculation (similar to Reddit's algorithm)
        let final_score = (upvote_score + engagement_score) * recency_score + reply_velocity * 10.0;
        
        Ok(ThreadRankingFactors {
            upvote_score,
            engagement_score,
            recency_score,
            reply_velocity,
            final_score,
        })
    }

    /// Calculates controversy score
    fn calculate_controversy_score(&self, upvotes: i64, downvotes: i64) -> f64 {
        let total_votes = upvotes + downvotes;
        if total_votes == 0 {
            return 0.0;
        }
        
        let ratio = upvotes as f64 / total_votes as f64;
        let controversy = if ratio > 0.5 {
            (1.0 - ratio) * 2.0
        } else {
            ratio * 2.0
        };
        
        controversy * total_votes as f64
    }

    /// Calculates rising score for trending content
    async fn calculate_rising_score(&self, thread: &Thread) -> Result<f64> {
        let now = Utc::now();
        let age_hours = (now - thread.created_at).num_hours() as f64;
        
        // Only consider recent threads (last 24 hours)
        if age_hours > 24.0 {
            return Ok(0.0);
        }
        
        let factors = self.calculate_thread_ranking_factors(thread).await?;
        
        // Rising score emphasizes recent engagement
        let rising_score = factors.engagement_score * (24.0 - age_hours) / 24.0 + factors.reply_velocity * 5.0;
        
        Ok(rising_score)
    }

    // ===== MODERATION TOOLS =====

    /// Performs a moderation action
    pub async fn moderate_content(&self, input: ForumModerationInput) -> Result<()> {
        // Verify moderator permissions based on target
        match input.target_type {
            ModerationTargetType::Thread => {
                let thread = self.repository.find_thread_by_id(input.target_id).await?
                    .ok_or_else(|| anyhow!("Thread not found"))?;
                
                let community = self.repository.find_community_by_id(thread.community_id).await?
                    .ok_or_else(|| anyhow!("Community not found"))?;
                
                if !self.can_moderate_community(&community, input.moderator_id).await? {
                    return Err(anyhow!("Not authorized to moderate this content"));
                }
            }
            ModerationTargetType::ThreadReply => {
                let reply = self.repository.find_thread_reply_by_id(input.target_id).await?
                    .ok_or_else(|| anyhow!("Reply not found"))?;
                
                let thread = self.repository.find_thread_by_id(reply.thread_id).await?
                    .ok_or_else(|| anyhow!("Thread not found"))?;
                
                let community = self.repository.find_community_by_id(thread.community_id).await?
                    .ok_or_else(|| anyhow!("Community not found"))?;
                
                if !self.can_moderate_community(&community, input.moderator_id).await? {
                    return Err(anyhow!("Not authorized to moderate this content"));
                }
            }
            _ => return Err(anyhow!("Unsupported moderation target type")),
        }
        
        // Apply the moderation action
        match input.action_type {
            ModerationActionType::Pin => {
                if input.target_type == ModerationTargetType::Thread {
                    self.repository.pin_thread(input.target_id).await?;
                }
            }
            ModerationActionType::Unpin => {
                if input.target_type == ModerationTargetType::Thread {
                    self.repository.unpin_thread(input.target_id).await?;
                }
            }
            ModerationActionType::Lock => {
                if input.target_type == ModerationTargetType::Thread {
                    self.repository.lock_thread(input.target_id).await?;
                }
            }
            ModerationActionType::Unlock => {
                if input.target_type == ModerationTargetType::Thread {
                    self.repository.unlock_thread(input.target_id).await?;
                }
            }
            ModerationActionType::Delete => {
                match input.target_type {
                    ModerationTargetType::Thread => {
                        self.repository.delete_thread(input.target_id).await?;
                    }
                    ModerationTargetType::ThreadReply => {
                        self.repository.delete_thread_reply(input.target_id).await?;
                    }
                    _ => return Err(anyhow!("Unsupported delete target")),
                }
            }
            _ => return Err(anyhow!("Unsupported moderation action")),
        }
        
        // Record the moderation action
        self.record_activity(input.moderator_id, ActivityType::Edit, "moderation", input.target_id).await?;
        
        Ok(())
    }

    // ===== HELPER METHODS =====

    /// Updates vote counts for a target
    async fn update_vote_counts(&self, target_type: VoteTargetType, target_id: Uuid) -> Result<()> {
        let (upvotes, downvotes) = self.repository.get_vote_counts(target_type, target_id).await?;
        
        match target_type {
            VoteTargetType::Thread => {
                if let Some(mut thread) = self.repository.find_thread_by_id(target_id).await? {
                    thread.upvote_count = upvotes;
                    thread.downvote_count = downvotes;
                    self.repository.update_thread(&thread).await?;
                }
            }
            VoteTargetType::ThreadReply => {
                if let Some(mut reply) = self.repository.find_thread_reply_by_id(target_id).await? {
                    reply.upvote_count = upvotes;
                    reply.downvote_count = downvotes;
                    self.repository.update_thread_reply(&reply).await?;
                }
            }
        }
        
        Ok(())
    }

    /// Validates community input
    fn validate_community_input(&self, input: &CreateCommunityInput) -> Result<()> {
        if input.name.trim().is_empty() {
            return Err(anyhow!("Community name cannot be empty"));
        }
        
        if input.name.len() > self.max_community_name_length {
            return Err(anyhow!("Community name too long"));
        }
        
        if input.display_name.trim().is_empty() {
            return Err(anyhow!("Community display name cannot be empty"));
        }
        
        if input.description.len() > 1000 {
            return Err(anyhow!("Community description too long"));
        }
        
        // Check for banned words
        let content_lower = format!("{} {}", input.name, input.description).to_lowercase();
        for banned_word in &self.banned_words {
            if content_lower.contains(banned_word) {
                return Err(anyhow!("Content contains prohibited words"));
            }
        }
        
        Ok(())
    }

    /// Validates forum input
    fn validate_forum_input(&self, input: &CreateForumInput) -> Result<()> {
        if input.name.trim().is_empty() {
            return Err(anyhow!("Forum name cannot be empty"));
        }
        
        if input.name.len() > 100 {
            return Err(anyhow!("Forum name too long"));
        }
        
        if input.description.len() > 500 {
            return Err(anyhow!("Forum description too long"));
        }
        
        Ok(())
    }

    /// Validates thread input
    fn validate_thread_input(&self, input: &CreateThreadInput) -> Result<()> {
        if input.title.trim().is_empty() {
            return Err(anyhow!("Thread title cannot be empty"));
        }
        
        if input.title.len() > self.max_thread_title_length {
            return Err(anyhow!("Thread title too long"));
        }
        
        if input.content.trim().is_empty() {
            return Err(anyhow!("Thread content cannot be empty"));
        }
        
        if input.content.len() > self.max_thread_content_length {
            return Err(anyhow!("Thread content too long"));
        }
        
        // Check for banned words
        let content_lower = format!("{} {}", input.title, input.content).to_lowercase();
        for banned_word in &self.banned_words {
            if content_lower.contains(banned_word) {
                return Err(anyhow!("Content contains prohibited words"));
            }
        }
        
        Ok(())
    }

    /// Validates reply input
    fn validate_reply_input(&self, input: &CreateThreadReplyInput) -> Result<()> {
        if input.content.trim().is_empty() {
            return Err(anyhow!("Reply content cannot be empty"));
        }
        
        if input.content.len() > self.max_reply_content_length {
            return Err(anyhow!("Reply content too long"));
        }
        
        // Check for banned words
        let content_lower = input.content.to_lowercase();
        for banned_word in &self.banned_words {
            if content_lower.contains(banned_word) {
                return Err(anyhow!("Content contains prohibited words"));
            }
        }
        
        Ok(())
    }

    /// Checks if a user can view a community
    async fn can_view_community(&self, community: &Community, viewer_id: Option<Uuid>) -> Result<bool> {
        if !community.is_private {
            return Ok(true);
        }
        
        match viewer_id {
            Some(user_id) => {
                // Check if user is a member
                let memberships = self.repository.get_user_communities(user_id).await?;
                Ok(memberships.iter().any(|m| m.community_id == community.id && !m.is_banned))
            }
            None => Ok(false),
        }
    }

    /// Checks if a user can post in a community
    async fn can_post_in_community(&self, community: &Community, user_id: Uuid) -> Result<bool> {
        let memberships = self.repository.get_user_communities(user_id).await?;
        let membership = memberships.iter().find(|m| m.community_id == community.id);
        
        match membership {
            Some(membership) => Ok(!membership.is_banned),
            None => Ok(!community.is_private), // Can post in public communities without membership
        }
    }

    /// Checks if a user can moderate a community
    async fn can_moderate_community(&self, community: &Community, user_id: Uuid) -> Result<bool> {
        if community.owner_id == user_id {
            return Ok(true);
        }
        
        if community.moderator_ids.contains(&user_id) {
            return Ok(true);
        }
        
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
        
        // TODO: Store activity in repository
        // self.repository.record_user_activity(&activity).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_service() -> ForumService {
        let repository = Box::new(MockForumRepository::new());
        ForumService::new(repository)
    }

    #[test]
    fn test_forum_service_creation() {
        let service = create_test_service();
        
        // Test that the service was created successfully
        assert_eq!(service.max_thread_title_length, 300);
        assert_eq!(service.max_thread_content_length, 10000);
        assert_eq!(service.max_reply_content_length, 5000);
        assert_eq!(service.max_community_name_length, 50);
    }

    #[test]
    fn test_validate_community_input() {
        let service = create_test_service();
        
        let valid_input = CreateCommunityInput {
            name: "test-community".to_string(),
            display_name: "Test Community".to_string(),
            description: "A test community".to_string(),
            is_private: false,
            is_nsfw: false,
            tags: vec!["test".to_string()],
        };
        
        assert!(service.validate_community_input(&valid_input).is_ok());
        
        let invalid_input = CreateCommunityInput {
            name: "".to_string(), // Empty name should fail
            display_name: "Test Community".to_string(),
            description: "A test community".to_string(),
            is_private: false,
            is_nsfw: false,
            tags: vec![],
        };
        
        assert!(service.validate_community_input(&invalid_input).is_err());
    }

    #[test]
    fn test_validate_thread_input() {
        let service = create_test_service();
        
        let valid_input = CreateThreadInput {
            forum_id: Uuid::new_v4(),
            title: "Test Thread".to_string(),
            content: "This is a test thread content".to_string(),
            tags: vec!["test".to_string()],
            flair: Some("Discussion".to_string()),
        };
        
        assert!(service.validate_thread_input(&valid_input).is_ok());
        
        let invalid_input = CreateThreadInput {
            forum_id: Uuid::new_v4(),
            title: "".to_string(), // Empty title should fail
            content: "This is a test thread content".to_string(),
            tags: vec![],
            flair: None,
        };
        
        assert!(service.validate_thread_input(&invalid_input).is_err());
    }

    #[test]
    fn test_calculate_controversy_score() {
        let service = create_test_service();
        
        // Test balanced votes (high controversy)
        let score = service.calculate_controversy_score(50, 50);
        assert!(score > 0.0);
        
        // Test one-sided votes (low controversy)
        let score = service.calculate_controversy_score(100, 0);
        assert_eq!(score, 0.0);
        
        // Test no votes
        let score = service.calculate_controversy_score(0, 0);
        assert_eq!(score, 0.0);
    }

    // Mock repository for testing
    struct MockForumRepository;
    
    impl MockForumRepository {
        fn new() -> Self {
            Self
        }
    }
    
    #[async_trait]
    impl ForumRepository for MockForumRepository {
        async fn create_community(&self, _data: CreateCommunityData) -> Result<Community> {
            unimplemented!()
        }
        
        async fn find_community_by_id(&self, _id: Uuid) -> Result<Option<Community>> {
            unimplemented!()
        }
        
        async fn find_community_by_name(&self, _name: &str) -> Result<Option<Community>> {
            unimplemented!()
        }
        
        async fn update_community(&self, _community: &Community) -> Result<()> {
            unimplemented!()
        }
        
        async fn delete_community(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_all_communities(&self, _limit: i32, _offset: i32) -> Result<Vec<Community>> {
            unimplemented!()
        }
        
        async fn search_communities(&self, _query: &str, _limit: i32, _offset: i32) -> Result<Vec<Community>> {
            unimplemented!()
        }
        
        async fn join_community(&self, _community_id: Uuid, _user_id: Uuid, _role: CommunityRole) -> Result<CommunityMembership> {
            unimplemented!()
        }
        
        async fn leave_community(&self, _community_id: Uuid, _user_id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_community_members(&self, _community_id: Uuid, _limit: i32, _offset: i32) -> Result<Vec<CommunityMembership>> {
            unimplemented!()
        }
        
        async fn get_user_communities(&self, _user_id: Uuid) -> Result<Vec<CommunityMembership>> {
            unimplemented!()
        }
        
        async fn update_member_role(&self, _community_id: Uuid, _user_id: Uuid, _role: CommunityRole) -> Result<()> {
            unimplemented!()
        }
        
        async fn ban_member(&self, _community_id: Uuid, _user_id: Uuid, _reason: String, _expires_at: Option<DateTime<Utc>>) -> Result<()> {
            unimplemented!()
        }
        
        async fn unban_member(&self, _community_id: Uuid, _user_id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn create_forum(&self, _data: CreateForumData) -> Result<Forum> {
            unimplemented!()
        }
        
        async fn find_forum_by_id(&self, _id: Uuid) -> Result<Option<Forum>> {
            unimplemented!()
        }
        
        async fn update_forum(&self, _forum: &Forum) -> Result<()> {
            unimplemented!()
        }
        
        async fn delete_forum(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_community_forums(&self, _community_id: Uuid) -> Result<Vec<Forum>> {
            unimplemented!()
        }
        
        async fn lock_forum(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn unlock_forum(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn pin_forum(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn unpin_forum(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn create_thread(&self, _data: CreateThreadData) -> Result<Thread> {
            unimplemented!()
        }
        
        async fn find_thread_by_id(&self, _id: Uuid) -> Result<Option<Thread>> {
            unimplemented!()
        }
        
        async fn update_thread(&self, _thread: &Thread) -> Result<()> {
            unimplemented!()
        }
        
        async fn delete_thread(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_forum_threads(&self, _forum_id: Uuid, _limit: i32, _offset: i32) -> Result<Vec<Thread>> {
            unimplemented!()
        }
        
        async fn get_community_threads(&self, _community_id: Uuid, _limit: i32, _offset: i32) -> Result<Vec<Thread>> {
            unimplemented!()
        }
        
        async fn search_threads(&self, _query: &str, _community_id: Option<Uuid>, _limit: i32, _offset: i32) -> Result<Vec<Thread>> {
            unimplemented!()
        }
        
        async fn lock_thread(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn unlock_thread(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn pin_thread(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn unpin_thread(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn increment_thread_views(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn create_thread_reply(&self, _data: CreateThreadReplyData) -> Result<ThreadReply> {
            unimplemented!()
        }
        
        async fn find_thread_reply_by_id(&self, _id: Uuid) -> Result<Option<ThreadReply>> {
            unimplemented!()
        }
        
        async fn update_thread_reply(&self, _reply: &ThreadReply) -> Result<()> {
            unimplemented!()
        }
        
        async fn delete_thread_reply(&self, _id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_thread_replies(&self, _thread_id: Uuid, _limit: i32, _offset: i32) -> Result<Vec<ThreadReply>> {
            unimplemented!()
        }
        
        async fn get_reply_children(&self, _parent_reply_id: Uuid, _limit: i32, _offset: i32) -> Result<Vec<ThreadReply>> {
            unimplemented!()
        }
        
        async fn create_vote(&self, _vote: &Vote) -> Result<()> {
            unimplemented!()
        }
        
        async fn remove_vote(&self, _user_id: Uuid, _target_type: VoteTargetType, _target_id: Uuid) -> Result<()> {
            unimplemented!()
        }
        
        async fn update_vote(&self, _user_id: Uuid, _target_type: VoteTargetType, _target_id: Uuid, _vote_type: VoteType) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_vote_counts(&self, _target_type: VoteTargetType, _target_id: Uuid) -> Result<(i64, i64)> {
            unimplemented!()
        }
        
        async fn get_user_vote(&self, _user_id: Uuid, _target_type: VoteTargetType, _target_id: Uuid) -> Result<Option<Vote>> {
            unimplemented!()
        }
        
        async fn update_moderation_settings(&self, _settings: &ModerationSettings) -> Result<()> {
            unimplemented!()
        }
        
        async fn get_moderation_settings(&self, _forum_id: Uuid) -> Result<Option<ModerationSettings>> {
            unimplemented!()
        }
    }
}