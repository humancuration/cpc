use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use shtairir::ast::{Command, Value, Script};
use shtairir_registry::model::{BlockSpec, PortSpec};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub block_spec: BlockSpec,
    pub position: (i32, i32),
    pub input_ports: Vec<Port>,
    pub output_ports: Vec<Port>,
    pub params: HashMap<String, Value>,
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

impl Node {
    pub fn new(id: String, block_spec: BlockSpec, position: (i32, i32)) -> Self {
        let input_ports = block_spec.inputs.iter().map(|spec| Port::from_spec(spec, true)).collect();
        let output_ports = block_spec.outputs.iter().map(|spec| Port::from_spec(spec, false)).collect();
        
        // Initialize params with defaults
        let mut params = HashMap::new();
        for param in &block_spec.params {
            if let Some(default_value) = &param.default {
                // Convert ValueLiteral to Value (this would need proper implementation)
                params.insert(param.name.clone(), Value::String("default".to_string()));
            }
        }
        
        Self {
            id,
            block_spec,
            position,
            input_ports,
            output_ports,
            params,
            status: None,
        }
    }
    
    pub fn get_input_index(&self, port_id: &str) -> Option<usize> {
        self.input_ports.iter().position(|p| p.id == port_id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Port {
    pub id: String,
    pub name: String,
    pub port_type: String, // This should be a proper Type enum
    pub is_input: bool,
    pub connections: Vec<String>, // Connection IDs
}

impl Port {
    pub fn from_spec(spec: &PortSpec, is_input: bool) -> Self {
        Self {
            id: format!("{}_{}", spec.name, if is_input { "in" } else { "out" }),
            name: spec.name.clone(),
            port_type: spec.ty.clone(),
            is_input,
            connections: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Connection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
    pub policy: Option<EdgePolicy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgePolicy {
    pub adapter: AdapterKind,
    pub backpressure: Backpressure,
    pub ordering: Ordering,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdapterKind {
    None,
    Map,
    Filter,
    Buffer,
    Window,
    Debounce,
    Merge,
    Zip,
    Boundary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Backpressure {
    Block,
    DropOldest,
    DropNewest,
    Expand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ordering {
    Source,
    Timestamp,
    StableKey,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub connections: Vec<Connection>,
    pub name: String,
    pub version: String,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: Vec::new(),
            name: "Untitled Graph".to_string(),
            version: "0.1.0".to_string(),
        }
    }
    
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }
    
    pub fn remove_node(&mut self, node_id: &str) {
        self.nodes.remove(node_id);
        self.connections.retain(|c| c.from_node != node_id && c.to_node != node_id);
    }
    
    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }
    
    pub fn remove_connection(&mut self, connection_id: &str) {
        self.connections.retain(|c| c.id != connection_id);
    }
    
    pub fn to_script(&self) -> Script {
        let mut commands = vec![];
        let mut visited = std::collections::HashSet::new();
        
        // Find starting nodes (no incoming connections)
        let start_nodes = self.nodes.values()
            .filter(|node| !self.connections.iter().any(|c| c.to_node == node.id))
            .collect::<Vec<_>>();
        
        for node in start_nodes {
            self.traverse_node(node, &mut commands, &mut visited);
        }
        
        Script { commands }
    }
    
    fn traverse_node(&self, node: &Node, commands: &mut Vec<Command>, visited: &mut std::collections::HashSet<String>) {
        if visited.contains(&node.id) {
            return;
        }
        visited.insert(node.id.clone());
        
        // Create command with resolved arguments
        let mut args = Vec::new();
        for (param_name, param_value) in &node.params {
            args.push(param_value.clone());
        }
        
        let mut command = Command {
            app: node.block_spec.namespace.clone(),
            function: node.block_spec.name.clone(),
            args,
        };
        
        // Process output connections
        for conn in self.connections.iter().filter(|c| c.from_node == node.id) {
            if let Some(target_node) = self.nodes.get(&conn.to_node) {
                // Handle argument passing through connections
                if let Some(arg_index) = target_node.get_input_index(&conn.to_port) {
                    command.args[arg_index] = Value::Identifier(format!("{}.{}", node.id, conn.from_port));
                }
                self.traverse_node(target_node, commands, visited);
            }
        }
        
        commands.push(command);
    }
}