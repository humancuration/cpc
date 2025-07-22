use async_graphql::{Schema, EmptyMutation, EmptySubscription};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use crate::graphql::schema::build_schema;
use crate::graphql::schema::{Query, Mutation};

pub async fn handler(
    schema: axum::extract::Extension<Schema<Query, Mutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}