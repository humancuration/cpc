use axum::{Router, http::StatusCode};

pub mod update;
pub mod publish;
pub mod upload;
pub mod impact;
pub mod social;
pub mod governance;
pub mod forum;
pub mod health;
pub mod auth;
pub mod discovery;
pub mod vendor;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn router() -> Router {
    Router::new()
        .merge(health::router())
        .merge(upload::router())
        .merge(publish::router())
        .merge(update::router())
        .merge(impact::router())
        .merge(social::router())
        .merge(forum::router())
        .merge(governance::router())
.merge(discovery::router())
        .merge(vendor::router())
}