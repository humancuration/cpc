//! GraphQL queries for social integration features

use async_graphql::{Context, Object, Result, ID, connection::{Connection, EmptyFields, QueryResult}};
use uuid::Uuid;
use std::sync::Arc;

use super::{
    error::GraphQLError,
};

/// Root query object
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // All query implementations have been removed as tip functionality
    // has been moved to the wallet package
}