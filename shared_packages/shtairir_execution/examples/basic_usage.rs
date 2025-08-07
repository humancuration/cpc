//! Basic usage example for the Shtairir execution scheduler

use shtairir_execution::scheduler::Scheduler;
use shtairir_execution::registry::RegistryAdapter;
use shtairir_registry::model::{
    Registry, GraphSpec, Node, NodeKind, Edge, Endpoint, EdgePolicy,
    EngineReq, BlockHandle, BlockSpec, Purity, Determinism, PortSpec
};
use shtairir_registry::literal::ValueLiteral;
use std::collections::BTreeMap;

fn main() {
    println!("Creating a basic Shtairir execution scheduler example");
    
    // Create a registry and populate it with blocks
    let mut registry = Registry::new();
    
    // Add a simple math block with inputs and outputs
    let add_block = BlockHandle {
        module: "math".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            id: "math/add@0.1.0".to_string(),
            namespace: "math".to_string(),
            name: "add".to_string(),
            version: "0.1.0".to_string(),
            title: "Add".to_string(),
            description: "Add two numbers".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    name: "a".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                },
                PortSpec {
                    name: "b".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                },
            ],
            outputs: vec![
                PortSpec {
                    name: "result".to_string(),
                    ty: "i64".to_string(),
                    default: None,
                    kind: None,
                },
            ],
            params: vec![],
            generics: vec![],
            engine: EngineReq {
                version_req: "0.2.0".to_string(),
                capability_flags: vec![],
            },
            examples: vec![],
            integrity: None,
            authors: vec![],
            license: "".to_string(),
            tags: vec![],
            metadata: None,
            tests: vec![],
        },
    };
    
    registry.insert_block(add_block);
    
    // Create a registry adapter
    let registry_adapter = RegistryAdapter::new(registry);
    
    // Create a scheduler
    let scheduler = Scheduler::new(registry_adapter);
    
    println!("Scheduler created successfully!");
    println!("This example demonstrates basic setup of the Shtairir execution scheduler.");
}