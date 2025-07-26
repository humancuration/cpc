use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::AppState;
use crate::routes::discovery::*;

pub fn router() -> Router<Arc<RwLock<AppState>>> {
    Router::new()
        .route("/discovery/feed", get(get_discovery_feed))
        .route("/discovery/item", get(get_discovery_item))
        .route("/discovery/like", post(like_discovery_item))
        .route("/discovery/unlike", post(unlike_discovery_item))
        .route("/discovery/save", post(save_discovery_item))
        .route("/discovery/unsave", post(unsave_discovery_item))
        .route("/discovery/view", post(increment_view_count))
        .route("/discovery/share", post(share_discovery_item))
        .route("/discovery/create", post(create_discovery_item))
}