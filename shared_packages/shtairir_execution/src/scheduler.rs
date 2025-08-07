//! Scheduler for Shtairir graphs
//!
//! This module implements deterministic scheduling using topological sorting
//! with support for concurrent execution of independent nodes.

use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::Result;
use futures_util::future;

use shtairir_registry::model::{GraphSpec, Node, NodeKind};

use crate::graph::{build_dependency_graph, topological_sort};
use crate::executor::{NodeExecutor, BlockExecutor, SubgraphExecutor, MacroExecutor};
use crate::registry::RegistryAdapter;
use crate::concurrency::ConcurrencyController;
use shtairir_registry::value::Value;

/// Scheduler for executing Shtairir graphs
pub struct Scheduler {
    /// Registry adapter for looking up blocks and graphs
    registry: RegistryAdapter,
    /// Concurrency controller for managing parallel execution
    concurrency_controller: ConcurrencyController,
}

/// Execution context for a single graph execution
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    /// Values produced by executed nodes
    pub node_outputs: HashMap<String, Value>,
    /// Execution order of nodes
    pub execution_order: Vec<String>,
}

impl Scheduler {
    /// Create a new scheduler with the given registry adapter
    pub fn new(registry: RegistryAdapter) -> Self {
        Self {
            registry,
            concurrency_controller: ConcurrencyController::new(),
        }
    }

    /// Schedule and execute a graph
    pub async fn schedule(&self, graph: &GraphSpec) -> Result<ExecutionContext> {
        // Build dependency graph
        let dependency_graph = build_dependency_graph(graph);
        
        // Perform topological sort to determine execution order
        let sorted_nodes = topological_sort(&dependency_graph, &graph.nodes)?;
        
        // Execute nodes in order
        let mut context = ExecutionContext {
            node_outputs: HashMap::new(),
            execution_order: Vec::new(),
        };
        
        // Group nodes by their dependency level for concurrent execution
        let levels = self.group_nodes_by_level(&dependency_graph, &sorted_nodes);
        for level in levels {
            // Execute all nodes in this level concurrently
            let mut futures = Vec::new();
            for node_id in &level {
                let node = graph.nodes.iter().find(|n| n.id == *node_id).unwrap().clone();
                let context_clone = context.clone();
                let future = self.execute_node(&node, &context_clone);
                futures.push(future);
            }
            
            // Wait for all nodes in this level to complete
            let results = futures_util::future::try_join_all(futures).await?;
            
            // Collect results
            for (node_id, result) in results {
                context.node_outputs.insert(node_id.clone(), result);
                context.execution_order.push(node_id);
            }
        }
        
        Ok(context)
    }
    
    /// Group nodes by their dependency level for concurrent execution
    fn group_nodes_by_level(
        &self,
        dependency_graph: &HashMap<String, HashSet<String>>,
        sorted_nodes: &[String],
    ) -> Vec<Vec<String>> {
        let mut levels: Vec<Vec<String>> = Vec::new();
        let mut nodes_at_current_level: HashSet<String> = HashSet::new();
        
        for node_id in sorted_nodes {
            let dependencies: HashSet<String> = dependency_graph
                .get(node_id)
                .cloned()
                .unwrap_or_default();
            
            // Find the level where this node can be placed
            // It must be placed after all its dependencies
            let mut level_index = 0;
            while level_index < levels.len() {
                // Check if any dependency is in this level or later levels
                let has_dependency_in_current_or_later_level = dependencies
                    .iter()
                    .any(|dep| {
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
                levels.push(Vec::new());
            }
            levels[level_index].push(node_id.clone());
        }
        
        levels
    }
    
    /// Execute a single node
    async fn execute_node(
        &self,
        node: &Node,
        context: &ExecutionContext,
    ) -> Result<(String, Value)> {
        let executor: Box<dyn NodeExecutor> = match node.kind {
            NodeKind::Block => Box::new(BlockExecutor::new(self.registry.clone())),
            NodeKind::Subgraph => Box::new(SubgraphExecutor::new(self.registry.clone())),
            NodeKind::Macro => Box::new(MacroExecutor::new(self.registry.clone())),
        };
        
        let result = executor.execute(node, context).await?;
        Ok((node.id.clone(), result))
    }
}