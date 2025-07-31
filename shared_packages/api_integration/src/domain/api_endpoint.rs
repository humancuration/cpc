//! API endpoint domain entities for the API & Integration Hub module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for API endpoint operations
#[derive(Error, Debug)]
pub enum ApiEndpointError {
    #[error("Invalid API endpoint data: {0}")]
    InvalidData(String),
    
    #[error("API endpoint not found: {0}")]
    NotFound(String),
    
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
}

/// HTTP methods supported by API endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// Authentication types for API endpoints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthenticationType {
    None,
    ApiKey,
    OAuth2,
    Jwt,
    Custom(String),
}

/// API endpoint entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiEndpoint {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub path: String,
    pub method: HttpMethod,
    pub authentication: AuthenticationType,
    pub adapter_config_id: Uuid,
    pub rate_limit_rule_id: Option<Uuid>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
}

impl ApiEndpoint {
    /// Create a new API endpoint
    pub fn new(
        name: String,
        path: String,
        method: HttpMethod,
        authentication: AuthenticationType,
        adapter_config_id: Uuid,
        owner_id: Uuid,
        description: Option<String>,
        rate_limit_rule_id: Option<Uuid>,
    ) -> Result<Self, ApiEndpointError> {
        if name.is_empty() {
            return Err(ApiEndpointError::InvalidData("API endpoint name cannot be empty".to_string()));
        }
        
        if path.is_empty() {
            return Err(ApiEndpointError::InvalidData("API endpoint path cannot be empty".to_string()));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            path,
            method,
            authentication,
            adapter_config_id,
            rate_limit_rule_id,
            enabled: true,
            created_at: now,
            updated_at: now,
            owner_id,
        })
    }
    
    /// Update API endpoint information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        path: Option<String>,
        method: Option<HttpMethod>,
        authentication: Option<AuthenticationType>,
        adapter_config_id: Option<Uuid>,
        rate_limit_rule_id: Option<Uuid>,
        enabled: Option<bool>,
    ) -> Result<(), ApiEndpointError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(ApiEndpointError::InvalidData("API endpoint name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(description) = description {
            self.description = Some(description);
        }
        
        if let Some(path) = path {
            if path.is_empty() {
                return Err(ApiEndpointError::InvalidData("API endpoint path cannot be empty".to_string()));
            }
            self.path = path;
        }
        
        if let Some(method) = method {
            self.method = method;
        }
        
        if let Some(authentication) = authentication {
            self.authentication = authentication;
        }
        
        if let Some(adapter_config_id) = adapter_config_id {
            self.adapter_config_id = adapter_config_id;
        }
        
        if let Some(rate_limit_rule_id) = rate_limit_rule_id {
            self.rate_limit_rule_id = Some(rate_limit_rule_id);
        }
        
        if let Some(enabled) = enabled {
            self.enabled = enabled;
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Validate the API endpoint
    pub fn validate(&self) -> Result<(), ApiEndpointError> {
        if self.name.is_empty() {
            return Err(ApiEndpointError::InvalidData("API endpoint name cannot be empty".to_string()));
        }
        
        if self.path.is_empty() {
            return Err(ApiEndpointError::InvalidData("API endpoint path cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    /// Check if endpoint requires authentication
    pub fn requires_authentication(&self) -> bool {
        !matches!(self.authentication, AuthenticationType::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_api_endpoint() {
        let owner_id = Uuid::new_v4();
        let adapter_config_id = Uuid::new_v4();
        
        let endpoint = ApiEndpoint::new(
            "Get User".to_string(),
            "/users/{id}".to_string(),
            HttpMethod::GET,
            AuthenticationType::ApiKey,
            adapter_config_id,
            owner_id,
            Some("Get user by ID".to_string()),
            None,
        ).unwrap();
        
        assert_eq!(endpoint.name, "Get User");
        assert_eq!(endpoint.path, "/users/{id}");
        assert_eq!(endpoint.method, HttpMethod::GET);
        assert_eq!(endpoint.authentication, AuthenticationType::ApiKey);
        assert_eq!(endpoint.adapter_config_id, adapter_config_id);
        assert_eq!(endpoint.owner_id, owner_id);
        assert!(endpoint.enabled);
    }
    
    #[test]
    fn test_update_api_endpoint_info() {
        let owner_id = Uuid::new_v4();
        let adapter_config_id = Uuid::new_v4();
        let rate_limit_rule_id = Uuid::new_v4();
        
        let mut endpoint = ApiEndpoint::new(
            "Get User".to_string(),
            "/users/{id}".to_string(),
            HttpMethod::GET,
            AuthenticationType::ApiKey,
            adapter_config_id,
            owner_id,
            Some("Get user by ID".to_string()),
            None,
        ).unwrap();
        
        let new_adapter_config_id = Uuid::new_v4();
        
        endpoint.update_info(
            Some("Update User".to_string()),
            Some("Update user information".to_string()),
            Some("/users/{id}".to_string()),
            Some(HttpMethod::PUT),
            Some(AuthenticationType::Jwt),
            Some(new_adapter_config_id),
            Some(rate_limit_rule_id),
            Some(false),
        ).unwrap();
        
        assert_eq!(endpoint.name, "Update User");
        assert_eq!(endpoint.description, Some("Update user information".to_string()));
        assert_eq!(endpoint.path, "/users/{id}");
        assert_eq!(endpoint.method, HttpMethod::PUT);
        assert_eq!(endpoint.authentication, AuthenticationType::Jwt);
        assert_eq!(endpoint.adapter_config_id, new_adapter_config_id);
        assert_eq!(endpoint.rate_limit_rule_id, Some(rate_limit_rule_id));
        assert!(!endpoint.enabled);
    }
    
    #[test]
    fn test_api_endpoint_validation() {
        let owner_id = Uuid::new_v4();
        let adapter_config_id = Uuid::new_v4();
        
        let endpoint = ApiEndpoint::new(
            "Get User".to_string(),
            "/users/{id}".to_string(),
            HttpMethod::GET,
            AuthenticationType::ApiKey,
            adapter_config_id,
            owner_id,
            Some("Get user by ID".to_string()),
            None,
        ).unwrap();
        
        assert!(endpoint.validate().is_ok());
    }
    
    #[test]
    fn test_requires_authentication() {
        let owner_id = Uuid::new_v4();
        let adapter_config_id = Uuid::new_v4();
        
        let endpoint_no_auth = ApiEndpoint::new(
            "Public Endpoint".to_string(),
            "/public".to_string(),
            HttpMethod::GET,
            AuthenticationType::None,
            adapter_config_id,
            owner_id,
            None,
            None,
        ).unwrap();
        
        let endpoint_with_auth = ApiEndpoint::new(
            "Protected Endpoint".to_string(),
            "/protected".to_string(),
            HttpMethod::GET,
            AuthenticationType::ApiKey,
            adapter_config_id,
            owner_id,
            None,
            None,
        ).unwrap();
        
        assert!(!endpoint_no_auth.requires_authentication());
        assert!(endpoint_with_auth.requires_authentication());
    }
}