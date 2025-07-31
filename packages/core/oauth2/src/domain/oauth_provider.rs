//! OAuth provider definitions and traits

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};
use crate::domain::{OAuthToken, AuthError};

/// Supported OAuth providers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OAuthProvider {
    TikTok,
    Google,
    Facebook,
    Twitter,
    YouTube,
    WhatsApp,
    Instagram,
    Threads,
    WeChat,
    Messenger,
    Snapchat,
    Discord,
    Twitch,
    Gmail,
}

impl OAuthProvider {
    /// Get the string representation of the provider
    pub fn as_str(&self) -> &'static str {
        match self {
            OAuthProvider::TikTok => "tiktok",
            OAuthProvider::Google => "google",
            OAuthProvider::Facebook => "facebook",
            OAuthProvider::Twitter => "twitter",
            OAuthProvider::YouTube => "youtube",
            OAuthProvider::WhatsApp => "whatsapp",
            OAuthProvider::Instagram => "instagram",
            OAuthProvider::Threads => "threads",
            OAuthProvider::WeChat => "wechat",
            OAuthProvider::Messenger => "messenger",
            OAuthProvider::Snapchat => "snapchat",
            OAuthProvider::Discord => "discord",
            OAuthProvider::Twitch => "twitch",
            OAuthProvider::Gmail => "gmail",
        }
    }
    
    /// Create a provider from its string representation
    pub fn from_str(s: &str) -> Result<Self, AuthError> {
        match s.to_lowercase().as_str() {
            "tiktok" => Ok(OAuthProvider::TikTok),
            "google" => Ok(OAuthProvider::Google),
            "facebook" => Ok(OAuthProvider::Facebook),
            "twitter" => Ok(OAuthProvider::Twitter),
            "youtube" => Ok(OAuthProvider::YouTube),
            "whatsapp" => Ok(OAuthProvider::WhatsApp),
            "instagram" => Ok(OAuthProvider::Instagram),
            "threads" => Ok(OAuthProvider::Threads),
            "wechat" => Ok(OAuthProvider::WeChat),
            "messenger" => Ok(OAuthProvider::Messenger),
            "snapchat" => Ok(OAuthProvider::Snapchat),
            "discord" => Ok(OAuthProvider::Discord),
            "twitch" => Ok(OAuthProvider::Twitch),
            "gmail" => Ok(OAuthProvider::Gmail),
            _ => Err(AuthError::UnsupportedProvider(s.to_string())),
        }
    }
}

impl Display for OAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trait that all OAuth provider adapters must implement
#[async_trait::async_trait]
pub trait ProviderAdapter: Send + Sync {
    /// Get the provider type
    fn provider(&self) -> OAuthProvider;
    
    /// Generate an authorization URL for the provider
    fn generate_auth_url(&self, redirect_uri: &str) -> Result<(String, String), AuthError>; // (auth_url, state)
    
    /// Exchange an authorization code for an access token
    async fn exchange_code(&self, code: String, state: String) -> Result<OAuthToken, AuthError>;
    
    /// Refresh an access token
    async fn refresh_token(&self, refresh_token: String) -> Result<OAuthToken, AuthError>;
    
    /// Fetch user profile information
    async fn fetch_profile(&self, token: &OAuthToken) -> Result<OAuthProfile, AuthError>;
}