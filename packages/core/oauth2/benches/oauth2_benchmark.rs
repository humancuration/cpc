//! Benchmark tests for the OAuth2 crate

use cpc_oauth2::{
    domain::{AuthConfig, ProviderConfig, OAuthProvider, ProviderAdapter},
    application::{AuthService, TokenService},
    infrastructure::storage::StorageAdapter,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use criterion::{criterion_group, criterion_main, Criterion, async_executor::AsyncExecutor};

// Mock provider adapter for benchmarking
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

// Mock storage adapter for benchmarking
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

// Async executor for criterion
struct TokioExecutor;

impl AsyncExecutor for TokioExecutor {
    fn block_on<T>(&self, future: impl std::future::Future<Output = T>) -> T {
        tokio::runtime::Runtime::new().unwrap().block_on(future)
    }
}

fn bench_auth_service(c: &mut Criterion) {
    let mut group = c.benchmark_group("auth_service");
    group.sample_size(10);
    
    group.bench_function("start_auth", |b| {
        b.to_async(TokioExecutor).iter(|| async {
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
            
            // Benchmark starting auth flow
            let user_id = Uuid::new_v4();
            let _auth_request = auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await.unwrap();
        })
    });
    
    group.bench_function("handle_callback", |b| {
        b.to_async(TokioExecutor).iter(|| async {
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
            
            // First start auth to get state
            let user_id = Uuid::new_v4();
            let auth_request = auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await.unwrap();
            
            // Benchmark handling callback
            let _result = auth_service.handle_callback(
                "mock_code".to_string(),
                auth_request.state,
            ).await.unwrap();
        })
    });
    
    group.finish();
}

fn bench_token_service(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_service");
    group.sample_size(10);
    
    group.bench_function("store_and_retrieve_token", |b| {
        b.to_async(TokioExecutor).iter(|| async {
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
            
            // Benchmark storing and retrieving token
            token_service.store_token(user_id, token.clone()).await.unwrap();
            let _retrieved_token = token_service.get_token(user_id, &OAuthProvider::TikTok).await.unwrap();
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_auth_service, bench_token_service);
criterion_main!(benches);