use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use futures_util::{Stream, StreamExt};
use cpc_core::models::social::{
    PostType, Community, CommunityRule, Forum, ForumCategory, ModerationSettings,
    Thread, ThreadReply, ThreadReplyEdit, CommunityMembership, CommunityRole,
    Vote as SocialVote, VoteTargetType, VoteType, ModerationAction, ModerationTargetType, ModerationActionType
};

/// GraphQL representation of a Community
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct CommunityType {
    pub id: ID,
    pub name: String,
    pub description: String,
    pub creator_id: ID,
    pub member_count: i32,
    pub post_count: i32,
    pub is_private: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl CommunityType {
    /// Get community creator
    async fn creator(&self, ctx: &Context<'_>) -> Result<Option<super::user_management::UserType>> {
        // TODO: Implement creator loading via service
        Ok(None)
    }

    /// Get community forums
    async fn forums(&self, ctx: &Context<'_>) -> Result<Vec<ForumType>> {
        // TODO: Implement forums loading via service
        Ok(vec![])
    }

    /// Get community rules
    async fn rules(&self, ctx: &Context<'_>) -> Result<Vec<CommunityRuleType>> {
        // TODO: Implement rules loading via service
        Ok(vec![])
    }

    /// Get community members
    async fn members(&self, ctx: &Context<'_>, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<CommunityMembershipType>> {
        // TODO: Implement members loading via service
        Ok(vec![])
    }

    /// Get community moderators
    async fn moderators(&self, ctx: &Context<'_>) -> Result<Vec<CommunityMembershipType>> {
        // TODO: Implement moderators loading via service
        Ok(vec![])
    }

    /// Check if current user is a member
    async fn is_member(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement membership check via service
        Ok(false)
    }

    /// Check if current user is a moderator
    async fn is_moderator(&self, ctx: &Context<'_>) -> Result<bool> {
        // TODO: Implement moderator check via service
        Ok(false)
    }

    /// Get recent threads
    async fn recent_threads(&self, ctx: &Context<'_>, limit: Option<i32>) -> Result<Vec<ThreadType>> {
        // TODO: Implement recent threads loading via service
        Ok(vec![])
    }
}

/// GraphQL representation of a Forum
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ForumType {
    pub id: ID,
    pub community_id: ID,
    pub name: String,
    pub description: String,
    pub category: ForumCategoryType,
    pub thread_count: i32,
    pub post_count: i32,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl ForumType {
    /// Get forum community
    async fn community(&self, ctx: &Context<'_>) -> Result<Option<CommunityType>> {
        // TODO: Implement community loading via service
        Ok(None)
    }

    /// Get forum threads
    async fn threads(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        sort_by: Option<ThreadSortType>,
    ) -> Result<Vec<ThreadType>> {
        // TODO: Implement threads loading via service
        Ok(vec![])
    }

    /// Get pinned threads
    async fn pinned_threads(&self, ctx: &Context<'_>) -> Result<Vec<ThreadType>> {
        // TODO: Implement pinned threads loading via service
        Ok(vec![])
    }

    /// Get moderation settings
    async fn moderation_settings(&self, ctx: &Context<'_>) -> Result<Option<ModerationSettingsType>> {
        // TODO: Implement moderation settings loading via service
        Ok(None)
    }
}

/// GraphQL enum for ForumCategory
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ForumCategoryType {
    General,
    Discussion,
    QAndA,
    Announcements,
    Support,
    Feedback,
    Projects,
    Marketplace,
}

/// GraphQL enum for thread sorting
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ThreadSortType {
    Recent,
    Popular,
    MostReplies,
    MostVotes,
    Oldest,
}

/// GraphQL representation of a Thread
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ThreadType {
    pub id: ID,
    pub forum_id: ID,
    pub author_id: ID,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub reply_count: i32,
    pub vote_score: i32,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

#[ComplexObject]
impl ThreadType {
    /// Get thread author
    async fn author(&self, ctx: &Context<'_>) -> Result<Option<super::user_management::UserType>> {
        // TODO: Implement author loading via service
        Ok(None)
    }

    /// Get thread forum
    async fn forum(&self, ctx: &Context<'_>) -> Result<Option<ForumType>> {
        // TODO: Implement forum loading via service
        Ok(None)
    }

    /// Get thread replies
    async fn replies(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        sort_by: Option<ReplySortType>,
    ) -> Result<Vec<ThreadReplyType>> {
        // TODO: Implement replies loading via service
        Ok(vec![])
    }

    /// Get thread votes
    async fn votes(&self, ctx: &Context<'_>) -> Result<Vec<VoteType>> {
        // TODO: Implement votes loading via service
        Ok(vec![])
    }

    /// Check if current user has voted
    async fn my_vote(&self, ctx: &Context<'_>) -> Result<Option<VoteType>> {
        // TODO: Implement user vote check via service
        Ok(None)
    }

    /// Get thread tags
    async fn tags(&self, ctx: &Context<'_>) -> Result<Vec<String>> {
        // TODO: Implement tags loading via service
        Ok(vec![])
    }
}

/// GraphQL enum for reply sorting
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ReplySortType {
    Chronological,
    Popular,
    MostVotes,
    Newest,
    Oldest,
}

/// GraphQL representation of a ThreadReply
#[derive(SimpleObject, Clone)]
#[graphql(complex)]
pub struct ThreadReplyType {
    pub id: ID,
    pub thread_id: ID,
    pub author_id: ID,
    pub content: String,
    pub parent_reply_id: Option<ID>,
    pub vote_score: i32,
    pub upvote_count: i32,
    pub downvote_count: i32,
    pub is_edited: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[ComplexObject]
impl ThreadReplyType {
    /// Get reply author
    async fn author(&self, ctx: &Context<'_>) -> Result<Option<super::user_management::UserType>> {
        // TODO: Implement author loading via service
        Ok(None)
    }

    /// Get parent reply
    async fn parent_reply(&self, ctx: &Context<'_>) -> Result<Option<ThreadReplyType>> {
        // TODO: Implement parent reply loading via service
        Ok(None)
    }

    /// Get child replies
    async fn child_replies(&self, ctx: &Context<'_>) -> Result<Vec<ThreadReplyType>> {
        // TODO: Implement child replies loading via service
        Ok(vec![])
    }

    /// Get reply votes
    async fn votes(&self, ctx: &Context<'_>) -> Result<Vec<VoteType>> {
        // TODO: Implement votes loading via service
        Ok(vec![])
    }

    /// Check if current user has voted
    async fn my_vote(&self, ctx: &Context<'_>) -> Result<Option<VoteType>> {
        // TODO: Implement user vote check via service
        Ok(None)
    }
}

/// GraphQL representation of a Vote
#[derive(SimpleObject, Clone)]
pub struct VoteType {
    pub id: ID,
    pub user_id: ID,
    pub target_id: ID,
    pub target_type: VoteTargetTypeGraphQL,
    pub vote_type: VoteTypeGraphQL,
    pub created_at: DateTime<Utc>,
}

/// GraphQL enum for VoteTargetType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VoteTargetTypeGraphQL {
    Thread,
    Reply,
    Post,
    Comment,
}

/// GraphQL enum for VoteType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum VoteTypeGraphQL {
    Upvote,
    Downvote,
}

/// GraphQL representation of CommunityRule
#[derive(SimpleObject, Clone)]
pub struct CommunityRuleType {
    pub id: ID,
    pub community_id: ID,
    pub title: String,
    pub description: String,
    pub rule_order: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

/// GraphQL representation of CommunityMembership
#[derive(SimpleObject, Clone)]
pub struct CommunityMembershipType {
    pub id: ID,
    pub community_id: ID,
    pub user_id: ID,
    pub role: CommunityRoleType,
    pub joined_at: DateTime<Utc>,
    pub is_active: bool,
}

/// GraphQL enum for CommunityRole
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum CommunityRoleType {
    Member,
    Moderator,
    Admin,
    Owner,
}

/// GraphQL representation of ModerationSettings
#[derive(SimpleObject, Clone)]
pub struct ModerationSettingsType {
    pub id: ID,
    pub forum_id: ID,
    pub require_approval: bool,
    pub auto_lock_after_days: Option<i32>,
    pub max_replies_per_thread: Option<i32>,
    pub allow_anonymous_posts: bool,
    pub content_filters: Vec<String>,
}

/// GraphQL representation of ModerationAction
#[derive(SimpleObject, Clone)]
pub struct ModerationActionType {
    pub id: ID,
    pub moderator_id: ID,
    pub target_id: ID,
    pub target_type: ModerationTargetTypeGraphQL,
    pub action_type: ModerationActionTypeGraphQL,
    pub reason: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// GraphQL enum for ModerationTargetType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ModerationTargetTypeGraphQL {
    Thread,
    Reply,
    User,
    Community,
}

/// GraphQL enum for ModerationActionType
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ModerationActionTypeGraphQL {
    Pin,
    Unpin,
    Lock,
    Unlock,
    Delete,
    Hide,
    Ban,
    Unban,
    Warn,
}

/// Input for creating a community
#[derive(InputObject)]
pub struct CreateCommunityInput {
    pub name: String,
    pub description: String,
    pub is_private: Option<bool>,
}

/// Input for updating a community
#[derive(InputObject)]
pub struct UpdateCommunityInput {
    pub community_id: ID,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
}

/// Input for creating a forum
#[derive(InputObject)]
pub struct CreateForumInput {
    pub community_id: ID,
    pub name: String,
    pub description: String,
    pub category: ForumCategoryType,
}

/// Input for creating a thread
#[derive(InputObject)]
pub struct CreateThreadInput {
    pub forum_id: ID,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

/// Input for updating a thread
#[derive(InputObject)]
pub struct UpdateThreadInput {
    pub thread_id: ID,
    pub title: Option<String>,
    pub content: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Input for creating a thread reply
#[derive(InputObject)]
pub struct CreateThreadReplyInput {
    pub thread_id: ID,
    pub content: String,
    pub parent_reply_id: Option<ID>,
}

/// Input for updating a thread reply
#[derive(InputObject)]
pub struct UpdateThreadReplyInput {
    pub reply_id: ID,
    pub content: String,
}

/// Input for voting
#[derive(InputObject)]
pub struct VoteInput {
    pub target_id: ID,
    pub target_type: VoteTargetTypeGraphQL,
    pub vote_type: VoteTypeGraphQL,
}

/// Input for moderation actions
#[derive(InputObject)]
pub struct ModerationActionInput {
    pub target_id: ID,
    pub target_type: ModerationTargetTypeGraphQL,
    pub action_type: ModerationActionTypeGraphQL,
    pub reason: String,
    pub notes: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Forum system queries
#[derive(Default)]
pub struct ForumQuery;

#[Object]
impl ForumQuery {
    /// Get community by ID
    async fn community(&self, ctx: &Context<'_>, id: ID) -> Result<Option<CommunityType>> {
        let community_id = Uuid::parse_str(&id.to_string())?;
        
        // TODO: Implement community service to get community by ID
        // let community_service = ctx.data::<std::sync::Arc<CommunityService>>()?;
        // match community_service.get_community_by_id(community_id).await {
        //     Ok(community) => Ok(Some(community.into())),
        //     Err(CommunityServiceError::NotFound) => Ok(None),
        //     Err(e) => Err(format!("Failed to get community: {:?}", e).into()),
        // }
        
        Ok(None)
    }

    /// Get communities with pagination
    async fn communities(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
        search: Option<String>,
    ) -> Result<Vec<CommunityType>> {
        // TODO: Implement community service to list communities
        // let community_service = ctx.data::<std::sync::Arc<CommunityService>>()?;
        // let communities = community_service.list_communities(
        //     limit.unwrap_or(20),
        //     offset.unwrap_or(0),
        //     search
        // ).await?;
        // Ok(communities.into_iter().map(Into::into).collect())
        
        Ok(vec![])
    }

    /// Get forum by ID
    async fn forum(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ForumType>> {
        // TODO: Implement forum retrieval
        Ok(None)
    }

    /// Get thread by ID
    async fn thread(&self, ctx: &Context<'_>, id: ID) -> Result<Option<ThreadType>> {
        // TODO: Implement thread retrieval
        Ok(None)
    }

    /// Search threads
    async fn search_threads(
        &self,
        ctx: &Context<'_>,
        query: String,
        forum_id: Option<ID>,
        community_id: Option<ID>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<ThreadType>> {
        // TODO: Implement thread search
        Ok(vec![])
    }

    /// Get trending threads
    async fn trending_threads(
        &self,
        ctx: &Context<'_>,
        time_range: Option<String>,
        community_id: Option<ID>,
        limit: Option<i32>,
    ) -> Result<Vec<ThreadType>> {
        // TODO: Implement trending threads retrieval
        Ok(vec![])
    }

    /// Get user's communities
    async fn my_communities(&self, ctx: &Context<'_>) -> Result<Vec<CommunityMembershipType>> {
        // TODO: Implement user communities retrieval
        Ok(vec![])
    }

    /// Get moderation actions for a target
    async fn moderation_actions(
        &self,
        ctx: &Context<'_>,
        target_id: ID,
        target_type: ModerationTargetTypeGraphQL,
    ) -> Result<Vec<ModerationActionType>> {
        // TODO: Implement moderation actions retrieval
        Ok(vec![])
    }
}

/// Forum system mutations
#[derive(Default)]
pub struct ForumMutation;

#[Object]
impl ForumMutation {
    /// Create a new community
    async fn create_community(&self, ctx: &Context<'_>, input: CreateCommunityInput) -> Result<CommunityType> {
        // TODO: Implement community creation
        Err("Not implemented".into())
    }

    /// Update a community
    async fn update_community(&self, ctx: &Context<'_>, input: UpdateCommunityInput) -> Result<CommunityType> {
        // TODO: Implement community update
        Err("Not implemented".into())
    }

    /// Join a community
    async fn join_community(&self, ctx: &Context<'_>, community_id: ID) -> Result<CommunityMembershipType> {
        // TODO: Implement community joining
        Err("Not implemented".into())
    }

    /// Leave a community
    async fn leave_community(&self, ctx: &Context<'_>, community_id: ID) -> Result<bool> {
        // TODO: Implement community leaving
        Err("Not implemented".into())
    }

    /// Create a new forum
    async fn create_forum(&self, ctx: &Context<'_>, input: CreateForumInput) -> Result<ForumType> {
        // TODO: Implement forum creation
        Err("Not implemented".into())
    }

    /// Create a new thread
    async fn create_thread(&self, ctx: &Context<'_>, input: CreateThreadInput) -> Result<ThreadType> {
        // TODO: Implement thread creation
        Err("Not implemented".into())
    }

    /// Update a thread
    async fn update_thread(&self, ctx: &Context<'_>, input: UpdateThreadInput) -> Result<ThreadType> {
        // TODO: Implement thread update
        Err("Not implemented".into())
    }

    /// Delete a thread
    async fn delete_thread(&self, ctx: &Context<'_>, thread_id: ID) -> Result<bool> {
        // TODO: Implement thread deletion
        Err("Not implemented".into())
    }

    /// Create a thread reply
    async fn create_thread_reply(&self, ctx: &Context<'_>, input: CreateThreadReplyInput) -> Result<ThreadReplyType> {
        // TODO: Implement thread reply creation
        Err("Not implemented".into())
    }

    /// Update a thread reply
    async fn update_thread_reply(&self, ctx: &Context<'_>, input: UpdateThreadReplyInput) -> Result<ThreadReplyType> {
        // TODO: Implement thread reply update
        Err("Not implemented".into())
    }

    /// Delete a thread reply
    async fn delete_thread_reply(&self, ctx: &Context<'_>, reply_id: ID) -> Result<bool> {
        // TODO: Implement thread reply deletion
        Err("Not implemented".into())
    }

    /// Vote on a thread or reply
    async fn vote(&self, ctx: &Context<'_>, input: VoteInput) -> Result<VoteType> {
        // TODO: Implement voting
        Err("Not implemented".into())
    }

    /// Remove a vote
    async fn remove_vote(&self, ctx: &Context<'_>, target_id: ID, target_type: VoteTargetTypeGraphQL) -> Result<bool> {
        // TODO: Implement vote removal
        Err("Not implemented".into())
    }

    /// Perform moderation action
    async fn moderate(&self, ctx: &Context<'_>, input: ModerationActionInput) -> Result<ModerationActionType> {
        // TODO: Implement moderation action
        Err("Not implemented".into())
    }
}

/// Forum system subscriptions
#[derive(Default)]
pub struct ForumSubscription;

#[Subscription]
impl ForumSubscription {
    /// Subscribe to new threads in a forum
    async fn forum_threads(&self, ctx: &Context<'_>, forum_id: ID) -> Result<impl Stream<Item = ThreadType>> {
        let forum_uuid = Uuid::parse_str(&forum_id.to_string())?;
        
        // Create a subscription stream for new threads in a forum
        Ok(async_graphql_simple_broker::SimpleBroker::<ThreadType>::subscribe()
            .filter(move |thread| {
                let thread_forum_id = Uuid::parse_str(&thread.forum_id.to_string()).unwrap_or_default();
                async move { thread_forum_id == forum_uuid }
            }))
    }

    /// Subscribe to new replies in a thread
    async fn thread_replies(&self, ctx: &Context<'_>, thread_id: ID) -> Result<impl Stream<Item = ThreadReplyType>> {
        let thread_uuid = Uuid::parse_str(&thread_id.to_string())?;
        
        // Create a subscription stream for new replies in a thread
        Ok(async_graphql_simple_broker::SimpleBroker::<ThreadReplyType>::subscribe()
            .filter(move |reply| {
                let reply_thread_id = Uuid::parse_str(&reply.thread_id.to_string()).unwrap_or_default();
                async move { reply_thread_id == thread_uuid }
            }))
    }

    /// Subscribe to vote updates on a thread or reply
    async fn vote_updates(&self, ctx: &Context<'_>, target_id: ID, target_type: VoteTargetTypeGraphQL) -> Result<impl Stream<Item = VoteType>> {
        let target_uuid = Uuid::parse_str(&target_id.to_string())?;
        
        // Create a subscription stream for vote updates
        Ok(async_graphql_simple_broker::SimpleBroker::<VoteType>::subscribe()
            .filter(move |vote| {
                let vote_target_id = Uuid::parse_str(&vote.target_id.to_string()).unwrap_or_default();
                async move { 
                    vote_target_id == target_uuid && vote.target_type == target_type
                }
            }))
    }

    /// Subscribe to community updates
    async fn community_updates(&self, ctx: &Context<'_>, community_id: ID) -> Result<impl Stream<Item = CommunityType>> {
        // TODO: Implement community updates subscription
        Ok(async_stream::stream! {
            // Empty stream for now
        })
    }

    /// Subscribe to moderation actions
    async fn moderation_actions(&self, ctx: &Context<'_>, target_id: ID) -> Result<impl Stream<Item = ModerationActionType>> {
        // TODO: Implement moderation actions subscription
        Ok(async_stream::stream! {
            // Empty stream for now
        })
    }
}

// Conversion implementations
impl From<ForumCategory> for ForumCategoryType {
    fn from(category: ForumCategory) -> Self {
        match category {
            ForumCategory::General => ForumCategoryType::General,
            ForumCategory::Discussion => ForumCategoryType::Discussion,
            ForumCategory::QAndA => ForumCategoryType::QAndA,
            ForumCategory::Announcements => ForumCategoryType::Announcements,
            ForumCategory::Support => ForumCategoryType::Support,
            ForumCategory::Feedback => ForumCategoryType::Feedback,
            ForumCategory::Projects => ForumCategoryType::Projects,
            ForumCategory::Marketplace => ForumCategoryType::Marketplace,
        }
    }
}

impl From<CommunityRole> for CommunityRoleType {
    fn from(role: CommunityRole) -> Self {
        match role {
            CommunityRole::Member => CommunityRoleType::Member,
            CommunityRole::Moderator => CommunityRoleType::Moderator,
            CommunityRole::Admin => CommunityRoleType::Admin,
            CommunityRole::Owner => CommunityRoleType::Owner,
        }
    }
}

impl From<VoteTargetType> for VoteTargetTypeGraphQL {
    fn from(target_type: VoteTargetType) -> Self {
        match target_type {
            VoteTargetType::Thread => VoteTargetTypeGraphQL::Thread,
            VoteTargetType::Reply => VoteTargetTypeGraphQL::Reply,
            VoteTargetType::Post => VoteTargetTypeGraphQL::Post,
            VoteTargetType::Comment => VoteTargetTypeGraphQL::Comment,
        }
    }
}

impl From<VoteType> for VoteTypeGraphQL {
    fn from(vote_type: VoteType) -> Self {
        match vote_type {
            VoteType::Upvote => VoteTypeGraphQL::Upvote,
            VoteType::Downvote => VoteTypeGraphQL::Downvote,
        }
    }
}

impl From<ModerationTargetType> for ModerationTargetTypeGraphQL {
    fn from(target_type: ModerationTargetType) -> Self {
        match target_type {
            ModerationTargetType::Thread => ModerationTargetTypeGraphQL::Thread,
            ModerationTargetType::Reply => ModerationTargetTypeGraphQL::Reply,
            ModerationTargetType::User => ModerationTargetTypeGraphQL::User,
            ModerationTargetType::Community => ModerationTargetTypeGraphQL::Community,
        }
    }
}

impl From<ModerationActionType> for ModerationActionTypeGraphQL {
    fn from(action_type: ModerationActionType) -> Self {
        match action_type {
            ModerationActionType::Pin => ModerationActionTypeGraphQL::Pin,
            ModerationActionType::Unpin => ModerationActionTypeGraphQL::Unpin,
            ModerationActionType::Lock => ModerationActionTypeGraphQL::Lock,
            ModerationActionType::Unlock => ModerationActionTypeGraphQL::Unlock,
            ModerationActionType::Delete => ModerationActionTypeGraphQL::Delete,
            ModerationActionType::Hide => ModerationActionTypeGraphQL::Hide,
            ModerationActionType::Ban => ModerationActionTypeGraphQL::Ban,
            ModerationActionType::Unban => ModerationActionTypeGraphQL::Unban,
            ModerationActionType::Warn => ModerationActionTypeGraphQL::Warn,
        }
    }
}

// Additional conversion implementations for core forum types
impl From<Community> for CommunityType {
    fn from(community: Community) -> Self {
        Self {
            id: community.id.into(),
            name: community.name,
            description: community.description,
            creator_id: community.owner_id.into(), // Using owner_id as creator_id
            member_count: community.member_count as i32,
            post_count: 0, // TODO: Calculate from actual data
            is_private: community.is_private,
            created_at: community.created_at,
            updated_at: community.updated_at,
        }
    }
}

impl From<Thread> for ThreadType {
    fn from(thread: Thread) -> Self {
        Self {
            id: thread.id.into(),
            forum_id: thread.forum_id.into(),
            author_id: thread.author_id.into(),
            title: thread.title,
            content: thread.content,
            is_pinned: thread.is_pinned,
            is_locked: thread.is_locked,
            reply_count: thread.reply_count,
            vote_score: thread.vote_score,
            upvote_count: thread.upvote_count,
            downvote_count: thread.downvote_count,
            created_at: thread.created_at,
            updated_at: thread.updated_at,
            last_activity: thread.last_activity,
        }
    }
}

impl From<ThreadReply> for ThreadReplyType {
    fn from(reply: ThreadReply) -> Self {
        Self {
            id: reply.id.into(),
            thread_id: reply.thread_id.into(),
            author_id: reply.author_id.into(),
            content: reply.content,
            parent_reply_id: reply.parent_reply_id.map(Into::into),
            vote_score: reply.vote_score,
            upvote_count: reply.upvote_count,
            downvote_count: reply.downvote_count,
            is_edited: reply.is_edited,
            created_at: reply.created_at,
            updated_at: reply.updated_at,
        }
    }
}