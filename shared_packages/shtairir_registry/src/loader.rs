use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use walkdir::WalkDir;

use crate::model::{
    BlockHandle, BlockSpec, ModuleHandle, ModuleManifest, Registry,
    GraphHandle, GraphSpec,
};

fn is_module_manifest(entry: &walkdir::DirEntry) -> bool {
    if !entry.file_type().is_file() {
        return false;
    }
    entry
        .file_name()
        .to_str()
        .map(|s| s.eq_ignore_ascii_case("MODULE.toml"))
        .unwrap_or(false)
}

pub fn load_from_paths(roots: &[PathBuf]) -> Result<Registry> {
    let mut reg = Registry::new();

    for root in roots {
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if !is_module_manifest(&entry) {
                continue;
            }
            let module_dir = entry
                .path()
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| root.clone());

            let manifest: ModuleManifest = {
                let data = fs::read_to_string(entry.path())
                    .with_context(|| format!("Reading MODULE.toml at {}", entry.path().display()))?;
                toml::from_str(&data).with_context(|| {
                    format!("Parsing MODULE.toml at {}", entry.path().display())
                })?
            };

            // Load blocks
            let mut block_names: Vec<String> = Vec::new();
            let mut loaded_blocks: Vec<BlockHandle> = Vec::new();

            for rel in &manifest.blocks {
                let block_path = module_dir.join(rel);
                let block: BlockSpec = {
                    let data = fs::read_to_string(&block_path).with_context(|| {
                        format!("Reading block spec at {}", block_path.display())
                    })?;
                    toml::from_str(&data)
                        .with_context(|| format!("Parsing block spec at {}", block_path.display()))?
                };

                block_names.push(block.name.clone());
                loaded_blocks.push(BlockHandle {
                    module: manifest.name.clone(),
                    version: manifest.version.clone(),
                    spec: block,
                });
            }

            // Load graphs (v0.2)
            let mut graph_names: Vec<String> = Vec::new();
            let mut loaded_graphs: Vec<GraphHandle> = Vec::new();

            for rel in &manifest.graphs {
                let graph_path = module_dir.join(rel);
                let graph: GraphSpec = {
                    let data = fs::read_to_string(&graph_path).with_context(|| {
                        format!("Reading graph spec at {}", graph_path.display())
                    })?;
                    
                    // Parse as TOML only
                    toml::from_str(&data)
                        .with_context(|| format!("Parsing TOML graph spec at {}", graph_path.display()))?
                };

                graph_names.push(graph.name.clone());
                loaded_graphs.push(GraphHandle {
                    module: manifest.name.clone(),
                    version: manifest.version.clone(),
                    spec: graph,
                });
            }

            // Insert module
            let module_handle = ModuleHandle {
                name: manifest.name.clone(),
                version: manifest.version.clone(),
                title: manifest.title.clone(),
                description: manifest.description.clone(),
                categories: manifest.categories.clone(),
                block_names: block_names.clone(),
                graph_names: graph_names.clone(),
            };
            reg.insert_module(module_handle);

            // Insert blocks
            for bh in loaded_blocks {
                reg.insert_block(bh);
            }

            // Insert graphs
            for gh in loaded_graphs {
                reg.insert_graph(gh);
            }
        }
    }

    Ok(reg)
}


/// Compute content hash for a block spec
pub fn compute_block_hash(block: &BlockSpec) -> Result<String, anyhow::Error> {
    // Convert block to a serializable format for hashing
    let toml_string = toml::to_string(block)?;
    crate::integrity::hash_toml_content(&toml_string)
}

/// Compute content hash for a graph spec
pub fn compute_graph_hash(graph: &GraphSpec) -> Result<String, anyhow::Error> {
    // Convert graph to a serializable format for hashing
    let toml_string = toml::to_string(graph)?;
    crate::integrity::hash_toml_content(&toml_string)
}
/// Verify integrity of a block spec
pub fn verify_block_integrity(block: &BlockSpec) -> Result<bool, anyhow::Error> {
    if let Some(integrity) = &block.integrity {
        let computed_hash = compute_block_hash(block)?;
        Ok(computed_hash == integrity.content_hash)
    } else {
        // No integrity info to verify
        Ok(true)
    }
}

/// Verify integrity of a graph spec
pub fn verify_graph_integrity(graph: &GraphSpec) -> Result<bool, anyhow::Error> {
    if let Some(integrity) = &graph.integrity {
        let computed_hash = compute_graph_hash(graph)?;
        Ok(computed_hash == integrity.content_hash)
    } else {
        // No integrity info to verify
        Ok(true)
    }
}

/// Update integrity hash for a block spec
pub fn update_block_integrity(block: &mut BlockSpec) -> Result<(), anyhow::Error> {
    let hash = compute_block_hash(block)?;
    
    if block.integrity.is_none() {
        block.integrity = Some(crate::model::Integrity {
            content_hash: hash,
            signature: None,
        });
    } else {
        block.integrity.as_mut().unwrap().content_hash = hash;
    }
    
    Ok(())
}

/// Update integrity hash for a graph spec
pub fn update_graph_integrity(graph: &mut GraphSpec) -> Result<(), anyhow::Error> {
    let hash = compute_graph_hash(graph)?;
    
    if graph.integrity.is_none() {
        graph.integrity = Some(crate::model::Integrity {
            content_hash: hash,
            signature: None,
        });
    } else {
        graph.integrity.as_mut().unwrap().content_hash = hash;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_load_empty_registry() {
        let roots = vec![PathBuf::from("nonexistent")];
        let reg = load_from_paths(&roots).unwrap();
        assert_eq!(reg.modules.len(), 0);
        assert_eq!(reg.blocks.len(), 0);
        assert_eq!(reg.graphs.len(), 0);
    }

    #[test]
    fn test_load_simple_module() {
        let dir = TempDir::new().unwrap();
        let module_dir = dir.path();

        // Write MODULE.toml
        let module_toml = r#"
name = "test.simple"
version = "0.1.0"
title = "Simple Test"
description = "A simple test module"
authors = ["Test Author"]
categories = ["test"]
min_shtairir_version = "0.2.0"

blocks = [
  "blocks/test.toml",
]

graphs = [
  "graphs/test.toml",
]
"#;
        fs::write(module_dir.join("MODULE.toml"), module_toml).unwrap();

        // Create blocks and graphs directories
        fs::create_dir_all(module_dir.join("blocks")).unwrap();
        fs::create_dir_all(module_dir.join("graphs")).unwrap();

        // Write a simple block
        let block_toml = r#"
id = "test.simple/test@0.1.0"
namespace = "test.simple"
name = "test"
version = "0.1.0"
title = "Test Block"
description = "A test block"
authors = ["Test Author"]
license = "Test"
tags = ["test"]
purity = "pure"
effects = []
determinism = "Deterministic"

[[inputs]]
name = "input"
ty = "i64"

[[outputs]]
name = "output"
ty = "i64"

[engine]
version_req = "^0.2.0"
capability_flags = ["serde"]
"#;
        fs::write(module_dir.join("blocks/test.toml"), block_toml).unwrap();

        // Write a simple graph
        let graph_toml = r#"
schema_version = "0.2"
id = "graph:test.simple/test@0.1.0"
namespace = "test.simple"
name = "test"
version = "0.1.0"
title = "Test Graph"
description = "A test graph"
authors = ["Test Author"]
tags = ["test"]
visibility = "public"
effects = []
nodes = []
edges = []
exports = []

[engine]
version_req = "^0.2.0"
capability_flags = ["serde"]
"#;
        fs::write(module_dir.join("graphs/test.toml"), graph_toml).unwrap();

        // Load the registry
        let roots = vec![module_dir.to_path_buf()];
        let reg = load_from_paths(&roots).unwrap();

        // Check that the module was loaded
        assert_eq!(reg.modules.len(), 1);
        assert!(reg.modules.contains_key("test.simple"));
        
        let versions = &reg.modules["test.simple"];
        assert_eq!(versions.len(), 1);
        assert!(versions.contains_key("0.1.0"));
        
        let module = &versions["0.1.0"];
        assert_eq!(module.block_names, vec!["test"]);
        assert_eq!(module.graph_names, vec!["test"]);

        // Check that the block was loaded
        assert_eq!(reg.blocks.len(), 1);
        assert!(reg.blocks.contains_key("test.simple@0.1.0:test"));
        
        // Check that the graph was loaded
        assert_eq!(reg.graphs.len(), 1);
        assert!(reg.graphs.contains_key("test.simple@0.1.0:test"));
    }
}