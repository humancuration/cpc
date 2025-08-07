use crate::models::{Graph, Node, Connection};
use shtairir_registry::types::Type;

pub struct Validator;

impl Validator {
    pub fn validate_graph(graph: &Graph) -> ValidationResult {
        let mut errors = vec![];
        let mut warnings = vec![];
        
        // Check for cycles
        if let Some(cycles) = Self::detect_cycles(graph) {
            for cycle in cycles {
                errors.push(ValidationError::CycleDetected(cycle));
            }
        }
        
        // Validate connections
        for connection in &graph.connections {
            if let Err(e) = Self::validate_connection(graph, connection) {
                errors.push(e);
            }
        }
        
        // Validate nodes
        for node in graph.nodes.values() {
            if let Err(e) = Self::validate_node(node) {
                errors.push(e);
            }
        }
        
        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
        }
    }
    
    fn detect_cycles(graph: &Graph) -> Option<Vec<Vec<String>>> {
        // Simple cycle detection using DFS
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut cycles = vec![];
        
        for node_id in graph.nodes.keys() {
            if !visited.contains(node_id) {
                let mut path = vec![];
                if Self::dfs_cycle_detection(graph, node_id, &mut visited, &mut rec_stack, &mut path) {
                    cycles.push(path);
                }
            }
        }
        
        if cycles.is_empty() {
            None
        } else {
            Some(cycles)
        }
    }
    
    fn dfs_cycle_detection(
        graph: &Graph,
        node_id: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        visited.insert(node_id.to_string());
        rec_stack.insert(node_id.to_string());
        path.push(node_id.to_string());
        
        // Check all outgoing connections
        for connection in &graph.connections {
            if connection.from_node == node_id {
                let neighbor = &connection.to_node;
                if !visited.contains(neighbor) {
                    if Self::dfs_cycle_detection(graph, neighbor, visited, rec_stack, path) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    // Cycle detected
                    path.push(neighbor.clone());
                    return true;
                }
            }
        }
        
        rec_stack.remove(node_id);
        path.pop();
        false
    }
    
    fn validate_connection(graph: &Graph, connection: &Connection) -> Result<(), ValidationError> {
        let from_node = graph.nodes.get(&connection.from_node)
            .ok_or_else(|| ValidationError::InvalidConnection(format!("Node {} not found", connection.from_node)))?;
            
        let to_node = graph.nodes.get(&connection.to_node)
            .ok_or_else(|| ValidationError::InvalidConnection(format!("Node {} not found", connection.to_node)))?;
        
        // Find port types
        let from_port = from_node.output_ports.iter()
            .find(|p| p.id == connection.from_port)
            .ok_or_else(|| ValidationError::InvalidConnection(format!("Output port {} not found in node {}", connection.from_port, connection.from_node)))?;
            
        let to_port = to_node.input_ports.iter()
            .find(|p| p.id == connection.to_port)
            .ok_or_else(|| ValidationError::InvalidConnection(format!("Input port {} not found in node {}", connection.to_port, connection.to_node)))?;
        
        // Type compatibility check (simplified)
        if from_port.port_type != to_port.port_type && 
           from_port.port_type != "any" && 
           to_port.port_type != "any" {
            return Err(ValidationError::TypeMismatch(
                connection.from_port.clone(),
                from_port.port_type.clone(),
                connection.to_port.clone(),
                to_port.port_type.clone(),
            ));
        }
        
        Ok(())
    }
    
    fn validate_node(node: &Node) -> Result<(), ValidationError> {
        // Check if all required parameters are set
        for param in &node.block_spec.params {
            if param.default.is_none() && !node.params.contains_key(&param.name) {
                return Err(ValidationError::MissingParameter(
                    node.id.clone(),
                    param.name.clone(),
                ));
            }
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
}

#[derive(Debug, Clone)]
pub enum ValidationError {
    CycleDetected(Vec<String>),
    InvalidConnection(String),
    TypeMismatch(String, String, String, String), // from_port, from_type, to_port, to_type
    MissingParameter(String, String), // node_id, param_name
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::CycleDetected(nodes) => {
                write!(f, "Cycle detected involving nodes: {}", nodes.join(" -> "))
            },
            ValidationError::InvalidConnection(msg) => {
                write!(f, "Invalid connection: {}", msg)
            },
            ValidationError::TypeMismatch(from_port, from_type, to_port, to_type) => {
                write!(f, "Type mismatch: {} ({}) -> {} ({})", from_port, from_type, to_port, to_type)
            },
            ValidationError::MissingParameter(node_id, param_name) => {
                write!(f, "Missing required parameter '{}' in node '{}'", param_name, node_id)
            },
        }
    }
}