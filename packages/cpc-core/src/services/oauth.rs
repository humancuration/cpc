use crate::error::PublishError;
use anyhow::{anyhow, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// OAuth2 provider configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

/// User information from OAuth providers
#[derive(Debug, Deserialize, Serialize)]
pub struct OAuthUserInfo {
    pub email: String,
    pub social_id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/// Supported OAuth providers
#[derive(Debug, Clone, Copy)]
pub enum OAuthProvider {
    Google,
    TikTok,
    Instagram,
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthProvider::Google => write!(f, "google"),
            OAuthProvider::TikTok => write!(f, "tiktok"),
            OAuthProvider::Instagram => write!(f, "instagram"),
        }
    }
}

/// OAuth2 client for social login
pub struct OAuthClient {
    client: reqwest::Client,
    configs: HashMap<OAuthProvider, OAuthConfig>,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    id: String,
    email: String,
    name: Option<String>,
    picture: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TikTokUserInfo {
    data: TikTokUserData,
}

#[derive(Debug, Deserialize)]
struct TikTokUserData {
    open_id: String,
    email: Option<String>,
    display_name: String,
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct InstagramUserInfo {
    id: String,
    username: String,
    account_type: Option<String>,
    media_count: Option<i32>,
}

impl OAuthClient {
    /// Creates a new OAuth client with provider configurations
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        
        // Google OAuth config
        if let Ok(client_id) = std::env::var("GOOGLE_CLIENT_ID") {
            if let Ok(client_secret) = std::env::var("GOOGLE_CLIENT_SECRET") {
                if let Ok(redirect_uri) = std::env::var("GOOGLE_REDIRECT_URI") {
                    configs.insert(
                        OAuthProvider::Google,
                        OAuthConfig {
                            client_id,
                            client_secret,
                            redirect_uri,
                        },
                    );
                }
            }
        }
        
        // TikTok OAuth config
        if let Ok(client_id) = std::env::var("TIKTOK_CLIENT_ID") {
            if let Ok(client_secret) = std::env::var("TIKTOK_CLIENT_SECRET") {
                if let Ok(redirect_uri) = std::env::var("TIKTOK_REDIRECT_URI") {
                    configs.insert(
                        OAuthProvider::TikTok,
                        OAuthConfig {
                            client_id,
                            client_secret,
                            redirect_uri,
                        },
                    );
                }
            }
        }
        
        // Instagram OAuth config
        if let Ok(client_id) = std::env::var("INSTAGRAM_CLIENT_ID") {
            if let Ok(client_secret) = std::env::var("INSTAGRAM_CLIENT_SECRET") {
                if let Ok(redirect_uri) = std::env::var("INSTAGRAM_REDIRECT_URI") {
                    configs.insert(
                        OAuthProvider::Instagram,
                        OAuthConfig {
                            client_id,
                            client_secret,
                            redirect_uri,
                        },
                    );
                }
            }
        }
        
        Self {
            client: reqwest::Client::new(),
            configs,
        }
    }
    
    /// Validates an access token with the provider and returns user info
    pub async fn validate_token(
        &self,
        provider: OAuthProvider,
        access_token: &str,
    ) -> Result<OAuthUserInfo> {
        match provider {
            OAuthProvider::Google => self.validate_google_token(access_token).await,
            OAuthProvider::TikTok => self.validate_tiktok_token(access_token).await,
            OAuthProvider::Instagram => self.validate_instagram_token(access_token).await,
        }
    }
    
    async fn validate_google_token(&self, access_token: &str) -> Result<OAuthUserInfo> {
        let response = self
            .client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to validate Google token"));
        }
        
        let user_info: GoogleUserInfo = response.json().await?;
        
        Ok(OAuthUserInfo {
            email: user_info.email,
            social_id: user_info.id,
            username: user_info.email.split('@').next().unwrap_or("user").to_string(),
            display_name: user_info.name,
            avatar_url: user_info.picture,
        })
    }
    
    async fn validate_tiktok_token(&self, access_token: &str) -> Result<OAuthUserInfo> {
        let response = self
            .client
            .get("https://open-api.tiktok.com/user/info/")
            .bearer_auth(access_token)
            .query(&[("fields", "open_id,display_name,avatar_url")])
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to validate TikTok token"));
        }
        
        let user_info: TikTokUserInfo = response.json().await?;
        
        Ok(OAuthUserInfo {
            email: user_info.data.email.unwrap_or_else(|| format!("{}@tiktok.com", user_info.data.open_id)),
            social_id: user_info.data.open_id,
            username: user_info.data.display_name.clone(),
            display_name: Some(user_info.data.display_name),
            avatar_url: user_info.data.avatar_url,
        })
    }
    
    async fn validate_instagram_token(&self, access_token: &str) -> Result<OAuthUserInfo> {
        let response = self
            .client
            .get("https://graph.instagram.com/me")
            .query(&[("fields", "id,username,account_type,media_count")])
            .bearer_auth(access_token)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to validate Instagram token"));
        }
        
        let user_info: InstagramUserInfo = response.json().await?;
        
        Ok(OAuthUserInfo {
            email: format!("{}@instagram.com", user_info.username),
            social_id: user_info.id,
            username: user_info.username.clone(),
            display_name: Some(user_info.username),
            avatar_url: None, // Instagram requires additional API call for profile picture
        })
    }
    
    /// Checks if a provider is configured
    pub fn is_provider_configured(&self, provider: OAuthProvider) -> bool {
        self.configs.contains_key(&provider)
    }
}