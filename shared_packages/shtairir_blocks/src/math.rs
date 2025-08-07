//! Math operations for Shtairir standard library
//!
//! This module provides basic mathematical operations that work with generic numeric types.

use async_trait::async_trait;
use shtairir::block::{Block, BlockInputs, BlockOutputs, BlockParams, ExecutionContext, ValidationError};
use shtairir_core::error::{ShtairirError, ShtairirResult};
use shtairir_registry::model::{BlockSpec, Determinism, Purity};
use shtairir_registry::value::Value;
use libm;
use nalgebra::Vector3;
use statrs::statistics::Statistics;
use fixed::types::I32F32;
use glam::Vec3;

/// Add block implementation
pub struct AddBlock {
    spec: BlockSpec,
}

impl AddBlock {
    /// Create a new AddBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for AddBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let a = inputs.get("a").ok_or_else(|| ShtairirError::Execution("Missing input 'a'".to_string()))?;
        let b = inputs.get("b").ok_or_else(|| ShtairirError::Execution("Missing input 'b'".to_string()))?;

        let result = match (a, b) {
            (Value::I64(a_val), Value::I64(b_val)) => Value::I64(a_val + b_val),
            (Value::F64(a_val), Value::F64(b_val)) => Value::F64(a_val + b_val),
            _ => return Err(ShtairirError::Execution("Unsupported types for addition".to_string())),
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

/// Subtract block implementation
pub struct SubtractBlock {
    spec: BlockSpec,
}

impl SubtractBlock {
    /// Create a new SubtractBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for SubtractBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let a = inputs.get("a").ok_or_else(|| ShtairirError::Execution("Missing input 'a'".to_string()))?;
        let b = inputs.get("b").ok_or_else(|| ShtairirError::Execution("Missing input 'b'".to_string()))?;

        let result = match (a, b) {
            (Value::I64(a_val), Value::I64(b_val)) => Value::I64(a_val - b_val),
            (Value::F64(a_val), Value::F64(b_val)) => Value::F64(a_val - b_val),
            _ => return Err(ShtairirError::Execution("Unsupported types for subtraction".to_string())),
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

/// Multiply block implementation
pub struct MultiplyBlock {
    spec: BlockSpec,
}

impl MultiplyBlock {
    /// Create a new MultiplyBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for MultiplyBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let a = inputs.get("a").ok_or_else(|| ShtairirError::Execution("Missing input 'a'".to_string()))?;
        let b = inputs.get("b").ok_or_else(|| ShtairirError::Execution("Missing input 'b'".to_string()))?;

        let result = match (a, b) {
            (Value::I64(a_val), Value::I64(b_val)) => Value::I64(a_val * b_val),
            (Value::F64(a_val), Value::F64(b_val)) => Value::F64(a_val * b_val),
            _ => return Err(ShtairirError::Execution("Unsupported types for multiplication".to_string())),
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

/// Divide block implementation
pub struct DivideBlock {
    spec: BlockSpec,
}

impl DivideBlock {
    /// Create a new DivideBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for DivideBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let a = inputs.get("a").ok_or_else(|| ShtairirError::Execution("Missing input 'a'".to_string()))?;
        let b = inputs.get("b").ok_or_else(|| ShtairirError::Execution("Missing input 'b'".to_string()))?;

        let result = match (a, b) {
            (Value::I64(a_val), Value::I64(b_val)) => {
                if *b_val == 0 {
                    return Err(ShtairirError::Execution("Division by zero".to_string()));
                }
                Value::I64(a_val / b_val)
            }
            (Value::F64(a_val), Value::F64(b_val)) => {
                if *b_val == 0.0 {
                    return Err(ShtairirError::Execution("Division by zero".to_string()));
                }
                Value::F64(a_val / b_val)
            }
            _ => return Err(ShtairirError::Execution("Unsupported types for division".to_string())),
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

/// Square root block implementation using libm
pub struct SqrtBlock {
    spec: BlockSpec,
}

impl SqrtBlock {
    /// Create a new SqrtBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for SqrtBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let value = inputs.get("value").ok_or_else(|| ShtairirError::Execution("Missing input 'value'".to_string()))?;

        let result = match value {
            Value::F64(f) => {
                if *f < 0.0 {
                    return Err(ShtairirError::Execution("Cannot compute square root of negative number".to_string()));
                }
                Value::F64(libm::sqrt(*f))
            }
            Value::I64(i) => {
                if *i < 0 {
                    return Err(ShtairirError::Execution("Cannot compute square root of negative number".to_string()));
                }
                Value::F64(libm::sqrt(*i as f64))
            }
            _ => return Err(ShtairirError::Execution("Unsupported type for square root".to_string())),
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

/// Vector addition block implementation using nalgebra
pub struct VectorAddBlock {
    spec: BlockSpec,
}

impl VectorAddBlock {
    /// Create a new VectorAddBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for VectorAddBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let a = inputs.get("a").ok_or_else(|| ShtairirError::Execution("Missing input 'a'".to_string()))?;
        let b = inputs.get("b").ok_or_else(|| ShtairirError::Execution("Missing input 'b'".to_string()))?;

        // Parse vector from list of 3 numbers
        let parse_vector = |value: &Value| -> Result<Vector3<f64>, ShtairirError> {
            match value {
                Value::List(list) if list.len() == 3 => {
                    let x = match &list[0] {
                        Value::F64(f) => *f,
                        Value::I64(i) => *i as f64,
                        _ => return Err(ShtairirError::Execution("Vector components must be numbers".to_string())),
                    };
                    let y = match &list[1] {
                        Value::F64(f) => *f,
                        Value::I64(i) => *i as f64,
                        _ => return Err(ShtairirError::Execution("Vector components must be numbers".to_string())),
                    };
                    let z = match &list[2] {
                        Value::F64(f) => *f,
                        Value::I64(i) => *i as f64,
                        _ => return Err(ShtairirError::Execution("Vector components must be numbers".to_string())),
                    };
                    Ok(Vector3::new(x, y, z))
                }
                _ => Err(ShtairirError::Execution("Input must be a list of 3 numbers".to_string())),
            }
        };

        let vec_a = parse_vector(a)?;
        let vec_b = parse_vector(b)?;

        let result_vec = vec_a + vec_b;
        let result_list = vec![
            Value::F64(result_vec.x),
            Value::F64(result_vec.y),
            Value::F64(result_vec.z),
        ];

        Ok(BlockOutputs::new().with_output("result".to_string(), Value::List(result_list)))
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

/// Mean calculation block implementation using statrs
pub struct MeanBlock {
    spec: BlockSpec,
}

impl MeanBlock {
    /// Create a new MeanBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for MeanBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let values = inputs.get("values").ok_or_else(|| ShtairirError::Execution("Missing input 'values'".to_string()))?;

        let numbers = match values {
            Value::List(list) => {
                let mut nums = Vec::new();
                for item in list {
                    match item {
                        Value::F64(f) => nums.push(*f),
                        Value::I64(i) => nums.push(*i as f64),
                        _ => return Err(ShtairirError::Execution("All values must be numbers".to_string())),
                    }
                }
                nums
            }
            _ => return Err(ShtairirError::Execution("Input must be a list of numbers".to_string())),
        };

        if numbers.is_empty() {
            return Err(ShtairirError::Execution("Cannot compute mean of empty list".to_string()));
        }

        let mean = numbers.mean();
        let result = Value::F64(mean);

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

/// Fixed-point multiplication block implementation using fixed
pub struct FixedMultiplyBlock {
    spec: BlockSpec,
}

impl FixedMultiplyBlock {
    /// Create a new FixedMultiplyBlock
    pub fn new(spec: BlockSpec) -> Self {
        Self { spec }
    }
}

#[async_trait]
impl Block for FixedMultiplyBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }

    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> ShtairirResult<BlockOutputs> {
        let a = inputs.get("a").ok_or_else(|| ShtairirError::Execution("Missing input 'a'".to_string()))?;
        let b = inputs.get("b").ok_or_else(|| ShtairirError::Execution("Missing input 'b'".to_string()))?;

        let parse_fixed = |value: &Value| -> Result<I32F32, ShtairirError> {
            match value {
                Value::String(s) => {
                    s.parse::<I32F32>()
                        .map_err(|_| ShtairirError::Execution("Invalid fixed-point number format".to_string()))
                }
                Value::F64(f) => Ok(I32F32::from_num(*f)),
                Value::I64(i) => Ok(I32F32::from_num(*i)),
                _ => Err(ShtairirError::Execution("Unsupported type for fixed-point conversion".to_string())),
            }
        };

        let fixed_a = parse_fixed(a)?;
        let fixed_b = parse_fixed(b)?;

        let result = fixed_a * fixed_b;
        let result = Value::String(result.to_string());

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

    #[tokio::test]
    async fn test_add_block() {
        let spec = BlockSpec::default(); // In practice, this would be loaded from the TOML spec
        let block = AddBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::i64(5))
            .with_input("b".to_string(), Value::i64(3));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(8)));
    }

    #[tokio::test]
    async fn test_subtract_block() {
        let spec = BlockSpec::default();
        let block = SubtractBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::i64(5))
            .with_input("b".to_string(), Value::i64(3));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(2)));
    }

    #[tokio::test]
    async fn test_multiply_block() {
        let spec = BlockSpec::default();
        let block = MultiplyBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::i64(5))
            .with_input("b".to_string(), Value::i64(3));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(15)));
    }

    #[tokio::test]
    async fn test_divide_block() {
        let spec = BlockSpec::default();
        let block = DivideBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::i64(6))
            .with_input("b".to_string(), Value::i64(3));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::i64(2)));
    }

    #[tokio::test]
    async fn test_divide_by_zero() {
        let spec = BlockSpec::default();
        let block = DivideBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::i64(6))
            .with_input("b".to_string(), Value::i64(0));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let result = block.execute(&inputs, &context).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Execution error: Division by zero");
    }

    #[tokio::test]
    async fn test_sqrt_block() {
        let spec = BlockSpec::default();
        let block = SqrtBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("value".to_string(), Value::f64(16.0));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::f64(4.0)));
    }

    #[tokio::test]
    async fn test_vector_add_block() {
        let spec = BlockSpec::default();
        let block = VectorAddBlock::new(spec);
        
        let vec_a = vec![Value::f64(1.0), Value::f64(2.0), Value::f64(3.0)];
        let vec_b = vec![Value::f64(4.0), Value::f64(5.0), Value::f64(6.0)];
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::list(vec_a))
            .with_input("b".to_string(), Value::list(vec_b));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        let expected = vec![Value::f64(5.0), Value::f64(7.0), Value::f64(9.0)];
        assert_eq!(outputs.get("result"), Some(&Value::list(expected)));
    }

    #[tokio::test]
    async fn test_mean_block() {
        let spec = BlockSpec::default();
        let block = MeanBlock::new(spec);
        
        let values = vec![Value::f64(1.0), Value::f64(2.0), Value::f64(3.0), Value::f64(4.0)];
        let inputs = BlockInputs::new()
            .with_input("values".to_string(), Value::list(values));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        assert_eq!(outputs.get("result"), Some(&Value::f64(2.5)));
    }

    #[tokio::test]
    async fn test_fixed_multiply_block() {
        let spec = BlockSpec::default();
        let block = FixedMultiplyBlock::new(spec);
        
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::string("1.5"))
            .with_input("b".to_string(), Value::string("2.0"));
        
        let context = ExecutionContext::new("test-execution".to_string(), std::sync::Arc::new(shtairir_registry::model::Registry::new()));
        let outputs = block.execute(&inputs, &context).await.unwrap();
        
        // The result should be approximately 3.0
        let result_str = match outputs.get("result").unwrap() {
            Value::String(s) => s,
            _ => panic!("Expected string result"),
        };
        
        // Parse the result and check it's close to 3.0
        let result_fixed: I32F32 = result_str.parse().unwrap();
        let result_f64 = result_fixed.to_num::<f64>();
        assert!((result_f64 - 3.0).abs() < 0.001);
    }
}