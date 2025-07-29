use anyhow::{Context, Result};
use chrono::Utc;
use cpc_core::auth::Claims;
use cpc_core::error::PublishError;
use cpc_core::project::{ProjectData, ProjectMetadata};
use cpc_core::scene::SceneData;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use rmp_serde::encode::to_vec;
use tauri::State;
use uuid::Uuid;

use crate::scene_manager::SceneManager;

const SECRET: &str = "my-secret-key";

/// Builds project data from current scene and publishes to PDS
pub async fn build_project_data(scene_manager: State<SceneManager>) -> Result<()> {
    // Get current scene from SceneManager
    let scene_data = scene_manager.current_scene()
        .ok_or_else(|| anyhow::anyhow!("No scene data available"))?;

    // Get and validate JWT token
    let token = get_jwt_token().await?;
    let claims = validate_jwt(&token)?;
    let user_id = claims.user_id;

    // Create project metadata
    let metadata = ProjectMetadata {
        project_id: Uuid::new_v4(),
        author_id: user_id,
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

    // Send to PDS endpoint
    let client = Client::new();
    let response = client
        .post("http://localhost:3030/publish")
        .header(AUTHORIZATION, format!("Bearer {}", token))
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

/// Validates JWT token and returns claims
fn validate_jwt(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => PublishError::TokenExpired.into(),
        _ => PublishError::JwtValidation(e.to_string()).into(),
    })?;

    Ok(token_data.claims)
}

/// Stub function to get JWT token (to be replaced with actual implementation)
async fn get_jwt_token() -> Result<String> {
    // TODO: Implement actual JWT retrieval from secure storage
    // For now, generate a valid token with nil user ID
    let claims = Claims {
        user_id: Uuid::nil(),
        exp: (Utc::now() + chrono::Duration::days(30)).timestamp() as usize,
    };
    
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(SECRET.as_bytes()),
    )?;
    
    Ok(token)
}