use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Utc};
use crate::domain::post::Post;
use super::{community::CommunityObject, user::UserObject, comment::CommentObject, media_asset::MediaAssetObject};

#[derive(SimpleObject)]
pub struct PostObject {
    pub id: ID,
    pub community: CommunityObject,
    pub user: UserObject,
    pub title: String,
    pub content: String,
    pub media_assets: Vec<MediaAssetObject>,
    pub comments: Vec<CommentObject>,
    pub votes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Post> for PostObject {
    fn from(post: Post) -> Self {
        // Calculate vote count
        let votes = post.votes.iter().fold(0, |acc, vote| {
            match vote.vote_type {
                crate::domain::vote::VoteType::Upvote => acc + 1,
                crate::domain::vote::VoteType::Downvote => acc - 1,
            }
        });
        
        // For now, we'll create placeholder objects
        // In a real implementation, we would fetch these from services
        let community = CommunityObject {
            id: ID::from(post.community_id.to_string()),
            name: "placeholder".to_string(),
            description: "placeholder".to_string(),
            rules: vec![],
            created_at: Utc::now(),
        };
        
        let user = UserObject {
            id: ID::from(post.user_id.to_string()),
            username: "placeholder".to_string(),
            karma: 0,
        };
        
        let comments = vec![]; // In a real implementation, we would fetch comments
        
        let media_assets = post.media_assets.into_iter().map(MediaAssetObject::from).collect();
        
        Self {
            id: ID::from(post.id.to_string()),
            community,
            user,
            title: post.title,
            content: post.content,
            media_assets,
            comments,
            votes,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}