//! Twitter OAuth provider adapter

use async_trait::async_trait;
use oauth2::{
    basic::BasicClient, AuthUrl, TokenUrl, ClientId, ClientSecret,
    AuthorizationCode, CsrfToken, Scope, TokenResponse,
};
use reqwest::Client;
use url::Url;
use crate::domain::{
    OAuthProvider, ProviderAdapter, OAuthToken, OAuthProfile, AuthError
};
use tracing::{info, debug, error};

/// Twitter OAuth provider adapter
pub struct TwitterAdapter {
    client: BasicClient,
    http_client: Client,
}

impl TwitterAdapter {
    /// Create a new Twitter adapter
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Result<Self, AuthError> {
        let auth_url = AuthUrl::new("https://twitter.com/i/oauth2/authorize".to_string())
            .map_err(|e| AuthError::ProviderError(format!("Invalid auth URL: {}", e)))?;
        
        let token_url = TokenUrl::new("https://api.twitter.com/2/oauth2/token".to_string())
            .map_err(|e| AuthError::ProviderError(format!("Invalid token URL: {}", e)))?;
        
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(
            oauth2::RedirectUrl::new(redirect_uri)
                .map_err(|e| AuthError::ProviderError(format!("Invalid redirect URI: {}", e)))?
        );
        
        Ok(Self {
            client,
            http_client: Client::new(),
        })
    }
}

#[async_trait]
impl ProviderAdapter for TwitterAdapter {
    fn provider(&self) -> OAuthProvider {
        OAuthProvider::Twitter
    }
    
    fn generate_auth_url(&self, redirect_uri: &str) -> Result<(String, String), AuthError> {
        debug!("Generating Twitter authorization URL");
        
        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("tweet.read".to_string()))
            .add_scope(Scope::new("users.read".to_string()))
            .url();
        
        info!("Generated Twitter authorization URL");
        
        Ok((auth_url.to_string(), csrf_token.secret().clone()))
    }
    
    async fn exchange_code(&self, code: String, _state: String) -> Result<OAuthToken, AuthError> {
        debug!(code = %code, "Exchanging Twitter authorization code for token");
        
        let token_result = self.client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| AuthError::TokenExchangeFailed(format!("Failed to exchange code: {}", e)))?;
        
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(
            token_result.expires_in().map(|d| d.as_secs() as i64).unwrap_or(3600)
        );
        
        let token = OAuthToken::new(
            token_result.access_token().secret().clone(),
            token_result.refresh_token().map(|t| t.secret().clone()),
            expires_at,
            token_result.scopes().iter().map(|s| s.to_string()).collect(),
            self.provider().to_string(),
        );
        
        info!("Successfully exchanged Twitter authorization code for token");
        
        Ok(token)
    }
    
    async fn refresh_token(&self, refresh_token: String) -> Result<OAuthToken, AuthError> {
        debug!(refresh_token = %refresh_token, "Refreshing Twitter access token");
        
        let token_result = self.client
            .exchange_refresh_token(&oauth2::RefreshToken::new(refresh_token))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| AuthError::TokenRefreshFailed(format!("Failed to refresh token: {}", e)))?;
        
        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(
            token_result.expires_in().map(|d| d.as_secs() as i64).unwrap_or(3600)
        );
        
        let token = OAuthToken::new(
            token_result.access_token().secret().clone(),
            token_result.refresh_token().map(|t| t.secret().clone()),
            expires_at,
            token_result.scopes().iter().map(|s| s.to_string()).collect(),
            self.provider().to_string(),
        );
        
        info!("Successfully refreshed Twitter access token");
        
        Ok(token)
    }
    
    async fn fetch_profile(&self, token: &OAuthToken) -> Result<OAuthProfile, AuthError> {
        debug!("Fetching Twitter user profile");
        
        let url = "https://api.twitter.com/2/users/me?user.fields=profile_image_url";
        
        let response = self.http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(format!("Failed to fetch profile: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(AuthError::ProviderError(format!(
                "Failed to fetch profile: HTTP {}",
                response.status()
            )));
        }
        
        let response_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ProviderError(format!("Failed to parse profile: {}", e)))?;
        
        let profile_data = response_data
            .get("data")
            .ok_or_else(|| AuthError::ProviderError("Missing user data in profile".to_string()))?;
        
        let provider_id = profile_data
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AuthError::ProviderError("Missing user ID in profile".to_string()))?
            .to_string();
        
        let name = profile_data
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        
        let username = profile_data
            .get("username")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let email = None; // Twitter API doesn't provide email in the basic profile endpoint
        
        let avatar_url = profile_data
            .get("profile_image_url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Format the name to include username if available
        let display_name = if let Some(username) = username {
            format!("{} (@{})", name, username)
        } else {
            name
        };
        
        let profile = OAuthProfile::new(
            provider_id,
            display_name,
            email,
            self.provider(),
            avatar_url,
            Some(response_data),
        );
        
        info!("Successfully fetched Twitter user profile");
        
        Ok(profile)
    }
}