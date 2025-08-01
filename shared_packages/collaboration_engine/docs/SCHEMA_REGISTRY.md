# Schema Registry

The Schema Registry is a component of the Collaboration Engine that manages JSON schemas for events and provides versioning capabilities.

## Features

- **Schema Storage**: Store and retrieve JSON schemas for different event types and versions
- **Version Management**: Handle semantic versioning of schemas using semver
- **Transformation Functions**: Convert events between different schema versions
- **Event Validation**: Validate events against their schemas
- **Deprecation Handling**: Mark schemas as deprecated and manage their lifecycle

## Usage

### Creating a Schema Registry

```rust
use collaboration_engine::schema_registry::SchemaRegistry;

let mut registry = SchemaRegistry::new();
```

### Registering Schemas

```rust
use collaboration_engine::schema_registry::{JsonSchema, ValidationError};
use serde_json::json;
use chrono::Utc;

let schema = JsonSchema {
    definition: json!({
        "type": "object",
        "properties": {
            "message": {"type": "string"}
        }
    }),
    created_at: Utc::now(),
    deprecated: false,
    deprecated_until: None,
};

registry.register_schema("UserMessage", "1.0.0", schema);
```

### Validating Events

```rust
use event_bus::DomainEvent;
use event_bus::EventSource;
use serde_json::json;

let event = DomainEvent::new(
    "collaboration".to_string(),
    "UserMessage".to_string(),
    json!({"message": "Hello, World!"}),
    EventSource::Local,
);

match registry.validate(&event) {
    Ok(()) => println!("Event is valid"),
    Err(e) => println!("Validation error: {}", e),
}
```

### Registering Transformation Functions

```rust
registry.register_transformation("UserMessage", "1.0.0", "2.0.0", |payload| {
    // Transform payload from version 1.0.0 to 2.0.0
    let message = payload["message"].as_str().unwrap_or("");
    Ok(json!({
        "content": message,
        "timestamp": Utc::now().to_rfc3339()
    }))
});
```

### Transforming Events

```rust
match registry.transform(&event, "2.0.0") {
    Ok(transformed) => println!("Transformed event: {}", transformed),
    Err(e) => println!("Transformation error: {}", e),
}
```

## API Reference

### `SchemaRegistry`

The main struct for managing schemas and transformations.

#### Methods

- `new()` - Create a new SchemaRegistry
- `register_schema(event_type, version, schema)` - Register a schema
- `get_schema(event_type, version)` - Retrieve a schema
- `register_transformation(event_type, from_version, to_version, transformer)` - Register a transformation function
- `get_transformer(event_type, from_version, to_version)` - Get a transformation function
- `validate(event)` - Validate an event against its schema
- `transform(event, target_version)` - Transform an event to a target version
- `is_deprecated(event_type, version)` - Check if a schema version is deprecated
- `list_versions(event_type)` - List all versions for an event type

### `JsonSchema`

Represents a JSON schema with metadata.

#### Fields

- `definition` - The schema definition in JSON format
- `created_at` - When this schema was created
- `deprecated` - Whether this schema is deprecated
- `deprecated_until` - If deprecated, when it will be removed

### `ValidationError`

Error types for schema validation and transformation.

#### Variants

- `SchemaNotFound` - Schema not found for event type
- `VersionNotFound` - Version not found
- `InvalidJson` - Invalid JSON format
- `ValidationFailed` - Validation failed
- `TransformationFailed` - Transformation failed
- `InvalidVersion` - Invalid version format
- `SchemaDeprecated` - Schema is deprecated