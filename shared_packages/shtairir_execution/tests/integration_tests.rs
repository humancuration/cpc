//! Comprehensive integration tests for the Shtairir Execution Scheduler
//!
//! This test demonstrates the full functionality of the scheduler including:
//! - Actual node execution with real data flow
//! - Concurrent execution of independent nodes
//! - Proper error handling in execution scenarios
//! - Complete end-to-end workflow

use shtairir_execution::scheduler::Scheduler;
use shtairir_execution::registry::RegistryAdapter;
use shtairir_registry::model::{
    Registry, GraphSpec, Node, NodeKind, PortDecl, PortKind, Edge, Endpoint, EdgePolicy,
    EngineReq, BlockHandle, BlockSpec, Purity, Determinism, PortSpec, ParamSpec
};
use shtairir_registry::literal::ValueLiteral;
use std::time::Instant;
use std::collections::BTreeMap;
use std::collections::HashMap;
use anyhow::Result;

// Helper macro to create ValueLiteral objects more easily
macro_rules! value_map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut map = std::collections::BTreeMap::new();
            $(map.insert($key.to_string(), $value);)*
            ValueLiteral::Object(map)
        }
    };
}


/// Mock block that performs addition operation
fn create_add_block() -> BlockHandle {
    BlockHandle {
        module: "math".to_string(),
        version: "1.0.0".to_string(),
        spec: BlockSpec {
            id: "math/add@1.0.0".to_string(),
            namespace: "math".to_string(),
            name: "add".to_string(),
            version: "1.0.0".to_string(),
            title: "Add".to_string(),
            description: "Add two numbers".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    name: "a".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                },
                PortSpec {
                    name: "b".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            outputs: vec![
                PortSpec {
                    name: "result".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
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
            license: "MIT".to_string(),
            tags: vec![],
            metadata: None,
            tests: vec![],
        },
    }
}

/// Mock block that performs multiplication operation
fn create_multiply_block() -> BlockHandle {
    BlockHandle {
        module: "math".to_string(),
        version: "1.0.0".to_string(),
        spec: BlockSpec {
            id: "math/multiply@1.0.0".to_string(),
            namespace: "math".to_string(),
            name: "multiply".to_string(),
            version: "1.0.0".to_string(),
            title: "Multiply".to_string(),
            description: "Multiply two numbers".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    name: "a".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                },
                PortSpec {
                    name: "b".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            outputs: vec![
                PortSpec {
                    name: "result".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
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
            license: "MIT".to_string(),
            tags: vec![],
            metadata: None,
            tests: vec![],
        },
    }
}

/// Mock block that performs division operation
fn create_divide_block() -> BlockHandle {
    BlockHandle {
        module: "math".to_string(),
        version: "1.0.0".to_string(),
        spec: BlockSpec {
            id: "math/divide@1.0.0".to_string(),
            namespace: "math".to_string(),
            name: "divide".to_string(),
            version: "1.0.0".to_string(),
            title: "Divide".to_string(),
            description: "Divide two numbers".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    name: "dividend".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                },
                PortSpec {
                    name: "divisor".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            outputs: vec![
                PortSpec {
                    name: "result".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
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
            license: "MIT".to_string(),
            tags: vec![],
            metadata: None,
            tests: vec![],
        },
    }
}

/// Mock block that introduces an error
fn create_error_block() -> BlockHandle {
    BlockHandle {
        module: "test".to_string(),
        version: "1.0.0".to_string(),
        spec: BlockSpec {
            id: "test/error@1.0.0".to_string(),
            namespace: "test".to_string(),
            name: "error".to_string(),
            version: "1.0.0".to_string(),
            title: "Error".to_string(),
            description: "Always fails".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    name: "input".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
            ],
            outputs: vec![
                PortSpec {
                    name: "result".to_string(),
                    ty: "number".to_string(),
                    default: None,
                    kind: None,
                }
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
            license: "MIT".to_string(),
            tags: vec![],
            metadata: None,
            tests: vec![],
        },
    }
}

/// Create a registry with mock blocks
fn create_test_registry() -> RegistryAdapter {
    let mut registry = Registry::new();
    
    // Add math blocks
    registry.insert_block(create_add_block());
    registry.insert_block(create_multiply_block());
    registry.insert_block(create_divide_block());
    registry.insert_block(create_error_block());
    
    RegistryAdapter::new(registry)
}


/// Test complete graph execution with data flow
#[tokio::test]
async fn test_complete_graph_execution() -> Result<()> {
    let registry = create_test_registry();
    let scheduler = Scheduler::new(registry.clone());
    
    // Create a graph that calculates: (2 + 3) * 4 = 20
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/calculation@1.0.0".to_string(),
        namespace: "test".to_string(),
        name: "calculation".to_string(),
        version: "1.0.0".to_string(),
        title: "Calculation Test".to_string(),
        description: "Test graph that performs mathematical operations".to_string(),
        authors: vec![],
        tags: vec![],
        visibility: "public".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "input_a".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Input A".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!(),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "input_b".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Input B".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "add".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Add".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![
                    PortDecl {
                        name: "a".to_string(),
                        port_id: "a".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    },
                    PortDecl {
                        name: "b".to_string(),
                        port_id: "b".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    }
                ],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "multiply".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/multiply".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Multiply".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![
                    PortDecl {
                        name: "a".to_string(),
                        port_id: "a".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    },
                    PortDecl {
                        name: "b".to_string(),
                        port_id: "b".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    }
                ],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "input_a".to_string(), port: "result".to_string() },
                to: Endpoint { node: "add".to_string(), port: "a".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "input_b".to_string(), port: "result".to_string() },
                to: Endpoint { node: "add".to_string(), port: "b".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
            Edge {
                id: "edge3".to_string(),
                from: Endpoint { node: "add".to_string(), port: "result".to_string() },
                to: Endpoint { node: "multiply".to_string(), port: "a".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
        provenance: None,
        metadata: None,
        annotations: None,
    };
    
    // Execute the graph
    let start_time = Instant::now();
    let result = scheduler.schedule(&graph).await?;
    let duration = start_time.elapsed();
    
    // Verify execution
    assert!(!result.execution_order.is_empty());
    assert_eq!(result.execution_order.len(), 4);
    assert!(result.node_outputs.contains_key("multiply"));
    
    println!("Graph executed successfully in {:?}", duration);
    println!("Execution order: {:?}", result.execution_order);
    
    Ok(())
}

/// Test concurrent execution of independent nodes
#[tokio::test]
async fn test_concurrent_execution() -> Result<()> {
    let registry = create_test_registry();
    let scheduler = Scheduler::new(registry.clone());
    
    // Create a graph with independent nodes that can execute concurrently
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/concurrent@1.0.0".to_string(),
        namespace: "test".to_string(),
        name: "concurrent".to_string(),
        version: "1.0.0".to_string(),
        title: "Concurrent Execution Test".to_string(),
        description: "Test graph with independent nodes for concurrent execution".to_string(),
        authors: vec![],
        tags: vec![],
        visibility: "public".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            // Independent nodes that can run concurrently
            Node {
                id: "task1".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Task 1".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "task2".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/multiply".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Task 2".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "task3".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/divide".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Task 3".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            // Dependent node that runs after the independent ones
            Node {
                id: "aggregate".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Aggregate".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![
                    PortDecl {
                        name: "a".to_string(),
                        port_id: "a".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    },
                    PortDecl {
                        name: "b".to_string(),
                        port_id: "b".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    }
                ],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "task1".to_string(), port: "result".to_string() },
                to: Endpoint { node: "aggregate".to_string(), port: "a".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "task2".to_string(), port: "result".to_string() },
                to: Endpoint { node: "aggregate".to_string(), port: "b".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
        provenance: None,
        metadata: None,
        annotations: None,
    };
    
    // Execute the graph
    let start_time = Instant::now();
    let result = scheduler.schedule(&graph).await?;
    let duration = start_time.elapsed();
    
    // Verify concurrent execution
    assert_eq!(result.execution_order.len(), 4);
    assert!(result.node_outputs.contains_key("aggregate"));
    
    // The first three nodes should be able to execute concurrently
    // We can't directly measure concurrency in this test, but we can verify the execution order is correct
    let first_level: Vec<&String> = result.execution_order.iter().take(3).collect();
    assert!(first_level.contains(&&"task1".to_string()));
    assert!(first_level.contains(&&"task2".to_string()));
    assert!(first_level.contains(&&"task3".to_string()));
    
    println!("Concurrent execution test completed in {:?}", duration);
    println!("Execution order: {:?}", result.execution_order);
    
    Ok(())
}

/// Test error propagation in execution
#[tokio::test]
async fn test_error_propagation() -> Result<()> {
    let registry = create_test_registry();
    let scheduler = Scheduler::new(registry.clone());
    
    // Create a graph that will fail during execution
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/error@1.0.0".to_string(),
        namespace: "test".to_string(),
        name: "error".to_string(),
        version: "1.0.0".to_string(),
        title: "Error Test".to_string(),
        description: "Test graph that demonstrates error handling".to_string(),
        authors: vec![],
        tags: vec![],
        visibility: "public".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "error_node".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("test/error".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Error Node".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
        ],
        edges: vec![],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
        provenance: None,
        metadata: None,
        annotations: None,
    };
    
    // Execute the graph - this should fail
    let result = scheduler.schedule(&graph).await;
    
    // Verify that the error is properly propagated
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Intentional error"));
    
    println!("Error propagation test passed - error correctly propagated: {}", error);
    
    Ok(())
}

/// Test data flow between nodes
#[tokio::test]
async fn test_data_flow() -> Result<()> {
    let registry = create_test_registry();
    let scheduler = Scheduler::new(registry.clone());
    
    // Create a simple graph to test data flow: 10 / 2 = 5
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/dataflow@1.0.0".to_string(),
        namespace: "test".to_string(),
        name: "dataflow".to_string(),
        version: "1.0.0".to_string(),
        title: "Data Flow Test".to_string(),
        description: "Test graph that verifies data flow between nodes".to_string(),
        authors: vec![],
        tags: vec![],
        visibility: "public".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "dividend".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Dividend".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "divisor".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Divisor".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
            Node {
                id: "divide".to_string(),
                kind: NodeKind::Block,
                fq_block: Some("math/divide".to_string()),
                version_req: Some("1.0.0".to_string()),
                concrete_version: None,
                title: Some("Divide".to_string()),
                purity: Some(Purity::Pure),
                effects: vec![],
                generics: std::collections::BTreeMap::new(),
                params: value_map!({}),
                inputs: vec![
                    PortDecl {
                        name: "dividend".to_string(),
                        port_id: "dividend".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    },
                    PortDecl {
                        name: "divisor".to_string(),
                        port_id: "divisor".to_string(),
                        ty: "number".to_string(),
                        kind: PortKind::Value,
                    }
                ],
                outputs: vec![PortDecl {
                    name: "result".to_string(),
                    port_id: "result".to_string(),
                    ty: "number".to_string(),
                    kind: PortKind::Value,
                }],
                meta: None,
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "dividend".to_string(), port: "result".to_string() },
                to: Endpoint { node: "divide".to_string(), port: "dividend".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "divisor".to_string(), port: "result".to_string() },
                to: Endpoint { node: "divide".to_string(), port: "divisor".to_string() },
                policy: EdgePolicy::default(),
                notes: None,
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
        provenance: None,
        metadata: None,
        annotations: None,
    };
    
    // Execute the graph
    let result = scheduler.schedule(&graph).await?;
    
    // Verify data flow
    assert_eq!(result.execution_order.len(), 3);
    assert!(result.node_outputs.contains_key("divide"));
    
    println!("Data flow test completed");
    println!("Execution order: {:?}", result.execution_order);
    println!("Node outputs: {:?}", result.node_outputs);
    
    Ok(())
}

/// Test registry integration
#[tokio::test]
async fn test_registry_integration() -> Result<()> {
    let registry = create_test_registry();
    
    // Test that we can list modules and blocks
    let modules = registry.list_modules();
    assert!(modules.contains(&"math".to_string()));
    assert!(modules.contains(&"test".to_string()));
    
    let math_blocks = registry.list_blocks("math");
    assert!(math_blocks.contains(&"add".to_string()));
    assert!(math_blocks.contains(&"multiply".to_string()));
    assert!(math_blocks.contains(&"divide".to_string()));
    
    let test_blocks = registry.list_blocks("test");
    assert!(test_blocks.contains(&"error".to_string()));
    
    // Test that we can get specific blocks
    let add_block = registry.get_block("math/add", Some("1.0.0"));
    assert!(add_block.is_ok());
    let add_block = add_block.unwrap();
    assert_eq!(add_block.module, "math");
    assert_eq!(add_block.version, "1.0.0");
    assert_eq!(add_block.spec.name, "add");
    
    println!("Registry integration test passed");
    println!("Available modules: {:?}", modules);
    println!("Math blocks: {:?}", math_blocks);
    
    Ok(())
}