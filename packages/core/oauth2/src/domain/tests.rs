//! Tests for the domain layer

#[cfg(test)]
mod oauth_provider_tests {
    use crate::domain::{OAuthProvider, AuthError};
    
    #[test]
    fn test_provider_as_str() {
        assert_eq!(OAuthProvider::TikTok.as_str(), "tiktok");
        assert_eq!(OAuthProvider::Google.as_str(), "google");
        assert_eq!(OAuthProvider::Facebook.as_str(), "facebook");
        assert_eq!(OAuthProvider::Twitter.as_str(), "twitter");
    }
    
    #[test]
    fn test_provider_from_str() {
        assert_eq!(OAuthProvider::from_str("tiktok").unwrap(), OAuthProvider::TikTok);
        assert_eq!(OAuthProvider::from_str("google").unwrap(), OAuthProvider::Google);
        assert_eq!(OAuthProvider::from_str("facebook").unwrap(), OAuthProvider::Facebook);
        assert_eq!(OAuthProvider::from_str("twitter").unwrap(), OAuthProvider::Twitter);
        
        // Test case insensitivity
        assert_eq!(OAuthProvider::from_str("TIKTOK").unwrap(), OAuthProvider::TikTok);
        assert_eq!(OAuthProvider::from_str("Google").unwrap(), OAuthProvider::Google);
        
        // Test unsupported provider
        assert!(matches!(
            OAuthProvider::from_str("unsupported"),
            Err(AuthError::UnsupportedProvider(_))
        ));
    }
}

#[cfg(test)]
mod oauth_token_tests {
    use crate::domain::{OAuthToken, AuthError};
    use chrono::{Utc, Duration};
    
    #[test]
    fn test_token_encryption_decryption() {
        let encryption_key = crate::domain::auth_config::AuthConfig::generate_encryption_key();
        
        let token = OAuthToken::new(
            "access_token_123".to_string(),
            Some("refresh_token_456".to_string()),
            Utc::now() + Duration::hours(1),
            vec!["read".to_string(), "write".to_string()],
            "tiktok".to_string(),
        );
        
        // Encrypt the token
        let encrypted_token = token.encrypt(&encryption_key).unwrap();
        
        // Decrypt the token
        let decrypted_token = OAuthToken::decrypt(&encrypted_token, &encryption_key).unwrap();
        
        // Verify the decrypted token matches the original
        assert_eq!(token.access_token, decrypted_token.access_token);
        assert_eq!(token.refresh_token, decrypted_token.refresh_token);
        assert_eq!(token.expires_at, decrypted_token.expires_at);
        assert_eq!(token.scopes, decrypted_token.scopes);
        assert_eq!(token.provider, decrypted_token.provider);
    }
    
    #[test]
    fn test_token_expiration() {
        let token = OAuthToken::new(
            "access_token_123".to_string(),
            Some("refresh_token_456".to_string()),
            Utc::now() - Duration::hours(1), // Expired 1 hour ago
            vec!["read".to_string()],
            "tiktok".to_string(),
        );
        
        assert!(token.is_expired());
        
        let token = OAuthToken::new(
            "access_token_123".to_string(),
            Some("refresh_token_456".to_string()),
            Utc::now() + Duration::hours(1), // Expires in 1 hour
            vec!["read".to_string()],
            "tiktok".to_string(),
        );
        
        assert!(!token.is_expired());
    }
    
    #[test]
    fn test_encrypted_token_encoding() {
        let encryption_key = crate::domain::auth_config::AuthConfig::generate_encryption_key();
        
        let token = OAuthToken::new(
            "access_token_123".to_string(),
            Some("refresh_token_456".to_string()),
            Utc::now() + Duration::hours(1),
            vec!["read".to_string()],
            "tiktok".to_string(),
        );
        
        let encrypted_token = token.encrypt(&encryption_key).unwrap();
        
        // Encode the token
        let encoded = encrypted_token.encode().unwrap();
        
        // Decode the token
        let decoded = crate::domain::oauth_token::EncryptedToken::decode(&encoded).unwrap();
        
        // Verify the decoded token matches the original
        assert_eq!(encrypted_token.access_token, decoded.access_token);
        assert_eq!(encrypted_token.refresh_token, decoded.refresh_token);
        assert_eq!(encrypted_token.nonce, decoded.nonce);
        assert_eq!(encrypted_token.expires_at, decoded.expires_at);
        assert_eq!(encrypted_token.scopes, decoded.scopes);
        assert_eq!(encrypted_token.provider, decoded.provider);
    }
}

#[cfg(test)]
mod auth_config_tests {
    use crate::domain::{AuthConfig, ProviderConfig, OAuthProvider};
    use std::collections::HashMap;
    
    #[test]
    fn test_auth_config_creation() {
        let encryption_key = AuthConfig::generate_encryption_key();
        let config = AuthConfig::new("http://localhost:3000/callback".to_string(), encryption_key);
        
        assert_eq!(config.default_redirect_uri, "http://localhost:3000/callback");
        assert_eq!(config.providers.len(), 0);
    }
    
    #[test]
    fn test_provider_config_management() {
        let encryption_key = AuthConfig::generate_encryption_key();
        let mut config = AuthConfig::new("http://localhost:3000/callback".to_string(), encryption_key);
        
        let provider_config = ProviderConfig {
            client_id: "client_id".to_string(),
            client_secret: "client_secret".to_string(),
            auth_url: "https://auth.example.com".to_string(),
            token_url: "https://token.example.com".to_string(),
            redirect_uris: vec!["http://localhost:3000/callback".to_string()],
            default_scopes: vec!["read".to_string()],
        };
        
        config.set_provider_config(OAuthProvider::TikTok, provider_config.clone());
        
        let retrieved_config = config.get_provider_config(&OAuthProvider::TikTok);
        assert!(retrieved_config.is_some());
        assert_eq!(retrieved_config.unwrap().client_id, "client_id");
    }
}