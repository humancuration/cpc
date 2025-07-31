//! TikTok OAuth provider adapter

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

/// TikTok OAuth provider adapter
pub struct TikTokAdapter {
    client: BasicClient,
    http_client: Client,
}

impl TikTokAdapter {
    /// Create a new TikTok adapter
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Result<Self, AuthError> {
        let auth_url = AuthUrl::new("https://open.tiktokapis.com/v2/oauth/authorize/".to_string())
            .map_err(|e| AuthError::ProviderError(format!("Invalid auth URL: {}", e)))?;
        
        let token_url = TokenUrl::new("https://open.tiktokapis.com/v2/oauth/token/".to_string())
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
impl ProviderAdapter for TikTokAdapter {
    fn provider(&self) -> OAuthProvider {
        OAuthProvider::TikTok
    }
    
    fn generate_auth_url(&self, redirect_uri: &str) -> Result<(String, String), AuthError> {
        debug!("Generating TikTok authorization URL");
        
        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user.info.basic".to_string()))
            .url();
        
        info!("Generated TikTok authorization URL");
        
        Ok((auth_url.to_string(), csrf_token.secret().clone()))
    }
    
    async fn exchange_code(&self, code: String, _state: String) -> Result<OAuthToken, AuthError> {
        debug!(code = %code, "Exchanging TikTok authorization code for token");
        
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
        
        info!("Successfully exchanged TikTok authorization code for token");
        
        Ok(token)
    }
    
    async fn refresh_token(&self, refresh_token: String) -> Result<OAuthToken, AuthError> {
        debug!(refresh_token = %refresh_token, "Refreshing TikTok access token");
        
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
        
        info!("Successfully refreshed TikTok access token");
        
        Ok(token)
    }
    
    async fn fetch_profile(&self, token: &OAuthToken) -> Result<OAuthProfile, AuthError> {
        debug!("Fetching TikTok user profile");
        
        let url = "https://open.tiktokapis.com/v2/user/info/";
        
        let response = self.http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", token.access_token))
            .header("Content-Type", "application/json")
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
            .and_then(|v| v.get("user"))
            .ok_or_else(|| AuthError::ProviderError("Missing user data in profile".to_string()))?;
        
        let provider_id = profile_data
            .get("open_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AuthError::ProviderError("Missing user open_id in profile".to_string()))?
            .to_string();
        
        let name = profile_data
            .get("display_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();
        
        let avatar_url = profile_data
            .get("avatar_url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let profile = OAuthProfile::new(
            provider_id,
            name,
            None, // TikTok API doesn't provide email in basic scope
            self.provider(),
            avatar_url,
            Some(response_data),
        );
        
        info!("Successfully fetched TikTok user profile");
        
        Ok(profile)
    }
}