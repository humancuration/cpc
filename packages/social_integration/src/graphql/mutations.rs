//! GraphQL mutations for social integration features

use async_graphql::{Context, Object, Result, ID};
use uuid::Uuid;
use std::sync::Arc;

use super::{
    error::GraphQLError,
};

/// Root mutation object
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    // All mutation implementations have been removed as tip functionality
    // has been moved to the wallet package
}