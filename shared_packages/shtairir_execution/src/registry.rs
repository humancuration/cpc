//! Registry adapter for Shtairir execution
//!
//! This module provides an adapter for looking up blocks and graphs
//! from the Shtairir registry during execution.

use std::sync::Arc;
use anyhow::Result;
use shtairir_registry::model::{Registry, BlockHandle, GraphHandle};

/// Adapter for registry lookups during execution
#[derive(Clone)]
pub struct RegistryAdapter {
    registry: Arc<Registry>,
}

impl RegistryAdapter {
    /// Create a new registry adapter
    pub fn new(registry: Registry) -> Self {
        Self {
            registry: Arc::new(registry),
        }
    }
    
    /// Get a block from the registry
    pub fn get_block(&self, fq_block: &str, version_req: Option<&str>) -> Result<BlockHandle> {
        // Parse the fully qualified block name (module@version:block_name)
        let parts: Vec<&str> = fq_block.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid fully qualified block name: {}", fq_block);
        }
        
        let module_version_part = parts[0];
        let block_name = parts[1];
        
        let module_parts: Vec<&str> = module_version_part.split('@').collect();
        let (module, version_spec) = if module_parts.len() == 2 {
            (module_parts[0], Some(module_parts[1]))
        } else {
            (module_version_part, None)
        };
        
        let version_spec = version_req.or(version_spec);
        
        self.registry
            .find_block(module, block_name, version_spec.as_deref())
            .ok_or_else(|| anyhow::anyhow!("Block not found: {}", fq_block))
    }
    
    /// Get a graph from the registry
    pub fn get_graph(&self, fq_graph: &str, version_req: Option<&str>) -> Result<GraphHandle> {
        // Parse the fully qualified graph name (module@version:graph_name)
        let parts: Vec<&str> = fq_graph.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid fully qualified graph name: {}", fq_graph);
        }
        
        let module_version_part = parts[0];
        let graph_name = parts[1];
        
        let module_parts: Vec<&str> = module_version_part.split('@').collect();
        let (module, version_spec) = if module_parts.len() == 2 {
            (module_parts[0], Some(module_parts[1]))
        } else {
            (module_version_part, None)
        };
        
        let version_spec = version_req.or(version_spec);
        
        self.registry
            .find_graph(module, graph_name, version_spec.as_deref())
            .ok_or_else(|| anyhow::anyhow!("Graph not found: {}", fq_graph))
    }
    
    /// List all modules in the registry
    pub fn list_modules(&self) -> Vec<String> {
        self.registry.list_modules()
    }
    
    /// List all blocks in a module
    pub fn list_blocks(&self, module: &str) -> Vec<String> {
        self.registry.list_blocks(module)
    }
    
    /// List all graphs in a module
    pub fn list_graphs(&self, module: &str) -> Vec<String> {
        self.registry.list_graphs(module)
    }
}