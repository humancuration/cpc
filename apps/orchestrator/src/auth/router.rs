use axum::{
    routing::post,
    Router,
};
use async_graphql_axum::GraphQL;
use super::schema::AuthMutation;
use crate::auth::service::AuthService;
use async_graphql::{EmptySubscription, Schema};

pub fn create_auth_router(auth_service: AuthService) -> Router {
    let schema = Schema::build(
        AuthMutation,
        async_graphql::EmptyMutation,
        EmptySubscription
    )
    .data(auth_service)
    .finish();

    Router::new()
        .route("/auth", post(GraphQL::new(schema)))
}