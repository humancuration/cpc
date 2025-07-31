//! Authentication service for orchestrating OAuth flows

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::domain::{
    OAuthProvider, ProviderAdapter, OAuthToken, OAuthProfile, AuthError, AuthConfig
};
use crate::application::TokenService;
use tracing::{info, error, debug};

/// Authentication request information
#[derive(Debug, Clone)]
pub struct AuthRequest {
    /// Authorization URL to redirect the user to
    pub auth_url: String,
    
    /// State parameter for CSRF protection
    pub state: String,
    
    /// Provider for this authentication request
    pub provider: OAuthProvider,
}

/// Authentication service
pub struct AuthService {
    /// Provider adapters for each supported provider
    providers: HashMap<OAuthProvider, Arc<dyn ProviderAdapter>>,
    
    /// Token service for managing tokens
    token_service: Arc<TokenService>,
    
    /// Authentication configuration
    config: AuthConfig,
    
    /// In-memory state storage for CSRF protection
    /// In a production environment, this should be stored in a distributed cache
    states: Arc<RwLock<HashMap<String, (Uuid, OAuthProvider)>>>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(
        providers: HashMap<OAuthProvider, Arc<dyn ProviderAdapter>>,
        token_service: Arc<TokenService>,
        config: AuthConfig,
    ) -> Self {
        Self {
            providers,
            token_service,
            config,
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Start an authentication flow with a provider
    pub async fn start_auth(
        &self,
        user_id: Uuid,
        provider: OAuthProvider,
        redirect_uri: Option<String>,
    ) -> Result<AuthRequest, AuthError> {
        info!(user_id = %user_id, provider = %provider, "Starting OAuth authentication flow");
        
        let adapter = self.providers.get(&provider)
            .ok_or_else(|| AuthError::UnsupportedProvider(provider.to_string()))?;
        
        let redirect_uri = redirect_uri.unwrap_or_else(|| self.config.default_redirect_uri.clone());
        
        let (auth_url, state) = adapter.generate_auth_url(&redirect_uri)?;
        
        // Store state for CSRF protection
        {
            let mut states = self.states.write().await;
            states.insert(state.clone(), (user_id, provider.clone()));
        }
        
        debug!(state = %state, "Stored authentication state");
        
        Ok(AuthRequest {
            auth_url,
            state,
            provider,
        })
    }
    
    /// Handle the callback from an OAuth provider
    pub async fn handle_callback(
        &self,
        code: String,
        state: String,
    ) -> Result<(Uuid, OAuthToken, OAuthProfile), AuthError> {
        debug!(state = %state, "Handling OAuth callback");
        
        // Verify state to prevent CSRF attacks
        let (user_id, provider) = {
            let mut states = self.states.write().await;
            states.remove(&state).ok_or(AuthError::InvalidState)?
        };
        
        info!(user_id = %user_id, provider = %provider, "Valid OAuth callback received");
        
        let adapter = self.providers.get(&provider)
            .ok_or_else(|| AuthError::UnsupportedProvider(provider.to_string()))?;
        
        // Exchange code for token
        let token = adapter.exchange_code(code, state).await?;
        
        // Get user profile
        let profile = adapter.fetch_profile(&token).await?;
        
        // Store the token
        self.token_service.store_token(user_id, token.clone()).await?;
        
        info!(user_id = %user_id, provider = %provider, "OAuth authentication completed successfully");
        
        Ok((user_id, token, profile))
    }
    
    /// Refresh an access token
    pub async fn refresh_token(
        &self,
        user_id: Uuid,
        provider: OAuthProvider,
    ) -> Result<OAuthToken, AuthError> {
        info!(user_id = %user_id, provider = %provider, "Refreshing OAuth token");
        
        let adapter = self.providers.get(&provider)
            .ok_or_else(|| AuthError::UnsupportedProvider(provider.to_string()))?;
        
        // Get the current token
        let current_token = self.token_service.get_token(user_id, &provider).await?;
        
        let refresh_token = current_token.refresh_token
            .ok_or_else(|| AuthError::TokenRefreshFailed("No refresh token available".to_string()))?;
        
        // Refresh the token
        let new_token = adapter.refresh_token(refresh_token).await?;
        
        // Store the new token
        self.token_service.store_token(user_id, new_token.clone()).await?;
        
        info!(user_id = %user_id, provider = %provider, "OAuth token refreshed successfully");
        
        Ok(new_token)
    }
    
    /// Get a provider adapter
    pub fn get_provider(&self, provider: &OAuthProvider) -> Option<&Arc<dyn ProviderAdapter>> {
        self.providers.get(provider)
    }
    
    /// Get the token service
    pub fn token_service(&self) -> &Arc<TokenService> {
        &self.token_service
    }
}