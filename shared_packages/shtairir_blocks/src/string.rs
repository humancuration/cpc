//! String manipulation operations for Shtairir standard library
//!
//! This module provides operations for manipulating strings, including concatenation,
//! splitting, trimming, and formatting. Unicode-aware implementations.

use async_trait::async_trait;
use shtairir::block::{Block, BlockInputs, BlockOutputs, BlockParams, ExecutionContext, ValidationError};
use shtairir_core::error::{ShtairirError, ShtairirResult};
use shtairir_registry::model::{BlockSpec, Determinism, Purity};
use shtairir_registry::value::Value;
use std::collections::BTreeMap;

/// Concat block implementation
pub struct ConcatBlock {
    spec: BlockSpec,
}

impl ConcatBlock {
    /// Create a new ConcatBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for ConcatBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let strings = inputs.get("strings").ok_or_else(|| ShtairirError::Execution("Missing input 'strings'".to_string()))?;
        let separator = inputs.get("separator").unwrap_or(&Value::Null);

        let string_list = match strings {
            Value::List(list) => list,
            _ => return Err(ShtairirError::Execution("Input 'strings' must be a list".to_string())),
        };

        let sep = match separator {
            Value::String(s) => s.as_str(),
            Value::Null => "",
            _ => return Err(ShtairirError::Execution("Input 'separator' must be a string or null".to_string())),
        };

        let mut result_parts = Vec::new();
        for (i, value) in string_list.iter().enumerate() {
            match value {
                Value::String(s) => {
                    if i > 0 && !sep.is_empty() {
                        result_parts.push(sep);
                    }
                    result_parts.push(s.as_str());
                }
                _ => return Err(ShtairirError::Execution("All elements in 'strings' must be strings".to_string())),
            }
        }

        let result = Value::String(result_parts.concat());

        Ok(BlockOutputs::new().with_output("result".to_string(), result))
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

/// Split block implementation
pub struct SplitBlock {
    spec: BlockSpec,
}

impl SplitBlock {
    /// Create a new SplitBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for SplitBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let text = inputs.get("text").ok_or_else(|| ShtairirError::Execution("Missing input 'text'".to_string()))?;
        let delimiter = inputs.get("delimiter").ok_or_else(|| ShtairirError::Execution("Missing input 'delimiter'".to_string()))?;

        let text_str = match text {
            Value::String(s) => s,
            _ => return Err(ShtairirError::Execution("Input 'text' must be a string".to_string())),
        };

        let delimiter_str = match delimiter {
            Value::String(s) => s,
            _ => return Err(ShtairirError::Execution("Input 'delimiter' must be a string".to_string())),
        };

        let parts: Vec<Value> = text_str
            .split(delimiter_str.as_str())
            .map(|s| Value::String(s.to_string()))
            .collect();

        let result = Value::List(parts);

        Ok(BlockOutputs::new().with_output("parts".to_string(), result))
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

/// Trim block implementation
pub struct TrimBlock {
    spec: BlockSpec,
}

impl TrimBlock {
    /// Create a new TrimBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for TrimBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let text = inputs.get("text").ok_or_else(|| ShtairirError::Execution("Missing input 'text'".to_string()))?;

        let text_str = match text {
            Value::String(s) => s,
            _ => return Err(ShtairirError::Execution("Input 'text' must be a string".to_string())),
        };

        let result = Value::String(text_str.trim().to_string());

        Ok(BlockOutputs::new().with_output("result".to_string(), result))
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

/// Format block implementation
pub struct FormatBlock {
    spec: BlockSpec,
}

impl FormatBlock {
    /// Create a new FormatBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for FormatBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let template = inputs.get("template").ok_or_else(|| ShtairirError::Execution("Missing input 'template'".to_string()))?;
        let values = inputs.get("values").ok_or_else(|| ShtairirError::Execution("Missing input 'values'".to_string()))?;

        let template_str = match template {
            Value::String(s) => s,
            _ => return Err(ShtairirError::Execution("Input 'template' must be a string".to_string())),
        };

        let values_map = match values {
            Value::Object(map) => map,
            _ => return Err(ShtairirError::Execution("Input 'values' must be an object".to_string())),
        };

        // Simple template replacement - in a real implementation, this would be more sophisticated
        let mut result = template_str.clone();
        for (key, value) in values_map {
            let placeholder = format!("{{{}}}", key);
            let replacement = match value {
                Value::String(s) => s.clone(),
                Value::I64(n) => n.to_string(),
                Value::F64(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                Value::List(_) => "[list]".to_string(), // Simplified representation
                Value::Object(_) => "[object]".to_string(), // Simplified representation
            };
            result = result.replace(&placeholder, &replacement);
        }

        let result = Value::String(result);

        Ok(BlockOutputs::new().with_output("result".to_string(), result))
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

#[cfg(test)]
mod tests {
    use super::*;
    use shtairir_registry::model::BlockSpec;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_concat_block() {
        let spec = BlockSpec::default();
        let block = ConcatBlock::new(spec);
        
        let strings = vec![Value::string("hello"), Value::string("world")];
        let inputs = BlockInputs::new()
            .with_input("strings".to_string(), Value::list(strings))
            .with_input("separator".to_string(), Value::string(" "));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::string("hello world")));
    }

    #[tokio::test]
    async fn test_split_block() {
        let spec = BlockSpec::default();
        let block = SplitBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("text".to_string(), Value::string("hello,world,test"))
            .with_input("delimiter".to_string(), Value::string(","));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        let expected = vec![Value::string("hello"), Value::string("world"), Value::string("test")];
        assert_eq!(outputs.get("parts"), Some(&Value::list(expected)));
    }

    #[tokio::test]
    async fn test_trim_block() {
        let spec = BlockSpec::default();
        let block = TrimBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("text".to_string(), Value::string("  hello world  "));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::string("hello world")));
    }

    #[tokio::test]
    async fn test_format_block() {
        let spec = BlockSpec::default();
        let block = FormatBlock::new(spec);
        
        let mut values = BTreeMap::new();
        values.insert("name".to_string(), Value::string("Alice"));
        values.insert("age".to_string(), Value::i64(30));
        
        let inputs = BlockInputs::new()
            .with_input("template".to_string(), Value::string("Hello {name}! You are {age} years old."))
            .with_input("values".to_string(), Value::object(values));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::string("Hello Alice! You are 30 years old.")));
    }
}