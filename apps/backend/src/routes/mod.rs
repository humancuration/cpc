use axum::http::StatusCode;

pub mod update;
pub mod publish;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}