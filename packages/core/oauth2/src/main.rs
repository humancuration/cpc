//! Main entry point for the OAuth2 service

use cpc_oauth2::{
    domain::{AuthConfig, ProviderConfig, OAuthProvider},
    application::{AuthService, TokenService},
    infrastructure::{
        storage::sled_storage::SledStorageAdapter,
        providers::{tiktok::TikTokAdapter, facebook::FacebookAdapter},
    },
};

#[cfg(feature = "twitter")]
use cpc_oauth2::infrastructure::providers::twitter::TwitterAdapter;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use sled::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();
    
    println!("Starting OAuth2 service...");
    
    // Create a temporary sled database for testing
    let db = Config::new().temporary(true).open()?;
    
    // Create storage adapter
    let storage_adapter = Arc::new(SledStorageAdapter::new(db));
    
    // Create auth configuration
    let encryption_key = AuthConfig::generate_encryption_key();
    let mut auth_config = AuthConfig::new(
        "http://localhost:3000/callback".to_string(),
        encryption_key,
    );
    
    // Add TikTok provider configuration (you would normally load this from environment variables)
    let tiktok_config = ProviderConfig {
        client_id: "your_tiktok_client_id".to_string(),
        client_secret: "your_tiktok_client_secret".to_string(),
        auth_url: "https://open.tiktokapis.com/v2/oauth/authorize/".to_string(),
        token_url: "https://open.tiktokapis.com/v2/oauth/token/".to_string(),
        redirect_uris: vec!["http://localhost:3000/callback".to_string()],
        default_scopes: vec!["user.info.basic".to_string()],
    };
    
    auth_config.set_provider_config(OAuthProvider::TikTok, tiktok_config);
    
    // Add Facebook provider configuration (you would normally load this from environment variables)
    let facebook_config = ProviderConfig {
        client_id: "your_facebook_client_id".to_string(),
        client_secret: "your_facebook_client_secret".to_string(),
        auth_url: "https://www.facebook.com/v12.0/dialog/oauth".to_string(),
        token_url: "https://graph.facebook.com/v12.0/oauth/access_token".to_string(),
        redirect_uris: vec!["http://localhost:3000/callback".to_string()],
        default_scopes: vec!["public_profile".to_string(), "email".to_string()],
    };
    
    auth_config.set_provider_config(OAuthProvider::Facebook, facebook_config);
    
    // Add Twitter provider configuration (you would normally load this from environment variables)
    #[cfg(feature = "twitter")]
    {
        let twitter_config = ProviderConfig {
            client_id: "your_twitter_client_id".to_string(),
            client_secret: "your_twitter_client_secret".to_string(),
            auth_url: "https://twitter.com/i/oauth2/authorize".to_string(),
            token_url: "https://api.twitter.com/2/oauth2/token".to_string(),
            redirect_uris: vec!["http://localhost:3000/callback".to_string()],
            default_scopes: vec!["tweet.read".to_string(), "users.read".to_string()],
        };
        
        auth_config.set_provider_config(OAuthProvider::Twitter, twitter_config);
    }
    
    // Create provider adapters
    let mut providers: HashMap<OAuthProvider, Arc<dyn cpc_oauth2::domain::ProviderAdapter>> = HashMap::new();
    
    // Create TikTok adapter (this would fail with fake credentials, but shows the pattern)
    match TikTokAdapter::new(
        "your_tiktok_client_id".to_string(),
        "your_tiktok_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    ) {
        Ok(adapter) => {
            providers.insert(OAuthProvider::TikTok, Arc::new(adapter));
        }
        Err(e) => {
            println!("Warning: Failed to create TikTok adapter: {}", e);
        }
    }
    
    // Create Facebook adapter (this would fail with fake credentials, but shows the pattern)
    match FacebookAdapter::new(
        "your_facebook_client_id".to_string(),
        "your_facebook_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    ) {
        Ok(adapter) => {
            providers.insert(OAuthProvider::Facebook, Arc::new(adapter));
        }
        Err(e) => {
            println!("Warning: Failed to create Facebook adapter: {}", e);
        }
    }
    
    // Create Twitter adapter (this would fail with fake credentials, but shows the pattern)
    #[cfg(feature = "twitter")]
    match TwitterAdapter::new(
        "your_twitter_client_id".to_string(),
        "your_twitter_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    ) {
        Ok(adapter) => {
            providers.insert(OAuthProvider::Twitter, Arc::new(adapter));
        }
        Err(e) => {
            println!("Warning: Failed to create Twitter adapter: {}", e);
        }
    }
    
    // Create token service
    let token_service = Arc::new(TokenService::new(storage_adapter, auth_config.clone()));
    
    // Create auth service
    let auth_service = Arc::new(AuthService::new(providers, token_service, auth_config));
    
    println!("OAuth2 service initialized successfully!");
    println!("You can now use the OAuth2 service in your applications.");
    
    // Example: Start an authentication flow with TikTok
    let user_id = Uuid::new_v4();
    println!("\nExample: Starting authentication flow for user {}", user_id);
    
    match auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await {
        Ok(auth_request) => {
            println!("TikTok Auth URL: {}", auth_request.auth_url);
            println!("State: {}", auth_request.state);
        }
        Err(e) => {
            println!("Failed to start TikTok authentication: {}", e);
        }
    }
    
    // Example: Start an authentication flow with Facebook
    match auth_service.start_auth(user_id, OAuthProvider::Facebook, None).await {
        Ok(auth_request) => {
            println!("Facebook Auth URL: {}", auth_request.auth_url);
            println!("State: {}", auth_request.state);
        }
        Err(e) => {
            println!("Failed to start Facebook authentication: {}", e);
        }
    }
    
    // Example: Start an authentication flow with Twitter
    #[cfg(feature = "twitter")]
    match auth_service.start_auth(user_id, OAuthProvider::Twitter, None).await {
        Ok(auth_request) => {
            println!("Twitter Auth URL: {}", auth_request.auth_url);
            println!("State: {}", auth_request.state);
        }
        Err(e) => {
            println!("Failed to start Twitter authentication: {}", e);
        }
    }
    
    Ok(())
}