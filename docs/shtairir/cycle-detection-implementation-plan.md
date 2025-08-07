# Shtairir v0.2 - Cycle Detection Implementation Plan

## Overview

This document provides a detailed implementation plan for adding cycle detection with stateful-breaker node support to the Shtairir Registry v0.2. This implementation corresponds to Task 1.3.3 in the implementation roadmap and addresses a critical gap in the current graph validation functionality.

## Current State Analysis

### What's Already Implemented

- Basic graph structure validation in `validate_graph_structure` function
- Type compatibility checking between connected ports
- Stream merge policy validation for multiple producers
- Node and edge ID validation
- Port kind validation for different port types

### What's Missing

- Cycle detection in the graph structure
- Identification of stateful-breaker nodes (fold, reduce, accumulator)
- Validation of cycles to ensure they are only allowed when broken by stateful nodes
- Clear error messages for invalid cycles

## Detailed Implementation Plan

### 1. Data Model Enhancements

#### 1.1 Add Cycle Detection Data Structures

We need to add new data structures to represent cycle information and stateful-breaker nodes:

```rust
/// Information about a detected cycle in the graph
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleInfo {
    /// Ordered list of node IDs forming the cycle
    pub node_ids: Vec<String>,
    /// Ordered list of edge IDs forming the cycle
    pub edge_ids: Vec<String>,
    /// Whether the cycle contains a stateful-breaker node
    pub has_stateful_breaker: bool,
    /// ID of the stateful-breaker node, if any
    pub stateful_breaker_id: Option<String>,
}

/// Result of cycle detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleDetectionResult {
    /// All detected cycles
    pub cycles: Vec<CycleInfo>,
    /// Whether the graph has invalid cycles (without stateful-breaker nodes)
    pub has_invalid_cycles: bool,
}
```

#### 1.2 Add Stateful-Breaker Node Identification

We need to add a helper function to identify stateful-breaker nodes:

```rust
/// Check if a node is a stateful-breaker node
fn is_stateful_breaker_node(node: &Node) -> bool {
    // Stateful-breaker nodes are typically fold, reduce, accumulator, etc.
    // We can identify them by their fq_block or by specific patterns
    
    if let Some(fq_block) = &node.fq_block {
        // Check for common stateful-breaker block patterns
        fq_block.contains("/fold") || 
        fq_block.contains("/reduce") || 
        fq_block.contains("/accumulator") ||
        fq_block.contains("/scan") ||
        fq_block.contains("/state")
    } else {
        false
    }
}
```

### 2. Cycle Detection Algorithm

#### 2.1 Core DFS-Based Cycle Detection

We'll implement a depth-first search (DFS) based algorithm to detect cycles:

```rust
/// Detect cycles in a graph using DFS
fn detect_cycles(nodes: &[Node], edges: &[Edge]) -> Result<CycleDetectionResult> {
    // Build adjacency list representation of the graph
    let mut graph = std::collections::HashMap::new();
    for node in nodes {
        graph.insert(node.id.clone(), Vec::new());
    }
    
    // Build edge mapping for quick lookup
    let mut edge_map = std::collections::HashMap::new();
    for edge in edges {
        let key = format!("{}->{}", edge.from.node, edge.to.node);
        edge_map.insert(key, edge.id.clone());
        
        // Add to adjacency list
        graph.entry(edge.from.node.clone())
            .or_insert_with(Vec::new)
            .push(edge.to.node.clone());
    }
    
    // Track visited nodes and nodes in recursion stack
    let mut visited = std::collections::HashSet::new();
    let mut recursion_stack = std::collections::HashSet::new();
    let mut cycles = Vec::new();
    
    // Perform DFS for each unvisited node
    for node_id in graph.keys() {
        if !visited.contains(node_id) {
            let mut path = Vec::new();
            let mut edge_path = Vec::new();
            if let Some(cycle) = dfs_detect_cycle(
                node_id, 
                &graph, 
                &edge_map, 
                nodes,
                &mut visited, 
                &mut recursion_stack,
                &mut path,
                &mut edge_path
            ) {
                cycles.push(cycle);
            }
        }
    }
    
    // Check if any cycles are invalid (without stateful-breaker nodes)
    let has_invalid_cycles = cycles.iter().any(|cycle| !cycle.has_stateful_breaker);
    
    Ok(CycleDetectionResult {
        cycles,
        has_invalid_cycles,
    })
}

/// Recursive DFS function to detect cycles
fn dfs_detect_cycle(
    node_id: &str,
    graph: &std::collections::HashMap<String, Vec<String>>,
    edge_map: &std::collections::HashMap<String, String>,
    nodes: &[Node],
    visited: &mut std::collections::HashSet<String>,
    recursion_stack: &mut std::collections::HashSet<String>,
    path: &mut Vec<String>,
    edge_path: &mut Vec<String>
) -> Option<CycleInfo> {
    // Mark current node as visited and add to recursion stack
    visited.insert(node_id.to_string());
    recursion_stack.insert(node_id.to_string());
    path.push(node_id.to_string());
    
    // Recur for all adjacent vertices
    if let Some(neighbors) = graph.get(node_id) {
        for neighbor_id in neighbors {
            let edge_key = format!("{}->{}", node_id, neighbor_id);
            if let Some(edge_id) = edge_map.get(&edge_key) {
                edge_path.push(edge_id.clone());
            }
            
            // If neighbor is not visited, recur
            if !visited.contains(neighbor_id) {
                if let Some(cycle) = dfs_detect_cycle(
                    neighbor_id,
                    graph,
                    edge_map,
                    nodes,
                    visited,
                    recursion_stack,
                    path,
                    edge_path
                ) {
                    return Some(cycle);
                }
            } 
            // If neighbor is in recursion stack, we have a cycle
            else if recursion_stack.contains(neighbor_id) {
                // Extract cycle from path
                let cycle_start_index = path.iter().position(|id| id == neighbor_id).unwrap();
                let mut cycle_node_ids = path[cycle_start_index..].to_vec();
                cycle_node_ids.push(neighbor_id.clone());
                
                // Extract corresponding edge IDs
                let mut cycle_edge_ids = Vec::new();
                for i in 0..cycle_node_ids.len() - 1 {
                    let edge_key = format!("{}->{}", cycle_node_ids[i], cycle_node_ids[i + 1]);
                    if let Some(edge_id) = edge_map.get(&edge_key) {
                        cycle_edge_ids.push(edge_id.clone());
                    }
                }
                
                // Check if cycle contains a stateful-breaker node
                let mut has_stateful_breaker = false;
                let mut stateful_breaker_id = None;
                
                for cycle_node_id in &cycle_node_ids {
                    if let Some(node) = nodes.iter().find(|n| n.id == *cycle_node_id) {
                        if is_stateful_breaker_node(node) {
                            has_stateful_breaker = true;
                            stateful_breaker_id = Some(node.id.clone());
                            break;
                        }
                    }
                }
                
                return Some(CycleInfo {
                    node_ids: cycle_node_ids,
                    edge_ids: cycle_edge_ids,
                    has_stateful_breaker,
                    stateful_breaker_id,
                });
            }
            
            // Backtrack edge path
            if let Some(edge_id) = edge_map.get(&edge_key) {
                edge_path.pop();
            }
        }
    }
    
    // Backtrack: remove node from recursion stack and path
    recursion_stack.remove(node_id);
    path.pop();
    
    None
}
```

#### 2.2 Cycle Validation

We need to validate detected cycles and provide appropriate error messages:

```rust
/// Validate cycles in a graph
fn validate_cycles(nodes: &[Node], edges: &[Edge]) -> Result<()> {
    let detection_result = detect_cycles(nodes, edges)?;
    
    if detection_result.has_invalid_cycles {
        // Find invalid cycles (without stateful-breaker nodes)
        let invalid_cycles: Vec<&CycleInfo> = detection_result.cycles
            .iter()
            .filter(|cycle| !cycle.has_stateful_breaker)
            .collect();
        
        if !invalid_cycles.is_empty() {
            let mut error_messages = Vec::new();
            
            for cycle in invalid_cycles {
                let node_path = cycle.node_ids.join(" -> ");
                error_messages.push(format!(
                    "Invalid cycle detected without stateful-breaker node: {}",
                    node_path
                ));
            }
            
            bail!("Invalid cycles detected:\n{}", error_messages.join("\n"));
        }
    }
    
    // Log valid cycles (with stateful-breaker nodes) for debugging
    for cycle in &detection_result.cycles {
        if cycle.has_stateful_breaker {
            let node_path = cycle.node_ids.join(" -> ");
            tracing::debug!(
                "Valid cycle detected with stateful-breaker node {}: {}",
                cycle.stateful_breaker_id.as_deref().unwrap_or("unknown"),
                node_path
            );
        }
    }
    
    Ok(())
}
```

### 3. Integration with Existing Validation

#### 3.1 Enhance validate_graph_structure Function

We'll integrate cycle detection into the existing `validate_graph_structure` function:

```rust
// Validate the overall graph structure, including stream merge policies and cycle detection
fn validate_graph_structure(nodes: &[Node], edges: &[Edge]) -> Result<()> {
    // Existing validation for stream merge policies
    // ... (keep existing code)
    
    // NEW: Validate cycles in the graph
    validate_cycles(nodes, edges)
        .with_context(|| "Cycle validation failed")?;
    
    Ok(())
}
```

### 4. Error Handling and Reporting

#### 4.1 Enhanced Error Messages

We'll provide detailed error messages for invalid cycles:

```rust
/// Format a cycle error message
fn format_cycle_error(cycle: &CycleInfo) -> String {
    let node_path = cycle.node_ids.join(" -> ");
    
    if cycle.has_stateful_breaker {
        format!(
            "Valid cycle detected with stateful-breaker node {}: {}",
            cycle.stateful_breaker_id.as_deref().unwrap_or("unknown"),
            node_path
        )
    } else {
        format!(
            "Invalid cycle detected without stateful-breaker node. \
            Add a fold, reduce, accumulator, or similar stateful node to break the cycle: {}",
            node_path
        )
    }
}

/// Validate cycles with detailed error reporting
fn validate_cycles_with_details(nodes: &[Node], edges: &[Edge]) -> Result<()> {
    let detection_result = detect_cycles(nodes, edges)?;
    
    if detection_result.has_invalid_cycles {
        let invalid_cycles: Vec<&CycleInfo> = detection_result.cycles
            .iter()
            .filter(|cycle| !cycle.has_stateful_breaker)
            .collect();
        
        if !invalid_cycles.is_empty() {
            let mut error_details = Vec::new();
            
            for (i, cycle) in invalid_cycles.iter().enumerate() {
                error_details.push(format!("Cycle {}:", i + 1));
                error_details.push(format!("  Path: {}", cycle.node_ids.join(" -> ")));
                error_details.push(format!("  Edges: {}", cycle.edge_ids.join(", ")));
                error_details.push(format!("  Issue: No stateful-breaker node found"));
                error_details.push(format!("  Solution: Add a fold, reduce, or accumulator node to break the cycle"));
                error_details.push("".to_string());
            }
            
            bail!("Invalid cycles detected:\n\n{}", error_details.join("\n"));
        }
    }
    
    Ok(())
}
```

### 5. Test Cases

#### 5.1 Unit Tests for Cycle Detection

We'll add comprehensive unit tests to verify the cycle detection functionality:

```rust
#[cfg(test)]
mod cycle_detection_tests {
    use super::*;
    
    #[test]
    fn test_no_cycles() {
        // Test a simple acyclic graph
        let nodes = vec![
            create_test_node("node1"),
            create_test_node("node2"),
            create_test_node("node3"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "node2"),
            create_test_edge("edge2", "node2", "node3"),
        ];
        
        let result = detect_cycles(&nodes, &edges).unwrap();
        assert!(result.cycles.is_empty());
        assert!(!result.has_invalid_cycles);
    }
    
    #[test]
    fn test_simple_cycle() {
        // Test a simple cycle without stateful-breaker
        let nodes = vec![
            create_test_node("node1"),
            create_test_node("node2"),
            create_test_node("node3"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "node2"),
            create_test_edge("edge2", "node2", "node3"),
            create_test_edge("edge3", "node3", "node1"), // Creates cycle
        ];
        
        let result = detect_cycles(&nodes, &edges).unwrap();
        assert_eq!(result.cycles.len(), 1);
        assert!(result.has_invalid_cycles);
        assert!(!result.cycles[0].has_stateful_breaker);
    }
    
    #[test]
    fn test_valid_cycle_with_stateful_breaker() {
        // Test a cycle with a stateful-breaker node
        let nodes = vec![
            create_test_node("node1"),
            create_test_stateful_node("fold_node"), // Stateful-breaker
            create_test_node("node3"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "fold_node"),
            create_test_edge("edge2", "fold_node", "node3"),
            create_test_edge("edge3", "node3", "node1"), // Creates cycle
        ];
        
        let result = detect_cycles(&nodes, &edges).unwrap();
        assert_eq!(result.cycles.len(), 1);
        assert!(!result.has_invalid_cycles);
        assert!(result.cycles[0].has_stateful_breaker);
        assert_eq!(result.cycles[0].stateful_breaker_id, Some("fold_node".to_string()));
    }
    
    #[test]
    fn test_multiple_cycles() {
        // Test a graph with multiple cycles
        let nodes = vec![
            create_test_node("node1"),
            create_test_node("node2"),
            create_test_node("node3"),
            create_test_node("node4"),
            create_test_stateful_node("fold_node"), // Stateful-breaker
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "node2"),
            create_test_edge("edge2", "node2", "node3"),
            create_test_edge("edge3", "node3", "node1"), // Cycle 1 (invalid)
            create_test_edge("edge4", "node1", "node4"),
            create_test_edge("edge5", "node4", "fold_node"),
            create_test_edge("edge6", "fold_node", "node1"), // Cycle 2 (valid)
        ];
        
        let result = detect_cycles(&nodes, &edges).unwrap();
        assert_eq!(result.cycles.len(), 2);
        assert!(result.has_invalid_cycles);
        
        // One cycle should be invalid, one valid
        let invalid_count = result.cycles.iter().filter(|c| !c.has_stateful_breaker).count();
        let valid_count = result.cycles.iter().filter(|c| c.has_stateful_breaker).count();
        assert_eq!(invalid_count, 1);
        assert_eq!(valid_count, 1);
    }
    
    #[test]
    fn test_complex_graph() {
        // Test a more complex graph structure
        let nodes = vec![
            create_test_node("source"),
            create_test_node("transform1"),
            create_test_node("transform2"),
            create_test_stateful_node("accumulator"),
            create_test_node("sink"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "source", "transform1"),
            create_test_edge("edge2", "transform1", "transform2"),
            create_test_edge("edge3", "transform2", "accumulator"),
            create_test_edge("edge4", "accumulator", "transform1"), // Valid cycle
            create_test_edge("edge5", "transform2", "sink"),
        ];
        
        let result = detect_cycles(&nodes, &edges).unwrap();
        assert_eq!(result.cycles.len(), 1);
        assert!(!result.has_invalid_cycles);
        assert!(result.cycles[0].has_stateful_breaker);
    }
    
    // Helper functions for creating test nodes and edges
    fn create_test_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            kind: NodeKind::Block,
            fq_block: Some("test/simple".to_string()),
            version_req: Some("^0.2".to_string()),
            inputs: vec![],
            outputs: vec![],
            effects: vec![],
            ..Default::default()
        }
    }
    
    fn create_test_stateful_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            kind: NodeKind::Block,
            fq_block: Some("std.stream/fold".to_string()), // Stateful-breaker
            version_req: Some("^0.2".to_string()),
            inputs: vec![],
            outputs: vec![],
            effects: vec![],
            ..Default::default()
        }
    }
    
    fn create_test_edge(id: &str, from: &str, to: &str) -> Edge {
        Edge {
            id: id.to_string(),
            from: Endpoint { node: from.to_string(), port: "out".to_string() },
            to: Endpoint { node: to.to_string(), port: "in".to_string() },
            ..Default::default()
        }
    }
}
```

#### 5.2 Integration Tests

We'll add integration tests to verify that cycle detection works correctly with the overall validation process:

```rust
#[cfg(test)]
mod cycle_validation_integration_tests {
    use super::*;
    
    #[test]
    fn test_validate_graph_with_valid_cycle() {
        // Create a graph with a valid cycle (containing a stateful-breaker)
        let nodes = vec![
            create_test_node("node1"),
            create_test_stateful_node("fold_node"),
            create_test_node("node3"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "fold_node"),
            create_test_edge("edge2", "fold_node", "node3"),
            create_test_edge("edge3", "node3", "node1"), // Creates valid cycle
        ];
        
        // Validation should succeed
        assert!(validate_graph_structure(&nodes, &edges).is_ok());
    }
    
    #[test]
    fn test_validate_graph_with_invalid_cycle() {
        // Create a graph with an invalid cycle (no stateful-breaker)
        let nodes = vec![
            create_test_node("node1"),
            create_test_node("node2"),
            create_test_node("node3"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "node2"),
            create_test_edge("edge2", "node2", "node3"),
            create_test_edge("edge3", "node3", "node1"), // Creates invalid cycle
        ];
        
        // Validation should fail
        assert!(validate_graph_structure(&nodes, &edges).is_err());
    }
    
    #[test]
    fn test_validate_graph_with_multiple_cycles() {
        // Create a graph with both valid and invalid cycles
        let nodes = vec![
            create_test_node("node1"),
            create_test_node("node2"),
            create_test_node("node3"),
            create_test_node("node4"),
            create_test_stateful_node("fold_node"),
        ];
        
        let edges = vec![
            create_test_edge("edge1", "node1", "node2"),
            create_test_edge("edge2", "node2", "node3"),
            create_test_edge("edge3", "node3", "node1"), // Invalid cycle
            create_test_edge("edge4", "node1", "node4"),
            create_test_edge("edge5", "node4", "fold_node"),
            create_test_edge("edge6", "fold_node", "node1"), // Valid cycle
        ];
        
        // Validation should fail due to the invalid cycle
        assert!(validate_graph_structure(&nodes, &edges).is_err());
    }
}
```

## Implementation Steps

1. **Add data structures**: Define `CycleInfo` and `CycleDetectionResult` in model.rs
2. **Implement cycle detection**: Add `detect_cycles` and `dfs_detect_cycle` functions to validator.rs
3. **Add stateful-breaker identification**: Implement `is_stateful_breaker_node` helper function
4. **Implement cycle validation**: Add `validate_cycles` and `validate_cycles_with_details` functions
5. **Integrate with existing validation**: Update `validate_graph_structure` to include cycle detection
6. **Add comprehensive tests**: Implement unit tests and integration tests
7. **Documentation**: Update relevant documentation with cycle detection information

## Impact on Existing Code

This implementation will enhance the existing validation functionality without breaking changes:

- The `validate_graph_structure` function will be extended but maintain its existing signature
- All existing validation logic will remain unchanged
- New error cases will be added for invalid cycles
- The overall validation process will become more robust

## Performance Considerations

- The cycle detection algorithm uses DFS with O(V + E) time complexity, where V is the number of vertices (nodes) and E is the number of edges
- For large graphs, this is efficient and scales well
- The algorithm uses minimal additional memory for tracking visited nodes and recursion state
- Cycle detection is only performed once per graph validation, so the performance impact is acceptable

## Future Enhancements

1. **Visual cycle highlighting**: Integration with visual editors to highlight detected cycles
2. **Cycle analysis tools**: Additional tools for analyzing and understanding cycles in graphs
3. **Advanced cycle validation**: Support for more complex cycle patterns and validation rules
4. **Cycle optimization**: Suggestions for optimizing cycles in graphs

## Conclusion

This implementation plan provides a comprehensive approach to adding cycle detection with stateful-breaker node support to the Shtairir Registry v0.2. The implementation will enhance the validation functionality, ensure graph correctness, and provide clear feedback to users about cycle-related issues.