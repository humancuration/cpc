use anyhow::Result;
use shtairir_registry::model::{
    BlockHandle, BlockSpec, Registry, GraphSpec, Node, Edge,
    PortSpec, PortKind, Endpoint, EdgePolicy, Purity, Determinism,
    GenericParam, GraphHandle, EngineReq, PortDecl, AdapterParams
};
use shtairir_registry::validator::validate_registry;
use shtairir_registry::literal::ValueLiteral;

#[test]
fn test_type_compatibility() -> Result<()> {
    let mut reg = Registry::default();
    
    // Add a simple math.add block
    let add_block = BlockHandle {
        module: "math".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "add".to_string(),
            title: "Add".to_string(),
            description: "Add two numbers".to_string(),
            id: "math/add@0.1.0".to_string(),
            namespace: "math".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    port_id: "a".to_string(),
                    name: "a".to_string(),
                    ty: "i64".to_string(),
                    description: "First number".to_string(),
                    kind: None,
                    default: None,
                },
                PortSpec {
                    port_id: "b".to_string(),
                    name: "b".to_string(),
                    ty: "i64".to_string(),
                    description: "Second number".to_string(),
                    kind: None,
                    default: None,
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "out".to_string(),
                    name: "out".to_string(),
                    ty: "i64".to_string(),
                    description: "Sum".to_string(),
                    kind: None,
                    default: None,
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
        },
    };
    
    reg.blocks.insert("math/add@0.1.0".to_string(), add_block);
    
    // Create a simple graph with compatible types
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/add_graph@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "add_graph".to_string(),
        title: "Add Graph".to_string(),
        description: "A simple graph using add".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "add1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("math/add".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "a".to_string(),
                        name: "a".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                    PortDecl {
                        port_id: "b".to_string(),
                        name: "b".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/add_graph@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_stream_merge_policy() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with multiple producers feeding a stream input
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/stream_merge@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "stream_merge".to_string(),
        title: "Stream Merge Test".to_string(),
        description: "Test stream merge policies".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "producer1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "producer2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "consumer".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/consumer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "producer1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "producer2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/stream_merge@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully with merge adapters
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_stream_merge_no_adapter() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with multiple producers feeding a stream input without merge adapters
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/stream_merge_no_adapter@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "stream_merge_no_adapter".to_string(),
        title: "Stream Merge No Adapter Test".to_string(),
        description: "Test stream merge without adapters (should fail)".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "producer1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "producer2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "consumer".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/consumer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "producer1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "producer2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/stream_merge_no_adapter@0.1.0".to_string(), graph_handle);
    
    // This should fail validation because there are multiple producers but no merge adapters
    let result = validate_registry(&reg);
    assert!(result.is_err(), "Validation should fail when multiple producers feed a stream without merge adapters");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("multiple producers") && error_msg.contains("merge adapter"),
            "Error message should mention multiple producers and missing merge adapter");
    
    Ok(())
}

#[test]
fn test_stream_merge_mixed_adapters() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with multiple producers feeding a stream input with mixed adapters
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/stream_merge_mixed_adapters@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "stream_merge_mixed_adapters".to_string(),
        title: "Stream Merge Mixed Adapters Test".to_string(),
        description: "Test stream merge with mixed adapters (should fail)".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "producer1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "producer2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "consumer".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/consumer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "producer1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "producer2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Map,
                    adapter_params: Some(AdapterParams::Map { transform: None }),
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/stream_merge_mixed_adapters@0.1.0".to_string(), graph_handle);
    
    // This should fail validation because not all edges have merge adapters
    let result = validate_registry(&reg);
    assert!(result.is_err(), "Validation should fail when edges have mixed adapter types");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("inappropriate adapter"),
            "Error message should mention inappropriate adapter");
    
    Ok(())
}

#[test]
fn test_stream_merge_incompatible_types() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with multiple producers feeding a stream input with incompatible types
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/stream_merge_incompatible_types@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "stream_merge_incompatible_types".to_string(),
        title: "Stream Merge Incompatible Types Test".to_string(),
        description: "Test stream merge with incompatible types (should fail)".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "producer1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "producer2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<string>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "consumer".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/consumer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "producer1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "producer2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/stream_merge_incompatible_types@0.1.0".to_string(), graph_handle);
    
    // This should fail validation because the types are incompatible
    let result = validate_registry(&reg);
    assert!(result.is_err(), "Validation should fail when types are incompatible");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("not compatible"),
            "Error message should mention type incompatibility");
    
    Ok(())
}

#[test]
fn test_stream_merge_with_single_producer() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with a single producer feeding a stream input (should work without merge adapter)
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/stream_merge_single_producer@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "stream_merge_single_producer".to_string(),
        title: "Stream Merge Single Producer Test".to_string(),
        description: "Test stream merge with single producer (should pass)".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "producer1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "consumer".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/consumer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "producer1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/stream_merge_single_producer@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully - single producer doesn't need merge adapter
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_stream_merge_with_complex_types() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with multiple producers feeding a stream input with complex types
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/stream_merge_complex_types@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "stream_merge_complex_types".to_string(),
        title: "Stream Merge Complex Types Test".to_string(),
        description: "Test stream merge with complex types (should pass)".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "producer1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<Struct{id:i64,name:string}>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "producer2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/producer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<Struct{id:i64,name:string}>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "consumer".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/consumer".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<Struct{id:i64,name:string}>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "producer1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "producer2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "consumer".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::Merge,
                    adapter_params: Some(AdapterParams::Merge { strategy: "round_robin".to_string() }),
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/stream_merge_complex_types@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully with merge adapters and compatible complex types
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_struct_enum_types() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a block with struct and enum types
    let block = BlockHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "data_processor".to_string(),
            title: "Data Processor".to_string(),
            description: "Process structured data".to_string(),
            id: "test/data_processor@0.1.0".to_string(),
            namespace: "test".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    port_id: "data".to_string(),
                    name: "data".to_string(),
                    ty: "Struct{name:string,value:i64}".to_string(),
                    description: "Input data structure".to_string(),
                    kind: None,
                    default: None,
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "result".to_string(),
                    name: "result".to_string(),
                    ty: "Enum{Success,Error}".to_string(),
                    description: "Processing result".to_string(),
                    kind: None,
                    default: None,
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
        },
    };
    
    reg.blocks.insert("test/data_processor@0.1.0".to_string(), block);
    
    // This should validate successfully
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_generic_bounds() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a block with generic parameters and bounds
    let block = BlockHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "generic_processor".to_string(),
            title: "Generic Processor".to_string(),
            description: "Process generic data".to_string(),
            id: "test/generic_processor@0.1.0".to_string(),
            namespace: "test".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                PortSpec {
                    port_id: "data".to_string(),
                    name: "data".to_string(),
                    ty: "T".to_string(),
                    description: "Input data".to_string(),
                    kind: None,
                    default: None,
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "result".to_string(),
                    name: "result".to_string(),
                    ty: "T".to_string(),
                    description: "Processed data".to_string(),
                    kind: None,
                    default: None,
                },
            ],
            params: vec![],
            generics: vec![
                GenericParam {
                    name: "T".to_string(),
                    bounds: vec!["Clone".to_string()],
                },
            ],
            engine: EngineReq {
                version_req: "0.2.0".to_string(),
                capability_flags: vec![],
            },
            examples: vec![],
            integrity: None,
        },
    };
    
    reg.blocks.insert("test/generic_processor@0.1.0".to_string(), block);
    
    // This should validate successfully
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_no_cycles() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a simple acyclic graph
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/no_cycles@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "no_cycles".to_string(),
        title: "No Cycles Test".to_string(),
        description: "Test validation of a simple acyclic graph".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "node1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "node2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "node3".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "node1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node2".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "node2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node3".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/no_cycles@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully - no cycles
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_invalid_cycle_without_stateful_breaker() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with a cycle that has no stateful-breaker node
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/invalid_cycle@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "invalid_cycle".to_string(),
        title: "Invalid Cycle Test".to_string(),
        description: "Test validation of a graph with a cycle that has no stateful-breaker node".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "node1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "node2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "node1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node2".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "node2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node1".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/invalid_cycle@0.1.0".to_string(), graph_handle);
    
    // This should fail validation due to invalid cycle
    let result = validate_registry(&reg);
    assert!(result.is_err(), "Validation should fail when invalid cycle is detected");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("cycle") && error_msg.contains("stateful"),
            "Error message should mention cycle and stateful-breaker");
    
    Ok(())
}

#[test]
fn test_valid_cycle_with_stateful_breaker() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with a cycle that has a stateful-breaker node
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/valid_cycle@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "valid_cycle".to_string(),
        title: "Valid Cycle Test".to_string(),
        description: "Test validation of a graph with a cycle that has a stateful-breaker node".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "node1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "fold_node".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("std.stream/fold".to_string()), // Stateful-breaker
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Impure),
            },
        ],
        edges: vec![
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "node1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "fold_node".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "fold_node".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node1".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/valid_cycle@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully - valid cycle with stateful-breaker
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_multiple_cycles_mixed_validity() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a graph with multiple cycles, some valid and some invalid
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/multiple_cycles@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "multiple_cycles".to_string(),
        title: "Multiple Cycles Test".to_string(),
        description: "Test validation of a graph with multiple cycles, some valid and some invalid".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "node1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "node2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "node3".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/simple".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "fold_node".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("std.stream/fold".to_string()), // Stateful-breaker
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Impure),
            },
        ],
        edges: vec![
            // Invalid cycle: node1 -> node2 -> node1
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "node1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node2".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "node2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node1".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            // Valid cycle: node1 -> node3 -> fold_node -> node1
            Edge {
                id: "edge3".to_string(),
                from: Endpoint { node: "node1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node3".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge4".to_string(),
                from: Endpoint { node: "node3".to_string(), port: "out".to_string() },
                to: Endpoint { node: "fold_node".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge5".to_string(),
                from: Endpoint { node: "fold_node".to_string(), port: "out".to_string() },
                to: Endpoint { node: "node1".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/multiple_cycles@0.1.0".to_string(), graph_handle);
    
    // This should fail validation due to the invalid cycle
    let result = validate_registry(&reg);
    assert!(result.is_err(), "Validation should fail when invalid cycle is detected");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("cycle") && error_msg.contains("stateful"),
            "Error message should mention cycle and stateful-breaker");
    
    Ok(())
}

#[test]
fn test_complex_graph_with_valid_cycle() -> Result<()> {
    let mut reg = Registry::default();
    
    // Create a complex graph with a valid feedback loop
    let graph = GraphSpec {
        schema_version: "0.2".to_string(),
        id: "test/complex_cycle@0.1.0".to_string(),
        namespace: "test".to_string(),
        name: "complex_cycle".to_string(),
        title: "Complex Cycle Test".to_string(),
        description: "Test validation of a more complex graph with a valid feedback loop".to_string(),
        version: "0.1.0".to_string(),
        generics: vec![],
        requires: vec![],
        effects: vec![],
        exports: vec![],
        nodes: vec![
            Node {
                id: "source".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/source".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "transform1".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/transform".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "transform2".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/transform".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
            Node {
                id: "accumulator".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("std.stream/accumulator".to_string()), // Stateful-breaker
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "Stream<i64>".to_string(),
                        kind: PortKind::Stream,
                    },
                ],
                outputs: vec![
                    PortDecl {
                        port_id: "out".to_string(),
                        name: "out".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                effects: vec![],
                purity: Some(Purity::Impure),
            },
            Node {
                id: "sink".to_string(),
                kind: shtairir_registry::model::NodeKind::Block,
                fq_block: Some("test/sink".to_string()),
                version_req: Some("^0.1".to_string()),
                inputs: vec![
                    PortDecl {
                        port_id: "in".to_string(),
                        name: "in".to_string(),
                        ty: "i64".to_string(),
                        kind: PortKind::Value,
                    },
                ],
                outputs: vec![],
                effects: vec![],
                purity: Some(Purity::Pure),
            },
        ],
        edges: vec![
            // Main flow: source -> transform1 -> transform2 -> accumulator -> sink
            Edge {
                id: "edge1".to_string(),
                from: Endpoint { node: "source".to_string(), port: "out".to_string() },
                to: Endpoint { node: "transform1".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge2".to_string(),
                from: Endpoint { node: "transform1".to_string(), port: "out".to_string() },
                to: Endpoint { node: "transform2".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge3".to_string(),
                from: Endpoint { node: "transform2".to_string(), port: "out".to_string() },
                to: Endpoint { node: "accumulator".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            Edge {
                id: "edge4".to_string(),
                from: Endpoint { node: "accumulator".to_string(), port: "out".to_string() },
                to: Endpoint { node: "sink".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
            // Valid feedback loop: accumulator -> transform1
            Edge {
                id: "edge5".to_string(),
                from: Endpoint { node: "accumulator".to_string(), port: "out".to_string() },
                to: Endpoint { node: "transform1".to_string(), port: "in".to_string() },
                policy: EdgePolicy {
                    adapter: shtairir_registry::model::AdapterKind::None,
                    ..Default::default()
                },
            },
        ],
        engine: EngineReq {
            version_req: "0.2.0".to_string(),
            capability_flags: vec![],
        },
        integrity: None,
    };
    
    let graph_handle = GraphHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: graph,
    };
    
    reg.graphs.insert("test/complex_cycle@0.1.0".to_string(), graph_handle);
    
    // This should validate successfully - valid cycle with stateful-breaker
    validate_registry(&reg)?;
    
    Ok(())
}
// Test that our new ValueLiteral implementation works correctly in validation tests

#[test]
fn test_default_value_validation() -> Result<()> {
    let mut reg = Registry::default();
    
    // Add a block with various default values to test validation
    let block = BlockHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "default_test".to_string(),
            title: "Default Test".to_string(),
            description: "Test block with various default values".to_string(),
            id: "test/default_test@0.1.0".to_string(),
            namespace: "test".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                // Test scalar defaults
                PortSpec {
                    port_id: "int_val".to_string(),
                    name: "int_val".to_string(),
                    ty: "i64".to_string(),
                    description: "Integer with default".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::i64(42)),
                },
                PortSpec {
                    port_id: "float_val".to_string(),
                    name: "float_val".to_string(),
                    ty: "f64".to_string(),
                    description: "Float with default".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::f64(3.14)),
                },
                PortSpec {
                    port_id: "bool_val".to_string(),
                    name: "bool_val".to_string(),
                    ty: "bool".to_string(),
                    description: "Boolean with default".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::bool(true)),
                },
                PortSpec {
                    port_id: "string_val".to_string(),
                    name: "string_val".to_string(),
                    ty: "string".to_string(),
                    description: "String with default".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::string("hello")),
                },
                // Test option defaults
                PortSpec {
                    port_id: "option_some".to_string(),
                    name: "option_some".to_string(),
                    ty: "option<i64>".to_string(),
                    description: "Option with some value".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::option_some(ValueLiteral::i64(100))),
                },
                PortSpec {
                    port_id: "option_none".to_string(),
                    name: "option_none".to_string(),
                    ty: "option<i64>".to_string(),
                    description: "Option with none value".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::option_none()),
                },
                // Test list defaults
                PortSpec {
                    port_id: "empty_list".to_string(),
                    name: "empty_list".to_string(),
                    ty: "list<i64>".to_string(),
                    description: "Empty list".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::list(vec![])),
                },
                PortSpec {
                    port_id: "int_list".to_string(),
                    name: "int_list".to_string(),
                    ty: "list<i64>".to_string(),
                    description: "List of integers".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::list(vec![
                        ValueLiteral::i64(1),
                        ValueLiteral::i64(2),
                        ValueLiteral::i64(3),
                    ])),
                },
                // Test map defaults
                PortSpec {
                    port_id: "empty_map".to_string(),
                    name: "empty_map".to_string(),
                    ty: "map<string,i64>".to_string(),
                    description: "Empty map".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::object(std::collections::HashMap::new())),
                },
                PortSpec {
                    port_id: "string_int_map".to_string(),
                    name: "string_int_map".to_string(),
                    ty: "map<string,i64>".to_string(),
                    description: "Map with string keys and int values".to_string(),
                    kind: None,
                    default: {
                        let mut map = std::collections::HashMap::new();
                        map.insert("a".to_string(), ValueLiteral::i64(1));
                        map.insert("b".to_string(), ValueLiteral::i64(2));
                        Some(ValueLiteral::object(map))
                    },
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "out".to_string(),
                    name: "out".to_string(),
                    ty: "i64".to_string(),
                    description: "Output".to_string(),
                    kind: None,
                    default: None,
                },
            ],
            params: vec![
                // Test param defaults similar to inputs
                PortSpec {
                    port_id: "param_int".to_string(),
                    name: "param_int".to_string(),
                    ty: "i64".to_string(),
                    description: "Parameter with integer default".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::i64(10)),
                },
            ],
            generics: vec![],
            engine: EngineReq {
                version_req: "0.2.0".to_string(),
                capability_flags: vec![],
            },
            examples: vec![],
            integrity: None,
        },
    };
    
    reg.blocks.insert("test/default_test@0.1.0".to_string(), block);
    
    // This should validate successfully - all defaults are compatible with their types
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_nested_composite_defaults() -> Result<()> {
    let mut reg = Registry::default();
    
    // Test deeply nested composite types
    let block = BlockHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "nested_test".to_string(),
            title: "Nested Test".to_string(),
            description: "Test block with nested composite defaults".to_string(),
            id: "test/nested_test@0.1.0".to_string(),
            namespace: "test".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                // Test nested lists
                PortSpec {
                    port_id: "nested_list".to_string(),
                    name: "nested_list".to_string(),
                    ty: "list<list<i64>>".to_string(),
                    description: "Nested list of integers".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::list(vec![
                        ValueLiteral::list(vec![
                            ValueLiteral::i64(1),
                            ValueLiteral::i64(2),
                        ]),
                        ValueLiteral::list(vec![
                            ValueLiteral::i64(3),
                            ValueLiteral::i64(4),
                        ]),
                    ])),
                },
                // Test nested maps
                PortSpec {
                    port_id: "nested_map".to_string(),
                    name: "nested_map".to_string(),
                    ty: "map<string,map<string,i64>>".to_string(),
                    description: "Nested map".to_string(),
                    kind: None,
                    default: {
                        let mut outer_map = std::collections::HashMap::new();
                        let mut inner_map1 = std::collections::HashMap::new();
                        inner_map1.insert("a".to_string(), ValueLiteral::i64(1));
                        inner_map1.insert("b".to_string(), ValueLiteral::i64(2));
                        let mut inner_map2 = std::collections::HashMap::new();
                        inner_map2.insert("c".to_string(), ValueLiteral::i64(3));
                        inner_map2.insert("d".to_string(), ValueLiteral::i64(4));
                        outer_map.insert("first".to_string(), ValueLiteral::object(inner_map1));
                        outer_map.insert("second".to_string(), ValueLiteral::object(inner_map2));
                        Some(ValueLiteral::object(outer_map))
                    },
                },
                // Test option with nested types
                PortSpec {
                    port_id: "option_nested".to_string(),
                    name: "option_nested".to_string(),
                    ty: "option<list<i64>>".to_string(),
                    description: "Option with nested list".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::option_some(ValueLiteral::list(vec![
                        ValueLiteral::i64(10),
                        ValueLiteral::i64(20),
                    ]))),
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "out".to_string(),
                    name: "out".to_string(),
                    ty: "i64".to_string(),
                    description: "Output".to_string(),
                    kind: None,
                    default: None,
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
        },
    };
    
    reg.blocks.insert("test/nested_test@0.1.0".to_string(), block);
    
    // This should validate successfully - all nested defaults are compatible
    validate_registry(&reg)?;
    
    Ok(())
}

#[test]
fn test_default_type_mismatch_fails() -> Result<()> {
    let mut reg = Registry::default();
    
    // Add a block with an incompatible default value
    let block = BlockHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "mismatch_test".to_string(),
            title: "Mismatch Test".to_string(),
            description: "Test block with type mismatch in defaults".to_string(),
            id: "test/mismatch_test@0.1.0".to_string(),
            namespace: "test".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                // This should fail - string default for i64 type
                PortSpec {
                    port_id: "bad_default".to_string(),
                    name: "bad_default".to_string(),
                    ty: "i64".to_string(),
                    description: "Bad default value".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::string("not a number".to_string())),
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "out".to_string(),
                    name: "out".to_string(),
                    ty: "i64".to_string(),
                    description: "Output".to_string(),
                    kind: None,
                    default: None,
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
        },
    };
    
    reg.blocks.insert("test/mismatch_test@0.1.0".to_string(), block);
    
    // This should fail validation due to type mismatch
    let result = validate_registry(&reg);
    assert!(result.is_err(), "Validation should fail when default value type doesn't match port type");
    
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("default") && error_msg.contains("compatible"),
            "Error message should mention default value incompatibility: {}", error_msg);
    
    Ok(())
}

#[test]
fn test_mixed_option_list_defaults() -> Result<()> {
    let mut reg = Registry::default();
    
    // Test mixed option and list types
    let block = BlockHandle {
        module: "test".to_string(),
        version: "0.1.0".to_string(),
        spec: BlockSpec {
            name: "mixed_test".to_string(),
            title: "Mixed Test".to_string(),
            description: "Test block with mixed option and list defaults".to_string(),
            id: "test/mixed_test@0.1.0".to_string(),
            namespace: "test".to_string(),
            version: "0.1.0".to_string(),
            schema_version: "0.2".to_string(),
            purity: Purity::Pure,
            determinism: Determinism::Deterministic,
            effects: vec![],
            inputs: vec![
                // Test list<option<i64>> with mixed values
                PortSpec {
                    port_id: "mixed_option_list".to_string(),
                    name: "mixed_option_list".to_string(),
                    ty: "list<option<i64>>".to_string(),
                    description: "List with mixed option values".to_string(),
                    kind: None,
                    default: Some(ValueLiteral::list(vec![
                        ValueLiteral::option_some(ValueLiteral::i64(1)),
                        ValueLiteral::option_none(),
                        ValueLiteral::option_some(ValueLiteral::i64(3)),
                    ])),
                },
            ],
            outputs: vec![
                PortSpec {
                    port_id: "out".to_string(),
                    name: "out".to_string(),
                    ty: "i64".to_string(),
                    description: "Output".to_string(),
                    kind: None,
                    default: None,
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
        },
    };
    
    reg.blocks.insert("test/mixed_test@0.1.0".to_string(), block);
    
    // This should validate successfully - mixed option/list defaults are compatible
    validate_registry(&reg)?;
    
    Ok(())
}
}