//! Basic usage example for Shtairir codeblocks
//! 
//! This example demonstrates how to use the core building blocks
//! of the Shtairir system.

use shtairir::block::{Block, BlockInputs, BlockOutputs, ExecutionContext, BlockResult, ValidationError};
use shtairir::composition::{BlockComposition, Connection, OutputPortRef, InputPortRef, EdgeAdapter};
use shtairir::visual::{VisualNode, VisualEdge, EdgeEndpoint, EdgePolicy, BackpressureStrategy, OrderingStrategy};
use shtairir::port::{InputPort, OutputPort};
use shtairir::block::PortKind;
use shtairir::edge::EdgeAdapter as EdgeAdapterPolicy;
use shtairir_registry::model::{BlockSpec, Determinism, Purity, EngineReq};
use shtairir_registry::value::Value;
use shtairir_registry::types::{Type, ScalarType};
use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;

/// A simple add block for demonstration
struct AddBlock {
    spec: BlockSpec,
}

impl AddBlock {
    fn new() -> Self {
        let spec = BlockSpec {
            id: "examples.shtairir/add@1.0.0".to_string(),
            namespace: "examples.shtairir".to_string(),
            name: "add".to_string(),
            version: "1.0.0".to_string(),
            title: "Add".to_string(),
            description: "Adds two numbers together".to_string(),
            authors: vec!["CPC Cooperative".to_string()],
            license: "CPC".to_string(),
            tags: vec!["math".to_string(), "arithmetic".to_string()],
            purity: Purity::Pure,
            effects: vec![],
            determinism: Determinism::Deterministic,
            generics: vec![],
            inputs: vec![
                shtairir_registry::model::PortSpec {
                    name: "a".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                },
                shtairir_registry::model::PortSpec {
                    name: "b".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            outputs: vec![
                shtairir_registry::model::PortSpec {
                    name: "result".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            params: vec![],
            examples: vec![],
            tests: vec![],
            engine: EngineReq {
                version_req: "^0.2".to_string(),
                capability_flags: vec![],
            },
            integrity: None,
            metadata: None,
        };
        
        Self { spec }
    }
}

#[async_trait]
impl Block for AddBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }
    
    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> BlockResult<BlockOutputs> {
        let a = inputs.get("a").and_then(|v| match v {
            Value::I64(n) => Some(*n),
            _ => None,
        }).unwrap_or(0);
        
        let b = inputs.get("b").and_then(|v| match v {
            Value::I64(n) => Some(*n),
            _ => None,
        }).unwrap_or(0);
        
        let result = a + b;
        Ok(BlockOutputs::new().with_output("result".to_string(), Value::I64(result)))
    }
    
    fn validate(&self, _params: &shtairir::block::BlockParams) -> Result<(), shtairir::block::ValidationError> {
        Ok(())
    }
    
    fn purity(&self) -> Purity {
        Purity::Pure
    }
    
    fn determinism(&self) -> Determinism {
        Determinism::Deterministic
    }
}

/// A simple multiply block for demonstration
struct MultiplyBlock {
    spec: BlockSpec,
}

impl MultiplyBlock {
    fn new() -> Self {
        let spec = BlockSpec {
            id: "examples.shtairir/multiply@1.0.0".to_string(),
            namespace: "examples.shtairir".to_string(),
            name: "multiply".to_string(),
            version: "1.0.0".to_string(),
            title: "Multiply".to_string(),
            description: "Multiplies two numbers together".to_string(),
            authors: vec!["CPC Cooperative".to_string()],
            license: "CPC".to_string(),
            tags: vec!["math".to_string(), "arithmetic".to_string()],
            purity: Purity::Pure,
            effects: vec![],
            determinism: Determinism::Deterministic,
            generics: vec![],
            inputs: vec![
                shtairir_registry::model::PortSpec {
                    name: "a".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                },
                shtairir_registry::model::PortSpec {
                    name: "b".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            outputs: vec![
                shtairir_registry::model::PortSpec {
                    name: "result".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            params: vec![],
            examples: vec![],
            tests: vec![],
            engine: EngineReq {
                version_req: "^0.2".to_string(),
                capability_flags: vec![],
            },
            integrity: None,
            metadata: None,
        };
        
        Self { spec }
    }
}

#[async_trait]
impl Block for MultiplyBlock {
    fn spec(&self) -> &BlockSpec {
        &self.spec
    }
    
    async fn execute(&self, inputs: &BlockInputs, _context: &ExecutionContext) -> BlockResult<BlockOutputs> {
        let a = inputs.get("a").and_then(|v| match v {
            Value::I64(n) => Some(*n),
            _ => None,
        }).unwrap_or(0);
        
        let b = inputs.get("b").and_then(|v| match v {
            Value::I64(n) => Some(*n),
            _ => None,
        }).unwrap_or(0);
        
        let result = a * b;
        Ok(BlockOutputs::new().with_output("result".to_string(), Value::I64(result)))
    }
    
    fn validate(&self, _params: &shtairir::block::BlockParams) -> Result<(), shtairir::block::ValidationError> {
        Ok(())
    }
    
    fn purity(&self) -> Purity {
        Purity::Pure
    }
    
    fn determinism(&self) -> Determinism {
        Determinism::Deterministic
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Shtairir Codeblocks Basic Usage Example");
    println!("=====================================");
    
    // Create blocks
    let add_block = Arc::new(AddBlock::new());
    let multiply_block = Arc::new(MultiplyBlock::new());
    
    // Demonstrate block execution
    println!("\n1. Executing individual blocks:");
    
    let add_inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::I64(5))
        .with_input("b".to_string(), Value::I64(3));
    
    let registry = Arc::new(shtairir_registry::model::Registry::new());
    let context = ExecutionContext::new("example-execution".to_string(), registry);
    
    let add_outputs = add_block.execute(&add_inputs, &context).await?;
    println!("   Add(5, 3) = {:?}", add_outputs.get("result"));
    
    let multiply_inputs = BlockInputs::new()
        .with_input("a".to_string(), Value::I64(4))
        .with_input("b".to_string(), Value::I64(6));
    
    let multiply_outputs = multiply_block.execute(&multiply_inputs, &context).await?;
    println!("   Multiply(4, 6) = {:?}", multiply_outputs.get("result"));
    
    // Demonstrate block composition
    println!("\n2. Creating block composition:");
    
    let composition = BlockComposition::Sequential {
        blocks: vec![add_block.clone(), multiply_block.clone()],
        connections: vec![
            Connection {
                from: OutputPortRef {
                    block_id: "add_block".to_string(),
                    port_id: "result".to_string(),
                },
                to: InputPortRef {
                    block_id: "multiply_block".to_string(),
                    port_id: "a".to_string(),
                },
                adapter: Some(EdgeAdapter::None),
            }
        ],
    };
    
    match composition {
        BlockComposition::Sequential { blocks, connections } => {
            println!("   Created sequential composition with {} blocks and {} connections",
                     blocks.len(), connections.len());
        }
        _ => {}
    }
    
    // Demonstrate visual programming components
    println!("\n3. Creating visual programming components:");
    
    let add_node = VisualNode::new("add_node".to_string(), add_block.clone())
        .with_position(100.0, 100.0);
    
    let multiply_node = VisualNode::new("multiply_node".to_string(), multiply_block.clone())
        .with_position(300.0, 100.0);
    
    let edge = VisualEdge::new(
        "edge1".to_string(),
        EdgeEndpoint {
            node_id: "add_node".to_string(),
            port_id: "result".to_string(),
        },
        EdgeEndpoint {
            node_id: "multiply_node".to_string(),
            port_id: "a".to_string(),
        },
    ).with_policy(EdgePolicy {
        adapter: EdgeAdapterPolicy::None,
        backpressure: BackpressureStrategy::Block,
        ordering: OrderingStrategy::Source,
    });
    
    println!("   Created visual nodes: {}, {}", add_node.id, multiply_node.id);
    println!("   Created visual edge: {}", edge.id);
    
    // Demonstrate port specifications
    println!("\n4. Creating port specifications:");
    
    let input_port = InputPort::new(
        "input1".to_string(),
        "first_number".to_string(),
        Type::Scalar(ScalarType::I64),
        PortKind::Value,
    ).with_default(Value::I64(0))
     .with_required(true)
     .with_description("The first number to process".to_string());
    
    let output_port = OutputPort::new(
        "output1".to_string(),
        "result".to_string(),
        Type::Scalar(ScalarType::I64),
        PortKind::Value,
    ).with_description("The result of the computation".to_string());
    
    println!("   Created input port: {} (type: {:?})", input_port.name, input_port.ty);
    println!("   Created output port: {} (type: {:?})", output_port.name, output_port.ty);
    
    println!("\nExample completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_add_block() {
        let add_block = AddBlock::new();
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::I64(2))
            .with_input("b".to_string(), Value::I64(3));
        
        let registry = Arc::new(shtairir_registry::model::Registry::new());
        let context = ExecutionContext::new("test-execution".to_string(), registry);
        
        let outputs = add_block.execute(&inputs, &context).await.unwrap();
        assert_eq!(outputs.get("result"), Some(&Value::I64(5)));
    }
    
    #[tokio::test]
    async fn test_multiply_block() {
        let multiply_block = MultiplyBlock::new();
        let inputs = BlockInputs::new()
            .with_input("a".to_string(), Value::I64(4))
            .with_input("b".to_string(), Value::I64(5));
        
        let registry = Arc::new(shtairir_registry::model::Registry::new());
        let context = ExecutionContext::new("test-execution".to_string(), registry);
        
        let outputs = multiply_block.execute(&inputs, &context).await.unwrap();
        assert_eq!(outputs.get("result"), Some(&Value::I64(20)));
    }
}