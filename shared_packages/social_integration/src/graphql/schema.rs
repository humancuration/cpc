//! GraphQL schema for social integration features

use async_graphql::{
    Schema, EmptySubscription, Context, Result, ID,
};
use std::sync::Arc;
use uuid::Uuid;

use super::{
    queries::QueryRoot,
    mutations::MutationRoot,
};

/// GraphQL schema for social integration features
pub type SocialIntegrationSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Create a new GraphQL schema
pub fn create_schema() -> SocialIntegrationSchema {
    Schema::build(
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    )
    .finish()
}