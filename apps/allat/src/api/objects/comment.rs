use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Utc};
use crate::domain::comment::Comment;
use super::{user::UserObject, media_asset::MediaAssetObject};

#[derive(SimpleObject)]
pub struct CommentObject {
    pub id: ID,
    pub post_id: ID,
    pub user: UserObject,
    pub content: String,
    pub votes: i32,
    pub created_at: DateTime<Utc>,
    pub parent_id: Option<ID>,
}

impl From<Comment> for CommentObject {
    fn from(comment: Comment) -> Self {
        // Calculate vote count
        let votes = comment.votes.iter().fold(0, |acc, vote| {
            match vote.vote_type {
                crate::domain::vote::VoteType::Upvote => acc + 1,
                crate::domain::vote::VoteType::Downvote => acc - 1,
            }
        });
        
        // For now, we'll create a placeholder user object
        // In a real implementation, we would fetch the user from a service
        let user = UserObject {
            id: ID::from(comment.user_id.to_string()),
            username: "placeholder".to_string(),
            karma: 0,
        };
        
        Self {
            id: ID::from(comment.id.to_string()),
            post_id: ID::from(comment.post_id.to_string()),
            user,
            content: comment.content,
            votes,
            created_at: comment.created_at,
            parent_id: comment.parent_id.map(|id| ID::from(id.to_string())),
        }
    }
}