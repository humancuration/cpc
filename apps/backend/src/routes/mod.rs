use axum::http::StatusCode;

pub mod update;
pub mod publish;
pub mod upload;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}