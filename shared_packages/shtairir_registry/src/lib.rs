use std::path::PathBuf;

use anyhow::Result;
use semver::{Version, VersionReq};

pub mod integrity;
pub mod literal;
pub mod loader;
pub mod model;
pub mod types;
pub mod validator;
pub mod value;

use model::{BlockHandle, GraphHandle, Registry};

impl Registry {
    /// Load modules from the provided root paths, then validate.
    /// Roots will be scanned recursively for files named "MODULE.toml".
    pub fn load(paths: &[PathBuf]) -> Result<Registry> {
        let reg = loader::load_from_paths(paths)?;
        validator::validate_registry(&reg)?;
        Ok(reg)
    }

    /// List all module names present in the registry.
    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    /// List block local names for a given module across all versions (deduplicated).
    pub fn list_blocks(&self, module: &str) -> Vec<String> {
        let mut set = std::collections::BTreeSet::new();
        if let Some(versions) = self.modules.get(module) {
            for (_ver, handle) in versions {
                for b in &handle.block_names {
                    set.insert(b.clone());
                }
            }
        }
        set.into_iter().collect()
    }

    /// Find a block by module, block name, and optional version requirement.
    ///
    /// - If version_spec is Some, it may be exact (e.g., "1.2.3") or a semver req:
    ///   - caret ("^1.2") or tilde ("~1.2.3") or ranges supported by semver::VersionReq.
    /// - If version_spec is None, returns the highest available version for that module containing the block.
    pub fn find_block(
        &self,
        module: &str,
        block: &str,
        version_spec: Option<&str>,
    ) -> Option<BlockHandle> {
        let versions = self.modules.get(module)?;
        // Collect versions that contain the block
        let mut candidates: Vec<Version> = Vec::new();
        for ver_str in versions.keys() {
            // Only consider versions that actually include this block
            let names = self.module_block_names(module, ver_str);
            if names.iter().any(|n| n == block) {
                if let Ok(v) = Version::parse(ver_str) {
                    candidates.push(v);
                }
            }
        }
        if candidates.is_empty() {
            return None;
        }
        candidates.sort(); // ascending
        let selected = if let Some(spec) = version_spec {
            // Try to parse as VersionReq; if fails, try exact match
            match VersionReq::parse(spec) {
                Ok(req) => candidates
                    .into_iter()
                    .rev() // prefer highest first
                    .find(|v| req.matches(v))?,
                Err(_) => {
                    // Exact match path
                    let target = Version::parse(spec).ok()?;
                    if candidates.iter().any(|v| v == &target) {
                        target
                    } else {
                        return None;
                    }
                }
            }
        } else {
            // pick highest
            candidates.into_iter().rev().next().unwrap()
        };
let ver_str = selected.to_string();
let key = format!("{}@{}:{}", module, ver_str, block);
self.blocks.get(&key).cloned()
}

/// Find a graph by module, graph name, and optional version requirement.
///
/// - If version_spec is Some, it may be exact (e.g., "1.2.3") or a semver req:
///   - caret ("^1.2") or tilde ("~1.2.3") or ranges supported by semver::VersionReq.
/// - If version_spec is None, returns the highest available version for that module containing the graph.
pub fn find_graph(
&self,
module: &str,
graph: &str,
version_spec: Option<&str>,
) -> Option<GraphHandle> {
let versions = self.modules.get(module)?;
// Collect versions that contain the graph
let mut candidates: Vec<Version> = Vec::new();
for ver_str in versions.keys() {
    // Only consider versions that actually include this graph
    let names = self.module_graph_names(module, ver_str);
    if names.iter().any(|n| n == graph) {
        if let Ok(v) = Version::parse(ver_str) {
            candidates.push(v);
        }
    }
}
if candidates.is_empty() {
    return None;
}
candidates.sort(); // ascending
let selected = if let Some(spec) = version_spec {
    // Try to parse as VersionReq; if fails, try exact match
    match VersionReq::parse(spec) {
        Ok(req) => candidates
            .into_iter()
            .rev() // prefer highest first
            .find(|v| req.matches(v))?,
        Err(_) => {
            // Exact match path
            let target = Version::parse(spec).ok()?;
            if candidates.iter().any(|v| v == &target) {
                target
            } else {
                return None;
            }
        }
    }
} else {
    // pick highest
    candidates.into_iter().rev().next().unwrap()
};

let ver_str = selected.to_string();
let key = format!("{}@{}:{}", module, ver_str, graph);
self.graphs.get(&key).cloned()
}

/// List graph local names for a given module across all versions (deduplicated).
pub fn list_graphs(&self, module: &str) -> Vec<String> {
let mut set = std::collections::BTreeSet::new();
if let Some(versions) = self.modules.get(module) {
    for (_ver, handle) in versions {
        for g in &handle.graph_names {
            set.insert(g.clone());
        }
    }
}
set.into_iter().collect()
}
}
}