//! Schema Registry for managing event schemas and transformations
//!
//! This module provides functionality for:
//! - Storing and retrieving JSON schemas for different event types and versions
//! - Managing transformations between different schema versions
//! - Validating events against their schemas
//! - Handling version compatibility and deprecation

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use semver::Version;
use event_bus::DomainEvent;

/// Represents a JSON schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonSchema {
    /// The schema definition in JSON format
    pub definition: JsonValue,
    /// When this schema was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Whether this schema is deprecated
    pub deprecated: bool,
    /// If deprecated, when it will be removed
    pub deprecated_until: Option<chrono::DateTime<chrono::Utc>>,
}

/// Function signature for transformation functions
pub type TransformationFunction = Arc<dyn Fn(&JsonValue) -> Result<JsonValue, ValidationError> + Send + Sync>;

/// Error types for schema validation and transformation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Schema not found for event type: {0}")]
    SchemaNotFound(String),
    
    #[error("Version not found: {0}")]
    VersionNotFound(String),
    
    #[error("Invalid JSON format: {0}")]
    InvalidJson(#[from] serde_json::Error),
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Transformation failed: {0}")]
    TransformationFailed(String),
    
    #[error("Invalid version format: {0}")]
    InvalidVersion(#[from] semver::Error),
    
    #[error("Schema is deprecated")]
    SchemaDeprecated,
}

/// Schema Registry for managing event schemas and transformations
///
/// The SchemaRegistry is responsible for:
/// - Storing and retrieving JSON schemas for different event types and versions
/// - Managing transformations between different schema versions
/// - Validating events against their schemas
/// - Handling version compatibility and deprecation
/// - Supporting conflict event validation
#[derive(Debug, Clone)]
pub struct SchemaRegistry {
    /// event_type -> version -> schema
    schemas: HashMap<String, HashMap<Version, JsonSchema>>,
    /// event_type -> (from_version, to_version) -> transformation_function
    transformations: HashMap<String, HashMap<(Version, Version), TransformationFunction>>,
}

impl SchemaRegistry {
    /// Create a new SchemaRegistry
    pub fn new() -> Self {
        let mut registry = Self {
            schemas: HashMap::new(),
            transformations: HashMap::new(),
        };
        
        // Register default conflict event schemas
        registry.register_default_conflict_schemas();
        
        registry
    }
    
    /// Register default conflict event schemas
    fn register_default_conflict_schemas(&mut self) {
        // ConflictDetected schema v1.1
        let conflict_detected_schema = JsonSchema {
            definition: serde_json::json!({
                "type": "object",
                "properties": {
                    "document_id": {"type": "string", "format": "uuid"},
                    "conflict": {
                        "type": "object",
                        "properties": {
                            "id": {"type": "string", "format": "uuid"},
                            "document_id": {"type": "string", "format": "uuid"},
                            "conflicting_operations": {"type": "array"},
                            "resolution_strategy": {"type": "string"},
                            "resolved": {"type": "boolean"},
                            "resolved_operations": {"type": "array"},
                            "resolved_at": {"type": "string", "format": "date-time"},
                            "created_at": {"type": "string", "format": "date-time"},
                            "metadata": {
                                "type": "object",
                                "properties": {
                                    "detection_method": {"type": "string"},
                                    "transformation_history": {"type": "array"},
                                    "resolution_details": {"type": "string"}
                                }
                            }
                        }
                    }
                }
            }),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        self.register_schema("ConflictDetected", "1.1.0", conflict_detected_schema);
        
        // ConflictResolved schema v1.0
        let conflict_resolved_schema = JsonSchema {
            definition: serde_json::json!({
                "type": "object",
                "properties": {
                    "document_id": {"type": "string", "format": "uuid"},
                    "conflict_id": {"type": "string", "format": "uuid"},
                    "resolved_operations": {"type": "array"}
                }
            }),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        self.register_schema("ConflictResolved", "1.0.0", conflict_resolved_schema);
        
        // OperationTransformed schema v1.0
        let operation_transformed_schema = JsonSchema {
            definition: serde_json::json!({
                "type": "object",
                "properties": {
                    "original_operation": {"type": "object"},
                    "transformed_operation": {"type": "object"},
                    "transformation_type": {"type": "string"},
                    "timestamp": {"type": "string", "format": "date-time"}
                }
            }),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        self.register_schema("OperationTransformed", "1.0.0", operation_transformed_schema);
        
        // ConflictHistoryEntry schema v1.0
        let conflict_history_schema = JsonSchema {
            definition: serde_json::json!({
                "type": "object",
                "properties": {
                    "conflict_id": {"type": "string", "format": "uuid"},
                    "resolved_at": {"type": "string", "format": "date-time"},
                    "resolution_strategy": {"type": "string"},
                    "involved_users": {
                        "type": "array",
                        "items": {"type": "string", "format": "uuid"}
                    }
                }
            }),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        self.register_schema("ConflictHistoryEntry", "1.0.0", conflict_history_schema);
        
        // MergeResult schema v1.0
        let merge_result_schema = JsonSchema {
            definition: serde_json::json!({
                "type": "object",
                "properties": {
                    "source_branch": {"type": "string"},
                    "target_branch": {"type": "string"},
                    "merged_version": {"type": "integer"},
                    "conflicts_resolved": {"type": "array"}
                }
            }),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        self.register_schema("MergeResult", "1.0.0", merge_result_schema);
    }

    /// Register a schema for a specific event type and version
    pub fn register_schema(&mut self, event_type: &str, version: &str, schema: JsonSchema) {
        let version = Version::parse(version).expect("Valid semver string");
        self.schemas
            .entry(event_type.to_string())
            .or_insert_with(HashMap::new)
            .insert(version, schema);
    }

    /// Get a schema for a specific event type and version
    pub fn get_schema(&self, event_type: &str, version: &str) -> Result<&JsonSchema, ValidationError> {
        let version = Version::parse(version)?;
        self.schemas
            .get(event_type)
            .and_then(|versions| versions.get(&version))
            .ok_or_else(|| ValidationError::SchemaNotFound(format!("{}@{}", event_type, version)))
    }

    /// Register a transformation function between two versions of an event type
    pub fn register_transformation<F>(
        &mut self,
        event_type: &str,
        from_version: &str,
        to_version: &str,
        transformer: F,
    ) where
        F: Fn(&JsonValue) -> Result<JsonValue, ValidationError> + Send + Sync + 'static,
    {
        let from_version = Version::parse(from_version).expect("Valid semver string");
        let to_version = Version::parse(to_version).expect("Valid semver string");
        
        self.transformations
            .entry(event_type.to_string())
            .or_insert_with(HashMap::new)
            .insert((from_version, to_version), Arc::new(transformer));
    }

    /// Get a transformation function between two versions of an event type
    pub fn get_transformer(
        &self,
        event_type: &str,
        from_version: &str,
        to_version: &str,
    ) -> Result<TransformationFunction, ValidationError> {
        let from_version = Version::parse(from_version)?;
        let to_version = Version::parse(to_version)?;
        
        self.transformations
            .get(event_type)
            .and_then(|transforms| transforms.get(&(from_version, to_version)))
            .cloned()
            .ok_or_else(|| ValidationError::VersionNotFound(format!("{}: {} -> {}", event_type, from_version, to_version)))
    }

    /// Validate an event against its schema
    pub fn validate(&self, event: &DomainEvent) -> Result<(), ValidationError> {
        // Get the schema for this event type
        // For now, we'll use a simple versioning scheme from the event_type
        // In a real implementation, you might want to extract version from event metadata
        let schema = self.get_schema(&event.event_type, "1.0.0")?;
        
        // Check if schema is deprecated
        if schema.deprecated {
            return Err(ValidationError::SchemaDeprecated);
        }
        
        // In a real implementation, you would use a JSON schema validation library
        // For now, we'll just do a basic check that the payload is valid JSON
        let _ = serde_json::to_string(&event.payload)?;
        
        Ok(())
    }

    /// Transform an event from one version to another
    pub fn transform(
        &self,
        event: &DomainEvent,
        target_version: &str,
    ) -> Result<JsonValue, ValidationError> {
        // For simplicity, we assume the source version is 1.0.0
        // In a real implementation, you'd extract this from the event
        let transformer = self.get_transformer(&event.event_type, "1.0.0", target_version)?;
        transformer(&event.payload)
    }

    /// Check if a schema version is deprecated
    pub fn is_deprecated(&self, event_type: &str, version: &str) -> Result<bool, ValidationError> {
        let schema = self.get_schema(event_type, version)?;
        Ok(schema.deprecated)
    }

    /// List all available versions for an event type
    pub fn list_versions(&self, event_type: &str) -> Vec<Version> {
        if let Some(versions) = self.schemas.get(event_type) {
            let mut version_list: Vec<Version> = versions.keys().cloned().collect();
            version_list.sort();
            version_list
        } else {
            Vec::new()
        }
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use uuid::Uuid;
    use event_bus::EventSource;

    #[test]
    fn test_schema_registry_creation() {
        let registry = SchemaRegistry::new();
        assert_eq!(registry.schemas.len(), 0);
        assert_eq!(registry.transformations.len(), 0);
    }

    #[test]
    fn test_register_and_get_schema() {
        let mut registry = SchemaRegistry::new();
        
        let schema = JsonSchema {
            definition: json!({"type": "object"}),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        
        registry.register_schema("test_event", "1.0.0", schema.clone());
        
        let retrieved = registry.get_schema("test_event", "1.0.0").unwrap();
        assert_eq!(retrieved.definition, schema.definition);
    }

    #[test]
    fn test_register_and_get_transformation() {
        let mut registry = SchemaRegistry::new();
        
        // Register a simple transformation function
        registry.register_transformation("test_event", "1.0.0", "2.0.0", |payload| {
            Ok(json!({
                "transformed": true,
                "original": payload
            }))
        });
        
        let transformer = registry.get_transformer("test_event", "1.0.0", "2.0.0").unwrap();
        let result = transformer(&json!({"test": "data"})).unwrap();
        
        assert_eq!(result["transformed"], true);
    }

    #[test]
    fn test_validate_event() {
        let mut registry = SchemaRegistry::new();
        
        let schema = JsonSchema {
            definition: json!({"type": "object"}),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        
        registry.register_schema("TestEvent", "1.0.0", schema);
        
        let event = DomainEvent::new(
            "test".to_string(),
            "TestEvent".to_string(),
            json!({"valid": "data"}),
            EventSource::Local,
        );
        
        assert!(registry.validate(&event).is_ok());
    }
    

    #[test]
    fn test_deprecated_schema_validation() {
        let mut registry = SchemaRegistry::new();
        
        let schema = JsonSchema {
            definition: json!({"type": "object"}),
            created_at: chrono::Utc::now(),
            deprecated: true,
            deprecated_until: Some(chrono::Utc::now() + chrono::Duration::days(30)),
        };
        
        registry.register_schema("TestEvent", "1.0.0", schema);
        
        let event = DomainEvent::new(
            "test".to_string(),
            "TestEvent".to_string(),
            json!({"valid": "data"}),
            EventSource::Local,
        );
        
        assert!(matches!(registry.validate(&event), Err(ValidationError::SchemaDeprecated)));
    }

    #[test]
    fn test_list_versions() {
        let mut registry = SchemaRegistry::new();
        
        let schema = JsonSchema {
            definition: json!({"type": "object"}),
            created_at: chrono::Utc::now(),
            deprecated: false,
            deprecated_until: None,
        };
        
        registry.register_schema("test_event", "1.0.0", schema.clone());
        registry.register_schema("test_event", "2.0.0", schema.clone());
        registry.register_schema("test_event", "1.5.0", schema);
        
        let versions = registry.list_versions("test_event");
        assert_eq!(versions.len(), 3);
        assert_eq!(versions[0].to_string(), "1.0.0");
        assert_eq!(versions[1].to_string(), "1.5.0");
        assert_eq!(versions[2].to_string(), "2.0.0");
    }
}