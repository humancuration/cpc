//! Scheduler for Shtairir graphs
//!
//! This module implements deterministic scheduling using topological sorting
//! with support for concurrent execution of independent nodes and advanced
//! planning capabilities.

use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::Result;
use futures_util::future;
use std::sync::Arc;

use shtairir_registry::model::{GraphSpec, Node, NodeKind};

use crate::graph::{build_dependency_graph, topological_sort};
use crate::executor::{NodeExecutor, BlockExecutor, SubgraphExecutor, MacroExecutor};
use crate::registry::RegistryAdapter;
use crate::concurrency::ConcurrencyController;
use crate::planning::{ExecutionPlanner, PlanningConfig};
use shtairir_registry::value::Value;

/// Scheduler for executing Shtairir graphs
pub struct Scheduler {
    /// Registry adapter for looking up blocks and graphs
    registry: RegistryAdapter,
    /// Concurrency controller for managing parallel execution
    concurrency_controller: ConcurrencyController,
    /// Execution planner for optimizing parallel execution
    execution_planner: ExecutionPlanner,
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
            execution_planner: ExecutionPlanner::new(PlanningConfig::default()),
        }
    }
    
    /// Create a new scheduler with custom configuration
    pub fn with_config(registry: RegistryAdapter, planning_config: PlanningConfig) -> Self {
        Self {
            registry,
            concurrency_controller: ConcurrencyController::new(),
            execution_planner: ExecutionPlanner::new(planning_config),
        }
    }

    /// Schedule and execute a graph
    pub async fn schedule(&self, graph: &GraphSpec) -> Result<ExecutionContext> {
        // Create an execution plan
        let plan = self.execution_planner.plan_execution(Arc::new(graph.clone()))?;
        let optimized_plan = self.execution_planner.optimize_plan(plan);
        
        // Execute according to the plan
        let mut context = ExecutionContext {
            node_outputs: HashMap::new(),
            execution_order: Vec::new(),
        };
        
        // Execute each stage
        for stage in optimized_plan.stages {
            // Execute all nodes in this stage concurrently
            let mut futures = Vec::new();
            for node in &stage.nodes {
                let context_clone = context.clone();
                let future = self.execute_node(node, &context_clone);
                futures.push(future);
            }
            
            // Wait for all nodes in this stage to complete
            let results = futures_util::future::try_join_all(futures).await?;
            
            // Collect results
            for (node_id, result) in results {
                context.node_outputs.insert(node_id.clone(), result);
                context.execution_order.push(node_id);
            }
        }
        
        Ok(context)
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