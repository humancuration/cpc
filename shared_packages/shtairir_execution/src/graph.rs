//! Graph utilities for dependency analysis and topological sorting
//!
//! This module implements Kahn's algorithm for topological sorting
//! and provides utilities for analyzing graph dependencies.

use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::{Result, bail};
use shtairir_registry::model::{GraphSpec, Node};

/// Build a dependency graph from a GraphSpec
///
/// Returns a map where keys are node IDs and values are sets of node IDs
/// that the key node depends on.
pub fn build_dependency_graph(graph: &GraphSpec) -> HashMap<String, HashSet<String>> {
    let mut dependencies: HashMap<String, HashSet<String>> = HashMap::new();
    
    // Initialize all nodes with empty dependency sets
    for node in &graph.nodes {
        dependencies.insert(node.id.clone(), HashSet::new());
    }
    
    // Add dependencies based on edges
    for edge in &graph.edges {
        // The 'to' node depends on the 'from' node
        dependencies
            .entry(edge.to.node.clone())
            .or_insert_with(HashSet::new)
            .insert(edge.from.node.clone());
    }
    
    dependencies
}

/// Perform topological sort using Kahn's algorithm
///
/// Returns a vector of node IDs in topologically sorted order.
/// Returns an error if the graph contains cycles.
pub fn topological_sort(
    dependency_graph: &HashMap<String, HashSet<String>>,
    nodes: &[Node],
) -> Result<Vec<String>> {
    // Calculate in-degrees for all nodes
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    
    // Initialize in-degrees to 0
    for node_id in dependency_graph.keys() {
        in_degree.insert(node_id.clone(), 0);
    }
    
    // Calculate actual in-degrees
    for (_, dependencies) in dependency_graph {
        for dependency in dependencies {
            *in_degree.get_mut(dependency).unwrap() += 1;
        }
    }
    
    // Find all nodes with in-degree 0 (no dependencies)
    let mut queue: VecDeque<String> = VecDeque::new();
    for (node_id, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(node_id.clone());
        }
    }
    
    let mut sorted_nodes: Vec<String> = Vec::new();
    
    // Process nodes in topological order
    while let Some(node_id) = queue.pop_front() {
        sorted_nodes.push(node_id.clone());
        
        // For each node that depends on this node, reduce its in-degree
        for (dependent_id, dependencies) in dependency_graph {
            if dependencies.contains(&node_id) {
                let degree = in_degree.get_mut(dependent_id).unwrap();
                *degree -= 1;
                
                // If in-degree becomes 0, add to queue
                if *degree == 0 {
                    queue.push_back(dependent_id.clone());
                }
            }
        }
    }
    
    // Check for cycles
    if sorted_nodes.len() != nodes.len() {
        bail!("Graph contains cycles and cannot be topologically sorted");
    }
    
    Ok(sorted_nodes)
}

/// Identify independent nodes that can be executed concurrently
///
/// Returns a list of sets, where each set contains node IDs that
/// can be executed concurrently (they have no dependencies on each other).
pub fn identify_independent_nodes(
    dependency_graph: &HashMap<String, HashSet<String>>,
    sorted_nodes: &[String],
) -> Vec<HashSet<String>> {
    let mut levels: Vec<HashSet<String>> = Vec::new();
    let mut processed_nodes: HashSet<String> = HashSet::new();
    
    for node_id in sorted_nodes {
        let dependencies = dependency_graph.get(node_id).cloned().unwrap_or_default();
        
        // Find the earliest level where this node can be placed
        let mut level_index = 0;
        while level_index < levels.len() {
            // Check if any dependency is in this level or later levels
            let has_dependency_in_current_or_later_level = dependencies
                .iter()
                .any(|dep| {
                    // Check if dependency is in current or later levels
                    levels.iter().skip(level_index).any(|level| level.contains(dep))
                });
            
            if has_dependency_in_current_or_later_level {
                level_index += 1;
            } else {
                break;
            }
        }
        
        // Add node to appropriate level
        if level_index >= levels.len() {
            levels.push(HashSet::new());
        }
        levels[level_index].insert(node_id.clone());
        processed_nodes.insert(node_id.clone());
    }
    
    levels
}