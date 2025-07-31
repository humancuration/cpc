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
        }
    }
}