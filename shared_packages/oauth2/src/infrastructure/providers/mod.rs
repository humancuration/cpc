//! Provider adapters for OAuth providers

use crate::domain::OAuthProvider;

#[cfg(feature = "tiktok")]
pub mod tiktok;

#[cfg(feature = "google")]
pub mod google;

#[cfg(feature = "facebook")]
pub mod facebook;

#[cfg(feature = "twitter")]
pub mod twitter;

/// Get the default scopes for a provider
pub fn get_default_scopes(provider: &OAuthProvider) -> Vec<String> {
    match provider {
        OAuthProvider::TikTok => vec!["user.info.basic".to_string()],
        OAuthProvider::Google => vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        OAuthProvider::Facebook => vec!["public_profile".to_string(), "email".to_string()],
        OAuthProvider::Twitter => vec!["tweet.read".to_string(), "users.read".to_string()],
        OAuthProvider::YouTube => vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        OAuthProvider::WhatsApp => vec!["whatsapp_business_messaging".to_string()],
        OAuthProvider::Instagram => vec!["user_profile".to_string(), "user_media".to_string()],
        OAuthProvider::Threads => vec!["threads_basic".to_string()],
        OAuthProvider::WeChat => vec!["snsapi_userinfo".to_string()],
        OAuthProvider::Messenger => vec!["public_profile".to_string(), "email".to_string()],
        OAuthProvider::Snapchat => vec!["snapchat-marketing-api".to_string()],
        OAuthProvider::Discord => vec!["identify".to_string(), "email".to_string()],
        OAuthProvider::Twitch => vec!["user:read:email".to_string()],
        OAuthProvider::Gmail => vec!["https://www.googleapis.com/auth/gmail.readonly".to_string()],
    }
}