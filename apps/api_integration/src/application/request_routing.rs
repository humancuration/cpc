//! Request routing service for the API & Integration Hub module

use async_trait::async_trait;
use uuid::Uuid;
use serde_json::Value as JsonValue;
use crate::{
    domain::{
        api_endpoint::{ApiEndpoint, HttpMethod, AuthenticationType},
        adapter_config::{AdapterConfig, AdapterType, TransformationRule, TransformationType},
    },
    application::api_management::ApiManagementError,
};
use thiserror::Error;
use tracing::{info, warn, error, debug};

/// Error types for request routing operations
#[derive(Error, Debug)]
pub enum RequestRoutingError {
    #[error("API endpoint not found: {0}")]
    EndpointNotFound(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
    
    #[error("Adapter error: {0}")]
    AdapterError(String),
    
    #[error("Transformation error: {0}")]
    TransformationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// Request routing service
pub struct RequestRoutingService<A: ApiEndpointRepository, AD: AdapterExecutor> {
    api_endpoint_repository: A,
    adapter_executor: AD,
}

impl<A: ApiEndpointRepository, AD: AdapterExecutor> RequestRoutingService<A, AD> {
    /// Create a new request routing service
    pub fn new(api_endpoint_repository: A, adapter_executor: AD) -> Self {
        Self {
            api_endpoint_repository,
            adapter_executor,
        }
    }
    
    /// Route an incoming request to the appropriate adapter
    pub async fn route_request(
        &self,
        path: &str,
        method: HttpMethod,
        headers: &std::collections::HashMap<String, String>,
        body: Option<&JsonValue>,
        api_key: Option<&str>,
        jwt_token: Option<&str>,
    ) -> Result<JsonValue, RequestRoutingError> {
        info!("Routing request to path: {} with method: {:?}", path, method);
        
        // Find the matching API endpoint
        let endpoint = self.api_endpoint_repository.find_endpoint(path, &method)
            .await
            .map_err(|e| {
                error!("Failed to find endpoint for path {} and method {:?}: {}", path, method, e);
                RequestRoutingError::EndpointNotFound(format!("No endpoint found for path {} and method {:?}", path, method))
            })?;
        
        // Check if endpoint is enabled
        if !endpoint.enabled {
            warn!("Endpoint {} is disabled", endpoint.id);
            return Err(RequestRoutingError::EndpointNotFound("Endpoint is disabled".to_string()));
        }
        
        // Authenticate the request if required
        if endpoint.requires_authentication() {
            self.authenticate_request(&endpoint, headers, api_key, jwt_token)
                .await?;
        }
        
        // Check rate limits
        if let Some(rate_limit_rule_id) = endpoint.rate_limit_rule_id {
            self.check_rate_limit(rate_limit_rule_id, &endpoint.owner_id)
                .await?;
        }
        
        // Get the adapter configuration
        let adapter_config = self.api_endpoint_repository.get_adapter_config(endpoint.adapter_config_id)
            .await
            .map_err(|e| {
                error!("Failed to get adapter config {}: {}", endpoint.adapter_config_id, e);
                RequestRoutingError::InternalError(format!("Failed to get adapter config: {}", e))
            })?;
        
        // Transform the request body if needed
        let transformed_body = if let Some(body) = body {
            self.transform_request_body(body, &adapter_config.transformation_rules)
                .map_err(|e| {
                    error!("Failed to transform request body: {}", e);
                    RequestRoutingError::TransformationError(e.to_string())
                })?
        } else {
            None
        };
        
        // Execute the request through the adapter
        let response = self.adapter_executor.execute_request(
            &adapter_config,
            path,
            &method,
            headers,
            transformed_body.as_ref(),
        ).await
        .map_err(|e| {
            error!("Adapter execution failed: {}", e);
            RequestRoutingError::AdapterError(e.to_string())
        })?;
        
        // Transform the response if needed
        let transformed_response = self.transform_response_body(&response, &adapter_config.transformation_rules)
            .map_err(|e| {
                error!("Failed to transform response body: {}", e);
                RequestRoutingError::TransformationError(e.to_string())
            })?;
        
        info!("Successfully routed request to endpoint: {}", endpoint.id);
        Ok(transformed_response)
    }
    
    /// Authenticate a request based on endpoint requirements
    async fn authenticate_request(
        &self,
        endpoint: &ApiEndpoint,
        headers: &std::collections::HashMap<String, String>,
        api_key: Option<&str>,
        jwt_token: Option<&str>,
    ) -> Result<(), RequestRoutingError> {
        debug!("Authenticating request for endpoint: {}", endpoint.id);
        
        match &endpoint.authentication {
            AuthenticationType::None => {
                // No authentication required
                Ok(())
            }
            AuthenticationType::ApiKey => {
                // Check for API key in headers or as parameter
                let provided_key = if let Some(key) = api_key {
                    Some(key)
                } else if let Some(key) = headers.get("x-api-key") {
                    Some(key.as_str())
                } else {
                    None
                };
                
                if provided_key.is_none() {
                    return Err(RequestRoutingError::AuthenticationFailed("API key required but not provided".to_string()));
                }
                
                // In a real implementation, we would validate the API key against stored keys
                // For now, we'll just check that it's present
                Ok(())
            }
            AuthenticationType::Jwt => {
                // Check for JWT token in Authorization header
                let token = if let Some(token) = jwt_token {
                    token
                } else if let Some(auth_header) = headers.get("authorization") {
                    if auth_header.starts_with("Bearer ") {
                        &auth_header[7..]
                    } else {
                        return Err(RequestRoutingError::AuthenticationFailed("Invalid Authorization header format".to_string()));
                    }
                } else {
                    return Err(RequestRoutingError::AuthenticationFailed("JWT token required but not provided".to_string()));
                };
                
                // In a real implementation, we would validate the JWT token
                // For now, we'll just check that it's present
                if token.is_empty() {
                    return Err(RequestRoutingError::AuthenticationFailed("JWT token is empty".to_string()));
                }
                
                Ok(())
            }
            AuthenticationType::OAuth2 => {
                // Check for OAuth2 token in Authorization header
                if let Some(auth_header) = headers.get("authorization") {
                    if !auth_header.starts_with("Bearer ") {
                        return Err(RequestRoutingError::AuthenticationFailed("Invalid OAuth2 Authorization header format".to_string()));
                    }
                    
                    // In a real implementation, we would validate the OAuth2 token
                    // For now, we'll just check that it's present
                    let token = &auth_header[7..];
                    if token.is_empty() {
                        return Err(RequestRoutingError::AuthenticationFailed("OAuth2 token is empty".to_string()));
                    }
                    
                    Ok(())
                } else {
                    Err(RequestRoutingError::AuthenticationFailed("OAuth2 token required but not provided".to_string()))
                }
            }
            AuthenticationType::Custom(auth_type) => {
                warn!("Custom authentication type not fully implemented: {}", auth_type);
                // For custom authentication, we'll just log a warning and allow the request
                Ok(())
            }
        }
    }
    
    /// Check rate limits for a request
    async fn check_rate_limit(
        &self,
        rate_limit_rule_id: Uuid,
        user_id: &Uuid,
    ) -> Result<(), RequestRoutingError> {
        debug!("Checking rate limit rule: {} for user: {}", rate_limit_rule_id, user_id);
        
        // In a real implementation, we would:
        // 1. Get the rate limit rule from the repository
        // 2. Check the current request count for this user/window
        // 3. Increment the request count
        // 4. Return an error if the limit is exceeded
        
        // For now, we'll just simulate the check
        // In a real implementation, this would involve Redis or similar for tracking requests
        Ok(())
    }
    
    /// Transform request body based on transformation rules
    fn transform_request_body(
        &self,
        body: &JsonValue,
        rules: &[TransformationRule],
    ) -> Result<Option<JsonValue>, RequestRoutingError> {
        debug!("Transforming request body with {} rules", rules.len());
        
        if rules.is_empty() {
            return Ok(Some(body.clone()));
        }
        
        // Apply transformation rules
        let mut transformed = body.clone();
        
        for rule in rules {
            transformed = self.apply_transformation_rule(&transformed, rule, true)?;
        }
        
        Ok(Some(transformed))
    }
    
    /// Transform response body based on transformation rules
    fn transform_response_body(
        &self,
        body: &JsonValue,
        rules: &[TransformationRule],
    ) -> Result<JsonValue, RequestRoutingError> {
        debug!("Transforming response body with {} rules", rules.len());
        
        if rules.is_empty() {
            return Ok(body.clone());
        }
        
        // Apply transformation rules
        let mut transformed = body.clone();
        
        for rule in rules {
            transformed = self.apply_transformation_rule(&transformed, rule, false)?;
        }
        
        Ok(transformed)
    }
    
    /// Apply a single transformation rule
    fn apply_transformation_rule(
        &self,
        data: &JsonValue,
        rule: &TransformationRule,
        is_request: bool,
    ) -> Result<JsonValue, RequestRoutingError> {
        // Determine source and target fields based on direction
        let (source_field, target_field) = if is_request {
            (&rule.source_field, &rule.target_field)
        } else {
            (&rule.target_field, &rule.source_field)
        };
        
        // For simplicity, we'll only handle direct mapping in this example
        // A full implementation would handle all transformation types
        match &rule.transformation_type {
            TransformationType::Direct => {
                // Direct mapping - copy value from source to target field
                let mut result = data.clone();
                
                if let Some(value) = data.get(source_field) {
                    if let JsonValue::Object(ref mut obj) = result {
                        obj.insert(target_field.clone(), value.clone());
                    }
                }
                
                Ok(result)
            }
            _ => {
                // For other transformation types, we'll just pass through the data
                // A real implementation would handle format conversion, math operations, etc.
                warn!("Transformation type {:?} not fully implemented", rule.transformation_type);
                Ok(data.clone())
            }
        }
    }
}

/// Repository trait for API endpoint access
#[async_trait]
pub trait ApiEndpointRepository: Send + Sync {
    /// Find an endpoint by path and method
    async fn find_endpoint(&self, path: &str, method: &HttpMethod) -> Result<ApiEndpoint, ApiManagementError>;
    
    /// Get an adapter configuration by ID
    async fn get_adapter_config(&self, id: Uuid) -> Result<AdapterConfig, ApiManagementError>;
}

/// Adapter executor trait
#[async_trait]
pub trait AdapterExecutor: Send + Sync {
    /// Execute a request through an adapter
    async fn execute_request(
        &self,
        adapter_config: &AdapterConfig,
        path: &str,
        method: &HttpMethod,
        headers: &std::collections::HashMap<String, String>,
        body: Option<&JsonValue>,
    ) -> Result<JsonValue, RequestRoutingError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        api_endpoint::{HttpMethod, AuthenticationType},
        adapter_config::{AdapterType, TransformationRule, TransformationType},
    };
    use serde_json::json;
    use std::collections::HashMap;
    
    // Mock API endpoint repository for testing
    struct MockApiEndpointRepository;
    
    #[async_trait]
    impl ApiEndpointRepository for MockApiEndpointRepository {
        async fn find_endpoint(&self, _path: &str, _method: &HttpMethod) -> Result<ApiEndpoint, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            let adapter_config_id = Uuid::new_v4();
            
            Ok(ApiEndpoint::new(
                "Test Endpoint".to_string(),
                "/test".to_string(),
                HttpMethod::GET,
                AuthenticationType::None,
                adapter_config_id,
                owner_id,
                Some("Test endpoint".to_string()),
                None,
            ).unwrap())
        }
        
        async fn get_adapter_config(&self, _id: Uuid) -> Result<AdapterConfig, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            let config = HashMap::new();
            
            Ok(AdapterConfig::new(
                "Test Adapter".to_string(),
                AdapterType::CustomHttp,
                config,
                owner_id,
                Some("Test adapter".to_string()),
                vec![],
                None,
            ).unwrap())
        }
    }
    
    // Mock adapter executor for testing
    struct MockAdapterExecutor;
    
    #[async_trait]
    impl AdapterExecutor for MockAdapterExecutor {
        async fn execute_request(
            &self,
            _adapter_config: &AdapterConfig,
            _path: &str,
            _method: &HttpMethod,
            _headers: &HashMap<String, String>,
            _body: Option<&JsonValue>,
        ) -> Result<JsonValue, RequestRoutingError> {
            // Return a simple JSON response
            Ok(json!({
                "status": "success",
                "data": "test response"
            }))
        }
    }
    
    #[tokio::test]
    async fn test_route_request() {
        let api_repo = MockApiEndpointRepository;
        let adapter_executor = MockAdapterExecutor;
        let service = RequestRoutingService::new(api_repo, adapter_executor);
        
        let headers = HashMap::new();
        let response = service.route_request(
            "/test",
            HttpMethod::GET,
            &headers,
            None,
            None,
            None,
        ).await.unwrap();
        
        assert_eq!(response["status"], "success");
        assert_eq!(response["data"], "test response");
    }
    
    #[tokio::test]
    async fn test_authenticate_request_no_auth() {
        let api_repo = MockApiEndpointRepository;
        let adapter_executor = MockAdapterExecutor;
        let service = RequestRoutingService::new(api_repo, adapter_executor);
        
        let owner_id = Uuid::new_v4();
        let adapter_config_id = Uuid::new_v4();
        
        let endpoint = ApiEndpoint::new(
            "Test Endpoint".to_string(),
            "/test".to_string(),
            HttpMethod::GET,
            AuthenticationType::None,
            adapter_config_id,
            owner_id,
            Some("Test endpoint".to_string()),
            None,
        ).unwrap();
        
        let headers = HashMap::new();
        let result = service.authenticate_request(&endpoint, &headers, None, None).await;
        
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_transform_request_body() {
        let api_repo = MockApiEndpointRepository;
        let adapter_executor = MockAdapterExecutor;
        let service = RequestRoutingService::new(api_repo, adapter_executor);
        
        let body = json!({
            "name": "John Doe",
            "email": "john@example.com"
        });
        
        let rule = TransformationRule::new(
            "name".to_string(),
            "full_name".to_string(),
            TransformationType::Direct,
            HashMap::new(),
        );
        
        let rules = vec![rule];
        let transformed = service.transform_request_body(&body, &rules).unwrap().unwrap();
        
        assert_eq!(transformed["name"], "John Doe");
        assert_eq!(transformed["email"], "john@example.com");
        // Note: In a full implementation, the "full_name" field would be added
    }
}