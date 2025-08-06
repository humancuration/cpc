use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationEvent {
    PostReply {
        post_id: Uuid,
        post_title: String,
        replier_id: Uuid,
        replier_name: String,
        community_id: Uuid,
        community_name: String,
    },
    CommentReply {
        comment_id: Uuid,
        parent_comment_id: Uuid,
        replier_id: Uuid,
        replier_name: String,
        post_id: Uuid,
        post_title: String,
    },
    PostUpvoted {
        post_id: Uuid,
        post_title: String,
        voter_id: Uuid,
        voter_name: String,
        upvote_count: u32,
    },
    CommentUpvoted {
        comment_id: Uuid,
        voter_id: Uuid,
        voter_name: String,
        upvote_count: u32,
    },
    NewPostInCommunity {
        post_id: Uuid,
        post_title: String,
        author_id: Uuid,
        author_name: String,
        community_id: Uuid,
        community_name: String,
    },
    ContentReported {
        content_id: Uuid,
        content_type: ContentType,
        reporter_id: Uuid,
        reporter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Post,
    Comment,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Post => "post".to_string(),
            ContentType::Comment => "comment".to_string(),
        }
    }
}