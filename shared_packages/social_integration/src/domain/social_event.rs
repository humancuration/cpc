//! Social events for the social integration system

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Social events for tracking user interactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SocialEvent {
    /// A post was created
    PostCreated {
        user_id: Uuid,
        post_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A comment was created
    CommentCreated {
        user_id: Uuid,
        comment_id: Uuid,
        post_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A post was voted on
    PostVoted {
        user_id: Uuid,
        post_id: Uuid,
        vote_type: VoteType,
        timestamp: DateTime<Utc>,
    },
    
    /// A post was shared
    PostShared {
        user_id: Uuid,
        post_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A user followed another user
    UserFollowed {
        follower_id: Uuid,
        followed_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// An opportunity was shared
    OpportunityShared {
        user_id: Uuid,
        opportunity_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A user volunteered for an opportunity
    Volunteered {
        user_id: Uuid,
        opportunity_id: Uuid,
        hours_contributed: f32,
        timestamp: DateTime<Utc>,
    },
    
    /// A stream was started
    StreamStarted {
        user_id: Uuid,
        stream_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A stream was ended
    StreamEnded {
        user_id: Uuid,
        stream_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A viewer joined a stream
    ViewerJoined {
        user_id: Uuid,
        broadcaster_id: Uuid, // New field
        stream_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A chat message was sent in a stream
    ChatMessageSent {
        user_id: Uuid,
        stream_id: Uuid,
        message_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    
    /// A user subscribed to a channel
    SubscriptionCreated {
        user_id: Uuid,
        channel_id: Uuid,
        tier: SubscriptionTier,
        timestamp: DateTime<Utc>,
    },
}

/// Subscription tier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SubscriptionTier {
    Tier1,
    Tier2,
    Tier3,
}

/// Type of vote
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Upvote,
    Downvote,
}

impl SocialEvent {
    /// Get the user ID associated with the event
    pub fn user_id(&self) -> &Uuid {
        match self {
            SocialEvent::PostCreated { user_id, .. } => user_id,
            SocialEvent::CommentCreated { user_id, .. } => user_id,
            SocialEvent::PostVoted { user_id, .. } => user_id,
            SocialEvent::PostShared { user_id, .. } => user_id,
            SocialEvent::UserFollowed { follower_id, .. } => follower_id,
            SocialEvent::OpportunityShared { user_id, .. } => user_id,
            SocialEvent::Volunteered { user_id, .. } => user_id,
            SocialEvent::StreamStarted { user_id, .. } => user_id,
            SocialEvent::StreamEnded { user_id, .. } => user_id,
            SocialEvent::ViewerJoined { user_id, .. } => user_id,
            SocialEvent::ChatMessageSent { user_id, .. } => user_id,
            SocialEvent::SubscriptionCreated { user_id, .. } => user_id,
        }
    }
    
    /// Get the timestamp of the event
    pub fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            SocialEvent::PostCreated { timestamp, .. } => timestamp,
            SocialEvent::CommentCreated { timestamp, .. } => timestamp,
            SocialEvent::PostVoted { timestamp, .. } => timestamp,
            SocialEvent::PostShared { timestamp, .. } => timestamp,
            SocialEvent::UserFollowed { timestamp, .. } => timestamp,
            SocialEvent::OpportunityShared { timestamp, .. } => timestamp,
            SocialEvent::Volunteered { timestamp, .. } => timestamp,
            SocialEvent::StreamStarted { timestamp, .. } => timestamp,
            SocialEvent::StreamEnded { timestamp, .. } => timestamp,
            SocialEvent::ViewerJoined { timestamp, .. } => timestamp,
            SocialEvent::ChatMessageSent { timestamp, .. } => timestamp,
            SocialEvent::SubscriptionCreated { timestamp, .. } => timestamp,
        }
    }
}