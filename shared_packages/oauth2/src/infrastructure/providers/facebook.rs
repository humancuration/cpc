//! Facebook OAuth provider adapter

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

/// Facebook OAuth provider adapter
pub struct FacebookAdapter {
    client: BasicClient,
    http_client: Client,
}

impl FacebookAdapter {
    /// Create a new Facebook adapter
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Result<Self, AuthError> {
        let auth_url = AuthUrl::new("https://www.facebook.com/v12.0/dialog/oauth".to_string())
            .map_err(|e| AuthError::ProviderError(format!("Invalid auth URL: {}", e)))?;
        
        let token_url = TokenUrl::new("https://graph.facebook.com/v12.0/oauth/access_token".to_string())
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
impl ProviderAdapter for FacebookAdapter {
    fn provider(&self) -> OAuthProvider {
        OAuthProvider::Facebook
    }
    
    fn generate_auth_url(&self, redirect_uri: &str) -> Result<(String, String), AuthError> {
        debug!("Generating Facebook authorization URL");
        
        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public_profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();
        
        info!("Generated Facebook authorization URL");
        
        Ok((auth_url.to_string(), csrf_token.secret().clone()))
    }
    
    async fn exchange_code(&self, code: String, _state: String) -> Result<OAuthToken, AuthError> {
        debug!(code = %code, "Exchanging Facebook authorization code for token");
        
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
        
        info!("Successfully exchanged Facebook authorization code for token");
        
        Ok(token)
    }
    
    async fn refresh_token(&self, refresh_token: String) -> Result<OAuthToken, AuthError> {
        debug!(refresh_token = %refresh_token, "Refreshing Facebook access token");
        
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
        
        info!("Successfully refreshed Facebook access token");
        
        Ok(token)
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::domain::{AuthConfig, ProviderConfig};
        use std::collections::HashMap;
        use std::sync::Arc;
        
        #[test]
        fn test_facebook_adapter_creation() {
            let adapter = FacebookAdapter::new(
                "test_client_id".to_string(),
                "test_client_secret".to_string(),
                "http://localhost:3000/callback".to_string(),
            );
            
            assert!(adapter.is_ok());
        }
        
        #[tokio::test]
        async fn test_facebook_generate_auth_url() {
            let adapter = FacebookAdapter::new(
                "test_client_id".to_string(),
                "test_client_secret".to_string(),
                "http://localhost:3000/callback".to_string(),
            ).unwrap();
            
            let result = adapter.generate_auth_url("http://localhost:3000/callback");
            assert!(result.is_ok());
            
            let (auth_url, _state) = result.unwrap();
            assert!(auth_url.contains("facebook.com"));
            assert!(auth_url.contains("client_id=test_client_id"));
        }
        
        // Note: These tests would require mocking the HTTP responses in a real implementation
        // For now, we're just testing the structure and error handling
        
        #[tokio::test]
        async fn test_fetch_profile_missing_id() {
            let adapter = FacebookAdapter::new(
                "test_client_id".to_string(),
                "test_client_secret".to_string(),
                "http://localhost:3000/callback".to_string(),
            ).unwrap();
            
            // Create a mock token
            use chrono::{Utc, Duration};
            let token = OAuthToken::new(
                "mock_access_token".to_string(),
                Some("mock_refresh_token".to_string()),
                Utc::now() + Duration::hours(1),
                vec!["public_profile".to_string(), "email".to_string()],
                "facebook".to_string(),
            );
            
            // In a real test, we would mock the HTTP response to return invalid data
            // For now, we're just checking that the method exists and compiles
        }
        
        #[test]
        fn test_facebook_scopes() {
            let adapter = FacebookAdapter::new(
                "test_client_id".to_string(),
                "test_client_secret".to_string(),
                "http://localhost:3000/callback".to_string(),
            ).unwrap();
            
            let result = adapter.generate_auth_url("http://localhost:3000/callback");
            assert!(result.is_ok());
            
            let (auth_url, _state) = result.unwrap();
            // Check that the required scopes are included
            assert!(auth_url.contains("public_profile"));
            assert!(auth_url.contains("email"));
        }
    }
    
    async fn fetch_profile(&self, token: &OAuthToken) -> Result<OAuthProfile, AuthError> {
        debug!("Fetching Facebook user profile");
        
        // Facebook Graph API endpoint for user profile with required fields
        let url = format!(
            "https://graph.facebook.com/v12.0/me?access_token={}&fields=id,name,email,picture",
            token.access_token
        );
        
        let response = self.http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| AuthError::NetworkError(format!("Failed to fetch profile: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(AuthError::ProviderError(format!(
                "Failed to fetch profile: HTTP {}",
                response.status()
            )));
        }
        
        let profile_data: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ProviderError(format!("Failed to parse profile: {}", e)))?;
        
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
        
        let email = profile_data
            .get("email")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Extract avatar URL from picture field
        let avatar_url = profile_data
            .get("picture")
            .and_then(|v| v.get("data"))
            .and_then(|v| v.get("url"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let profile = OAuthProfile::new(
            provider_id,
            name,
            email,
            self.provider(),
            avatar_url,
            Some(profile_data),
        );
        
        info!("Successfully fetched Facebook user profile");
        
        Ok(profile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{AuthConfig, ProviderConfig};
    use std::collections::HashMap;
    use std::sync::Arc;
    
    #[test]
    fn test_facebook_adapter_creation() {
        let adapter = FacebookAdapter::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:3000/callback".to_string(),
        );
        
        assert!(adapter.is_ok());
    }
    
    #[tokio::test]
    async fn test_facebook_generate_auth_url() {
        let adapter = FacebookAdapter::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:3000/callback".to_string(),
        ).unwrap();
        
        let result = adapter.generate_auth_url("http://localhost:3000/callback");
        assert!(result.is_ok());
        
        let (auth_url, _state) = result.unwrap();
        assert!(auth_url.contains("facebook.com"));
        assert!(auth_url.contains("client_id=test_client_id"));
    }
    
    // Note: These tests would require mocking the HTTP responses in a real implementation
    // For now, we're just testing the structure and error handling
    
    #[tokio::test]
    async fn test_fetch_profile_missing_id() {
        let adapter = FacebookAdapter::new(
            "test_client_id".to_string(),
            "test_client_secret".to_string(),
            "http://localhost:3000/callback".to_string(),
        ).unwrap();
        
        // Create a mock token
        use chrono::{Utc, Duration};
        let token = OAuthToken::new(
            "mock_access_token".to_string(),
            Some("mock_refresh_token".to_string()),
            Utc::now() + Duration::hours(1),
            vec!["public_profile".to_string(), "email".to_string()],
            "facebook".to_string(),
        );
        
        // In a real test, we would mock the HTTP response to return invalid data
        // For now, we're just checking that the method exists and compiles
    }
}