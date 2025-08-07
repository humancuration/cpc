//! Execution context and adapter system for Shtairir
//! 
//! This module defines the execution context that provides the environment
//! in which blocks execute, as well as the adapter system for connecting blocks.

use crate::block::{BlockId, Value, Type, PortId};
use shtairir_registry::model::Registry;
use std::collections::HashMap;
use std::sync::Arc;
use shtairir_core::error::ShtairirError;

/// Execution context for block execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Unique identifier for this execution
    pub execution_id: String,
    
    /// Registry for looking up blocks and graphs
    pub registry: Arc<Registry>,
    
    /// Event system for cross-block communication
    // TODO: Implement event system
    // pub event_system: Arc<dyn EventSystem>,
    
    /// Configuration manager
    // TODO: Implement config manager
    // pub config: Arc<dyn ConfigManager>,
    
    /// Type system
    // TODO: Implement type system
    // pub type_system: Arc<TypeSystem>,
    
    /// Memory manager
    // TODO: Implement memory manager
    // pub memory_manager: Arc<MemoryManager>,
    
    /// Caching system
    // TODO: Implement cache system
    // pub cache: Arc<dyn CacheSystem>,
    
    /// Security context
    // TODO: Implement security context
    // pub security_context: SecurityContext,
    
    /// Execution metadata
    pub metadata: HashMap<String, Value>,
}

impl ExecutionContext {
    /// Create a new execution context
    pub fn new(execution_id: String, registry: Arc<Registry>) -> Self {
        Self {
            execution_id,
            registry,
            metadata: HashMap::new(),
        }
    }
    
    /// Add metadata to the context
    pub fn with_metadata(mut self, key: String, value: Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Adapter system for connecting blocks
pub trait BlockAdapter: Send + Sync {
    /// Adapt output from one block to input of another
    fn adapt(&self, output: &Value, target_type: &Type) -> Result<Value, AdapterError>;
    
    /// Get the adapter's specification
    // TODO: Define adapter specification
    // fn spec(&self) -> &AdapterSpec;
    
    /// Validate the adapter configuration
    // TODO: Define adapter configuration
    // fn validate(&self, config: &AdapterConfig) -> Result<(), AdapterError>;
}

/// Built-in adapters
pub enum BuiltInAdapter {
    /// Type conversion adapter
    TypeConversion(TypeConversionAdapter),
    /// Data transformation adapter
    Transform(TransformAdapter),
    /// Filtering adapter
    Filter(FilterAdapter),
    /// Aggregation adapter
    Aggregate(AggregateAdapter),
}

/// Type conversion adapter
pub struct TypeConversionAdapter {
    // TODO: Implement type conversion adapter
}

impl BlockAdapter for TypeConversionAdapter {
    fn adapt(&self, output: &Value, target_type: &Type) -> Result<Value, AdapterError> {
        // TODO: Implement type conversion logic
        todo!("Implement type conversion logic")
    }
}

/// Data transformation adapter
pub struct TransformAdapter {
    // TODO: Implement transform adapter
}

impl BlockAdapter for TransformAdapter {
    fn adapt(&self, output: &Value, target_type: &Type) -> Result<Value, AdapterError> {
        // TODO: Implement transformation logic
        todo!("Implement transformation logic")
    }
}

/// Filtering adapter
pub struct FilterAdapter {
    // TODO: Implement filter adapter
}

impl BlockAdapter for FilterAdapter {
    fn adapt(&self, output: &Value, target_type: &Type) -> Result<Value, AdapterError> {
        // TODO: Implement filtering logic
        todo!("Implement filtering logic")
    }
}

/// Aggregation adapter
pub struct AggregateAdapter {
    // TODO: Implement aggregation adapter
}

impl BlockAdapter for AggregateAdapter {
    fn adapt(&self, output: &Value, target_type: &Type) -> Result<Value, AdapterError> {
        // TODO: Implement aggregation logic
        todo!("Implement aggregation logic")
    }
}

/// Adapter error
#[derive(Debug, Clone)]
pub struct AdapterError {
    /// Error message
    pub message: String,
    
    /// Error details
    pub details: Option<Value>,
}

impl AdapterError {
    /// Create a new adapter error
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: None,
        }
    }
    
    /// Create a new adapter error with details
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Security context
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// User ID (if any)
    pub user_id: Option<String>,
    
    /// Permissions
    pub permissions: Vec<String>,
    
    /// Security policies
    pub policies: HashMap<String, Value>,
}

impl SecurityContext {
    /// Create a new security context
    pub fn new() -> Self {
        Self {
            user_id: None,
            permissions: vec![],
            policies: HashMap::new(),
        }
    }
    
    /// Set user ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    /// Add permission
    pub fn with_permission(mut self, permission: String) -> Self {
        self.permissions.push(permission);
        self
    }
    
    /// Add policy
    pub fn with_policy(mut self, name: String, policy: Value) -> Self {
        self.policies.insert(name, policy);
        self
    }
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_execution_context_creation() {
        let registry = Arc::new(Registry::new());
        let context = ExecutionContext::new("test-execution".to_string(), registry.clone());
        
        assert_eq!(context.execution_id, "test-execution");
        assert!(Arc::ptr_eq(&context.registry, &registry));
    }
    
    #[test]
    fn test_security_context_creation() {
        let security_context = SecurityContext::new()
            .with_user_id("user123".to_string())
            .with_permission("read".to_string())
            .with_policy("max_memory".to_string(), Value::i64(1024));
        
        assert_eq!(security_context.user_id, Some("user123".to_string()));
        assert!(security_context.permissions.contains(&"read".to_string()));
        assert_eq!(security_context.policies.get("max_memory"), Some(&Value::i64(1024)));
    }
}