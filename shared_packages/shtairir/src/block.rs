//! Core building block architecture for Shtairir
//! 
//! This module defines the fundamental block interface and related traits
//! that form the foundation of the Shtairir visual programming system.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use shtairir_core::error::ShtairirError;
use shtairir_registry::model::{BlockSpec, Determinism, Purity};
use shtairir_registry::value::Value;
use shtairir_registry::types::Type;

/// Unique identifier for a block
pub type BlockId = String;

/// Unique identifier for a port
pub type PortId = String;

/// Unique identifier for an execution
pub type ExecutionId = String;

/// Result type for block execution operations
pub type BlockResult<T> = Result<T, ShtairirError>;

/// Core trait that all blocks must implement
#[async_trait]
pub trait Block: Send + Sync {
    /// Get the block specification
    fn spec(&self) -> &BlockSpec;
    
    /// Execute the block with given inputs
    async fn execute(&self, inputs: &BlockInputs, context: &ExecutionContext) -> BlockResult<BlockOutputs>;
    
    /// Validate the block configuration
    fn validate(&self, params: &BlockParams) -> Result<(), ValidationError>;
    
    /// Get the block's purity (pure/effect)
    fn purity(&self) -> Purity;
    
    /// Get the block's determinism (deterministic/non-deterministic)
    fn determinism(&self) -> Determinism;
}

/// Trait for blocks that can be compiled
pub trait CompilableBlock: Block {
    /// Compile the block to executable form
    fn compile(&self, compilation_context: &CompilationContext) -> Result<CompiledBlock, CompilationError>;
    
    /// Get the compiled form if available
    fn get_compiled(&self) -> Option<&CompiledBlock>;
}

/// Trait for blocks that can be introspected
pub trait IntrospectableBlock: Block {
    /// Get the block's schema
    fn schema(&self) -> &BlockSchema;
    
    /// Get the block's metadata
    fn metadata(&self) -> &BlockMetadata;
    
    /// Get the block's documentation
    fn documentation(&self) -> &BlockDocumentation;
}

/// Block inputs container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInputs {
    /// Input values by port name
    pub values: HashMap<String, Value>,
}

impl BlockInputs {
    /// Create new block inputs
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Add an input value
    pub fn with_input(mut self, name: String, value: Value) -> Self {
        self.values.insert(name, value);
        self
    }
    
    /// Get an input value by name
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
    
    /// Check if an input exists
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}

impl Default for BlockInputs {
    fn default() -> Self {
        Self::new()
    }
}

/// Block outputs container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockOutputs {
    /// Output values by port name
    pub values: HashMap<String, Value>,
}

impl BlockOutputs {
    /// Create new block outputs
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Add an output value
    pub fn with_output(mut self, name: String, value: Value) -> Self {
        self.values.insert(name, value);
        self
    }
    
    /// Get an output value by name
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

impl Default for BlockOutputs {
    fn default() -> Self {
        Self::new()
    }
}

/// Block parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockParams {
    /// Parameter values by name
    pub values: HashMap<String, Value>,
}

impl BlockParams {
    /// Create new block parameters
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Add a parameter value
    pub fn with_param(mut self, name: String, value: Value) -> Self {
        self.values.insert(name, value);
        self
    }
    
    /// Get a parameter value by name
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.values.get(name)
    }
}

impl Default for BlockParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution context for block execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Unique identifier for this execution
    pub execution_id: ExecutionId,
    
    /// Registry for looking up blocks and graphs
    pub registry: Arc<shtairir_registry::model::Registry>,
    
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
    pub fn new(execution_id: ExecutionId, registry: Arc<shtairir_registry::model::Registry>) -> Self {
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

/// Validation error for blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Error message
    pub message: String,
    
    /// Error details
    pub details: Option<Value>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: None,
        }
    }
    
    /// Create a new validation error with details
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Compilation context for blocks
#[derive(Debug, Clone)]
pub struct CompilationContext {
    /// Target platform
    pub target: String,
    
    /// Optimization level
    pub optimization_level: u8,
    
    /// Compilation flags
    pub flags: HashMap<String, Value>,
}

impl CompilationContext {
    /// Create a new compilation context
    pub fn new(target: String) -> Self {
        Self {
            target,
            optimization_level: 0,
            flags: HashMap::new(),
        }
    }
    
    /// Set optimization level
    pub fn with_optimization_level(mut self, level: u8) -> Self {
        self.optimization_level = level;
        self
    }
    
    /// Add a compilation flag
    pub fn with_flag(mut self, name: String, value: Value) -> Self {
        self.flags.insert(name, value);
        self
    }
}

/// Compiled block representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledBlock {
    /// Compiled code or representation
    pub code: Vec<u8>,
    
    /// Entry point for the compiled block
    pub entry_point: String,
    
    /// Dependencies
    pub dependencies: Vec<String>,
    
    /// Metadata about the compilation
    pub metadata: HashMap<String, Value>,
}

/// Compilation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationError {
    /// Error message
    pub message: String,
    
    /// Error details
    pub details: Option<Value>,
}

impl CompilationError {
    /// Create a new compilation error
    pub fn new(message: String) -> Self {
        Self {
            message,
            details: None,
        }
    }
    
    /// Create a new compilation error with details
    pub fn with_details(mut self, details: Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Block schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSchema {
    /// Input ports
    pub inputs: Vec<InputPort>,
    
    /// Output ports
    pub outputs: Vec<OutputPort>,
    
    /// Parameters
    pub params: Vec<ParamSpec>,
}

/// Block metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockMetadata {
    /// Block category
    pub category: String,
    
    /// Block tags
    pub tags: Vec<String>,
    
    /// Block version
    pub version: String,
    
    /// Additional metadata
    pub additional: HashMap<String, Value>,
}

/// Block documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockDocumentation {
    /// Block description
    pub description: String,
    
    /// Usage examples
    pub examples: Vec<String>,
    
    /// Documentation URL
    pub url: Option<String>,
}

/// Input port specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputPort {
    /// Unique identifier for the port
    pub id: PortId,
    
    /// Port name
    pub name: String,
    
    /// Port type
    pub ty: Type,
    
    /// Port kind (value, stream, event)
    pub kind: PortKind,
    
    /// Default value
    pub default: Option<Value>,
    
    /// Whether the port is required
    pub required: bool,
    
    /// Port description
    pub description: Option<String>,
}

/// Output port specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputPort {
    /// Unique identifier for the port
    pub id: PortId,
    
    /// Port name
    pub name: String,
    
    /// Port type
    pub ty: Type,
    
    /// Port kind (value, stream, event)
    pub kind: PortKind,
    
    /// Port description
    pub description: Option<String>,
}

/// Port kind
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PortKind {
    /// Value port (synchronous data)
    Value,
    /// Stream port (asynchronous data stream)
    Stream,
    /// Event port (event notifications)
    Event,
    /// Composite port (structured data)
    Composite,
}

/// Parameter specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSpec {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub ty: Type,
    
    /// Default value
    pub default: Option<Value>,
    
    /// Whether the parameter is required
    pub required: bool,
    
    /// Parameter description
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use shtairir_registry::model::BlockSpec;
    
    // Mock block implementation for testing
    struct MockBlock {
        spec: BlockSpec,
    }
    
    #[async_trait]
    impl Block for MockBlock {
        fn spec(&self) -> &BlockSpec {
            &self.spec
        }
        
        async fn execute(&self, _inputs: &BlockInputs, _context: &ExecutionContext) -> BlockResult<BlockOutputs> {
            Ok(BlockOutputs::new())
        }
        
        fn validate(&self, _params: &BlockParams) -> Result<(), ValidationError> {
            Ok(())
        }
        
        fn purity(&self) -> Purity {
            Purity::Pure
        }
        
        fn determinism(&self) -> Determinism {
            Determinism::Deterministic
        }
    }
    
    #[test]
    fn test_block_inputs() {
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::i64(1))
            .with_input("b".to_string(), Value::i64(2));
        
        assert_eq!(inputs.get("a"), Some(&Value::i64(1)));
        assert_eq!(inputs.get("b"), Some(&Value::i64(2)));
        assert!(inputs.contains("a"));
        assert!(inputs.contains("b"));
        assert!(!inputs.contains("c"));
    }
    
    #[test]
    fn test_block_outputs() {
        let outputs = BlockOutputs::new()
            .with_output("result".to_string(), Value::i64(3));
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(3)));
    }
    
    #[test]
    fn test_block_params() {
        let params = BlockParams::new()
            .with_param("bias".to_string(), Value::i64(10));
        
        assert_eq!(params.get("bias"), Some(&Value::i64(10)));
    }
}