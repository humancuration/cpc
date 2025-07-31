//! Adapter configuration domain entities for the API & Integration Hub module

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use thiserror::Error;

/// Error types for adapter configuration operations
#[derive(Error, Debug)]
pub enum AdapterConfigError {
    #[error("Invalid adapter configuration data: {0}")]
    InvalidData(String),
    
    #[error("Adapter configuration not found: {0}")]
    NotFound(String),
    
    #[error("Adapter type error: {0}")]
    AdapterTypeError(String),
}

/// Adapter types supported by the integration hub
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AdapterType {
    /// SAP ERP integration adapter
    SAP,
    
    /// Oracle integration adapter
    Oracle,
    
    /// Custom HTTP/REST adapter
    CustomHttp,
    
    /// Custom gRPC adapter
    CustomGrpc,
    
    /// Database adapter
    Database,
    
    /// File system adapter
    FileSystem,
    
    /// Custom adapter type
    Custom(String),
}

/// Transformation rule for data mapping
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformationRule {
    pub id: Uuid,
    pub source_field: String,
    pub target_field: String,
    pub transformation_type: TransformationType,
    pub parameters: HashMap<String, String>,
}

/// Transformation types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransformationType {
    /// Direct mapping (no transformation)
    Direct,
    
    /// Format conversion (e.g., date format)
    Format,
    
    /// Mathematical operation (e.g., multiplication, addition)
    Math,
    
    /// String manipulation (e.g., uppercase, substring)
    String,
    
    /// Custom transformation function
    Custom(String),
}

/// Rate limiting rule
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RateLimitRule {
    pub id: Uuid,
    pub requests_per_window: u32,
    pub window_seconds: u32,
    pub burst_limit: u32,
    pub enabled: bool,
}

/// Adapter configuration entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdapterConfig {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub adapter_type: AdapterType,
    pub config: HashMap<String, String>,
    pub transformation_rules: Vec<TransformationRule>,
    pub rate_limit_rule: Option<RateLimitRule>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: Uuid,
}

impl AdapterConfig {
    /// Create a new adapter configuration
    pub fn new(
        name: String,
        adapter_type: AdapterType,
        config: HashMap<String, String>,
        owner_id: Uuid,
        description: Option<String>,
        transformation_rules: Vec<TransformationRule>,
        rate_limit_rule: Option<RateLimitRule>,
    ) -> Result<Self, AdapterConfigError> {
        if name.is_empty() {
            return Err(AdapterConfigError::InvalidData("Adapter configuration name cannot be empty".to_string()));
        }
        
        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            description,
            adapter_type,
            config,
            transformation_rules,
            rate_limit_rule,
            enabled: true,
            created_at: now,
            updated_at: now,
            owner_id,
        })
    }
    
    /// Update adapter configuration information
    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        config: Option<HashMap<String, String>>,
        transformation_rules: Option<Vec<TransformationRule>>,
        rate_limit_rule: Option<Option<RateLimitRule>>,
        enabled: Option<bool>,
    ) -> Result<(), AdapterConfigError> {
        if let Some(name) = name {
            if name.is_empty() {
                return Err(AdapterConfigError::InvalidData("Adapter configuration name cannot be empty".to_string()));
            }
            self.name = name;
        }
        
        if let Some(description) = description {
            self.description = Some(description);
        }
        
        if let Some(config) = config {
            self.config = config;
        }
        
        if let Some(transformation_rules) = transformation_rules {
            self.transformation_rules = transformation_rules;
        }
        
        if let Some(rate_limit_rule) = rate_limit_rule {
            self.rate_limit_rule = rate_limit_rule;
        }
        
        if let Some(enabled) = enabled {
            self.enabled = enabled;
        }
        
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Validate the adapter configuration
    pub fn validate(&self) -> Result<(), AdapterConfigError> {
        if self.name.is_empty() {
            return Err(AdapterConfigError::InvalidData("Adapter configuration name cannot be empty".to_string()));
        }
        
        // Validate configuration based on adapter type
        match self.adapter_type {
            AdapterType::SAP => {
                if !self.config.contains_key("sap_host") {
                    return Err(AdapterConfigError::InvalidData("SAP adapter requires 'sap_host' configuration".to_string()));
                }
                if !self.config.contains_key("sap_username") {
                    return Err(AdapterConfigError::InvalidData("SAP adapter requires 'sap_username' configuration".to_string()));
                }
                if !self.config.contains_key("sap_password") {
                    return Err(AdapterConfigError::InvalidData("SAP adapter requires 'sap_password' configuration".to_string()));
                }
            }
            AdapterType::Oracle => {
                if !self.config.contains_key("oracle_connection_string") {
                    return Err(AdapterConfigError::InvalidData("Oracle adapter requires 'oracle_connection_string' configuration".to_string()));
                }
            }
            AdapterType::CustomHttp => {
                if !self.config.contains_key("http_base_url") {
                    return Err(AdapterConfigError::InvalidData("Custom HTTP adapter requires 'http_base_url' configuration".to_string()));
                }
            }
            AdapterType::CustomGrpc => {
                if !self.config.contains_key("grpc_target") {
                    return Err(AdapterConfigError::InvalidData("Custom gRPC adapter requires 'grpc_target' configuration".to_string()));
                }
            }
            AdapterType::Database => {
                if !self.config.contains_key("database_url") {
                    return Err(AdapterConfigError::InvalidData("Database adapter requires 'database_url' configuration".to_string()));
                }
            }
            AdapterType::FileSystem => {
                if !self.config.contains_key("file_path") {
                    return Err(AdapterConfigError::InvalidData("File system adapter requires 'file_path' configuration".to_string()));
                }
            }
            AdapterType::Custom(_) => {
                // For custom adapters, we don't enforce specific configuration keys
                // but we still require at least one configuration parameter
                if self.config.is_empty() {
                    return Err(AdapterConfigError::InvalidData("Custom adapter requires at least one configuration parameter".to_string()));
                }
            }
        }
        
        Ok(())
    }
    
    /// Get a configuration value
    pub fn get_config_value(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
    
    /// Add a transformation rule
    pub fn add_transformation_rule(&mut self, rule: TransformationRule) {
        self.transformation_rules.push(rule);
        self.updated_at = Utc::now();
    }
    
    /// Remove a transformation rule
    pub fn remove_transformation_rule(&mut self, rule_id: Uuid) {
        self.transformation_rules.retain(|r| r.id != rule_id);
        self.updated_at = Utc::now();
    }
}

impl TransformationRule {
    /// Create a new transformation rule
    pub fn new(
        source_field: String,
        target_field: String,
        transformation_type: TransformationType,
        parameters: HashMap<String, String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_field,
            target_field,
            transformation_type,
            parameters,
        }
    }
}

impl RateLimitRule {
    /// Create a new rate limit rule
    pub fn new(
        requests_per_window: u32,
        window_seconds: u32,
        burst_limit: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            requests_per_window,
            window_seconds,
            burst_limit,
            enabled: true,
        }
    }
    
    /// Check if rate limiting is exceeded
    pub fn is_exceeded(&self, request_count: u32) -> bool {
        request_count > self.requests_per_window
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    
    #[test]
    fn test_create_adapter_config() {
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let adapter_config = AdapterConfig::new(
            "Test HTTP Adapter".to_string(),
            AdapterType::CustomHttp,
            config.clone(),
            owner_id,
            Some("Test HTTP adapter configuration".to_string()),
            vec![],
            None,
        ).unwrap();
        
        assert_eq!(adapter_config.name, "Test HTTP Adapter");
        assert_eq!(adapter_config.adapter_type, AdapterType::CustomHttp);
        assert_eq!(adapter_config.config, config);
        assert_eq!(adapter_config.owner_id, owner_id);
        assert!(adapter_config.enabled);
    }
    
    #[test]
    fn test_update_adapter_config_info() {
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let mut adapter_config = AdapterConfig::new(
            "Test HTTP Adapter".to_string(),
            AdapterType::CustomHttp,
            config,
            owner_id,
            Some("Test HTTP adapter configuration".to_string()),
            vec![],
            None,
        ).unwrap();
        
        let mut new_config = HashMap::new();
        new_config.insert("http_base_url".to_string(), "https://api.newexample.com".to_string());
        new_config.insert("api_key".to_string(), "secret123".to_string());
        
        let transformation_rule = TransformationRule::new(
            "source_field".to_string(),
            "target_field".to_string(),
            TransformationType::Direct,
            HashMap::new(),
        );
        
        let rate_limit_rule = RateLimitRule::new(100, 60, 20);
        
        adapter_config.update_info(
            Some("Updated HTTP Adapter".to_string()),
            Some("Updated description".to_string()),
            Some(new_config.clone()),
            Some(vec![transformation_rule.clone()]),
            Some(Some(rate_limit_rule.clone())),
            Some(false),
        ).unwrap();
        
        assert_eq!(adapter_config.name, "Updated HTTP Adapter");
        assert_eq!(adapter_config.description, Some("Updated description".to_string()));
        assert_eq!(adapter_config.config, new_config);
        assert_eq!(adapter_config.transformation_rules, vec![transformation_rule]);
        assert_eq!(adapter_config.rate_limit_rule, Some(rate_limit_rule));
        assert!(!adapter_config.enabled);
    }
    
    #[test]
    fn test_adapter_config_validation() {
        let owner_id = Uuid::new_v4();
        
        // Test valid SAP configuration
        let mut sap_config = HashMap::new();
        sap_config.insert("sap_host".to_string(), "sap.example.com".to_string());
        sap_config.insert("sap_username".to_string(), "user".to_string());
        sap_config.insert("sap_password".to_string(), "pass".to_string());
        
        let sap_adapter = AdapterConfig::new(
            "SAP Adapter".to_string(),
            AdapterType::SAP,
            sap_config,
            owner_id,
            None,
            vec![],
            None,
        ).unwrap();
        
        assert!(sap_adapter.validate().is_ok());
        
        // Test invalid SAP configuration (missing required fields)
        let invalid_sap_config = HashMap::new();
        let invalid_sap_adapter = AdapterConfig::new(
            "Invalid SAP Adapter".to_string(),
            AdapterType::SAP,
            invalid_sap_config,
            owner_id,
            None,
            vec![],
            None,
        ).unwrap();
        
        assert!(invalid_sap_adapter.validate().is_err());
    }
    
    #[test]
    fn test_transformation_rule() {
        let rule = TransformationRule::new(
            "source_field".to_string(),
            "target_field".to_string(),
            TransformationType::Direct,
            HashMap::new(),
        );
        
        assert_eq!(rule.source_field, "source_field");
        assert_eq!(rule.target_field, "target_field");
        assert_eq!(rule.transformation_type, TransformationType::Direct);
    }
    
    #[test]
    fn test_rate_limit_rule() {
        let rule = RateLimitRule::new(100, 60, 20);
        
        assert_eq!(rule.requests_per_window, 100);
        assert_eq!(rule.window_seconds, 60);
        assert_eq!(rule.burst_limit, 20);
        assert!(rule.enabled);
        
        assert!(!rule.is_exceeded(50));
        assert!(rule.is_exceeded(150));
    }
    
    #[test]
    fn test_add_remove_transformation_rules() {
        let owner_id = Uuid::new_v4();
        let mut config = HashMap::new();
        config.insert("http_base_url".to_string(), "https://api.example.com".to_string());
        
        let mut adapter_config = AdapterConfig::new(
            "Test HTTP Adapter".to_string(),
            AdapterType::CustomHttp,
            config,
            owner_id,
            None,
            vec![],
            None,
        ).unwrap();
        
        let rule1 = TransformationRule::new(
            "field1".to_string(),
            "target1".to_string(),
            TransformationType::Direct,
            HashMap::new(),
        );
        
        let rule2 = TransformationRule::new(
            "field2".to_string(),
            "target2".to_string(),
            TransformationType::Direct,
            HashMap::new(),
        );
        
        adapter_config.add_transformation_rule(rule1.clone());
        adapter_config.add_transformation_rule(rule2.clone());
        
        assert_eq!(adapter_config.transformation_rules.len(), 2);
        
        adapter_config.remove_transformation_rule(rule1.id);
        assert_eq!(adapter_config.transformation_rules.len(), 1);
        assert_eq!(adapter_config.transformation_rules[0].id, rule2.id);
    }
}