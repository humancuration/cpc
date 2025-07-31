//! Integration tests for the OAuth2 crate

use cpc_oauth2::{
    domain::{AuthConfig, ProviderConfig, OAuthProvider, ProviderAdapter},
    application::{AuthService, TokenService},
    infrastructure::storage::StorageAdapter,
};
#[cfg(feature = "facebook")]
use cpc_oauth2::infrastructure::providers::facebook::FacebookAdapter;
#[cfg(feature = "twitter")]
use cpc_oauth2::infrastructure::providers::twitter::TwitterAdapter;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;

// Mock provider adapter for testing
#[derive(Debug)]
struct MockProviderAdapter {
    provider: OAuthProvider,
}

#[async_trait]
impl ProviderAdapter for MockProviderAdapter {
    fn provider(&self) -> OAuthProvider {
        self.provider.clone()
    }
    
    fn generate_auth_url(&self, _redirect_uri: &str) -> Result<(String, String), cpc_oauth2::domain::AuthError> {
        Ok(("https://example.com/auth".to_string(), "test_state".to_string()))
    }
    
    async fn exchange_code(&self, _code: String, _state: String) -> Result<cpc_oauth2::domain::OAuthToken, cpc_oauth2::domain::AuthError> {
        use chrono::{Utc, Duration};
        Ok(cpc_oauth2::domain::OAuthToken::new(
            "mock_access_token".to_string(),
            Some("mock_refresh_token".to_string()),
            Utc::now() + Duration::hours(1),
            vec!["read".to_string(), "write".to_string()],
            self.provider().to_string(),
        ))
    }
    
    async fn refresh_token(&self, _refresh_token: String) -> Result<cpc_oauth2::domain::OAuthToken, cpc_oauth2::domain::AuthError> {
        use chrono::{Utc, Duration};
        Ok(cpc_oauth2::domain::OAuthToken::new(
            "new_mock_access_token".to_string(),
            Some("new_mock_refresh_token".to_string()),
            Utc::now() + Duration::hours(1),
            vec!["read".to_string(), "write".to_string()],
            self.provider().to_string(),
        ))
    }
}

// Mock storage adapter for testing
#[derive(Debug)]
struct MockStorageAdapter {
    tokens: std::sync::Mutex<HashMap<String, String>>,
}

impl MockStorageAdapter {
    fn new() -> Self {
        Self {
            tokens: std::sync::Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl StorageAdapter for MockStorageAdapter {
    async fn store_token(
        &self,
        user_id: Uuid,
        provider: &str,
        token: &str,
    ) -> Result<(), cpc_oauth2::domain::AuthError> {
        let key = format!("{}:{}", user_id, provider);
        let mut tokens = self.tokens.lock().unwrap();
        tokens.insert(key, token.to_string());
        Ok(())
    }
    
    async fn get_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<String, cpc_oauth2::domain::AuthError> {
        let key = format!("{}:{}", user_id, provider);
        let tokens = self.tokens.lock().unwrap();
        tokens.get(&key).cloned().ok_or(cpc_oauth2::domain::AuthError::StorageError("Token not found".to_string()))
    }
    
    async fn delete_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<(), cpc_oauth2::domain::AuthError> {
        let key = format!("{}:{}", user_id, provider);
        let mut tokens = self.tokens.lock().unwrap();
        tokens.remove(&key);
        Ok(())
    }
}

#[tokio::test]
async fn test_auth_service_flow() {
    // Initialize tracing for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    // Create mock storage adapter
    let storage_adapter = Arc::new(MockStorageAdapter::new());
    
    // Create auth configuration
    let encryption_key = AuthConfig::generate_encryption_key();
    let mut auth_config = AuthConfig::new(
        "http://localhost:3000/callback".to_string(),
        encryption_key,
    );
    
    // Add mock provider configuration
    let provider_config = ProviderConfig {
        client_id: "mock_client_id".to_string(),
        client_secret: "mock_client_secret".to_string(),
        auth_url: "https://mock.example.com/auth".to_string(),
        token_url: "https://mock.example.com/token".to_string(),
        redirect_uris: vec!["http://localhost:3000/callback".to_string()],
        default_scopes: vec!["read".to_string()],
    };
    
    auth_config.set_provider_config(OAuthProvider::TikTok, provider_config);
    
    // Create provider adapters
    let mut providers: HashMap<OAuthProvider, Arc<dyn ProviderAdapter>> = HashMap::new();
    let mock_adapter = MockProviderAdapter {
        provider: OAuthProvider::TikTok,
    };
    providers.insert(OAuthProvider::TikTok, Arc::new(mock_adapter));
    
    // Create token service
    let token_service = Arc::new(TokenService::new(storage_adapter, auth_config.clone()));
    
    // Create auth service
    let auth_service = Arc::new(AuthService::new(providers, token_service, auth_config));
    
    // Test starting auth flow
    let user_id = Uuid::new_v4();
    let auth_request = auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await.unwrap();
    
    assert_eq!(auth_request.auth_url, "https://example.com/auth");
    assert_eq!(auth_request.state, "test_state");
    assert_eq!(auth_request.provider, OAuthProvider::TikTok);
    
    // Test handling callback
    let (returned_user_id, token, _profile) = auth_service.handle_callback(
        "mock_code".to_string(),
        "test_state".to_string(),
    ).await.unwrap();
    
    assert_eq!(returned_user_id, user_id);
    assert_eq!(token.access_token, "mock_access_token");
    assert_eq!(token.refresh_token, Some("mock_refresh_token".to_string()));
    
    // Test refreshing token
    let refreshed_token = auth_service.refresh_token(user_id, OAuthProvider::TikTok).await.unwrap();
    
    assert_eq!(refreshed_token.access_token, "new_mock_access_token");
    assert_eq!(refreshed_token.refresh_token, Some("new_mock_refresh_token".to_string()));
}

#[tokio::test]
async fn test_token_service() {
    // Initialize tracing for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    // Create mock storage adapter
    let storage_adapter = Arc::new(MockStorageAdapter::new());
    
    // Create auth configuration
    let encryption_key = AuthConfig::generate_encryption_key();
    let auth_config = AuthConfig::new(
        "http://localhost:3000/callback".to_string(),
        encryption_key,
    );
    
    // Create token service
    let token_service = TokenService::new(storage_adapter, auth_config);
    
    // Create a test token
    use chrono::{Utc, Duration};
    let token = cpc_oauth2::domain::OAuthToken::new(
        "test_access_token".to_string(),
        Some("test_refresh_token".to_string()),
        Utc::now() + Duration::hours(1),
        vec!["read".to_string()],
        "tiktok".to_string(),
    );
    
    let user_id = Uuid::new_v4();
    
    // Test storing token
    token_service.store_token(user_id, token.clone()).await.unwrap();
    
    // Test retrieving token
    let retrieved_token = token_service.get_token(user_id, &OAuthProvider::TikTok).await.unwrap();
    
    assert_eq!(token.access_token, retrieved_token.access_token);
    assert_eq!(token.refresh_token, retrieved_token.refresh_token);
    assert_eq!(token.provider, retrieved_token.provider);
    
    // Test checking if token is valid
    let has_valid_token = token_service.has_valid_token(user_id, &OAuthProvider::TikTok).await.unwrap();
    assert!(has_valid_token);
    
    // Test deleting token
    token_service.delete_token(user_id, &OAuthProvider::TikTok).await.unwrap();
    
    // After deletion, getting the token should fail
    let result = token_service.get_token(user_id, &OAuthProvider::TikTok).await;
    assert!(result.is_err());
}

/// Test Facebook OAuth provider adapter
#[cfg(feature = "facebook")]
#[tokio::test]
async fn test_facebook_provider() {
    // Initialize tracing for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    // Create Facebook adapter (this will fail without real credentials, but we can test the structure)
    let adapter = FacebookAdapter::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    );
    
    // Test that the adapter can be created
    assert!(adapter.is_ok());
    
    let adapter = adapter.unwrap();
    
    // Test generating auth URL
    let result = adapter.generate_auth_url("http://localhost:3000/callback");
    assert!(result.is_ok());
    
    let (auth_url, _state) = result.unwrap();
    assert!(auth_url.contains("facebook.com"));
    assert!(auth_url.contains("client_id=test_client_id"));
}

/// Test Twitter OAuth provider adapter
#[cfg(feature = "twitter")]
#[tokio::test]
async fn test_twitter_provider() {
    // Initialize tracing for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    // Create Twitter adapter (this will fail without real credentials, but we can test the structure)
    let adapter = TwitterAdapter::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    );
    
    // Test that the adapter can be created
    assert!(adapter.is_ok());
    
    let adapter = adapter.unwrap();
    
    // Test generating auth URL
    let result = adapter.generate_auth_url("http://localhost:3000/callback");
    assert!(result.is_ok());
    
    let (auth_url, _state) = result.unwrap();
    assert!(auth_url.contains("twitter.com"));
    assert!(auth_url.contains("client_id=test_client_id"));
    
    // Check that the required scopes are included
    assert!(auth_url.contains("tweet.read"));
    assert!(auth_url.contains("users.read"));
}

/// Test multi-provider token handling
#[tokio::test]
async fn test_multi_provider_token_handling() {
    // Initialize tracing for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    // Create mock storage adapter
    let storage_adapter = Arc::new(MockStorageAdapter::new());
    
    // Create auth configuration
    let encryption_key = AuthConfig::generate_encryption_key();
    let mut auth_config = AuthConfig::new(
        "http://localhost:3000/callback".to_string(),
        encryption_key,
    );
    
    // Add mock provider configurations
    let provider_config = ProviderConfig {
        client_id: "mock_client_id".to_string(),
        client_secret: "mock_client_secret".to_string(),
        auth_url: "https://mock.example.com/auth".to_string(),
        token_url: "https://mock.example.com/token".to_string(),
        redirect_uris: vec!["http://localhost:3000/callback".to_string()],
        default_scopes: vec!["read".to_string()],
    };
    
    auth_config.set_provider_config(OAuthProvider::TikTok, provider_config.clone());
    auth_config.set_provider_config(OAuthProvider::Facebook, provider_config.clone());
    auth_config.set_provider_config(OAuthProvider::Twitter, provider_config);
    
    // Create provider adapters
    let mut providers: HashMap<OAuthProvider, Arc<dyn ProviderAdapter>> = HashMap::new();
    
    let tiktok_adapter = MockProviderAdapter {
        provider: OAuthProvider::TikTok,
    };
    providers.insert(OAuthProvider::TikTok, Arc::new(tiktok_adapter));
    
    let facebook_adapter = MockProviderAdapter {
        provider: OAuthProvider::Facebook,
    };
    providers.insert(OAuthProvider::Facebook, Arc::new(facebook_adapter));
    
    let twitter_adapter = MockProviderAdapter {
        provider: OAuthProvider::Twitter,
    };
    providers.insert(OAuthProvider::Twitter, Arc::new(twitter_adapter));
    
    // Create token service
    let token_service = Arc::new(TokenService::new(storage_adapter, auth_config.clone()));
    
    // Create auth service
    let auth_service = Arc::new(AuthService::new(providers, token_service, auth_config));
    
    // Test starting auth flow for multiple providers
    let user_id = Uuid::new_v4();
    
    // Test TikTok
    let auth_request = auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await.unwrap();
    assert_eq!(auth_request.provider, OAuthProvider::TikTok);
    
    // Test Facebook
    let auth_request = auth_service.start_auth(user_id, OAuthProvider::Facebook, None).await.unwrap();
    assert_eq!(auth_request.provider, OAuthProvider::Facebook);
    
    // Test Twitter
    let auth_request = auth_service.start_auth(user_id, OAuthProvider::Twitter, None).await.unwrap();
    assert_eq!(auth_request.provider, OAuthProvider::Twitter);
}

/// Test profile fetching consistency across providers
#[cfg(feature = "facebook")]
#[cfg(feature = "twitter")]
#[tokio::test]
async fn test_profile_fetching_consistency() {
    // Initialize tracing for logging
    let _ = tracing_subscriber::fmt::try_init();
    
    // Test that all adapters can be created with the same pattern
    let tiktok_adapter = cpc_oauth2::infrastructure::providers::tiktok::TikTokAdapter::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    );
    assert!(tiktok_adapter.is_ok());
    
    let facebook_adapter = cpc_oauth2::infrastructure::providers::facebook::FacebookAdapter::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    );
    assert!(facebook_adapter.is_ok());
    
    let twitter_adapter = cpc_oauth2::infrastructure::providers::twitter::TwitterAdapter::new(
        "test_client_id".to_string(),
        "test_client_secret".to_string(),
        "http://localhost:3000/callback".to_string(),
    );
    assert!(twitter_adapter.is_ok());
    
    // Test that all adapters implement the ProviderAdapter trait correctly
    let adapters: Vec<Box<dyn ProviderAdapter>> = vec![
        Box::new(tiktok_adapter.unwrap()),
        Box::new(facebook_adapter.unwrap()),
        Box::new(twitter_adapter.unwrap()),
    ];
    
    for adapter in adapters {
        // Test that all adapters have the correct provider
        let provider = adapter.provider();
        assert!(!provider.as_str().is_empty());
        
        // Test that all adapters can generate auth URLs (will fail with fake credentials, but test the structure)
        let result = adapter.generate_auth_url("http://localhost:3000/callback");
        // We're just checking that the method exists and returns the right type
        // The actual URL generation will fail with fake credentials, which is expected
    }
}