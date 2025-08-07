//! Parallel execution planning for Shtairir
//!
//! This module implements dependency analysis, execution plan optimization,
//! and resource allocation strategies for parallel execution of Shtairir programs.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use shtairir_core::error::ShtairirError;
use shtairir_registry::model::{GraphSpec, Node, NodeKind};
use crate::graph::{build_dependency_graph, topological_sort};

/// Execution planner for optimizing parallel execution
pub struct ExecutionPlanner {
    /// Configuration for planning
    config: PlanningConfig,
}

/// Configuration for execution planning
#[derive(Debug, Clone)]
pub struct PlanningConfig {
    /// Maximum number of concurrent executions
    pub max_concurrency: usize,
    
    /// Whether to enable resource-aware planning
    pub resource_aware: bool,
    
    /// Resource limits
    pub resource_limits: ResourceLimits,
    
    /// Optimization level
    pub optimization_level: OptimizationLevel,
}

impl Default for PlanningConfig {
    fn default() -> Self {
        Self {
            max_concurrency: 8,
            resource_aware: true,
            resource_limits: ResourceLimits::default(),
            optimization_level: OptimizationLevel::Balanced,
        }
    }
}

/// Resource limits for execution planning
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum CPU cores to use
    pub max_cpu_cores: usize,
    
    /// Maximum memory usage (in bytes)
    pub max_memory_bytes: usize,
    
    /// Maximum I/O operations per second
    pub max_io_ops_per_sec: usize,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu_cores: 4,
            max_memory_bytes: 1024 * 1024 * 1024, // 1GB
            max_io_ops_per_sec: 1000,
        }
    }
}

/// Optimization level for planning
#[derive(Debug, Clone)]
pub enum OptimizationLevel {
    /// No optimization
    None,
    
    /// Basic optimization
    Basic,
    
    /// Balanced optimization (default)
    Balanced,
    
    /// Aggressive optimization
    Aggressive,
}

/// Execution plan for a graph
#[derive(Debug, Clone)]
pub struct ExecutionPlan {
    /// Stages of execution
    pub stages: Vec<ExecutionStage>,
    
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    
    /// Estimated execution time
    pub estimated_time: std::time::Duration,
}

/// Execution stage containing nodes that can be executed in parallel
#[derive(Debug, Clone)]
pub struct ExecutionStage {
    /// Nodes in this stage
    pub nodes: Vec<Node>,
    
    /// Dependencies for this stage
    pub dependencies: HashSet<String>,
    
    /// Resource requirements for this stage
    pub resource_requirements: ResourceRequirements,
}

/// Resource requirements for execution
#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    /// CPU cores required
    pub cpu_cores: usize,
    
    /// Memory required (in bytes)
    pub memory_bytes: usize,
    
    /// I/O operations required
    pub io_ops: usize,
    
    /// Execution time estimate
    pub execution_time: std::time::Duration,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu_cores: 1,
            memory_bytes: 1024 * 1024, // 1MB
            io_ops: 10,
            execution_time: std::time::Duration::from_millis(100),
        }
    }
}

/// Dependency analyzer for graph execution
pub struct DependencyAnalyzer {
    /// Graph to analyze
    graph: Arc<GraphSpec>,
    
    /// Dependency graph
    dependency_graph: HashMap<String, HashSet<String>>,
}

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    pub fn new(graph: Arc<GraphSpec>) -> Self {
        let dependency_graph = build_dependency_graph(&graph);
        Self {
            graph,
            dependency_graph,
        }
    }
    
    /// Get dependencies for a node
    pub fn get_dependencies(&self, node_id: &str) -> Option<&HashSet<String>> {
        self.dependency_graph.get(node_id)
    }
    
    /// Get nodes that depend on a node
    pub fn get_dependents(&self, node_id: &str) -> HashSet<String> {
        let mut dependents = HashSet::new();
        for (dependent_id, dependencies) in &self.dependency_graph {
            if dependencies.contains(node_id) {
                dependents.insert(dependent_id.clone());
            }
        }
        dependents
    }
    
    /// Check if two nodes are independent (can be executed in parallel)
    pub fn are_independent(&self, node1_id: &str, node2_id: &str) -> bool {
        // Nodes are independent if neither depends on the other
        !self.dependency_graph.get(node1_id).map_or(false, |deps| deps.contains(node2_id)) &&
        !self.dependency_graph.get(node2_id).map_or(false, |deps| deps.contains(node1_id))
    }
    
    /// Get critical path of the graph
    pub fn get_critical_path(&self) -> Result<Vec<String>, ShtairirError> {
        let sorted_nodes = topological_sort(&self.dependency_graph, &self.graph.nodes)?;
        let mut critical_path = Vec::new();
        
        // For now, we'll just return the topologically sorted nodes
        // In a real implementation, this would calculate the actual critical path
        critical_path.extend(sorted_nodes);
        
        Ok(critical_path)
    }
    
    /// Get nodes with no dependencies (entry points)
    pub fn get_entry_points(&self) -> Vec<String> {
        self.graph.nodes
            .iter()
            .filter(|node| {
                self.dependency_graph
                    .get(&node.id)
                    .map_or(true, |deps| deps.is_empty())
            })
            .map(|node| node.id.clone())
            .collect()
    }
    
    /// Get nodes with no dependents (exit points)
    pub fn get_exit_points(&self) -> Vec<String> {
        let all_node_ids: HashSet<String> = self.graph.nodes.iter().map(|n| n.id.clone()).collect();
        let dependent_ids: HashSet<String> = self.dependency_graph
            .values()
            .flatten()
            .cloned()
            .collect();
        
        all_node_ids
            .difference(&dependent_ids)
            .cloned()
            .collect()
    }
}

impl ExecutionPlanner {
    /// Create a new execution planner
    pub fn new(config: PlanningConfig) -> Self {
        Self { config }
    }
    
    /// Create a plan for executing a graph
    pub fn plan_execution(&self, graph: Arc<GraphSpec>) -> Result<ExecutionPlan, ShtairirError> {
        let analyzer = DependencyAnalyzer::new(graph.clone());
        
        // Group nodes by dependency level for parallel execution
        let dependency_graph = build_dependency_graph(&graph);
        let sorted_nodes = topological_sort(&dependency_graph, &graph.nodes)?;
        let levels = self.group_nodes_by_level(&dependency_graph, &sorted_nodes, &graph);
        
        // Create execution stages
        let mut stages = Vec::new();
        let mut total_time = std::time::Duration::from_secs(0);
        let mut total_resources = ResourceRequirements::default();
        
        for level in levels {
            let nodes: Vec<Node> = level
                .iter()
                .filter_map(|node_id| {
                    graph.nodes.iter().find(|n| n.id == *node_id).cloned()
                })
                .collect();
            
            // Calculate resource requirements for this stage
            let stage_resources = self.calculate_stage_resources(&nodes);
            
            let stage = ExecutionStage {
                nodes,
                dependencies: HashSet::new(), // TODO: Calculate actual dependencies
                resource_requirements: stage_resources.clone(),
            };
            
            total_time += stage_resources.execution_time;
            total_resources.cpu_cores = total_resources.cpu_cores.max(stage_resources.cpu_cores);
            total_resources.memory_bytes += stage_resources.memory_bytes;
            total_resources.io_ops += stage_resources.io_ops;
            
            stages.push(stage);
        }
        
        Ok(ExecutionPlan {
            stages,
            resource_requirements: total_resources,
            estimated_time: total_time,
        })
    }
    
    /// Group nodes by their dependency level for concurrent execution
    fn group_nodes_by_level(
        &self,
        dependency_graph: &HashMap<String, HashSet<String>>,
        sorted_nodes: &[String],
        graph: &GraphSpec,
    ) -> Vec<Vec<String>> {
        let mut levels: Vec<Vec<String>> = Vec::new();
        
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
        
        // Apply concurrency limits
        if self.config.max_concurrency < levels.len() {
            // Merge levels to respect concurrency limits
            self.merge_levels(levels, self.config.max_concurrency)
        } else {
            levels
        }
    }
    
    /// Merge levels to respect concurrency limits
    fn merge_levels(&self, levels: Vec<Vec<String>>, max_levels: usize) -> Vec<Vec<String>> {
        if levels.len() <= max_levels {
            return levels;
        }
        
        let mut merged_levels = Vec::new();
        let merge_factor = (levels.len() + max_levels - 1) / max_levels; // Ceiling division
        
        for chunk in levels.chunks(merge_factor) {
            let mut merged_level = Vec::new();
            for level in chunk {
                merged_level.extend(level.iter().cloned());
            }
            merged_levels.push(merged_level);
        }
        
        merged_levels
    }
    
    /// Calculate resource requirements for a stage
    fn calculate_stage_resources(&self, nodes: &[Node]) -> ResourceRequirements {
        let mut resources = ResourceRequirements::default();
        
        for node in nodes {
            let node_resources = self.estimate_node_resources(node);
            resources.cpu_cores = resources.cpu_cores.max(node_resources.cpu_cores);
            resources.memory_bytes += node_resources.memory_bytes;
            resources.io_ops += node_resources.io_ops;
            resources.execution_time = std::cmp::max(
                resources.execution_time,
                node_resources.execution_time
            );
        }
        
        resources
    }
    
    /// Estimate resource requirements for a node
    fn estimate_node_resources(&self, node: &Node) -> ResourceRequirements {
        // This is a simplified estimation
        // In a real implementation, this would be based on node type and parameters
        match node.kind {
            NodeKind::Block => ResourceRequirements {
                cpu_cores: 1,
                memory_bytes: 1024 * 1024, // 1MB
                io_ops: 5,
                execution_time: std::time::Duration::from_millis(50),
            },
            NodeKind::Subgraph => ResourceRequirements {
                cpu_cores: 2,
                memory_bytes: 10 * 1024 * 1024, // 10MB
                io_ops: 20,
                execution_time: std::time::Duration::from_millis(200),
            },
            NodeKind::Macro => ResourceRequirements {
                cpu_cores: 1,
                memory_bytes: 512 * 1024, // 512KB
                io_ops: 2,
                execution_time: std::time::Duration::from_millis(25),
            },
        }
    }
    
    /// Optimize an execution plan
    pub fn optimize_plan(&self, plan: ExecutionPlan) -> ExecutionPlan {
        // Apply optimizations based on configuration
        match self.config.optimization_level {
            OptimizationLevel::None => plan,
            OptimizationLevel::Basic => self.basic_optimization(plan),
            OptimizationLevel::Balanced => self.balanced_optimization(plan),
            OptimizationLevel::Aggressive => self.aggressive_optimization(plan),
        }
    }
    
    /// Apply basic optimizations
    fn basic_optimization(&self, plan: ExecutionPlan) -> ExecutionPlan {
        // For now, just return the plan
        // In a real implementation, this would apply basic optimizations
        plan
    }
    
    /// Apply balanced optimizations
    fn balanced_optimization(&self, plan: ExecutionPlan) -> ExecutionPlan {
        // For now, just return the plan
        // In a real implementation, this would apply balanced optimizations
        plan
    }
    
    /// Apply aggressive optimizations
    fn aggressive_optimization(&self, plan: ExecutionPlan) -> ExecutionPlan {
        // For now, just return the plan
        // In a real implementation, this would apply aggressive optimizations
        plan
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shtairir_registry::model::{BlockSpec, Determinism, Purity};
    
    #[test]
    fn test_planning_config_default() {
        let config = PlanningConfig::default();
        assert_eq!(config.max_concurrency, 8);
        assert_eq!(config.resource_aware, true);
        assert_eq!(config.optimization_level, OptimizationLevel::Balanced);
    }
    
    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.max_cpu_cores, 4);
        assert_eq!(limits.max_memory_bytes, 1024 * 1024 * 1024);
        assert_eq!(limits.max_io_ops_per_sec, 1000);
    }
    
    #[test]
    fn test_resource_requirements_default() {
        let requirements = ResourceRequirements::default();
        assert_eq!(requirements.cpu_cores, 1);
        assert_eq!(requirements.memory_bytes, 1024 * 1024);
        assert_eq!(requirements.io_ops, 10);
    }
    
    #[test]
    fn test_execution_planner_creation() {
        let config = PlanningConfig::default();
        let planner = ExecutionPlanner::new(config);
        // Just test that it can be created
        assert!(true);
    }
    
    #[test]
    fn test_dependency_analyzer() {
        // Create a simple graph for testing
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
        
        let nodes = vec![
            Node {
                id: "node1".to_string(),
                kind: NodeKind::Block,
                spec: block_spec.clone(),
                inputs: vec![],
                outputs: vec![],
                position: None,
                metadata: None,
            },
            Node {
                id: "node2".to_string(),
                kind: NodeKind::Block,
                spec: block_spec.clone(),
                inputs: vec![],
                outputs: vec![],
                position: None,
                metadata: None,
            },
        ];
        
        let graph = Arc::new(GraphSpec {
            id: "test_graph".to_string(),
            name: "Test Graph".to_string(),
            description: "A test graph".to_string(),
            version: "1.0.0".to_string(),
            authors: vec![],
            license: "CPC".to_string(),
            nodes: nodes.clone(),
            connections: vec![],
            metadata: None,
        });
        
        let analyzer = DependencyAnalyzer::new(graph);
        
        // Test entry and exit points
        let entry_points = analyzer.get_entry_points();
        let exit_points = analyzer.get_exit_points();
        
        assert_eq!(entry_points.len(), 2);
        assert_eq!(exit_points.len(), 2);
        assert!(entry_points.contains(&"node1".to_string()));
        assert!(entry_points.contains(&"node2".to_string()));
        assert!(exit_points.contains(&"node1".to_string()));
        assert!(exit_points.contains(&"node2".to_string()));
    }
}