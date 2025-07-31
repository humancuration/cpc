//! Basic usage example for the OAuth2 crate

use cpc_oauth2::{
    domain::{AuthConfig, ProviderConfig, OAuthProvider},
    application::{AuthService, TokenService},
    infrastructure::{
        storage::sled_storage::SledStorageAdapter,
        providers::tiktok::TikTokAdapter,
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
    
    println!("OAuth2 Basic Usage Example");
    println!("==========================");
    
    // Create a temporary sled database for this example
    let db = Config::new().temporary(true).open()?;
    
    // Create storage adapter
    let storage_adapter = Arc::new(SledStorageAdapter::new(db));
    
    // Create auth configuration
    let encryption_key = AuthConfig::generate_encryption_key();
    let mut auth_config = AuthConfig::new(
        "http://localhost:3000/callback".to_string(),
        encryption_key,
    );
    
    // Add TikTok provider configuration
    // Note: In a real application, you would load these from environment variables
    let tiktok_config = ProviderConfig {
        client_id: "your_tiktok_client_id_here".to_string(),
        client_secret: "your_tiktok_client_secret_here".to_string(),
        auth_url: "https://open.tiktokapis.com/v2/oauth/authorize/".to_string(),
        token_url: "https://open.tiktokapis.com/v2/oauth/token/".to_string(),
        redirect_uris: vec!["http://localhost:3000/callback".to_string()],
        default_scopes: vec!["user.info.basic".to_string()],
    };
    
    auth_config.set_provider_config(OAuthProvider::TikTok, tiktok_config.clone());
    
    // Add Twitter provider configuration
    #[cfg(feature = "twitter")]
    {
        let twitter_config = ProviderConfig {
            client_id: "your_twitter_client_id_here".to_string(),
            client_secret: "your_twitter_client_secret_here".to_string(),
            auth_url: "https://twitter.com/i/oauth2/authorize".to_string(),
            token_url: "https://api.twitter.com/2/oauth2/token".to_string(),
            redirect_uris: vec!["http://localhost:3000/callback".to_string()],
            default_scopes: vec!["tweet.read".to_string(), "users.read".to_string()],
        };
        
        auth_config.set_provider_config(OAuthProvider::Twitter, twitter_config);
    }
    
    // Create provider adapters
    let mut providers: HashMap<OAuthProvider, Arc<dyn cpc_oauth2::domain::ProviderAdapter>> = HashMap::new();
    
    // Create TikTok adapter (this will fail with fake credentials, but shows the pattern)
    match TikTokAdapter::new(
        "your_tiktok_client_id_here".to_string(),
        "your_tiktok_client_secret_here".to_string(),
        "http://localhost:3000/callback".to_string(),
    ) {
        Ok(adapter) => {
            providers.insert(OAuthProvider::TikTok, Arc::new(adapter));
            println!("✓ TikTok adapter created successfully");
        }
        Err(e) => {
            println!("⚠ Warning: Failed to create TikTok adapter: {}", e);
            println!("  (This is expected with fake credentials in this example)");
        }
    }
    
    // Create Twitter adapter (this will fail with fake credentials, but shows the pattern)
    #[cfg(feature = "twitter")]
    match TwitterAdapter::new(
        "your_twitter_client_id_here".to_string(),
        "your_twitter_client_secret_here".to_string(),
        "http://localhost:3000/callback".to_string(),
    ) {
        Ok(adapter) => {
            providers.insert(OAuthProvider::Twitter, Arc::new(adapter));
            println!("✓ Twitter adapter created successfully");
        }
        Err(e) => {
            println!("⚠ Warning: Failed to create Twitter adapter: {}", e);
            println!("  (This is expected with fake credentials in this example)");
        }
    }
    
    // Create token service
    let token_service = Arc::new(TokenService::new(storage_adapter, auth_config.clone()));
    println!("✓ Token service created successfully");
    
    // Create auth service
    let auth_service = Arc::new(AuthService::new(providers, token_service, auth_config));
    println!("✓ Auth service created successfully");
    
    // Example: Start an authentication flow
    let user_id = Uuid::new_v4();
    println!("\nStarting authentication flow for user: {}", user_id);
    
    match auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await {
        Ok(auth_request) => {
            println!("✓ Auth flow started successfully");
            println!("  Auth URL: {}", auth_request.auth_url);
            println!("  State: {}", auth_request.state);
            println!("  Provider: {:?}", auth_request.provider);
        }
        Err(e) => {
            println!("⚠ Failed to start authentication: {}", e);
            println!("  (This is expected with fake credentials in this example)");
        }
    }
    
    // Example: Start Twitter authentication flow
    #[cfg(feature = "twitter")]
    {
        println!("\nStarting Twitter authentication flow for user: {}", user_id);
        
        match auth_service.start_auth(user_id, OAuthProvider::Twitter, None).await {
            Ok(auth_request) => {
                println!("✓ Twitter Auth flow started successfully");
                println!("  Auth URL: {}", auth_request.auth_url);
                println!("  State: {}", auth_request.state);
                println!("  Provider: {:?}", auth_request.provider);
            }
            Err(e) => {
                println!("⚠ Failed to start Twitter authentication: {}", e);
                println!("  (This is expected with fake credentials in this example)");
            }
        }
    }
    
    println!("\nExample completed!");
    println!("In a real application, you would:");
    println!("1. Redirect the user to the auth URL");
    println!("2. Handle the callback at your redirect URI");
    println!("3. Exchange the code for tokens");
    println!("4. Store and use the tokens for API calls");
    
    Ok(())
}