use axum::{Router, routing::post};
use async_graphql::http::GraphQLRequest;
use axum::response::IntoResponse;
use axum::extract::State;
use crate::graphql::Schema;

use super::vendor_mutation::VendorMutation;

pub fn router() -> Router<Schema> {
    Router::new()
        .route("/create", post(create_vendor))
        .route("/update-verification", post(update_verification))
}

async fn create_vendor(
    State(schema): State<Schema>,
    req: GraphQLRequest,
) -> impl IntoResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn update_verification(
    State(schema): State<Schema>,
    req: GraphQLRequest,
) -> impl IntoResponse {
    schema.execute(req.into_inner()).await.into()
}