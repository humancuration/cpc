//! Example app integration for Shtairir Core

use shtairir_core::{
    AppIntegrationExt, CommandDefinition, ParameterDefinition, 
    DataSchema, FieldDefinition, ShtairirType, ShtairirValue, ShtairirResult, ShtairirError,
    Event, ExecutionContext, CommandResult, HealthCheckResult, HealthStatus,
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Example app integration
pub struct ExampleApp {
    name: String,
    version: String,
    initialized: bool,
}

impl ExampleApp {
    /// Create a new example app
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            initialized: false,
        }
    }
}

#[async_trait]
impl AppIntegration for ExampleApp {
    fn app_name(&self) -> &str {
        &self.name
    }
    
    fn app_version(&self) -> &str {
        &self.version
    }
    
    async fn initialize(&self) -> ShtairirResult<()> {
        println!("Initializing {} app version {}", self.name, self.version);
        // In a real app, you would initialize resources here
        Ok(())
    }
    
    async fn shutdown(&self) -> ShtairirResult<()> {
        println!("Shutting down {} app", self.name);
        // In a real app, you would clean up resources here
        Ok(())
    }
    
    async fn get_commands(&self) -> ShtairirResult<Vec<CommandDefinition>> {
        Ok(vec![
            CommandDefinition {
                name: "greet".to_string(),
                app: self.name.clone(),
                description: "Greet someone".to_string(),
                parameters: vec![
                    ParameterDefinition {
                        name: "name".to_string(),
                        param_type: ShtairirType::String,
                        required: true,
                        default_value: None,
                        description: Some("Name to greet".to_string()),
                    },
                    ParameterDefinition {
                        name: "enthusiastic".to_string(),
                        param_type: ShtairirType::Boolean,
                        required: false,
                        default_value: Some(ShtairirValue::Boolean(false)),
                        description: Some("Whether to be enthusiastic".to_string()),
                    }
                ],
                return_type: ShtairirType::String,
                category: "greetings".to_string(),
                is_async: false,
                version: "1.0.0".to_string(),
                example: Some("greet(name=\"World\", enthusiastic=true)".to_string()),
            },
            CommandDefinition {
                name: "calculate".to_string(),
                app: self.name.clone(),
                description: "Perform a calculation".to_string(),
                parameters: vec![
                    ParameterDefinition {
                        name: "a".to_string(),
                        param_type: ShtairirType::Number,
                        required: true,
                        default_value: None,
                        description: Some("First number".to_string()),
                    },
                    ParameterDefinition {
                        name: "b".to_string(),
                        param_type: ShtairirType::Number,
                        required: true,
                        default_value: None,
                        description: Some("Second number".to_string()),
                    },
                    ParameterDefinition {
                        name: "operation".to_string(),
                        param_type: ShtairirType::String,
                        required: true,
                        default_value: None,
                        description: Some("Operation to perform (add, subtract, multiply, divide)".to_string()),
                    }
                ],
                return_type: ShtairirType::Number,
                category: "math".to_string(),
                is_async: false,
                version: "1.0.0".to_string(),
                example: Some("calculate(a=10, b=5, operation=\"add\")".to_string()),
            }
        ])
    }
    
    async fn execute_command(&self, command: &str, args: HashMap<String, ShtairirValue>) -> ShtairirResult<ShtairirValue> {
        match command {
            "greet" => {
                let name = args.get("name")
                    .ok_or_else(|| ShtairirError::Validation("Missing required parameter: name".to_string()))?;
                let name = match name {
                    ShtairirValue::String(s) => s,
                    _ => return Err(ShtairirError::Validation("Parameter 'name' must be a string".to_string())),
                };
                
                let enthusiastic = args.get("enthusiastic")
                    .map(|v| match v {
                        ShtairirValue::Boolean(b) => *b,
                        _ => false,
                    })
                    .unwrap_or(false);
                
                let greeting = if enthusiastic {
                    format!("Hello, {}! Great to see you!", name)
                } else {
                    format!("Hello, {}.", name)
                };
                
                Ok(ShtairirValue::String(greeting))
            },
            "calculate" => {
                let a = args.get("a")
                    .ok_or_else(|| ShtairirError::Validation("Missing required parameter: a".to_string()))?;
                let a = match a {
                    ShtairirValue::Number(n) => *n,
                    _ => return Err(ShtairirError::Validation("Parameter 'a' must be a number".to_string())),
                };
                
                let b = args.get("b")
                    .ok_or_else(|| ShtairirError::Validation("Missing required parameter: b".to_string()))?;
                let b = match b {
                    ShtairirValue::Number(n) => *n,
                    _ => return Err(ShtairirError::Validation("Parameter 'b' must be a number".to_string())),
                };
                
                let operation = args.get("operation")
                    .ok_or_else(|| ShtairirError::Validation("Missing required parameter: operation".to_string()))?;
                let operation = match operation {
                    ShtairirValue::String(s) => s.as_str(),
                    _ => return Err(ShtairirError::Validation("Parameter 'operation' must be a string".to_string())),
                };
                
                let result = match operation {
                    "add" => a + b,
                    "subtract" => a - b,
                    "multiply" => a * b,
                    "divide" => {
                        if b == 0.0 {
                            return Err(ShtairirError::Validation("Cannot divide by zero".to_string()));
                        }
                        a / b
                    },
                    _ => return Err(ShtairirError::Validation(format!("Unknown operation: {}", operation))),
                };
                
                Ok(ShtairirValue::Number(result))
            },
            _ => Err(ShtairirError::Adapter(format!("Unknown command: {}", command))),
        }
    }
    
    async fn handle_event(&self, event: &Event) -> ShtairirResult<()> {
        println!("App {} received event: {} from {}", 
                 self.name, event.event_type, event.source);
        
        // Handle specific event types
        match event.event_type.as_str() {
            "system.startup" => {
                println!("System startup event received");
            },
            "user.login" => {
                println!("User login event received");
            },
            _ => {
                println!("Unhandled event type: {}", event.event_type);
            }
        }
        
        Ok(())
    }
    
    async fn get_schemas(&self) -> ShtairirResult<Vec<DataSchema>> {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), FieldDefinition {
            name: "id".to_string(),
            field_type: ShtairirType::Uuid,
            required: true,
            default_value: None,
            description: Some("Unique identifier".to_string()),
            validation: None,
        });
        fields.insert("name".to_string(), FieldDefinition {
            name: "name".to_string(),
            field_type: ShtairirType::String,
            required: true,
            default_value: None,
            description: Some("Name".to_string()),
            validation: Some(vec![
                shtairir_core::ValidationRule::MinLength(1),
                shtairir_core::ValidationRule::MaxLength(100),
            ]),
        });
        fields.insert("email".to_string(), FieldDefinition {
            name: "email".to_string(),
            field_type: ShtairirType::String,
            required: true,
            default_value: None,
            description: Some("Email address".to_string()),
            validation: Some(vec![
                shtairir_core::ValidationRule::Pattern(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$".to_string()),
            ]),
        });
        
        let user_schema = DataSchema {
            name: "user".to_string(),
            version: "1.0.0".to_string(),
            fields,
            description: Some("User schema".to_string()),
            metadata: HashMap::new(),
        };
        
        Ok(vec![user_schema])
    }
    
    async fn validate_data(&self, schema_name: &str, data: &ShtairirValue) -> ShtairirResult<bool> {
        match schema_name {
            "user" => {
                // Simple validation for user data
                if let ShtairirValue::Object(obj) = data {
                    let has_id = obj.contains_key("id");
                    let has_name = obj.contains_key("name");
                    let has_email = obj.contains_key("email");
                    Ok(has_id && has_name && has_email)
                } else {
                    Ok(false)
                }
            },
            _ => Err(ShtairirError::Validation(format!("Unknown schema: {}", schema_name))),
        }
    }
}

#[async_trait]
impl AppIntegrationExt for ExampleApp {
    async fn health_check(&self) -> ShtairirResult<HealthCheckResult> {
        // In a real app, you would check actual health metrics
        let mut metrics = HashMap::new();
        metrics.insert("uptime_seconds".to_string(), ShtairirValue::Number(3600.0));
        metrics.insert("memory_usage_mb".to_string(), ShtairirValue::Number(64.0));
        metrics.insert("active_connections".to_string(), ShtairirValue::Number(42.0));
        
        Ok(HealthCheckResult::new(
            self.name.clone(),
            HealthStatus::Healthy,
        ).with_metric("version".to_string(), ShtairirValue::String(self.version.clone())))
    }
    
    async fn get_capabilities(&self) -> ShtairirResult<Vec<String>> {
        Ok(vec![
            "greetings".to_string(),
            "math".to_string(),
            "event_handling".to_string(),
        ])
    }
    
    async fn get_dependencies(&self) -> ShtairirResult<Vec<String>> {
        Ok(vec![
            "redis".to_string(),
            "database".to_string(),
        ])
    }
    
    async fn can_handle_event(&self, event_type: &str) -> ShtairirResult<bool> {
        Ok(matches!(event_type, "system.startup" | "user.login" | "data.changed"))
    }
}

#[tokio::main]
async fn main() -> ShtairirResult<()> {
    use shtairir_core::{AdapterRegistry, Event, EventBusFactory, EventBusConfig};
    
    // Create a registry
    let registry = AdapterRegistry::new();
    
    // Create and register the example app
    let app = Arc::new(ExampleApp::new("example_app".to_string(), "1.0.0".to_string()));
    registry.register_app(app).await?;
    
    // List registered apps
    let app_names = registry.get_app_names().await;
    println!("Registered apps: {:?}", app_names);
    
    // Get app commands
    let commands = registry.get_app_commands("example_app").await?;
    println!("Available commands:");
    for command in commands {
        println!("- {}: {}", command.name, command.description);
    }
    
    // Execute a command
    let mut args = HashMap::new();
    args.insert("name".to_string(), ShtairirValue::String("World".to_string()));
    args.insert("enthusiastic".to_string(), ShtairirValue::Boolean(true));
    
    let result = registry.execute_command("example_app", "greet", args, None).await?;
    println!("Command result: {:?}", result);
    
    // Execute another command
    let mut args = HashMap::new();
    args.insert("a".to_string(), ShtairirValue::Number(10.0));
    args.insert("b".to_string(), ShtairirValue::Number(5.0));
    args.insert("operation".to_string(), ShtairirValue::String("multiply".to_string()));
    
    let result = registry.execute_command("example_app", "calculate", args, None).await?;
    println!("Calculation result: {:?}", result);
    
    // Create an event bus
    let config = EventBusConfig {
        redis_url: "memory".to_string(), // Use in-memory for this example
        ..Default::default()
    };
    let event_bus = EventBusFactory::create_event_bus(config).await?;
    
    // Publish an event
    let event = Event::new(
        "system.startup".to_string(),
        "system".to_string(),
        ShtairirValue::String("System starting up".to_string()),
    );
    event_bus.publish(event).await?;
    
    // Get registry stats
    let stats = registry.get_stats().await;
    println!("Registry stats: {:?}", stats);
    
    // Perform health checks
    let health_results = registry.health_check_all().await;
    for (app_name, health) in health_results {
        println!("Health check for {}: {:?}", app_name, health.status);
    }
    
    Ok(())
}