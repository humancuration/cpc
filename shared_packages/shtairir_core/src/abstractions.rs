//! Common abstractions for Shtairir Core

use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{ShtairirResult, ShtairirError, ShtairirValue, ShtairirType};

/// Command definition for Shtairir scripts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandDefinition {
    /// Command name
    pub name: String,
    /// App that provides this command
    pub app: String,
    /// Command description
    pub description: String,
    /// Parameter definitions
    pub parameters: Vec<ParameterDefinition>,
    /// Return type definition
    pub return_type: ShtairirType,
    /// Command category
    pub category: String,
    /// Whether this command is async
    pub is_async: bool,
    /// Command version
    pub version: String,
    /// Example usage
    pub example: Option<String>,
}

/// Parameter definition for commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterDefinition {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ShtairirType,
    /// Whether the parameter is required
    pub required: bool,
    /// Default value
    pub default_value: Option<ShtairirValue>,
    /// Parameter description
    pub description: Option<String>,
}

/// Data schema definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSchema {
    /// Schema name
    pub name: String,
    /// Schema version
    pub version: String,
    /// Field definitions
    pub fields: HashMap<String, FieldDefinition>,
    /// Schema description
    pub description: Option<String>,
    /// Schema metadata
    pub metadata: HashMap<String, ShtairirValue>,
}

/// Event data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event ID
    pub id: Uuid,
    /// Event type
    pub event_type: String,
    /// Event source (app name)
    pub source: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event data
    pub data: ShtairirValue,
    /// Event metadata
    pub metadata: HashMap<String, ShtairirValue>,
}

impl Event {
    /// Create a new event
    pub fn new(event_type: String, source: String, data: ShtairirValue) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            source,
            timestamp: Utc::now(),
            data,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the event
    pub fn with_metadata(mut self, key: String, value: ShtairirValue) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// App integration trait for connecting apps to Shtairir
#[async_trait]
pub trait AppIntegration: Send + Sync {
    /// Get the app name
    fn app_name(&self) -> &str;
    
    /// Get the app version
    fn app_version(&self) -> &str;
    
    /// Initialize the app integration
    async fn initialize(&self) -> ShtairirResult<()>;
    
    /// Shutdown the app integration
    async fn shutdown(&self) -> ShtairirResult<()>;
    
    /// Get available commands from this app
    async fn get_commands(&self) -> ShtairirResult<Vec<CommandDefinition>>;
    
    /// Execute a command
    async fn execute_command(&self, command: &str, args: HashMap<String, ShtairirValue>) -> ShtairirResult<ShtairirValue>;
    
    /// Handle an event
    async fn handle_event(&self, event: &Event) -> ShtairirResult<()>;
    
    /// Get data schemas provided by this app
    async fn get_schemas(&self) -> ShtairirResult<Vec<DataSchema>>;
    
    /// Validate data against a schema
    async fn validate_data(&self, schema_name: &str, data: &ShtairirValue) -> ShtairirResult<bool>;
}

/// Event system trait for cross-app communication
#[async_trait]
pub trait EventSystem: Send + Sync {
    /// Publish an event
    async fn publish(&self, event: Event) -> ShtairirResult<()>;
    
    /// Subscribe to events of a specific type
    async fn subscribe(&self, event_type: &str, handler: Arc<dyn EventHandler>) -> ShtairirResult<()>;
    
    /// Unsubscribe from events
    async fn unsubscribe(&self, event_type: &str, handler_id: &str) -> ShtairirResult<()>;
    
    /// Get event history
    async fn get_event_history(&self, filter: EventFilter) -> ShtairirResult<Vec<Event>>;
}

/// Event handler trait
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle(&self, event: &Event) -> ShtairirResult<()>;
    
    /// Get handler ID
    fn handler_id(&self) -> &str;
}

/// Event filter for querying event history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    /// Event types to include (empty means all)
    pub event_types: Vec<String>,
    /// Sources to include (empty means all)
    pub sources: Vec<String>,
    /// Time range start
    pub start_time: Option<DateTime<Utc>>,
    /// Time range end
    pub end_time: Option<DateTime<Utc>>,
    /// Maximum number of events to return
    pub limit: Option<usize>,
    /// Filter by data content
    pub data_filter: Option<HashMap<String, ShtairirValue>>,
}

impl Default for EventFilter {
    fn default() -> Self {
        Self {
            event_types: Vec::new(),
            sources: Vec::new(),
            start_time: None,
            end_time: None,
            limit: None,
            data_filter: None,
        }
    }
}

/// Configuration manager trait
#[async_trait]
pub trait ConfigManager: Send + Sync {
    /// Get a configuration value
    async fn get_config(&self, key: &str) -> ShtairirResult<Option<ShtairirValue>>;
    
    /// Set a configuration value
    async fn set_config(&self, key: &str, value: ShtairirValue) -> ShtairirResult<()>;
    
    /// Get all configuration values for an app
    async fn get_app_config(&self, app_name: &str) -> ShtairirResult<HashMap<String, ShtairirValue>>;
    
    /// Set multiple configuration values for an app
    async fn set_app_config(&self, app_name: &str, config: HashMap<String, ShtairirValue>) -> ShtairirResult<()>;
    
    /// Watch for configuration changes
    async fn watch_config(&self, key_pattern: &str, handler: Arc<dyn ConfigChangeHandler>) -> ShtairirResult<()>;
    
    /// Remove a configuration watcher
    async fn unwatch_config(&self, key_pattern: &str, handler_id: &str) -> ShtairirResult<()>;
}

/// Configuration change handler trait
#[async_trait]
pub trait ConfigChangeHandler: Send + Sync {
    /// Handle a configuration change
    async fn handle_change(&self, key: &str, old_value: Option<&ShtairirValue>, new_value: Option<&ShtairirValue>) -> ShtairirResult<()>;
    
    /// Get handler ID
    fn handler_id(&self) -> &str;
}

/// Context for command execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Execution ID
    pub execution_id: Uuid,
    /// App executing the command
    pub app_name: String,
    /// User ID (if any)
    pub user_id: Option<String>,
    /// Session ID (if any)
    pub session_id: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Execution metadata
    pub metadata: HashMap<String, ShtairirValue>,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(app_name: String) -> Self {
        Self {
            execution_id: Uuid::new_v4(),
            app_name,
            user_id: None,
            session_id: None,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the context
    pub fn with_metadata(mut self, key: String, value: ShtairirValue) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Set user ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    /// Set session ID
    pub fn with_session_id(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }
}

/// Result of command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    /// Whether the command was successful
    pub success: bool,
    /// Result data
    pub data: Option<ShtairirValue>,
    /// Error message (if any)
    pub error: Option<String>,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
    /// Additional metadata
    pub metadata: HashMap<String, ShtairirValue>,
}

impl CommandResult {
    /// Create a successful result
    pub fn success(data: ShtairirValue, execution_time_ms: u64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            execution_time_ms,
            metadata: HashMap::new(),
        }
    }
    
    /// Create a failed result
    pub fn failure(error: String, execution_time_ms: u64) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            execution_time_ms,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the result
    pub fn with_metadata(mut self, key: String, value: ShtairirValue) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Health status of an app integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    /// The app is healthy
    Healthy,
    /// The app is degraded but still functional
    Degraded(String),
    /// The app is unhealthy
    Unhealthy(String),
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// App name
    pub app_name: String,
    /// Health status
    pub status: HealthStatus,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Additional health metrics
    pub metrics: HashMap<String, ShtairirValue>,
}

impl HealthCheckResult {
    /// Create a new health check result
    pub fn new(app_name: String, status: HealthStatus) -> Self {
        Self {
            app_name,
            status,
            timestamp: Utc::now(),
            metrics: HashMap::new(),
        }
    }
    
    /// Add a metric to the health check
    pub fn with_metric(mut self, key: String, value: ShtairirValue) -> Self {
        self.metrics.insert(key, value);
        self
    }
}

/// Extended app integration trait with health checks
#[async_trait]
pub trait AppIntegrationExt: AppIntegration {
    /// Perform a health check
    async fn health_check(&self) -> ShtairirResult<HealthCheckResult>;
    
    /// Get app capabilities
    async fn get_capabilities(&self) -> ShtairirResult<Vec<String>>;
    
    /// Get app dependencies
    async fn get_dependencies(&self) -> ShtairirResult<Vec<String>>;
    
    /// Check if the app can handle a specific event type
    async fn can_handle_event(&self, event_type: &str) -> ShtairirResult<bool>;
}