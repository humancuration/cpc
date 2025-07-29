// DEPRECATED: This file has been replaced by the Tauri backend command in src-tauri/src/scene_commands.rs
// Please use the new command `build_project_data` which is now managed by Tauri state.
use anyhow::{Context, Result};
use chrono::Utc;
use cpc_core::project::{ProjectData, ProjectMetadata};
use cpc_core::scene::SceneData;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rmp_serde::encode::to_vec;
use uuid::Uuid;

/// Builds project data from scene data and publishes to PDS
pub async fn build_project_data(scene_data: SceneData) -> Result<()> {
    // Create project metadata
    let metadata = ProjectMetadata {
        project_id: Uuid::new_v4(),
        author_id: Uuid::nil(), // TODO: Replace with actual user ID
        title: "Untitled Project".to_string(), // TODO: Get from UI
        version: 1,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    // Construct project data
    let project_data = ProjectData { metadata, scene: scene_data };

    // Serialize to MessagePack
    let msgpack = to_vec(&project_data)
        .context("Failed to serialize project data to MessagePack")?;

    // Get JWT (stub implementation)
    let jwt_token = get_jwt_token().await?;

    // Send to PDS endpoint
    let client = Client::new();
    let response = client
        .post("http://localhost:3030/publish")
        .header(AUTHORIZATION, format!("Bearer {}", jwt_token))
        .header(CONTENT_TYPE, "application/msgpack")
        .body(msgpack)
        .send()
        .await
        .context("Failed to send project data to PDS")?;

    // Handle error responses
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("PDS returned error: {} - {}", status, body);
    }

    Ok(())
}

/// Stub function to get JWT token (to be replaced with actual implementation)
async fn get_jwt_token() -> Result<String> {
    // TODO: Implement actual JWT retrieval from secure storage
    Ok("stub_jwt_token".to_string())
}