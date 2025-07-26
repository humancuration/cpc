use async_graphql_simple_broker::SimpleBroker;
use crate::graphql::{
    user_management::{UserProfileType, CooperativeScoreType, UserRelationshipGraphQL},
    social_interactions::{PostType, CommentType, LikeType, ShareType, NotificationType, FeedItemType},
    forum_system::{ThreadType, ThreadReplyType, VoteType, CommunityType, ModerationActionType},
    governance_system::{ProposalType, GovernanceVoteType, VotingResultType, GovernanceParticipationType},
};

/// Event publisher for GraphQL subscriptions
/// This module provides functions to publish events that will be sent to subscribers
pub struct SubscriptionEventPublisher;

impl SubscriptionEventPublisher {
    // User Management Events
    
    /// Publish user profile update event
    pub fn publish_user_profile_updated(profile: UserProfileType) {
        SimpleBroker::publish(profile);
    }
    
    /// Publish cooperative score update event
    pub fn publish_cooperative_score_updated(score: CooperativeScoreType) {
        SimpleBroker::publish(score);
    }
    
    /// Publish user relationship update event
    pub fn publish_relationship_updated(relationship: UserRelationshipGraphQL) {
        SimpleBroker::publish(relationship);
    }
    
    // Social Interaction Events
    
    /// Publish post update event
    pub fn publish_post_updated(post: PostType) {
        SimpleBroker::publish(post);
    }
    
    /// Publish new comment event
    pub fn publish_comment_created(comment: CommentType) {
        SimpleBroker::publish(comment);
    }
    
    /// Publish new like event
    pub fn publish_like_created(like: LikeType) {
        SimpleBroker::publish(like);
    }
    
    /// Publish new share event
    pub fn publish_share_created(share: ShareType) {
        SimpleBroker::publish(share);
    }
    
    /// Publish notification event
    pub fn publish_notification(notification: NotificationType) {
        SimpleBroker::publish(notification);
    }
    
    /// Publish feed update event
    pub fn publish_feed_updated(feed_item: FeedItemType) {
        SimpleBroker::publish(feed_item);
    }
    
    // Forum System Events
    
    /// Publish new thread event
    pub fn publish_thread_created(thread: ThreadType) {
        SimpleBroker::publish(thread);
    }
    
    /// Publish new thread reply event
    pub fn publish_thread_reply_created(reply: ThreadReplyType) {
        SimpleBroker::publish(reply);
    }
    
    /// Publish vote update event
    pub fn publish_vote_updated(vote: VoteType) {
        SimpleBroker::publish(vote);
    }
    
    /// Publish community update event
    pub fn publish_community_updated(community: CommunityType) {
        SimpleBroker::publish(community);
    }
    
    /// Publish moderation action event
    pub fn publish_moderation_action(action: ModerationActionType) {
        SimpleBroker::publish(action);
    }
    
    // Governance System Events
    
    /// Publish new proposal event
    pub fn publish_proposal_created(proposal: ProposalType) {
        SimpleBroker::publish(proposal);
    }
    
    /// Publish proposal update event
    pub fn publish_proposal_updated(proposal: ProposalType) {
        SimpleBroker::publish(proposal);
    }
    
    /// Publish new governance vote event
    pub fn publish_governance_vote_cast(vote: GovernanceVoteType) {
        SimpleBroker::publish(vote);
    }
    
    /// Publish voting results update event
    pub fn publish_voting_results_updated(results: VotingResultType) {
        SimpleBroker::publish(results);
    }
    
    /// Publish governance participation update event
    pub fn publish_governance_participation_updated(participation: GovernanceParticipationType) {
        SimpleBroker::publish(participation);
    }
}

/// Convenience macros for publishing events
#[macro_export]
macro_rules! publish_user_event {
    (profile_updated, $profile:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_user_profile_updated($profile);
    };
    (score_updated, $score:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_cooperative_score_updated($score);
    };
    (relationship_updated, $relationship:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_relationship_updated($relationship);
    };
}

#[macro_export]
macro_rules! publish_social_event {
    (post_updated, $post:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_post_updated($post);
    };
    (comment_created, $comment:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_comment_created($comment);
    };
    (like_created, $like:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_like_created($like);
    };
    (notification, $notification:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_notification($notification);
    };
}

#[macro_export]
macro_rules! publish_forum_event {
    (thread_created, $thread:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_thread_created($thread);
    };
    (reply_created, $reply:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_thread_reply_created($reply);
    };
    (vote_updated, $vote:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_vote_updated($vote);
    };
}

#[macro_export]
macro_rules! publish_governance_event {
    (proposal_created, $proposal:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_proposal_created($proposal);
    };
    (proposal_updated, $proposal:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_proposal_updated($proposal);
    };
    (vote_cast, $vote:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_governance_vote_cast($vote);
    };
    (results_updated, $results:expr) => {
        $crate::graphql::subscription_events::SubscriptionEventPublisher::publish_voting_results_updated($results);
    };
}