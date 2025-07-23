use crate::graphql::social::{SocialMutation, SocialQuery};
use crate::services::social_service::SocialService;
use async_graphql::{EmptySubscription, MergedObject, Schema};
use cpc_core::repositories::social::post_repository::PostRepository;
use std::sync::Arc;

#[derive(MergedObject, Default)]
pub struct Query(SocialQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(SocialMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(social_service: Arc<SocialService>) -> AppSchema {

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(Arc::new(social_service))
        .finish()
}