//! Authentication and OAuth2 integration for the Messenger application

use oauth2::{
    basic::BasicClient, AuthUrl, TokenUrl, ClientId, ClientSecret,
    AuthorizationCode, CsrfToken, PkceCodeChallenge, Scope,
};
use std::collections::HashMap;
use uuid::Uuid;
use tracing::debug;

/// OAuth2 identity provider for social platform integrations
pub struct OAuth2IdentityProvider {
    /// OAuth2 clients for different social platforms
    clients: HashMap<String, BasicClient>,
}

impl OAuth2IdentityProvider {
    /// Create a new OAuth2 identity provider
    pub fn new() -> Self {
        let mut clients = HashMap::new();
        
        // Add TikTok provider
        if let (Ok(client_id), Ok(client_secret)) = (
            std::env::var("TIKTOK_CLIENT_ID"),
            std::env::var("TIKTOK_CLIENT_SECRET"),
        ) {
            clients.insert(
                "tiktok".to_string(),
                BasicClient::new(
                    ClientId::new(client_id),
                    Some(ClientSecret::new(client_secret)),
                    AuthUrl::new("https://open.tiktokapis.com/v2/oauth/authorize/".to_string()).unwrap(),
                    Some(TokenUrl::new("https://open.tiktokapis.com/v2/oauth/token/".to_string()).unwrap()),
                )
            );
        }
        
        // Add Facebook provider
        if let (Ok(client_id), Ok(client_secret)) = (
            std::env::var("FACEBOOK_CLIENT_ID"),
            std::env::var("FACEBOOK_CLIENT_SECRET"),
        ) {
            clients.insert(
                "facebook".to_string(),
                BasicClient::new(
                    ClientId::new(client_id),
                    Some(ClientSecret::new(client_secret)),
                    AuthUrl::new("https://www.facebook.com/v19.0/dialog/oauth".to_string()).unwrap(),
                    Some(TokenUrl::new("https://graph.facebook.com/v19.0/oauth/access_token".to_string()).unwrap()),
                )
            );
        }
        
        // Add other providers as needed
        // Instagram, Threads, WeChat, Messenger, Snapchat, Discord, X, Twitch, Gmail, etc.
        
        Self { clients }
    }
    
    /// Get the OAuth2 client for a specific platform
    pub fn get_client(&self, platform: &str) -> Option<&BasicClient> {
        self.clients.get(platform)
    }
    
    /// Generate an authorization URL for a platform
    pub fn generate_auth_url(&self, platform: &str, redirect_url: &str) -> Option<(String, CsrfToken)> {
        let client = self.get_client(platform)?;
        
        let (pkce_challenge, _pkce_verifier) = PkceCodeChallenge::new_random_sha256();
        
        let (auth_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("public_profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();
        
        Some((auth_url.to_string(), csrf_token))
    }
    
    /// Exchange an authorization code for an access token
    pub async fn exchange_code(
        &self,
        platform: &str,
        code: AuthorizationCode,
        csrf_token: CsrfToken,
    ) -> Result<oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>, oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>> {
        let client = self.get_client(platform).unwrap(); // Safe because we checked above
        
        client.exchange_code(code).request_async(oauth2::reqwest::async_http_client).await
    }
}

impl Default for OAuth2IdentityProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// User session information
#[derive(Debug, Clone)]
pub struct UserSession {
    /// User identifier
    pub user_id: Uuid,
    
    /// Session identifier
    pub session_id: Uuid,
    
    /// Platform used for authentication
    pub platform: String,
    
    /// Expiration time
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Authentication service
pub struct AuthService {
    /// Identity provider
    identity_provider: OAuth2IdentityProvider,
    
    /// Active user sessions
    sessions: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, UserSession>>>,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new(identity_provider: OAuth2IdentityProvider) -> Self {
        Self {
            identity_provider,
            sessions: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
    
    /// Get the identity provider
    pub fn identity_provider(&self) -> &OAuth2IdentityProvider {
        &self.identity_provider
    }
    
    /// Create a new user session
    pub async fn create_session(&self, user_id: Uuid, platform: String) -> UserSession {
        let session_id = Uuid::new_v4();
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24); // 24 hour session
        
        let session = UserSession {
            user_id,
            session_id,
            platform,
            expires_at,
        };
        
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session_id, session.clone());
        }
        
        // Start a task to clean up expired sessions
        let sessions = self.sessions.clone();
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; // Check every minute
            let now = chrono::Utc::now();
            let mut sessions = sessions.write().await;
            sessions.retain(|_, session| session.expires_at > now);
        });
        
        session
    }
    
    /// Validate a user session
    pub async fn validate_session(&self, session_id: Uuid) -> Option<UserSession> {
        let sessions = self.sessions.read().await;
        let session = sessions.get(&session_id)?;
        
        if session.expires_at > chrono::Utc::now() {
            Some(session.clone())
        } else {
            None
        }
    }
    
    /// Revoke a user session
    pub async fn revoke_session(&self, session_id: Uuid) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(&session_id);
    }
}