use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::media_service::{MediaService, MediaServiceError};

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub media_id: Uuid,
    pub status: String,
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub media_id: Uuid,
    pub processing_status: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UploadQuery {
    pub post_id: Uuid,
    pub media_type: String,
}

pub async fn upload_media(
    State(media_service): State<Arc<MediaService>>,
    mut multipart: Multipart,
    query: axum::extract::Query<UploadQuery>,
) -> Result<Json<UploadResponse>, (StatusCode, String)> {
    let post_id = query.post_id;
    let media_type = match query.media_type.as_str() {
        "image" => cpc_core::models::social::post::MediaType::Image,
        "video" => cpc_core::models::social::post::MediaType::Video,
        "audio" => cpc_core::models::social::post::MediaType::Audio,
        _ => cpc_core::models::social::post::MediaType::Unknown,
    };

    // Process multipart form data
    let mut temp_file_path = None;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (StatusCode::BAD_REQUEST, format!("Failed to process multipart: {}", e))
    })? {
        let file_name = field.file_name().unwrap_or("upload").to_string();
        let content_type = field.content_type().unwrap_or("application/octet-stream");
        
        // Create temporary file
        let temp_path = std::env::temp_dir().join(file_name);
        let data = field.bytes().await.map_err(|e| {
            (StatusCode::BAD_REQUEST, format!("Failed to read file data: {}", e))
        })?;
        
        tokio::fs::write(&temp_path, &data).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to save file: {}", e))
        })?;
        
        temp_file_path = Some(temp_path);
        break;
    }

    let file_path = temp_file_path.ok_or_else(|| {
        (StatusCode::BAD_REQUEST, "No file uploaded".to_string())
    })?;

    // Upload media using service
    let media_item = media_service
        .upload_media(file_path, media_type, post_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Clean up temp file
    let _ = tokio::fs::remove_file(&file_path).await;

    Ok(Json(UploadResponse {
        media_id: media_item.id,
        status: "uploaded".to_string(),
        url: media_item.url,
    }))
}

pub async fn get_media_status(
    State(media_service): State<Arc<MediaService>>,
    Path(media_id): Path<Uuid>,
) -> Result<Json<StatusResponse>, (StatusCode, String)> {
    let media_items = media_service
        .get_media_for_post(media_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let media_item = media_items.into_iter().find(|item| item.id == media_id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Media not found".to_string()))?;

    Ok(Json(StatusResponse {
        media_id: media_item.id,
        processing_status: format!("{:?}", media_item.processing_status),
        url: Some(media_item.url),
    }))
}

pub fn create_media_routes(media_service: Arc<MediaService>) -> Router {
    Router::new()
        .route("/api/media/upload", post(upload_media))
        .route("/api/media/:id/status", get(get_media_status))
        .with_state(media_service)
}