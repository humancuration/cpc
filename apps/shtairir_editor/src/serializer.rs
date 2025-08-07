use crate::models::{Graph, Node, Connection, EdgePolicy, AdapterKind, Backpressure, Ordering};
use shtairir_registry::model::{GraphSpec, Node as RegistryNode, Edge, Endpoint, PortDecl, PortKind, NodeKind, EdgePolicy as RegistryEdgePolicy, AdapterKind as RegistryAdapterKind, Backpressure as RegistryBackpressure, Ordering as RegistryOrdering};
use anyhow::Result;

pub struct Serializer;

impl Serializer {
    pub fn graph_to_toml(graph: &Graph) -> Result<String> {
        let graph_spec = Self::convert_to_graph_spec(graph)?;
        let toml_string = toml::to_string_pretty(&graph_spec)?;
        Ok(toml_string)
    }
    
    pub fn toml_to_graph(toml_str: &str) -> Result<Graph> {
        let graph_spec: GraphSpec = toml::from_str(toml_str)?;
        let graph = Self::convert_from_graph_spec(graph_spec)?;
        Ok(graph)
    }
    
    fn convert_to_graph_spec(graph: &Graph) -> Result<GraphSpec> {
        use shtairir_registry::model::*;
        
        let nodes: Vec<RegistryNode> = graph.nodes.values().map(|node| {
            RegistryNode {
                id: node.id.clone(),
                kind: NodeKind::Block,
                fq_block: Some(format!("{}@{}:{}", node.block_spec.namespace, node.block_spec.version, node.block_spec.name)),
                version_req: Some("^0.1".to_string()),
                concrete_version: Some(node.block_spec.version.clone()),
                title: Some(node.block_spec.title.clone()),
                purity: Some(node.block_spec.purity.clone()),
                effects: node.block_spec.effects.clone(),
                generics: std::collections::BTreeMap::new(),
                params: shtairir_registry::literal::ValueLiteral::Object(std::collections::BTreeMap::new()), // Simplified
                inputs: node.input_ports.iter().map(|port| PortDecl {
                    name: port.name.clone(),
                    port_id: port.id.clone(),
                    ty: port.port_type.clone(),
                    kind: PortKind::Value,
                }).collect(),
                outputs: node.output_ports.iter().map(|port| PortDecl {
                    name: port.name.clone(),
                    port_id: port.id.clone(),
                    ty: port.port_type.clone(),
                    kind: PortKind::Value,
                }).collect(),
                meta: None,
            }
        }).collect();
        
        let edges: Vec<Edge> = graph.connections.iter().map(|conn| {
            Edge {
                id: conn.id.clone(),
                from: Endpoint {
                    node: conn.from_node.clone(),
                    port: conn.from_port.clone(),
                },
                to: Endpoint {
                    node: conn.to_node.clone(),
                    port: conn.to_port.clone(),
                },
                policy: match &conn.policy {
                    Some(policy) => Self::convert_edge_policy(policy),
                    None => EdgePolicy::default(),
                },
                notes: None,
            }
        }).collect();
        
        Ok(GraphSpec {
            schema_version: "0.2".to_string(),
            id: format!("graph:local/{}@{}", graph.name, graph.version),
            namespace: "local".to_string(),
            name: graph.name.clone(),
            version: graph.version.clone(),
            title: graph.name.clone(),
            description: format!("Graph created in Shtairir Visual Editor"),
            authors: vec!["Shtairir Editor User".to_string()],
            tags: vec![],
            visibility: "public".to_string(),
            generics: vec![],
            requires: vec![],
            effects: vec![],
            exports: vec![],
            nodes,
            edges,
            engine: EngineReq {
                version_req: "^0.2".to_string(),
                capability_flags: vec![],
            },
            integrity: None,
            provenance: None,
            metadata: None,
            annotations: None,
        })
    }
    
    fn convert_from_graph_spec(graph_spec: GraphSpec) -> Result<Graph> {
        // This would convert a GraphSpec back to our internal Graph representation
        // For now, we'll create a basic graph
        Ok(Graph::new())
    }
    
    fn convert_edge_policy(policy: &EdgePolicy) -> RegistryEdgePolicy {
        use shtairir_registry::model::*;
        
        RegistryEdgePolicy {
            adapter: match &policy.adapter {
                AdapterKind::None => RegistryAdapterKind::None,
                AdapterKind::Map => RegistryAdapterKind::Map,
                AdapterKind::Filter => RegistryAdapterKind::Filter,
                AdapterKind::Buffer => RegistryAdapterKind::Buffer,
                AdapterKind::Window => RegistryAdapterKind::Window,
                AdapterKind::Debounce => RegistryAdapterKind::Debounce,
                AdapterKind::Merge => RegistryAdapterKind::Merge,
                AdapterKind::Zip => RegistryAdapterKind::Zip,
                AdapterKind::Boundary => RegistryAdapterKind::Boundary,
            },
            adapter_params: None, // Simplified
            backpressure: match &policy.backpressure {
                Backpressure::Block => RegistryBackpressure::Block,
                Backpressure::DropOldest => RegistryBackpressure::DropOldest,
                Backpressure::DropNewest => RegistryBackpressure::DropNewest,
                Backpressure::Expand => RegistryBackpressure::Expand,
            },
            ordering: match &policy.ordering {
                Ordering::Source => RegistryOrdering::Source,
                Ordering::Timestamp => RegistryOrdering::Timestamp,
                Ordering::StableKey => RegistryOrdering::StableKey,
            },
            timestamp_source: shtairir_registry::model::TimestampSource::Inherit,
        }
    }
}