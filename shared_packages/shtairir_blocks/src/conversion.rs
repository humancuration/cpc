//! Type conversion operations for Shtairir standard library
//!
//! This module provides operations for converting between different types,
//! including string serialization, numeric parsing, and JSON parsing.

use async_trait::async_trait;
use shtairir::block::{Block, BlockInputs, BlockOutputs, BlockParams, ExecutionContext, ValidationError};
use shtairir_core::error::{ShtairirError, ShtairirResult};
use shtairir_registry::model::{BlockSpec, Determinism, Purity};
use shtairir_registry::value::Value;
use serde_json;

/// ToString block implementation
pub struct ToStringBlock {
    spec: BlockSpec,
}

impl ToStringBlock {
    /// Create a new ToStringBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for ToStringBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let value = inputs.get("value").ok_or_else(|| ShtairirError::Execution("Missing input 'value'".to_string()))?;

        let result = match value {
            Value::String(s) => Value::String(s.clone()),
            Value::I64(n) => Value::String(n.to_string()),
            Value::F64(n) => Value::String(n.to_string()),
            Value::Bool(b) => Value::String(b.to_string()),
            Value::Null => Value::String("null".to_string()),
            Value::List(_) => {
                // In a real implementation, we would serialize the list to JSON
                Value::String("[list]".to_string())
            }
            Value::Object(_) => {
                // In a real implementation, we would serialize the object to JSON
                Value::String("[object]".to_string())
            }
        };

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

/// ToNumber block implementation
pub struct ToNumberBlock {
    spec: BlockSpec,
}

impl ToNumberBlock {
    /// Create a new ToNumberBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for ToNumberBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, params: &BlockParams) -> ShtairirResult<BlockOutputs> {
        let text = inputs.get("text").ok_or_else(|| ShtairirError::Execution("Missing input 'text'".to_string()))?;
        let target_type = params.get("target_type").unwrap_or(&Value::String("i64".to_string()));

        let text_str = match text {
            Value::String(s) => s,
            _ => return Err(ShtairirError::Execution("Input 'text' must be a string".to_string())),
        };

        let type_str = match target_type {
            Value::String(s) => s.as_str(),
            _ => "i64",
        };

        let result = match type_str {
            "i64" => {
                text_str.parse::<i64>()
                    .map(Value::I64)
                    .map_err(|e| ShtairirError::Execution(format!("Failed to parse as i64: {}", e)))?
            }
            "f64" => {
                text_str.parse::<f64>()
                    .map(Value::F64)
                    .map_err(|e| ShtairirError::Execution(format!("Failed to parse as f64: {}", e)))?
            }
            _ => return Err(ShtairirError::Execution(format!("Unsupported target type: {}", type_str))),
        };

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

/// ParseJson block implementation
pub struct ParseJsonBlock {
    spec: BlockSpec,
}

impl ParseJsonBlock {
    /// Create a new ParseJsonBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for ParseJsonBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let json_text = inputs.get("json_text").ok_or_else(|| ShtairirError::Execution("Missing input 'json_text'".to_string()))?;

        let text_str = match json_text {
            Value::String(s) => s,
            _ => return Err(ShtairirError::Execution("Input 'json_text' must be a string".to_string())),
        };

        // Parse JSON text into a serde_json::Value
        let parsed: serde_json::Result<serde_json::Value> = serde_json::from_str(text_str);
        
        match parsed {
            Ok(json_value) => {
                // Convert serde_json::Value to shtairir Value
                let value = convert_json_value(json_value);
                Ok(BlockOutputs::new().with_output("result".to_string(), value))
            }
            Err(e) => Err(ShtairirError::Execution(format!("Failed to parse JSON: {}", e))),
        }
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

/// Convert a serde_json::Value to a shtairir Value
fn convert_json_value(json_value: serde_json::Value) -> Value {
    match json_value {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::I64(i)
            } else if let Some(f) = n.as_f64() {
                Value::F64(f)
            } else {
                // This shouldn't happen with valid JSON numbers, but just in case
                Value::String(n.to_string())
            }
        }
        serde_json::Value::String(s) => Value::String(s),
        serde_json::Value::Array(arr) => {
            let converted: Vec<Value> = arr.into_iter().map(convert_json_value).collect();
            Value::List(converted)
        }
        serde_json::Value::Object(obj) => {
            let converted: std::collections::BTreeMap<String, Value> = obj
                .into_iter()
                .map(|(k, v)| (k, convert_json_value(v)))
                .collect();
            Value::Object(converted)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shtairir_registry::model::BlockSpec;
    use std::collections::BTreeMap;

    #[tokio::test]
    async fn test_to_string_block() {
        let spec = BlockSpec::default();
        let block = ToStringBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("value".to_string(), Value::i64(42));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::string("42")));
    }

    #[tokio::test]
    async fn test_to_number_block() {
        let spec = BlockSpec::default();
        let block = ToNumberBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("text".to_string(), Value::string("42"));
        
        let params = BlockParams::new()
            .with_param("target_type".to_string(), Value::string("i64"));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &params).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(42)));
    }

    #[tokio::test]
    async fn test_parse_json_block() {
        let spec = BlockSpec::default();
        let block = ParseJsonBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("json_text".to_string(), Value::string("{\"name\":\"Alice\",\"age\":30}"));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        let mut expected_map = BTreeMap::new();
        expected_map.insert("name".to_string(), Value::string("Alice"));
        expected_map.insert("age".to_string(), Value::i64(30));
        
        assert_eq!(outputs.get("result"), Some(&Value::object(expected_map)));
    }
}