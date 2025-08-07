//! Memory-efficient data structures for Shtairir
//! 
//! This module provides memory-efficient containers for block inputs,
//! outputs, and other data structures used in the Shtairir system.

use crate::block::{Value, BlockInputs, BlockOutputs};
use shtairir_registry::types::Type;
use std::collections::HashMap;
use std::sync::Arc;

/// Memory-efficient block input container
#[derive(Debug, Clone)]
pub struct MemoryEfficientBlockInputs {
    /// Input values
    values: HashMap<String, Value>,
    
    /// Memory layout
    memory_layout: MemoryLayout,
    
    /// Memory allocator
    // allocator: Arc<dyn MemoryAllocator>,
}

impl MemoryEfficientBlockInputs {
    /// Create new memory-efficient block inputs
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            memory_layout: MemoryLayout::default(),
            // allocator,
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
    
    /// Get all input names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }
    
    /// Get the number of inputs
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Check if there are no inputs
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for MemoryEfficientBlockInputs {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BlockInputs> for MemoryEfficientBlockInputs {
    fn from(inputs: BlockInputs) -> Self {
        Self {
            values: inputs.values,
            memory_layout: MemoryLayout::default(),
        }
    }
}

impl From<MemoryEfficientBlockInputs> for BlockInputs {
    fn from(inputs: MemoryEfficientBlockInputs) -> Self {
        Self {
            values: inputs.values,
        }
    }
}

/// Memory-efficient block output container
#[derive(Debug, Clone)]
pub struct MemoryEfficientBlockOutputs {
    /// Output values
    values: HashMap<String, Value>,
    
    /// Memory layout
    memory_layout: MemoryLayout,
    
    /// Memory allocator
    // allocator: Arc<dyn MemoryAllocator>,
}

impl MemoryEfficientBlockOutputs {
    /// Create new memory-efficient block outputs
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            memory_layout: MemoryLayout::default(),
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
    
    /// Get all output names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.values.keys()
    }
    
    /// Get the number of outputs
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Check if there are no outputs
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for MemoryEfficientBlockOutputs {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BlockOutputs> for MemoryEfficientBlockOutputs {
    fn from(outputs: BlockOutputs) -> Self {
        Self {
            values: outputs.values,
            memory_layout: MemoryLayout::default(),
        }
    }
}

impl From<MemoryEfficientBlockOutputs> for BlockOutputs {
    fn from(outputs: MemoryEfficientBlockOutputs) -> Self {
        Self {
            values: outputs.values,
        }
    }
}

/// Memory layout for efficient storage
#[derive(Debug, Clone)]
pub struct MemoryLayout {
    /// Layout specification
    spec: MemoryLayoutSpec,
    
    /// Memory alignment
    alignment: usize,
    
    /// Memory padding
    padding: usize,
    
    /// Memory optimization flags
    optimization_flags: MemoryOptimizationFlags,
}

impl Default for MemoryLayout {
    fn default() -> Self {
        Self {
            spec: MemoryLayoutSpec::Compact,
            alignment: 8, // Default to 8-byte alignment
            padding: 0,
            optimization_flags: MemoryOptimizationFlags::default(),
        }
    }
}

/// Memory layout specification
#[derive(Debug, Clone)]
pub enum MemoryLayoutSpec {
    /// Compact layout (minimal memory usage)
    Compact,
    
    /// Cache-friendly layout (optimized for cache access)
    CacheFriendly,
    
    /// SIMD-friendly layout (optimized for SIMD operations)
    SimdFriendly,
    
    /// Custom layout
    Custom {
        layout: Box<dyn CustomMemoryLayout>,
    },
}

/// Custom memory layout trait
pub trait CustomMemoryLayout: Send + Sync {
    /// Get the layout specification
    fn spec(&self) -> &CustomLayoutSpec;
    
    /// Calculate memory requirements
    fn memory_requirements(&self, data_size: usize) -> MemoryRequirements;
}

/// Custom layout specification
#[derive(Debug, Clone)]
pub struct CustomLayoutSpec {
    /// Layout name
    pub name: String,
    
    /// Layout description
    pub description: String,
    
    /// Layout parameters
    pub parameters: HashMap<String, Value>,
}

/// Memory requirements
#[derive(Debug, Clone)]
pub struct MemoryRequirements {
    /// Required memory size in bytes
    pub size: usize,
    
    /// Required memory alignment
    pub alignment: usize,
    
    /// Additional memory flags
    pub flags: MemoryFlags,
}

/// Memory flags
#[derive(Debug, Clone)]
pub struct MemoryFlags {
    /// Whether the memory should be zero-initialized
    pub zero_initialized: bool,
    
    /// Whether the memory should be executable
    pub executable: bool,
    
    /// Whether the memory should be readable
    pub readable: bool,
    
    /// Whether the memory should be writable
    pub writable: bool,
}

impl Default for MemoryFlags {
    fn default() -> Self {
        Self {
            zero_initialized: true,
            executable: false,
            readable: true,
            writable: true,
        }
    }
}

/// Memory optimization flags
#[derive(Debug, Clone)]
pub struct MemoryOptimizationFlags {
    /// Whether to use memory pooling
    pub use_pooling: bool,
    
    /// Whether to use memory compression
    pub use_compression: bool,
    
    /// Whether to use memory sharing
    pub use_sharing: bool,
    
    /// Whether to use memory prefetching
    pub use_prefetching: bool,
}

impl Default for MemoryOptimizationFlags {
    fn default() -> Self {
        Self {
            use_pooling: true,
            use_compression: false,
            use_sharing: true,
            use_prefetching: false,
        }
    }
}

/// Memory-efficient value container
#[derive(Debug, Clone)]
pub struct MemoryEfficientValue {
    /// The actual value
    value: Value,
    
    /// Memory layout
    layout: MemoryLayout,
    
    /// Reference count for sharing
    ref_count: usize,
}

impl MemoryEfficientValue {
    /// Create a new memory-efficient value
    pub fn new(value: Value) -> Self {
        Self {
            value,
            layout: MemoryLayout::default(),
            ref_count: 1,
        }
    }
    
    /// Get the underlying value
    pub fn value(&self) -> &Value {
        &self.value
    }
    
    /// Get a mutable reference to the value
    pub fn value_mut(&mut self) -> &mut Value {
        &mut self.value
    }
    
    /// Convert to the underlying value
    pub fn into_value(self) -> Value {
        self.value
    }
}

impl From<Value> for MemoryEfficientValue {
    fn from(value: Value) -> Self {
        Self::new(value)
    }
}

impl From<MemoryEfficientValue> for Value {
    fn from(value: MemoryEfficientValue) -> Self {
        value.value
    }
}

impl std::ops::Deref for MemoryEfficientValue {
    type Target = Value;
    
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::ops::DerefMut for MemoryEfficientValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

/// Memory-efficient type container
#[derive(Debug, Clone)]
pub struct MemoryEfficientType {
    /// The actual type
    ty: Type,
    
    /// Memory layout
    layout: MemoryLayout,
    
    /// Size in bytes (if known)
    size: Option<usize>,
}

impl MemoryEfficientType {
    /// Create a new memory-efficient type
    pub fn new(ty: Type) -> Self {
        Self {
            ty,
            layout: MemoryLayout::default(),
            size: None, // Size calculation would be complex and type-dependent
        }
    }
    
    /// Get the underlying type
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    
    /// Get a mutable reference to the type
    pub fn ty_mut(&mut self) -> &mut Type {
        &mut self.ty
    }
    
    /// Convert to the underlying type
    pub fn into_type(self) -> Type {
        self.ty
    }
}

impl From<Type> for MemoryEfficientType {
    fn from(ty: Type) -> Self {
        Self::new(ty)
    }
}

impl From<MemoryEfficientType> for Type {
    fn from(ty: MemoryEfficientType) -> Self {
        ty.ty
    }
}

impl std::ops::Deref for MemoryEfficientType {
    type Target = Type;
    
    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl std::ops::DerefMut for MemoryEfficientType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shtairir_registry::types::ScalarType;
    
    #[test]
    fn test_memory_efficient_block_inputs() {
        let inputs = MemoryEfficientBlockInputs::new()
            .with_input("a".to_string(), Value::i64(1))
            .with_input("b".to_string(), Value::i64(2));
        
        assert_eq!(inputs.get("a"), Some(&Value::i64(1)));
        assert_eq!(inputs.get("b"), Some(&Value::i64(2)));
        assert!(inputs.contains("a"));
        assert!(inputs.contains("b"));
        assert!(!inputs.contains("c"));
        assert_eq!(inputs.len(), 2);
        assert!(!inputs.is_empty());
    }
    
    #[test]
    fn test_memory_efficient_block_outputs() {
        let outputs = MemoryEfficientBlockOutputs::new()
            .with_output("result".to_string(), Value::i64(3));
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(3)));
        assert_eq!(outputs.len(), 1);
        assert!(!outputs.is_empty());
    }
    
    #[test]
    fn test_memory_layout_default() {
        let layout = MemoryLayout::default();
        
        match layout.spec {
            MemoryLayoutSpec::Compact => {}, // Test passes
            _ => panic!("Expected Compact layout spec"),
        }
        assert_eq!(layout.alignment, 8);
        assert_eq!(layout.padding, 0);
    }
    
    #[test]
    fn test_memory_flags_default() {
        let flags = MemoryFlags::default();
        
        assert_eq!(flags.zero_initialized, true);
        assert_eq!(flags.executable, false);
        assert_eq!(flags.readable, true);
        assert_eq!(flags.writable, true);
    }
    
    #[test]
    fn test_memory_optimization_flags_default() {
        let flags = MemoryOptimizationFlags::default();
        
        assert_eq!(flags.use_pooling, true);
        assert_eq!(flags.use_compression, false);
        assert_eq!(flags.use_sharing, true);
        assert_eq!(flags.use_prefetching, false);
    }
    
    #[test]
    fn test_memory_efficient_value() {
        let value = MemoryEfficientValue::new(Value::string("test"));
        
        assert_eq!(value.value(), &Value::string("test"));
        assert_eq!(*value, Value::string("test"));
    }
    
    #[test]
    fn test_memory_efficient_type() {
        let ty = MemoryEfficientType::new(Type::Scalar(ScalarType::I64));
        
        assert_eq!(ty.ty(), &Type::Scalar(ScalarType::I64));
        assert_eq!(*ty, Type::Scalar(ScalarType::I64));
    }
}