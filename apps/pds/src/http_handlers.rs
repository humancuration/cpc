use axum::{extract::State, response::IntoResponse, body::Bytes};
use cpc_core::project::ProjectData;
use rmp_serde::Deserializer;
use serde::Deserialize;
use anyhow::Result;
use std::sync::Arc;
use crate::file_utils::FileProcessor;
use crate::AppState;

pub async fn publish_handler(
    State(app_state): State<Arc<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    // Deserialize the MessagePack body
    let mut deserializer = Deserializer::new(&body[..]);
    let project_data: ProjectData = match Deserialize::deserialize(&mut deserializer) {
        Ok(data) => data,
        Err(e) => return (axum::http::StatusCode::BAD_REQUEST, format!("Invalid data: {}", e)).into_response(),
    };

    // Process the project data
    let file_processor = &app_state.file_processor;
    match file_processor.process_project(&project_data) {
        Ok(content_address) => (axum::http::StatusCode::OK, content_address).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, format!("Processing failed: {}", e)).into_response(),
    }
}