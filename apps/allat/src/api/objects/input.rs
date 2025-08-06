use async_graphql::{InputObject, ID};

#[derive(InputObject)]
pub struct CreateCommunityInput {
    pub name: String,
    pub description: String,
    pub rules: Vec<String>,
}

#[derive(InputObject)]
pub struct UpdateCommunityInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub rules: Option<Vec<String>>,
}

#[derive(InputObject)]
pub struct CreatePostInput {
    pub community_id: ID,
    pub title: String,
    pub content: String,
}

#[derive(InputObject)]
pub struct UpdatePostInput {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(InputObject)]
pub struct CreateCommentInput {
    pub post_id: ID,
    pub content: String,
    pub parent_id: Option<ID>,
}

#[derive(InputObject)]
pub struct UpdateCommentInput {
    pub content: String,
}

#[derive(InputObject)]
pub struct VotePostInput {
    pub post_id: ID,
    pub vote_type: String, // "UPVOTE" or "DOWNVOTE"
}

#[derive(InputObject, Debug)]
pub struct SearchCriteriaInput {
    pub query: String,
    pub community_id: Option<uuid::Uuid>,
    pub author_id: Option<uuid::Uuid>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}