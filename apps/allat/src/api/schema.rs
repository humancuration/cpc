use async_graphql::{Schema, EmptySubscription, Context};
use crate::application::{
    community_service::CommunityService,
    post_service::PostService,
    comment_service::CommentService,
    vote_service::VoteService,
};
use std::sync::Arc;

// Object types
pub mod objects;
// Query root
pub mod queries;
// Mutation root
pub mod mutations;
// Subscription root
pub mod subscriptions;

pub type AllatSchema = Schema<queries::QueryRoot, mutations::MutationRoot, subscriptions::SubscriptionRoot>;

pub fn create_schema(
    community_service: Arc<dyn CommunityService>,
    post_service: Arc<dyn PostService>,
    comment_service: Arc<dyn CommentService>,
    vote_service: Arc<dyn VoteService>,
) -> AllatSchema {
    Schema::build(
        queries::QueryRoot,
        mutations::MutationRoot,
        subscriptions::SubscriptionRoot,
    )
    .data(community_service)
    .data(post_service)
    .data(comment_service)
    .data(vote_service)
    .finish()
}