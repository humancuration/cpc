# Shtairir Core Infrastructure

The Shtairir Core infrastructure provides the foundation for integrating Shtairir scripting capabilities across the CPC platform. It enables cross-app communication, command execution, and type-safe data exchange between applications.

## Overview

Shtairir Core implements a modular architecture with the following key components:

1. **Adapter Registry System** - Manages registration and discovery of app integrations
2. **Common Abstractions** - Defines traits and structures for app integration
3. **Type Mapping System** - Provides type safety and validation for data exchange
4. **Error Handling Framework** - Centralized error management
5. **Event Bus** - Redis-based cross-app communication system

## Components

### Adapter Registry (`registry.rs`)

The adapter registry manages the registration and discovery of app integrations. It provides:

- App registration and unregistration
- Command discovery and execution
- Schema management
- Health check monitoring
- Event delivery

#### Example Usage

```rust
use shtairir_core::{AdapterRegistry, AppIntegrationExt, ShtairirValue};
use std::collections::HashMap;
use std::sync::Arc;

// Create a registry
let registry = AdapterRegistry::new();

// Register an app
let app = Arc::new(MyAppIntegration::new());
registry.register_app(app).await?;

// Execute a command
let mut args = HashMap::new();
args.insert("param1".to_string(), ShtairirValue::String("value".to_string()));
let result = registry.execute_command("my_app", "my_command", args, None).await?;
```

### Common Abstractions (`abstractions.rs`)

The common abstractions define the traits and structures that apps must implement to integrate with Shtairir:

- `AppIntegration` - Base trait for app integration
- `AppIntegrationExt` - Extended trait with health checks
- `EventSystem` - Trait for event publishing and subscription
- `ConfigManager` - Trait for configuration management
- `CommandDefinition` - Structure for defining commands
- `DataSchema` - Structure for defining data schemas

#### Example Implementation

```rust
use shtairir_core::{
    AppIntegrationExt, CommandDefinition, ParameterDefinition, 
    DataSchema, FieldDefinition, ShtairirType, ShtairirValue,
};
use async_trait::async_trait;
use std::collections::HashMap;

struct MyAppIntegration {
    name: String,
    version: String,
}

#[async_trait]
impl AppIntegration for MyAppIntegration {
    fn app_name(&self) -> &str {
        &self.name
    }
    
    fn app_version(&self) -> &str {
        &self.version
    }
    
    async fn initialize(&self) -> ShtairirResult<()> {
        // Initialize your app
        Ok(())
    }
    
    async fn get_commands(&self) -> ShtairirResult<Vec<CommandDefinition>> {
        Ok(vec![
            CommandDefinition {
                name: "my_command".to_string(),
                app: self.name.clone(),
                description: "My command description".to_string(),
                parameters: vec![
                    ParameterDefinition {
                        name: "param1".to_string(),
                        param_type: ShtairirType::String,
                        required: true,
                        default_value: None,
                        description: Some("Parameter 1".to_string()),
                    }
                ],
                return_type: ShtairirType::String,
                category: "general".to_string(),
                is_async: false,
                version: "1.0.0".to_string(),
                example: Some("my_command(param1=\"value\")".to_string()),
            }
        ])
    }
    
    async fn execute_command(&self, command: &str, args: HashMap<String, ShtairirValue>) -> ShtairirResult<ShtairirValue> {
        match command {
            "my_command" => {
                // Execute your command
                Ok(ShtairirValue::String("Command executed".to_string()))
            }
            _ => Err(ShtairirError::Adapter(format!("Unknown command: {}", command))),
        }
    }
    
    // Implement other required methods...
}

#[async_trait]
impl AppIntegrationExt for MyAppIntegration {
    async fn health_check(&self) -> ShtairirResult<HealthCheckResult> {
        Ok(HealthCheckResult::new(self.name.clone(), HealthStatus::Healthy))
    }
    
    // Implement other extended methods...
}
```

### Type Mapping System (`types.rs`)

The type mapping system provides type safety and validation for data exchange between apps:

- `ShtairirValue` - Enhanced value types with metadata
- `ShtairirType` - Type information for validation
- `Schema` - Schema definitions for complex types
- `TypeRegistry` - Registry for managing schemas and type definitions
- `ShtairirTypeConvert` - Trait for type conversion

#### Example Usage

```rust
use shtairir_core::{TypeRegistry, Schema, FieldDefinition, ShtairirType, ShtairirValue, ValidationRule};

// Create a type registry
let mut registry = TypeRegistry::new();

// Define a schema
let mut fields = HashMap::new();
fields.insert("name".to_string(), FieldDefinition {
    name: "name".to_string(),
    field_type: ShtairirType::String,
    required: true,
    default_value: None,
    description: Some("User name".to_string()),
    validation: Some(vec![
        ValidationRule::MinLength(1),
        ValidationRule::MaxLength(100),
    ]),
});

let schema = Schema {
    name: "user".to_string(),
    version: "1.0.0".to_string(),
    fields,
    inherits: None,
};

// Register the schema
registry.register_schema(schema)?;

// Validate data
let user_data = ShtairirValue::Object({
    let mut obj = HashMap::new();
    obj.insert("name".to_string(), ShtairirValue::String("John Doe".to_string()));
    obj
});

registry.validate_against_schema(&user_data, "user")?;
```

### Error Handling Framework (`error.rs`)

The error handling framework provides centralized error management:

- `ShtairirError` - Comprehensive error types
- `ShtairirResult` - Result type for Shtairir operations
- Automatic error conversion from common types

#### Example Usage

```rust
use shtairir_core::{ShtairirResult, ShtairirError};

fn my_function() -> ShtairirResult<String> {
    // Return an error
    Err(ShtairirError::Validation("Invalid input".to_string()))
}

// Automatic conversion from other error types
fn my_other_function() -> ShtairirResult<String> {
    let json_str = "{ invalid json }";
    let value: serde_json::Value = serde_json::from_str(json_str)?; // Automatically converts to ShtairirError
    Ok(value.to_string())
}
```

### Event Bus (`event_bus.rs`)

The event bus provides Redis-based cross-app communication:

- `RedisEventBus` - Redis-backed event bus for production
- `InMemoryEventBus` - In-memory event bus for testing/development
- `EventBusFactory` - Factory for creating event bus instances
- Event persistence and history
- Event filtering and subscription

#### Example Usage

```rust
use shtairir_core::{
    EventBusFactory, EventBusConfig, Event, EventHandler, 
    ExampleEventHandler, ShtairirValue,
};
use std::sync::Arc;

// Create an event bus
let config = EventBusConfig {
    redis_url: "redis://localhost:6379".to_string(),
    ..Default::default()
};
let event_bus = EventBusFactory::create_event_bus(config).await?;

// Create and subscribe an event handler
let handler = Arc::new(ExampleEventHandler::new("my_handler".to_string()));
event_bus.subscribe("my_event_type", handler).await?;

// Publish an event
let event = Event::new(
    "my_event_type".to_string(),
    "my_app".to_string(),
    ShtairirValue::String("Event data".to_string()),
);
event_bus.publish(event).await?;

// Get event history
let filter = EventFilter {
    event_types: vec!["my_event_type".to_string()],
    ..Default::default()
};
let history = event_bus.get_event_history(filter).await?;
```

## Architecture Principles

The Shtairir Core infrastructure follows CPC's architectural principles:

### Hexagonal Architecture

- Core business logic is independent of external concerns
- Dependencies point inward
- External interfaces are implemented as adapters
- Easy to test and extend

### Screaming Architecture

- The code structure reflects the business domain
- Clear separation of concerns
- Domain-first organization
- Self-documenting structure

### Vertical Slices

- Features are organized vertically
- Each component handles all aspects of its domain
- Minimized coupling between components
- High cohesion within components

## Integration with Existing Shtairir Implementation

The Shtairir Core infrastructure is designed to work with the existing Shtairir implementation:

1. The `AppIntegration` trait extends the existing `AppAdapter` concept
2. `ShtairirValue` is an enhanced version of the existing `Value` type
3. The registry system provides a more robust way to manage adapters
4. The event bus enables cross-app communication beyond simple command execution

## Contributing

When contributing to Shtairir Core, please:

1. Follow the established architectural patterns
2. Ensure comprehensive test coverage
3. Update documentation for new features
4. Adhere to the Rust API Guidelines
5. Maintain backwards compatibility where possible

## License

Shtairir Core is part of the CPC platform and follows the CPC license terms.