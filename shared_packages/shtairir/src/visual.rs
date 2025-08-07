//! Visual programming components for Shtairir
//! 
//! This module defines the node and edge structures used in the visual
//! programming interface, along with their specifications and policies.

use crate::block::{Block, BlockId, PortId, InputPort, OutputPort, PortKind, Value};
use std::collections::HashMap;
use std::sync::Arc;

/// Unique identifier for a node
pub type NodeId = String;

/// Unique identifier for an edge
pub type EdgeId = String;

/// Position in the visual editor
#[derive(Debug, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Visual properties for nodes and edges
#[derive(Debug, Clone)]
pub struct VisualProperties {
    pub color: Option<String>,
    pub icon: Option<String>,
    pub size: Option<(f64, f64)>,
    pub style: Option<String>,
}

/// Node in a visual graph
#[derive(Debug, Clone)]
pub struct VisualNode {
    /// Unique identifier for the node
    pub id: NodeId,
    
    /// Position in the visual editor
    pub position: Position,
    
    /// Block that this node represents
    pub block: Arc<dyn Block>,
    
    /// Input ports
    pub inputs: Vec<InputPort>,
    
    /// Output ports
    pub outputs: Vec<OutputPort>,
    
    /// Visual properties
    pub visual_properties: VisualProperties,
    
    /// User data
    pub user_data: HashMap<String, Value>,
}

impl VisualNode {
    /// Create a new visual node
    pub fn new(id: NodeId, block: Arc<dyn Block>) -> Self {
        Self {
            id,
            position: Position { x: 0.0, y: 0.0 },
            block,
            inputs: vec![], // Will be populated from block spec
            outputs: vec![], // Will be populated from block spec
            visual_properties: VisualProperties {
                color: None,
                icon: None,
                size: None,
                style: None,
            },
            user_data: HashMap::new(),
        }
    }
    
    /// Set position
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.position = Position { x, y };
        self
    }
    
    /// Set visual properties
    pub fn with_visual_properties(mut self, properties: VisualProperties) -> Self {
        self.visual_properties = properties;
        self
    }
    
    /// Add user data
    pub fn with_user_data(mut self, key: String, value: Value) -> Self {
        self.user_data.insert(key, value);
        self
    }
}

/// Edge connecting nodes
#[derive(Debug, Clone)]
pub struct VisualEdge {
    /// Unique identifier for the edge
    pub id: EdgeId,
    
    /// Source node and port
    pub source: EdgeEndpoint,
    
    /// Target node and port
    pub target: EdgeEndpoint,
    
    /// Edge policy for data flow control
    pub policy: EdgePolicy,
    
    /// Visual properties
    pub visual_properties: VisualProperties,
}

impl VisualEdge {
    /// Create a new visual edge
    pub fn new(id: EdgeId, source: EdgeEndpoint, target: EdgeEndpoint) -> Self {
        Self {
            id,
            source,
            target,
            policy: EdgePolicy::default(),
            visual_properties: VisualProperties {
                color: None,
                icon: None,
                size: None,
                style: None,
            },
        }
    }
    
    /// Set edge policy
    pub fn with_policy(mut self, policy: EdgePolicy) -> Self {
        self.policy = policy;
        self
    }
    
    /// Set visual properties
    pub fn with_visual_properties(mut self, properties: VisualProperties) -> Self {
        self.visual_properties = properties;
        self
    }
}

/// Endpoint for edges
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeEndpoint {
    pub node_id: NodeId,
    pub port_id: PortId,
}

/// Policy for edge behavior
#[derive(Debug, Clone)]
pub struct EdgePolicy {
    /// Adapter for data transformation
    pub adapter: EdgeAdapter,
    
    /// Backpressure strategy
    pub backpressure: BackpressureStrategy,
    
    /// Ordering strategy
    pub ordering: OrderingStrategy,
    
    /// Buffering strategy
    // TODO: Implement buffering strategy
    // pub buffering: BufferingStrategy,
    
    /// Error handling strategy
    // TODO: Implement error handling strategy
    // pub error_handling: ErrorHandlingStrategy,
}

impl Default for EdgePolicy {
    fn default() -> Self {
        Self {
            adapter: EdgeAdapter::None,
            backpressure: BackpressureStrategy::Block,
            ordering: OrderingStrategy::Source,
        }
    }
}

/// Edge adapter
#[derive(Debug, Clone)]
pub enum EdgeAdapter {
    /// No adapter (direct connection)
    None,
    /// Map adapter (transform data)
    Map(MapAdapter),
    /// Filter adapter (filter data)
    Filter(FilterAdapter),
    /// Buffer adapter (buffer data)
    Buffer(BufferAdapter),
    /// Window adapter (window operations)
    Window(WindowAdapter),
    /// Debounce adapter (debounce events)
    Debounce(DebounceAdapter),
    /// Merge adapter (merge multiple streams)
    Merge(MergeAdapter),
    /// Zip adapter (combine streams)
    Zip(ZipAdapter),
    /// Boundary adapter (boundary detection)
    Boundary(BoundaryAdapter),
}

/// Map adapter for data transformation
#[derive(Debug, Clone)]
pub struct MapAdapter {
    pub transform_function: String, // TODO: Define proper function type
}

/// Filter adapter for data filtering
#[derive(Debug, Clone)]
pub struct FilterAdapter {
    pub predicate: String, // TODO: Define proper predicate type
}

/// Buffer adapter for data buffering
#[derive(Debug, Clone)]
pub struct BufferAdapter {
    pub capacity: usize,
}

/// Window adapter for window operations
#[derive(Debug, Clone)]
pub struct WindowAdapter {
    pub size: usize,
    pub slide: Option<usize>,
}

/// Debounce adapter for event debouncing
#[derive(Debug, Clone)]
pub struct DebounceAdapter {
    pub delay_ms: u64,
}

/// Merge adapter for merging streams
#[derive(Debug, Clone)]
pub struct MergeAdapter {
    pub strategy: String, // e.g., "round_robin", "priority", "zip"
}

/// Zip adapter for combining streams
#[derive(Debug, Clone)]
pub struct ZipAdapter {
    // Zip adapter doesn't need additional parameters
}

/// Boundary adapter for boundary detection
#[derive(Debug, Clone)]
pub struct BoundaryAdapter {
    // Boundary adapter doesn't need additional parameters
}

/// Backpressure strategy
#[derive(Debug, Clone)]
pub enum BackpressureStrategy {
    /// Block when downstream is full
    Block,
    /// Drop oldest data when downstream is full
    DropOldest,
    /// Drop newest data when downstream is full
    DropNewest,
    /// Expand buffer when full
    Expand,
}

/// Ordering strategy
#[derive(Debug, Clone)]
pub enum OrderingStrategy {
    /// Preserve source ordering
    Source,
    /// Order by timestamp
    Timestamp,
    /// Order by stable key
    StableKey,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::{BlockSpec, Determinism, Purity};
    use async_trait::async_trait;
    use shtairir_core::error::ShtairirError;
    
    // Mock block implementation for testing
    struct MockBlock {
        spec: BlockSpec,
    }
    
    #[async_trait]
    impl Block for MockBlock {
        fn spec(&self) -> &BlockSpec {
            &self.spec
        }
        
        async fn execute(&self, _inputs: &crate::block::BlockInputs, _context: &crate::block::ExecutionContext) -> Result<crate::block::BlockOutputs, ShtairirError> {
            Ok(crate::block::BlockOutputs::new())
        }
        
        fn validate(&self, _params: &crate::block::BlockParams) -> Result<(), crate::block::ValidationError> {
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
    fn test_visual_node_creation() {
        // Create a mock block spec (simplified for testing)
        let block_spec = BlockSpec {
            id: "test.block@1.0.0".to_string(),
            namespace: "test".to_string(),
            name: "test_block".to_string(),
            version: "1.0.0".to_string(),
            title: "Test Block".to_string(),
            description: "A test block".to_string(),
            authors: vec![],
            license: "CPC".to_string(),
            tags: vec![],
            purity: Purity::Pure,
            effects: vec![],
            determinism: Determinism::Deterministic,
            generics: vec![],
            inputs: vec![],
            outputs: vec![],
            params: vec![],
            examples: vec![],
            tests: vec![],
            engine: shtairir_registry::model::EngineReq {
                version_req: "^0.2".to_string(),
                capability_flags: vec![],
            },
            integrity: None,
            metadata: None,
        };
        
        let mock_block = MockBlock { spec: block_spec };
        let block_arc: Arc<dyn Block> = Arc::new(mock_block);
        
        let node = VisualNode::new("node1".to_string(), block_arc)
            .with_position(100.0, 200.0)
            .with_visual_properties(VisualProperties {
                color: Some("#FF0000".to_string()),
                icon: Some("test-icon".to_string()),
                size: Some((50.0, 30.0)),
                style: Some("filled".to_string()),
            });
        
        assert_eq!(node.id, "node1");
        assert_eq!(node.position.x, 100.0);
        assert_eq!(node.position.y, 200.0);
        assert_eq!(node.visual_properties.color, Some("#FF0000".to_string()));
    }
    
    #[test]
    fn test_visual_edge_creation() {
        let edge = VisualEdge::new(
            "edge1".to_string(),
            EdgeEndpoint {
                node_id: "node1".to_string(),
                port_id: "output1".to_string(),
            },
            EdgeEndpoint {
                node_id: "node2".to_string(),
                port_id: "input1".to_string(),
            },
        ).with_policy(EdgePolicy {
            adapter: EdgeAdapter::None,
            backpressure: BackpressureStrategy::DropOldest,
            ordering: OrderingStrategy::Timestamp,
        });
        
        assert_eq!(edge.id, "edge1");
        assert_eq!(edge.source.node_id, "node1");
        assert_eq!(edge.target.port_id, "input1");
        match edge.policy.backpressure {
            BackpressureStrategy::DropOldest => {}, // Test passes
            _ => panic!("Expected DropOldest backpressure strategy"),
        }
    }
}