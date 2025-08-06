//! Adapter registry system for Shtairir Core

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{
    ShtairirResult, ShtairirError, 
    AppIntegration, AppIntegrationExt, CommandDefinition, DataSchema, Event, 
    ExecutionContext, CommandResult, HealthCheckResult,
};

/// Registry for managing app integrations
pub struct AdapterRegistry {
    /// Registered app integrations
    apps: RwLock<HashMap<String, Arc<dyn AppIntegrationExt>>>,
    /// Command registry (app_name -> command_name -> command_def)
    commands: RwLock<HashMap<String, HashMap<String, CommandDefinition>>>,
    /// Schema registry (app_name -> schema_name -> schema)
    schemas: RwLock<HashMap<String, HashMap<String, DataSchema>>>,
    /// Registry metadata
    metadata: RwLock<HashMap<String, ShtairirValue>>,
    /// Registry creation time
    created_at: DateTime<Utc>,
}

impl AdapterRegistry {
    /// Create a new adapter registry
    pub fn new() -> Self {
        Self {
            apps: RwLock::new(HashMap::new()),
            commands: RwLock::new(HashMap::new()),
            schemas: RwLock::new(HashMap::new()),
            metadata: RwLock::new(HashMap::new()),
            created_at: Utc::now(),
        }
    }
    
    /// Register an app integration
    pub async fn register_app(&self, app: Arc<dyn AppIntegrationExt>) -> ShtairirResult<()> {
        let app_name = app.app_name().to_string();
        
        // Initialize the app
        app.initialize().await?;
        
        // Register the app
        {
            let mut apps = self.apps.write().await;
            if apps.contains_key(&app_name) {
                return Err(ShtairirError::Registry(format!(
                    "App '{}' is already registered", app_name
                )));
            }
            apps.insert(app_name.clone(), app.clone());
        }
        
        // Register the app's commands
        let commands = app.get_commands().await?;
        {
            let mut command_registry = self.commands.write().await;
            let mut app_commands = HashMap::new();
            
            for command in commands {
                app_commands.insert(command.name.clone(), command);
            }
            
            command_registry.insert(app_name.clone(), app_commands);
        }
        
        // Register the app's schemas
        let schemas = app.get_schemas().await?;
        {
            let mut schema_registry = self.schemas.write().await;
            let mut app_schemas = HashMap::new();
            
            for schema in schemas {
                app_schemas.insert(schema.name.clone(), schema);
            }
            
            schema_registry.insert(app_name, app_schemas);
        }
        
        Ok(())
    }
    
    /// Unregister an app integration
    pub async fn unregister_app(&self, app_name: &str) -> ShtairirResult<()> {
        // Get the app
        let app = {
            let apps = self.apps.read().await;
            apps.get(app_name).cloned()
                .ok_or_else(|| ShtairirError::Registry(format!(
                    "App '{}' is not registered", app_name
                )))?
        };
        
        // Shutdown the app
        app.shutdown().await?;
        
        // Remove the app
        {
            let mut apps = self.apps.write().await;
            apps.remove(app_name);
        }
        
        // Remove commands
        {
            let mut command_registry = self.commands.write().await;
            command_registry.remove(app_name);
        }
        
        // Remove schemas
        {
            let mut schema_registry = self.schemas.write().await;
            schema_registry.remove(app_name);
        }
        
        Ok(())
    }
    
    /// Get a registered app
    pub async fn get_app(&self, app_name: &str) -> ShtairirResult<Arc<dyn AppIntegrationExt>> {
        let apps = self.apps.read().await;
        apps.get(app_name).cloned()
            .ok_or_else(|| ShtairirError::Registry(format!(
                "App '{}' is not registered", app_name
            )))
    }
    
    /// Get all registered app names
    pub async fn get_app_names(&self) -> Vec<String> {
        let apps = self.apps.read().await;
        apps.keys().cloned().collect()
    }
    
    /// Check if an app is registered
    pub async fn is_app_registered(&self, app_name: &str) -> bool {
        let apps = self.apps.read().await;
        apps.contains_key(app_name)
    }
    
    /// Get a command definition
    pub async fn get_command(&self, app_name: &str, command_name: &str) -> ShtairirResult<CommandDefinition> {
        let command_registry = self.commands.read().await;
        let app_commands = command_registry.get(app_name)
            .ok_or_else(|| ShtairirError::Registry(format!(
                "App '{}' is not registered", app_name
            )))?;
        
        app_commands.get(command_name).cloned()
            .ok_or_else(|| ShtairirError::Registry(format!(
                "Command '{}' not found in app '{}'", command_name, app_name
            )))
    }
    
    /// Get all commands for an app
    pub async fn get_app_commands(&self, app_name: &str) -> ShtairirResult<Vec<CommandDefinition>> {
        let command_registry = self.commands.read().await;
        let app_commands = command_registry.get(app_name)
            .ok_or_else(|| ShtairirError::Registry(format!(
                "App '{}' is not registered", app_name
            )))?;
        
        Ok(app_commands.values().cloned().collect())
    }
    
    /// Get all commands from all apps
    pub async fn get_all_commands(&self) -> Vec<CommandDefinition> {
        let command_registry = self.commands.read().await;
        let mut all_commands = Vec::new();
        
        for app_commands in command_registry.values() {
            all_commands.extend(app_commands.values().cloned());
        }
        
        all_commands
    }
    
    /// Get a schema definition
    pub async fn get_schema(&self, app_name: &str, schema_name: &str) -> ShtairirResult<DataSchema> {
        let schema_registry = self.schemas.read().await;
        let app_schemas = schema_registry.get(app_name)
            .ok_or_else(|| ShtairirError::Registry(format!(
                "App '{}' is not registered", app_name
            )))?;
        
        app_schemas.get(schema_name).cloned()
            .ok_or_else(|| ShtairirError::Registry(format!(
                "Schema '{}' not found in app '{}'", schema_name, app_name
            )))
    }
    
    /// Get all schemas for an app
    pub async fn get_app_schemas(&self, app_name: &str) -> ShtairirResult<Vec<DataSchema>> {
        let schema_registry = self.schemas.read().await;
        let app_schemas = schema_registry.get(app_name)
            .ok_or_else(|| ShtairirError::Registry(format!(
                "App '{}' is not registered", app_name
            )))?;
        
        Ok(app_schemas.values().cloned().collect())
    }
    
    /// Get all schemas from all apps
    pub async fn get_all_schemas(&self) -> Vec<DataSchema> {
        let schema_registry = self.schemas.read().await;
        let mut all_schemas = Vec::new();
        
        for app_schemas in schema_registry.values() {
            all_schemas.extend(app_schemas.values().cloned());
        }
        
        all_schemas
    }
    
    /// Execute a command on an app
    pub async fn execute_command(
        &self,
        app_name: &str,
        command_name: &str,
        args: HashMap<String, ShtairirValue>,
        context: Option<ExecutionContext>,
    ) -> ShtairirResult<CommandResult> {
        let start_time = std::time::Instant::now();
        
        // Get the app
        let app = self.get_app(app_name).await?;
        
        // Get the command definition
        let command_def = self.get_command(app_name, command_name).await?;
        
        // Validate arguments
        self.validate_command_args(&command_def, &args)?;
        
        // Execute the command
        let result = app.execute_command(command_name, args).await;
        
        let execution_time_ms = start_time.elapsed().as_millis() as u64;
        
        match result {
            Ok(data) => Ok(CommandResult::success(data, execution_time_ms)),
            Err(error) => Ok(CommandResult::failure(error.to_string(), execution_time_ms)),
        }
    }
    
    /// Validate command arguments against the command definition
    fn validate_command_args(
        &self,
        command_def: &CommandDefinition,
        args: &HashMap<String, ShtairirValue>,
    ) -> ShtairirResult<()> {
        for param in &command_def.parameters {
            if param.required && !args.contains_key(&param.name) {
                return Err(ShtairirError::Validation(format!(
                    "Required parameter '{}' is missing", param.name
                )));
            }
            
            if let Some(arg_value) = args.get(&param.name) {
                // Type checking would be implemented here
                // For now, we'll skip detailed type checking
            }
        }
        
        Ok(())
    }
    
    /// Deliver an event to an app
    pub async fn deliver_event(&self, app_name: &str, event: &Event) -> ShtairirResult<()> {
        let app = self.get_app(app_name).await?;
        app.handle_event(event).await
    }
    
    /// Deliver an event to all apps that can handle it
    pub async fn deliver_event_to_all(&self, event: &Event) -> ShtairirResult<Vec<String>> {
        let apps = self.apps.read().await;
        let mut delivered_to = Vec::new();
        
        for (app_name, app) in apps.iter() {
            if app.can_handle_event(&event.event_type).await.unwrap_or(false) {
                if let Err(e) = app.handle_event(event).await {
                    tracing::warn!("Failed to deliver event to {}: {}", app_name, e);
                } else {
                    delivered_to.push(app_name.clone());
                }
            }
        }
        
        Ok(delivered_to)
    }
    
    /// Perform health checks on all registered apps
    pub async fn health_check_all(&self) -> HashMap<String, HealthCheckResult> {
        let apps = self.apps.read().await;
        let mut results = HashMap::new();
        
        for (app_name, app) in apps.iter() {
            match app.health_check().await {
                Ok(result) => {
                    results.insert(app_name.clone(), result);
                }
                Err(e) => {
                    results.insert(app_name.clone(), HealthCheckResult {
                        app_name: app_name.clone(),
                        status: crate::HealthStatus::Unhealthy(e.to_string()),
                        timestamp: Utc::now(),
                        metrics: HashMap::new(),
                    });
                }
            }
        }
        
        results
    }
    
    /// Get registry metadata
    pub async fn get_metadata(&self) -> HashMap<String, ShtairirValue> {
        let metadata = self.metadata.read().await;
        metadata.clone()
    }
    
    /// Set registry metadata
    pub async fn set_metadata(&self, key: String, value: ShtairirValue) {
        let mut metadata = self.metadata.write().await;
        metadata.insert(key, value);
    }
    
    /// Get registry statistics
    pub async fn get_stats(&self) -> RegistryStats {
        let apps = self.apps.read().await;
        let commands = self.commands.read().await;
        let schemas = self.schemas.read().await;
        
        let total_commands: usize = commands.values().map(|cmds| cmds.len()).sum();
        let total_schemas: usize = schemas.values().map(|schs| schs.len()).sum();
        
        RegistryStats {
            total_apps: apps.len(),
            total_commands,
            total_schemas,
            created_at: self.created_at,
        }
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStats {
    /// Total number of registered apps
    pub total_apps: usize,
    /// Total number of registered commands
    pub total_commands: usize,
    /// Total number of registered schemas
    pub total_schemas: usize,
    /// Registry creation time
    pub created_at: DateTime<Utc>,
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Adapter registry builder
pub struct AdapterRegistryBuilder {
    registry: AdapterRegistry,
}

impl AdapterRegistryBuilder {
    /// Create a new registry builder
    pub fn new() -> Self {
        Self {
            registry: AdapterRegistry::new(),
        }
    }
    
    /// Register an app with the registry
    pub async fn with_app(mut self, app: Arc<dyn AppIntegrationExt>) -> ShtairirResult<Self> {
        self.registry.register_app(app).await?;
        Ok(self)
    }
    
    /// Set metadata for the registry
    pub async fn with_metadata(mut self, key: String, value: ShtairirValue) -> Self {
        self.registry.set_metadata(key, value).await;
        self
    }
    
    /// Build the registry
    pub fn build(self) -> AdapterRegistry {
        self.registry
    }
}

impl Default for AdapterRegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}