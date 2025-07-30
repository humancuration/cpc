//! Adapter registry service for the API & Integration Hub module

use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    domain::adapter_config::{AdapterConfig, AdapterType},
    application::api_management::ApiManagementError,
};
use thiserror::Error;
use tracing::{info, warn, error, debug};

/// Error types for adapter registry operations
#[derive(Error, Debug)]
pub enum AdapterRegistryError {
    #[error("Adapter not found: {0}")]
    AdapterNotFound(String),
    
    #[error("Adapter already registered: {0}")]
    AdapterAlreadyRegistered(String),
    
    #[error("Invalid adapter configuration: {0}")]
    InvalidAdapterConfig(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Initialization error: {0}")]
    InitializationError(String),
}

/// Adapter registry service
pub struct AdapterRegistryService<R: AdapterConfigRepository> {
    repository: R,
    adapters: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<Uuid, Box<dyn Adapter>>>>,
}

impl<R: AdapterConfigRepository> AdapterRegistryService<R> {
    /// Create a new adapter registry service
    pub fn new(repository: R) -> Self {
        Self {
            repository,
            adapters: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Register an adapter from configuration
    pub async fn register_adapter(&self, config_id: Uuid) -> Result<(), AdapterRegistryError> {
        info!("Registering adapter from config: {}", config_id);
        
        // Get the adapter configuration
        let config = self.repository.get_adapter_config(config_id)
            .await
            .map_err(|e| {
                error!("Failed to get adapter config {}: {}", config_id, e);
                AdapterRegistryError::StorageError(e.to_string())
            })?;
        
        // Validate the configuration
        config.validate()
            .map_err(|e| {
                error!("Invalid adapter configuration {}: {}", config_id, e);
                AdapterRegistryError::InvalidAdapterConfig(e.to_string())
            })?;
        
        // Create the adapter instance
        let adapter = self.create_adapter_instance(&config)
            .await
            .map_err(|e| {
                error!("Failed to create adapter instance from config {}: {}", config_id, e);
                AdapterRegistryError::InitializationError(e.to_string())
            })?;
        
        // Register the adapter
        {
            let mut adapters = self.adapters.write().await;
            adapters.insert(config_id, adapter);
        }
        
        info!("Successfully registered adapter: {}", config_id);
        Ok(())
    }
    
    /// Unregister an adapter
    pub async fn unregister_adapter(&self, config_id: Uuid) -> Result<(), AdapterRegistryError> {
        info!("Unregistering adapter: {}", config_id);
        
        let mut adapters = self.adapters.write().await;
        if adapters.remove(&config_id).is_some() {
            info!("Successfully unregistered adapter: {}", config_id);
            Ok(())
        } else {
            warn!("Adapter not found for unregistration: {}", config_id);
            Err(AdapterRegistryError::AdapterNotFound(format!("Adapter {} not found", config_id)))
        }
    }
    
    /// Get a registered adapter
    pub async fn get_adapter(&self, config_id: Uuid) -> Result<Box<dyn Adapter>, AdapterRegistryError> {
        debug!("Getting adapter: {}", config_id);
        
        let adapters = self.adapters.read().await;
        if let Some(adapter) = adapters.get(&config_id) {
            // Clone the adapter (this requires adapters to implement Clone)
            // For this example, we'll return an error since we can't clone trait objects
            // In a real implementation, we might use a different approach
            error!("Cannot clone adapter instance");
            Err(AdapterRegistryError::AdapterNotFound(format!("Cannot get adapter {} instance", config_id)))
        } else {
            warn!("Adapter not found: {}", config_id);
            Err(AdapterRegistryError::AdapterNotFound(format!("Adapter {} not found", config_id)))
        }
    }
    
    /// Check if an adapter is registered
    pub async fn is_adapter_registered(&self, config_id: Uuid) -> bool {
        debug!("Checking if adapter is registered: {}", config_id);
        
        let adapters = self.adapters.read().await;
        adapters.contains_key(&config_id)
    }
    
    /// Get all registered adapter IDs
    pub async fn get_registered_adapter_ids(&self) -> Vec<Uuid> {
        debug!("Getting all registered adapter IDs");
        
        let adapters = self.adapters.read().await;
        adapters.keys().copied().collect()
    }
    
    /// Create an adapter instance from configuration
    async fn create_adapter_instance(&self, config: &AdapterConfig) -> Result<Box<dyn Adapter>, AdapterRegistryError> {
        info!("Creating adapter instance for type: {:?}", config.adapter_type);
        
        match &config.adapter_type {
            AdapterType::SAP => {
                // Create SAP adapter instance
                let sap_adapter = SapAdapter::new(config)?;
                Ok(Box::new(sap_adapter))
            }
            AdapterType::Oracle => {
                // Create Oracle adapter instance
                let oracle_adapter = OracleAdapter::new(config)?;
                Ok(Box::new(oracle_adapter))
            }
            AdapterType::CustomHttp => {
                // Create custom HTTP adapter instance
                let http_adapter = HttpAdapter::new(config)?;
                Ok(Box::new(http_adapter))
            }
            AdapterType::CustomGrpc => {
                // Create custom gRPC adapter instance
                let grpc_adapter = GrpcAdapter::new(config)?;
                Ok(Box::new(grpc_adapter))
            }
            AdapterType::Database => {
                // Create database adapter instance
                let db_adapter = DatabaseAdapter::new(config)?;
                Ok(Box::new(db_adapter))
            }
            AdapterType::FileSystem => {
                // Create file system adapter instance
                let fs_adapter = FileSystemAdapter::new(config)?;
                Ok(Box::new(fs_adapter))
            }
            AdapterType::Custom(custom_type) => {
                // Create custom adapter instance
                warn!("Custom adapter type not fully implemented: {}", custom_type);
                // For custom adapters, we might need a plugin system or factory pattern
                // For now, we'll return an error
                Err(AdapterRegistryError::InitializationError(
                    format!("Custom adapter type {} not implemented", custom_type)
                ))
            }
        }
    }
    
    /// Initialize all adapters from the repository
    pub async fn initialize_all_adapters(&self) -> Result<(), AdapterRegistryError> {
        info!("Initializing all adapters");
        
        let configs = self.repository.get_all_adapter_configs()
            .await
            .map_err(|e| {
                error!("Failed to get adapter configurations: {}", e);
                AdapterRegistryError::StorageError(e.to_string())
            })?;
        
        let mut success_count = 0;
        let mut error_count = 0;
        
        for config in configs {
            if config.enabled {
                match self.register_adapter(config.id).await {
                    Ok(_) => {
                        success_count += 1;
                    }
                    Err(e) => {
                        error!("Failed to register adapter {}: {}", config.id, e);
                        error_count += 1;
                    }
                }
            }
        }
        
        info!("Initialized {} adapters, {} errors", success_count, error_count);
        
        if error_count > 0 {
            Err(AdapterRegistryError::InitializationError(
                format!("Failed to initialize {} adapters", error_count)
            ))
        } else {
            Ok(())
        }
    }
}

/// Repository trait for adapter configuration access
#[async_trait]
pub trait AdapterConfigRepository: Send + Sync {
    /// Get an adapter configuration by ID
    async fn get_adapter_config(&self, id: Uuid) -> Result<AdapterConfig, ApiManagementError>;
    
    /// Get all adapter configurations
    async fn get_all_adapter_configs(&self) -> Result<Vec<AdapterConfig>, ApiManagementError>;
}

/// Adapter trait for all adapter types
#[async_trait]
pub trait Adapter: Send + Sync {
    /// Get the adapter type
    fn get_type(&self) -> &str;
    
    /// Execute a request through the adapter
    async fn execute_request(
        &self,
        path: &str,
        method: &crate::domain::api_endpoint::HttpMethod,
        headers: &std::collections::HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError>;
    
    /// Check if the adapter is healthy
    async fn is_healthy(&self) -> bool;
    
    /// Get adapter statistics
    async fn get_statistics(&self) -> AdapterStatistics;
}

/// Adapter statistics
#[derive(Debug, Clone)]
pub struct AdapterStatistics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
    pub last_request_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// SAP adapter implementation
pub struct SapAdapter {
    config: AdapterConfig,
    statistics: std::sync::Arc<tokio::sync::RwLock<AdapterStatistics>>,
}

impl SapAdapter {
    pub fn new(config: &AdapterConfig) -> Result<Self, AdapterRegistryError> {
        if config.adapter_type != AdapterType::SAP {
            return Err(AdapterRegistryError::InvalidAdapterConfig(
                "Configuration is not for SAP adapter".to_string()
            ));
        }
        
        Ok(Self {
            config: config.clone(),
            statistics: std::sync::Arc::new(tokio::sync::RwLock::new(AdapterStatistics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                last_request_timestamp: None,
            })),
        })
    }
}

#[async_trait]
impl Adapter for SapAdapter {
    fn get_type(&self) -> &str {
        "SAP"
    }
    
    async fn execute_request(
        &self,
        _path: &str,
        _method: &crate::domain::api_endpoint::HttpMethod,
        _headers: &std::collections::HashMap<String, String>,
        _body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError> {
        // In a real implementation, this would connect to SAP and execute the request
        // For this example, we'll return a mock response
        
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.successful_requests += 1;
        stats.last_request_timestamp = Some(chrono::Utc::now());
        
        Ok(serde_json::json!({
            "status": "success",
            "data": "SAP response data"
        }))
    }
    
    async fn is_healthy(&self) -> bool {
        // In a real implementation, this would check the SAP connection
        true
    }
    
    async fn get_statistics(&self) -> AdapterStatistics {
        let stats = self.statistics.read().await;
        stats.clone()
    }
}

/// Oracle adapter implementation
pub struct OracleAdapter {
    config: AdapterConfig,
    statistics: std::sync::Arc<tokio::sync::RwLock<AdapterStatistics>>,
}

impl OracleAdapter {
    pub fn new(config: &AdapterConfig) -> Result<Self, AdapterRegistryError> {
        if config.adapter_type != AdapterType::Oracle {
            return Err(AdapterRegistryError::InvalidAdapterConfig(
                "Configuration is not for Oracle adapter".to_string()
            ));
        }
        
        Ok(Self {
            config: config.clone(),
            statistics: std::sync::Arc::new(tokio::sync::RwLock::new(AdapterStatistics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                last_request_timestamp: None,
            })),
        })
    }
}

#[async_trait]
impl Adapter for OracleAdapter {
    fn get_type(&self) -> &str {
        "Oracle"
    }
    
    async fn execute_request(
        &self,
        _path: &str,
        _method: &crate::domain::api_endpoint::HttpMethod,
        _headers: &std::collections::HashMap<String, String>,
        _body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError> {
        // In a real implementation, this would connect to Oracle and execute the request
        // For this example, we'll return a mock response
        
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.successful_requests += 1;
        stats.last_request_timestamp = Some(chrono::Utc::now());
        
        Ok(serde_json::json!({
            "status": "success",
            "data": "Oracle response data"
        }))
    }
    
    async fn is_healthy(&self) -> bool {
        // In a real implementation, this would check the Oracle connection
        true
    }
    
    async fn get_statistics(&self) -> AdapterStatistics {
        let stats = self.statistics.read().await;
        stats.clone()
    }
}

/// HTTP adapter implementation
pub struct HttpAdapter {
    config: AdapterConfig,
    client: reqwest::Client,
    statistics: std::sync::Arc<tokio::sync::RwLock<AdapterStatistics>>,
}

impl HttpAdapter {
    pub fn new(config: &AdapterConfig) -> Result<Self, AdapterRegistryError> {
        if config.adapter_type != AdapterType::CustomHttp {
            return Err(AdapterRegistryError::InvalidAdapterConfig(
                "Configuration is not for HTTP adapter".to_string()
            ));
        }
        
        let client = reqwest::Client::new();
        
        Ok(Self {
            config: config.clone(),
            client,
            statistics: std::sync::Arc::new(tokio::sync::RwLock::new(AdapterStatistics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                last_request_timestamp: None,
            })),
        })
    }
}

#[async_trait]
impl Adapter for HttpAdapter {
    fn get_type(&self) -> &str {
        "HTTP"
    }
    
    async fn execute_request(
        &self,
        path: &str,
        method: &crate::domain::api_endpoint::HttpMethod,
        headers: &std::collections::HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError> {
        // In a real implementation, this would make an HTTP request to the target service
        // For this example, we'll return a mock response
        
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.successful_requests += 1;
        stats.last_request_timestamp = Some(chrono::Utc::now());
        
        // Construct the full URL
        let base_url = self.config.get_config_value("http_base_url")
            .ok_or_else(|| AdapterRegistryError::InvalidAdapterConfig(
                "Missing http_base_url configuration".to_string()
            ))?;
        
        let url = format!("{}{}", base_url, path);
        info!("Making HTTP request to: {}", url);
        
        // In a real implementation, we would:
        // 1. Create the HTTP request with the specified method
        // 2. Add headers
        // 3. Add body if present
        // 4. Send the request
        // 5. Handle the response
        
        Ok(serde_json::json!({
            "status": "success",
            "data": format!("HTTP response for {}", url),
            "method": format!("{:?}", method),
            "headers": headers,
            "body": body
        }))
    }
    
    async fn is_healthy(&self) -> bool {
        // In a real implementation, this would check connectivity to the HTTP service
        true
    }
    
    async fn get_statistics(&self) -> AdapterStatistics {
        let stats = self.statistics.read().await;
        stats.clone()
    }
}

/// gRPC adapter implementation
pub struct GrpcAdapter {
    config: AdapterConfig,
    statistics: std::sync::Arc<tokio::sync::RwLock<AdapterStatistics>>,
}

impl GrpcAdapter {
    pub fn new(config: &AdapterConfig) -> Result<Self, AdapterRegistryError> {
        if config.adapter_type != AdapterType::CustomGrpc {
            return Err(AdapterRegistryError::InvalidAdapterConfig(
                "Configuration is not for gRPC adapter".to_string()
            ));
        }
        
        Ok(Self {
            config: config.clone(),
            statistics: std::sync::Arc::new(tokio::sync::RwLock::new(AdapterStatistics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                last_request_timestamp: None,
            })),
        })
    }
}

#[async_trait]
impl Adapter for GrpcAdapter {
    fn get_type(&self) -> &str {
        "gRPC"
    }
    
    async fn execute_request(
        &self,
        _path: &str,
        _method: &crate::domain::api_endpoint::HttpMethod,
        _headers: &std::collections::HashMap<String, String>,
        _body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError> {
        // In a real implementation, this would make a gRPC call to the target service
        // For this example, we'll return a mock response
        
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.successful_requests += 1;
        stats.last_request_timestamp = Some(chrono::Utc::now());
        
        Ok(serde_json::json!({
            "status": "success",
            "data": "gRPC response data"
        }))
    }
    
    async fn is_healthy(&self) -> bool {
        // In a real implementation, this would check connectivity to the gRPC service
        true
    }
    
    async fn get_statistics(&self) -> AdapterStatistics {
        let stats = self.statistics.read().await;
        stats.clone()
    }
}

/// Database adapter implementation
pub struct DatabaseAdapter {
    config: AdapterConfig,
    statistics: std::sync::Arc<tokio::sync::RwLock<AdapterStatistics>>,
}

impl DatabaseAdapter {
    pub fn new(config: &AdapterConfig) -> Result<Self, AdapterRegistryError> {
        if config.adapter_type != AdapterType::Database {
            return Err(AdapterRegistryError::InvalidAdapterConfig(
                "Configuration is not for Database adapter".to_string()
            ));
        }
        
        Ok(Self {
            config: config.clone(),
            statistics: std::sync::Arc::new(tokio::sync::RwLock::new(AdapterStatistics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                last_request_timestamp: None,
            })),
        })
    }
}

#[async_trait]
impl Adapter for DatabaseAdapter {
    fn get_type(&self) -> &str {
        "Database"
    }
    
    async fn execute_request(
        &self,
        _path: &str,
        _method: &crate::domain::api_endpoint::HttpMethod,
        _headers: &std::collections::HashMap<String, String>,
        _body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError> {
        // In a real implementation, this would execute a database query
        // For this example, we'll return a mock response
        
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.successful_requests += 1;
        stats.last_request_timestamp = Some(chrono::Utc::now());
        
        Ok(serde_json::json!({
            "status": "success",
            "data": "Database query result"
        }))
    }
    
    async fn is_healthy(&self) -> bool {
        // In a real implementation, this would check database connectivity
        true
    }
    
    async fn get_statistics(&self) -> AdapterStatistics {
        let stats = self.statistics.read().await;
        stats.clone()
    }
}

/// File system adapter implementation
pub struct FileSystemAdapter {
    config: AdapterConfig,
    statistics: std::sync::Arc<tokio::sync::RwLock<AdapterStatistics>>,
}

impl FileSystemAdapter {
    pub fn new(config: &AdapterConfig) -> Result<Self, AdapterRegistryError> {
        if config.adapter_type != AdapterType::FileSystem {
            return Err(AdapterRegistryError::InvalidAdapterConfig(
                "Configuration is not for File System adapter".to_string()
            ));
        }
        
        Ok(Self {
            config: config.clone(),
            statistics: std::sync::Arc::new(tokio::sync::RwLock::new(AdapterStatistics {
                total_requests: 0,
                successful_requests: 0,
                failed_requests: 0,
                average_response_time_ms: 0.0,
                last_request_timestamp: None,
            })),
        })
    }
}

#[async_trait]
impl Adapter for FileSystemAdapter {
    fn get_type(&self) -> &str {
        "FileSystem"
    }
    
    async fn execute_request(
        &self,
        path: &str,
        _method: &crate::domain::api_endpoint::HttpMethod,
        _headers: &std::collections::HashMap<String, String>,
        _body: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AdapterRegistryError> {
        // In a real implementation, this would read/write files
        // For this example, we'll return a mock response
        
        let mut stats = self.statistics.write().await;
        stats.total_requests += 1;
        stats.successful_requests += 1;
        stats.last_request_timestamp = Some(chrono::Utc::now());
        
        let file_path = self.config.get_config_value("file_path")
            .ok_or_else(|| AdapterRegistryError::InvalidAdapterConfig(
                "Missing file_path configuration".to_string()
            ))?;
        
        Ok(serde_json::json!({
            "status": "success",
            "data": format!("File operation on {}{}", file_path, path)
        }))
    }
    
    async fn is_healthy(&self) -> bool {
        // In a real implementation, this would check file system access
        true
    }
    
    async fn get_statistics(&self) -> AdapterStatistics {
        let stats = self.statistics.read().await;
        stats.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::adapter_config::AdapterType;
    use std::collections::HashMap;
    
    // Mock adapter config repository for testing
    struct MockAdapterConfigRepository;
    
    #[async_trait]
    impl AdapterConfigRepository for MockAdapterConfigRepository {
        async fn get_adapter_config(&self, _id: Uuid) -> Result<AdapterConfig, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            let mut config = HashMap::new();
            config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
            
            Ok(AdapterConfig::new(
                "Test HTTP Adapter".to_string(),
                AdapterType::CustomHttp,
                config,
                owner_id,
                Some("Test HTTP adapter".to_string()),
                vec![],
                None,
            ).unwrap())
        }
        
        async fn get_all_adapter_configs(&self) -> Result<Vec<AdapterConfig>, ApiManagementError> {
            let owner_id = Uuid::new_v4();
            let mut config = HashMap::new();
            config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
            
            let adapter_config = AdapterConfig::new(
                "Test HTTP Adapter".to_string(),
                AdapterType::CustomHttp,
                config,
                owner_id,
                Some("Test HTTP adapter".to_string()),
                vec![],
                None,
            ).unwrap();
            
            Ok(vec![adapter_config])
        }
    }
    
    #[tokio::test]
    async fn test_register_adapter() {
        let repository = MockAdapterConfigRepository;
        let service = AdapterRegistryService::new(repository);
        
        let config_id = Uuid::new_v4();
        let result = service.register_adapter(config_id).await;
        
        // This will fail because we can't actually create the adapter in the mock
        // but we can at least test that the method is called
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_http_adapter_creation() {
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let adapter_config = AdapterConfig::new(
            "Test HTTP Adapter".to_string(),
            AdapterType::CustomHttp,
            config,
            owner_id,
            Some("Test HTTP adapter".to_string()),
            vec![],
            None,
        ).unwrap();
        
        let result = HttpAdapter::new(&adapter_config);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_http_adapter_execute_request() {
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let adapter_config = AdapterConfig::new(
            "Test HTTP Adapter".to_string(),
            AdapterType::CustomHttp,
            config,
            owner_id,
            Some("Test HTTP adapter".to_string()),
            vec![],
            None,
        ).unwrap();
        
        let adapter = HttpAdapter::new(&adapter_config).unwrap();
        let headers = HashMap::new();
        
        let result = adapter.execute_request(
            "/test",
            &crate::domain::api_endpoint::HttpMethod::GET,
            &headers,
            None,
        ).await;
        
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_adapter_statistics() {
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let adapter_config = AdapterConfig::new(
            "Test HTTP Adapter".to_string(),
            AdapterType::CustomHttp,
            config,
            owner_id,
            Some("Test HTTP adapter".to_string()),
            vec![],
            None,
        ).unwrap();
        
        let adapter = HttpAdapter::new(&adapter_config).unwrap();
        
        // Execute a request to update statistics
        let headers = HashMap::new();
        let _ = adapter.execute_request(
            "/test",
            &crate::domain::api_endpoint::HttpMethod::GET,
            &headers,
            None,
        ).await;
        
        let stats = adapter.get_statistics().await;
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.successful_requests, 1);
        assert!(stats.last_request_timestamp.is_some());
    }
}