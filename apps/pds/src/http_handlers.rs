use axum::{extract::State, response::{IntoResponse, Json}, body::Bytes, http::StatusCode};
use axum_extra::TypedHeader;
use headers::Authorization;
use headers::authorization::Bearer;
use cpc_core::project::ProjectData;
use cpc_core::error::PublishError;
use rmp_serde::Deserializer;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use tracing;
use uuid::Uuid;
use crate::file_utils::FileProcessor;
use crate::AppState;
use crate::auth::authenticate;

pub async fn publish_handler(
    State(state): State<Arc<AppState>>,
    bearer: TypedHeader<Authorization<Bearer>>,
    bytes: Bytes
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = authenticate(&state, bearer).map_err(|e| {
        tracing::error!("Authentication failed: {:?}", e);
        (StatusCode::UNAUTHORIZED, Json(json!({ "error": e.to_string() })))
    })?;

    // Deserialize the MessagePack body
    let mut deserializer = Deserializer::new(&bytes);
    let mut project_data: ProjectData = Deserialize::deserialize(&mut deserializer)
        .map_err(|e| {
            tracing::error!("Deserialization failed: {:?}", e);
            (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid request body" })))
        })?;

    // Set the authenticated user as author
    project_data.metadata.author_id = user_id;

    // Add tracing to processing steps
    tracing::info!("Processing project: {}", project_data.metadata.project_id);

    // Process the project data
    let file_processor = &state.file_processor;
    let content_address = file_processor.process_project(&project_data)
        .map_err(|e| {
            tracing::error!("PDS processing error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": PublishError::PdsProcessing(e.to_string()).to_string() })))
        })?;

    tracing::info!("Successfully processed project: {}", project_data.metadata.project_id);

    Ok(Json(json!({ "content_address": content_address })))
}