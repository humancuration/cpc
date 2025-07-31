//! API management service for the API & Integration Hub module

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{
    api_endpoint::{ApiEndpoint, HttpMethod, AuthenticationType, ApiEndpointError},
    adapter_config::{AdapterConfig, AdapterConfigError},
    rate_limit::RateLimitRule,
};
use thiserror::Error;
use tracing::{info, warn, error};

/// Error types for API management operations
#[derive(Error, Debug)]
pub enum ApiManagementError {
    #[error("API endpoint error: {0}")]
    ApiEndpointError(#[from] ApiEndpointError),
    
    #[error("Adapter configuration error: {0}")]
    AdapterConfigError(#[from] AdapterConfigError),
    
    #[error("Rate limit error: {0}")]
    RateLimitError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
}

/// API management service
pub struct ApiManagementService<R: ApiRepository, A: AdapterRepository, RL: RateLimitRepository> {
    api_repository: R,
    adapter_repository: A,
    rate_limit_repository: RL,
}

impl<R: ApiRepository, A: AdapterRepository, RL: RateLimitRepository> ApiManagementService<R, A, RL> {
    /// Create a new API management service
    pub fn new(api_repository: R, adapter_repository: A, rate_limit_repository: RL) -> Self {
        Self {
            api_repository,
            adapter_repository,
            rate_limit_repository,
        }
    }
    
    /// Create a new API endpoint
    pub async fn create_api_endpoint(
        &self,
        name: String,
        path: String,
        method: HttpMethod,
        authentication: AuthenticationType,
        adapter_config_id: Uuid,
        owner_id: Uuid,
        description: Option<String>,
        rate_limit_rule_id: Option<Uuid>,
    ) -> Result<ApiEndpoint, ApiManagementError> {
        info!("Creating API endpoint: {}", name);
        
        // Validate that the adapter config exists
        let _adapter_config = self.adapter_repository.get_adapter_config(adapter_config_id)
            .await
            .map_err(|e| {
                error!("Failed to find adapter config {}: {}", adapter_config_id, e);
                ApiManagementError::ValidationError(format!("Adapter config not found: {}", e))
            })?;
        
        // Validate that the rate limit rule exists if specified
        if let Some(rule_id) = rate_limit_rule_id {
            let _rate_limit_rule = self.rate_limit_repository.get_rate_limit_rule(rule_id)
                .await
                .map_err(|e| {
                    error!("Failed to find rate limit rule {}: {}", rule_id, e);
                    ApiManagementError::ValidationError(format!("Rate limit rule not found: {}", e))
                })?;
        }
        
        let endpoint = ApiEndpoint::new(
            name,
            path,
            method,
            authentication,
            adapter_config_id,
            owner_id,
            description,
            rate_limit_rule_id,
        )?;
        
        self.api_repository.save_api_endpoint(&endpoint)
            .await
            .map_err(|e| {
                error!("Failed to save API endpoint: {}", e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        info!("Successfully created API endpoint: {}", endpoint.id);
        Ok(endpoint)
    }
    
    /// Get an API endpoint by ID
    pub async fn get_api_endpoint(&self, id: Uuid) -> Result<ApiEndpoint, ApiManagementError> {
        info!("Getting API endpoint: {}", id);
        
        let endpoint = self.api_repository.get_api_endpoint(id)
            .await
            .map_err(|e| {
                error!("Failed to get API endpoint {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        Ok(endpoint)
    }
    
    /// Get API endpoints by owner
    pub async fn get_api_endpoints_by_owner(&self, owner_id: Uuid) -> Result<Vec<ApiEndpoint>, ApiManagementError> {
        info!("Getting API endpoints for owner: {}", owner_id);
        
        let endpoints = self.api_repository.get_api_endpoints_by_owner(owner_id)
            .await
            .map_err(|e| {
                error!("Failed to get API endpoints for owner {}: {}", owner_id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        Ok(endpoints)
    }
    
    /// Update an API endpoint
    pub async fn update_api_endpoint(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        path: Option<String>,
        method: Option<HttpMethod>,
        authentication: Option<AuthenticationType>,
        adapter_config_id: Option<Uuid>,
        rate_limit_rule_id: Option<Option<Uuid>>,
        enabled: Option<bool>,
    ) -> Result<ApiEndpoint, ApiManagementError> {
        info!("Updating API endpoint: {}", id);
        
        let mut endpoint = self.api_repository.get_api_endpoint(id)
            .await
            .map_err(|e| {
                error!("Failed to get API endpoint {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        // Validate that the adapter config exists if specified
        if let Some(adapter_id) = adapter_config_id {
            let _adapter_config = self.adapter_repository.get_adapter_config(adapter_id)
                .await
                .map_err(|e| {
                    error!("Failed to find adapter config {}: {}", adapter_id, e);
                    ApiManagementError::ValidationError(format!("Adapter config not found: {}", e))
                })?;
        }
        
        // Validate that the rate limit rule exists if specified
        if let Some(Some(rule_id)) = rate_limit_rule_id {
            let _rate_limit_rule = self.rate_limit_repository.get_rate_limit_rule(rule_id)
                .await
                .map_err(|e| {
                    error!("Failed to find rate limit rule {}: {}", rule_id, e);
                    ApiManagementError::ValidationError(format!("Rate limit rule not found: {}", e))
                })?;
        }
        
        endpoint.update_info(
            name,
            description,
            path,
            method,
            authentication,
            adapter_config_id,
            rate_limit_rule_id.unwrap_or(None),
            enabled,
        )?;
        
        self.api_repository.save_api_endpoint(&endpoint)
            .await
            .map_err(|e| {
                error!("Failed to save API endpoint: {}", e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        info!("Successfully updated API endpoint: {}", endpoint.id);
        Ok(endpoint)
    }
    
    /// Delete an API endpoint
    pub async fn delete_api_endpoint(&self, id: Uuid) -> Result<(), ApiManagementError> {
        info!("Deleting API endpoint: {}", id);
        
        self.api_repository.delete_api_endpoint(id)
            .await
            .map_err(|e| {
                error!("Failed to delete API endpoint {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        info!("Successfully deleted API endpoint: {}", id);
        Ok(())
    }
    
    /// Create a new adapter configuration
    pub async fn create_adapter_config(
        &self,
        name: String,
        adapter_type: crate::domain::adapter_config::AdapterType,
        config: std::collections::HashMap<String, String>,
        owner_id: Uuid,
        description: Option<String>,
        transformation_rules: Vec<crate::domain::adapter_config::TransformationRule>,
        rate_limit_rule: Option<RateLimitRule>,
    ) -> Result<AdapterConfig, ApiManagementError> {
        info!("Creating adapter configuration: {}", name);
        
        let adapter_config = AdapterConfig::new(
            name,
            adapter_type,
            config,
            owner_id,
            description,
            transformation_rules,
            rate_limit_rule,
        )?;
        
        self.adapter_repository.save_adapter_config(&adapter_config)
            .await
            .map_err(|e| {
                error!("Failed to save adapter config: {}", e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        info!("Successfully created adapter configuration: {}", adapter_config.id);
        Ok(adapter_config)
    }
    
    /// Get an adapter configuration by ID
    pub async fn get_adapter_config(&self, id: Uuid) -> Result<AdapterConfig, ApiManagementError> {
        info!("Getting adapter configuration: {}", id);
        
        let adapter_config = self.adapter_repository.get_adapter_config(id)
            .await
            .map_err(|e| {
                error!("Failed to get adapter config {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        Ok(adapter_config)
    }
    
    /// Get adapter configurations by owner
    pub async fn get_adapter_configs_by_owner(&self, owner_id: Uuid) -> Result<Vec<AdapterConfig>, ApiManagementError> {
        info!("Getting adapter configurations for owner: {}", owner_id);
        
        let adapter_configs = self.adapter_repository.get_adapter_configs_by_owner(owner_id)
            .await
            .map_err(|e| {
                error!("Failed to get adapter configs for owner {}: {}", owner_id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        Ok(adapter_configs)
    }
    
    /// Update an adapter configuration
    pub async fn update_adapter_config(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        config: Option<std::collections::HashMap<String, String>>,
        transformation_rules: Option<Vec<crate::domain::adapter_config::TransformationRule>>,
        rate_limit_rule: Option<Option<RateLimitRule>>,
        enabled: Option<bool>,
    ) -> Result<AdapterConfig, ApiManagementError> {
        info!("Updating adapter configuration: {}", id);
        
        let mut adapter_config = self.adapter_repository.get_adapter_config(id)
            .await
            .map_err(|e| {
                error!("Failed to get adapter config {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        adapter_config.update_info(
            name,
            description,
            config,
            transformation_rules,
            rate_limit_rule,
            enabled,
        )?;
        
        self.adapter_repository.save_adapter_config(&adapter_config)
            .await
            .map_err(|e| {
                error!("Failed to save adapter config: {}", e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        info!("Successfully updated adapter configuration: {}", adapter_config.id);
        Ok(adapter_config)
    }
    
    /// Delete an adapter configuration
    pub async fn delete_adapter_config(&self, id: Uuid) -> Result<(), ApiManagementError> {
        info!("Deleting adapter configuration: {}", id);
        
        // Check if any API endpoints are using this adapter config
        let endpoints = self.api_repository.get_api_endpoints_by_adapter_config(id)
            .await
            .map_err(|e| {
                error!("Failed to check API endpoints for adapter config {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        if !endpoints.is_empty() {
            warn!("Cannot delete adapter config {} because it's used by {} API endpoints", id, endpoints.len());
            return Err(ApiManagementError::ValidationError(
                format!("Cannot delete adapter config because it's used by {} API endpoints", endpoints.len())
            ));
        }
        
        self.adapter_repository.delete_adapter_config(id)
            .await
            .map_err(|e| {
                error!("Failed to delete adapter config {}: {}", id, e);
                ApiManagementError::StorageError(e.to_string())
            })?;
        
        info!("Successfully deleted adapter configuration: {}", id);
        Ok(())
    }
}

/// Repository trait for API endpoint storage
#[async_trait]
pub trait ApiRepository: Send + Sync {
    /// Save an API endpoint
    async fn save_api_endpoint(&self, endpoint: &ApiEndpoint) -> Result<(), ApiManagementError>;
    
    /// Get an API endpoint by ID
    async fn get_api_endpoint(&self, id: Uuid) -> Result<ApiEndpoint, ApiManagementError>;
    
    /// Get API endpoints by owner
    async fn get_api_endpoints_by_owner(&self, owner_id: Uuid) -> Result<Vec<ApiEndpoint>, ApiManagementError>;
    
    /// Get API endpoints by adapter config ID
    async fn get_api_endpoints_by_adapter_config(&self, adapter_config_id: Uuid) -> Result<Vec<ApiEndpoint>, ApiManagementError>;
    
    /// Delete an API endpoint
    async fn delete_api_endpoint(&self, id: Uuid) -> Result<(), ApiManagementError>;
}

/// Repository trait for adapter configuration storage
#[async_trait]
pub trait AdapterRepository: Send + Sync {
    /// Save an adapter configuration
    async fn save_adapter_config(&self, config: &AdapterConfig) -> Result<(), ApiManagementError>;
    
    /// Get an adapter configuration by ID
    async fn get_adapter_config(&self, id: Uuid) -> Result<AdapterConfig, ApiManagementError>;
    
    /// Get adapter configurations by owner
    async fn get_adapter_configs_by_owner(&self, owner_id: Uuid) -> Result<Vec<AdapterConfig>, ApiManagementError>;
    
    /// Delete an adapter configuration
    async fn delete_adapter_config(&self, id: Uuid) -> Result<(), ApiManagementError>;
}

/// Repository trait for rate limit storage
#[async_trait]
pub trait RateLimitRepository: Send + Sync {
    /// Save a rate limit rule
    async fn save_rate_limit_rule(&self, rule: &RateLimitRule) -> Result<(), ApiManagementError>;
    
    /// Get a rate limit rule by ID
    async fn get_rate_limit_rule(&self, id: Uuid) -> Result<RateLimitRule, ApiManagementError>;
    
    /// Get rate limit rules by owner
    async fn get_rate_limit_rules_by_owner(&self, owner_id: Uuid) -> Result<Vec<RateLimitRule>, ApiManagementError>;
    
    /// Delete a rate limit rule
    async fn delete_rate_limit_rule(&self, id: Uuid) -> Result<(), ApiManagementError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        api_endpoint::{HttpMethod, AuthenticationType},
        adapter_config::AdapterType,
    };
    use std::collections::HashMap;
    
    // Mock API repository for testing
    struct MockApiRepository;
    
    #[async_trait]
    impl ApiRepository for MockApiRepository {
        async fn save_api_endpoint(&self, _endpoint: &ApiEndpoint) -> Result<(), ApiManagementError> {
            Ok(())
        }
        
        async fn get_api_endpoint(&self, _id: Uuid) -> Result<ApiEndpoint, ApiManagementError> {
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
        
        async fn get_api_endpoints_by_owner(&self, _owner_id: Uuid) -> Result<Vec<ApiEndpoint>, ApiManagementError> {
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
            
            Ok(vec![endpoint])
        }
        
        async fn get_api_endpoints_by_adapter_config(&self, _adapter_config_id: Uuid) -> Result<Vec<ApiEndpoint>, ApiManagementError> {
            Ok(vec![])
        }
        
        async fn delete_api_endpoint(&self, _id: Uuid) -> Result<(), ApiManagementError> {
            Ok(())
        }
    }
    
    // Mock adapter repository for testing
    struct MockAdapterRepository;
    
    #[async_trait]
    impl AdapterRepository for MockAdapterRepository {
        async fn save_adapter_config(&self, _config: &AdapterConfig) -> Result<(), ApiManagementError> {
            Ok(())
        }
        
        async fn get_adapter_config(&self, _id: Uuid) -> Result<AdapterConfig, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            let mut config = HashMap::new();
            config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
            
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
        
        async fn get_adapter_configs_by_owner(&self, _owner_id: Uuid) -> Result<Vec<AdapterConfig>, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            let mut config = HashMap::new();
            config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
            
            let adapter_config = AdapterConfig::new(
                "Test Adapter".to_string(),
                AdapterType::CustomHttp,
                config,
                owner_id,
                Some("Test adapter".to_string()),
                vec![],
                None,
            ).unwrap();
            
            Ok(vec![adapter_config])
        }
        
        async fn delete_adapter_config(&self, _id: Uuid) -> Result<(), ApiManagementError> {
            Ok(())
        }
    }
    
    // Mock rate limit repository for testing
    struct MockRateLimitRepository;
    
    #[async_trait]
    impl RateLimitRepository for MockRateLimitRepository {
        async fn save_rate_limit_rule(&self, _rule: &RateLimitRule) -> Result<(), ApiManagementError> {
            Ok(())
        }
        
        async fn get_rate_limit_rule(&self, _id: Uuid) -> Result<RateLimitRule, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            
            Ok(RateLimitRule::new(
                "Test Rate Limit".to_string(),
                100,
                60,
                20,
                owner_id,
                Some("Test rate limit rule".to_string()),
            ).unwrap())
        }
        
        async fn get_rate_limit_rules_by_owner(&self, _owner_id: Uuid) -> Result<Vec<RateLimitRule>, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            
            let rule = RateLimitRule::new(
                "Test Rate Limit".to_string(),
                100,
                60,
                20,
                owner_id,
                Some("Test rate limit rule".to_string()),
            ).unwrap();
            
            Ok(vec![rule])
        }
        
        async fn delete_rate_limit_rule(&self, _id: Uuid) -> Result<(), ApiManagementError> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_create_api_endpoint() {
        let api_repo = MockApiRepository;
        let adapter_repo = MockAdapterRepository;
        let rate_limit_repo = MockRateLimitRepository;
        let service = ApiManagementService::new(api_repo, adapter_repo, rate_limit_repo);
        
        let owner_id = Uuid::new_v4();
        let adapter_config_id = Uuid::new_v4();
        
        let endpoint = service.create_api_endpoint(
            "Test Endpoint".to_string(),
            "/test".to_string(),
            HttpMethod::GET,
            AuthenticationType::None,
            adapter_config_id,
            owner_id,
            Some("Test endpoint".to_string()),
            None,
        ).await.unwrap();
        
        assert_eq!(endpoint.name, "Test Endpoint");
        assert_eq!(endpoint.path, "/test");
        assert_eq!(endpoint.method, HttpMethod::GET);
    }
    
    #[tokio::test]
    async fn test_create_adapter_config() {
        let api_repo = MockApiRepository;
        let adapter_repo = MockAdapterRepository;
        let rate_limit_repo = MockRateLimitRepository;
        let service = ApiManagementService::new(api_repo, adapter_repo, rate_limit_repo);
        
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let adapter_config = service.create_adapter_config(
            "Test Adapter".to_string(),
            AdapterType::CustomHttp,
            config,
            owner_id,
            Some("Test adapter".to_string()),
            vec![],
            None,
        ).await.unwrap();
        
        assert_eq!(adapter_config.name, "Test Adapter");
        assert_eq!(adapter_config.adapter_type, AdapterType::CustomHttp);
    }
    
    #[tokio::test]
    async fn test_get_api_endpoint() {
        let api_repo = MockApiRepository;
        let adapter_repo = MockAdapterRepository;
        let rate_limit_repo = MockRateLimitRepository;
        let service = ApiManagementService::new(api_repo, adapter_repo, rate_limit_repo);
        
        let endpoint_id = Uuid::new_v4();
        let endpoint = service.get_api_endpoint(endpoint_id).await.unwrap();
        
        assert_eq!(endpoint.name, "Test Endpoint");
    }
    
    #[tokio::test]
    async fn test_get_adapter_config() {
        let api_repo = MockApiRepository;
        let adapter_repo = MockAdapterRepository;
        let rate_limit_repo = MockRateLimitRepository;
        let service = ApiManagementService::new(api_repo, adapter_repo, rate_limit_repo);
        
        let adapter_config_id = Uuid::new_v4();
        let adapter_config = service.get_adapter_config(adapter_config_id).await.unwrap();
        
        assert_eq!(adapter_config.name, "Test Adapter");
    }
}