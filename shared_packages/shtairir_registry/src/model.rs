use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// Import our ValueLiteral type for typed default values
use crate::literal::ValueLiteral;

/// Module manifest as stored in MODULE.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub min_shtairir_version: String,
    /// Relative paths to block spec TOML files
    #[serde(default)]
    pub blocks: Vec<PathBuf>,
    /// Relative paths to graph spec TOML files (v0.2)
    #[serde(default)]
    pub graphs: Vec<PathBuf>,
}

/// Determinism annotation for a block
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum Determinism {
    Deterministic,
    Nondeterministic,
}

/// Purity annotation for a block (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Purity {
    Pure,
    Effect,
}

/// Port kind for inputs/outputs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PortKind {
    Value,
    Stream,
    Event,
    Composite,
}

/// Generic parameter for blocks and graphs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenericParam {
    pub name: String,
    #[serde(default)]
    pub bounds: Vec<String>,
}

/// Engine requirements for blocks and graphs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineReq {
    pub version_req: String,
    #[serde(default)]
    pub capability_flags: Vec<String>,
}

/// Integrity information for blocks and graphs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integrity {
    pub content_hash: String,
    #[serde(default)]
    pub signature: Option<String>,
}

/// Port specification for inputs/outputs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortSpec {
    pub name: String,
    /// Shtairir type string (TypeSpec)
    #[serde(rename = "ty")]
    pub ty: String,
    /// Default value for inputs/params. Outputs ignore this.
    #[serde(default)]
    pub default: Option<ValueLiteral>,
    /// Port kind (v0.2)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<PortKind>,
}

/// Allowed set or range for params
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParamAllowed {
    /// Enumerated allowed values
    #[serde(default)]
    pub enum_values: Option<Vec<ValueLiteral>>,
    /// Inclusive range (low, high)
    #[serde(default)]
    pub range: Option<(ValueLiteral, ValueLiteral)>,
}

/// Parameter specification (compile-time / invocation-time immutable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSpec {
    pub name: String,
    #[serde(rename = "ty")]
    pub ty: String,
    #[serde(default)]
    pub default: Option<ValueLiteral>,
    #[serde(default)]
    pub allowed: Option<ParamAllowed>,
}

/// Test reference for blocks (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRef {
    pub name: String,
    pub kind: String,
    pub graph: ValueLiteral, // TODO: Replace with more structured test graph type
    #[serde(default)]
    pub expect: Option<ValueLiteral>,
}

/// Block specification TOML (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSpec {
    /// Block stable identifier (v0.2)
    pub id: String,
    /// Block namespace (v0.2)
    pub namespace: String,
    /// Block name local to module (may contain dots for namespaces)
    pub name: String,
    /// Block version (semver) (v0.2)
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// Purity annotation (v0.2)
    pub purity: Purity,
    #[serde(default)]
    pub effects: Vec<String>,
    /// Determinism annotation
    pub determinism: Determinism,
    /// Generic parameters (v0.2)
    #[serde(default)]
    pub generics: Vec<GenericParam>,
    #[serde(default)]
    pub inputs: Vec<PortSpec>,
    #[serde(default)]
    pub outputs: Vec<PortSpec>,
    #[serde(default)]
    pub params: Vec<ParamSpec>,
    #[serde(default)]
    pub examples: Vec<String>,
    /// Test references (v0.2)
    #[serde(default)]
    pub tests: Vec<TestRef>,
    /// Engine requirements (v0.2)
    pub engine: EngineReq,
    /// Integrity information (v0.2)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity: Option<Integrity>,
    /// Additional metadata (v0.2)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ValueLiteral>,
}

/// Handle summarizing a module loaded into the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleHandle {
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub categories: Vec<String>,
    /// Block local names contained in this module
    #[serde(default)]
    pub block_names: Vec<String>,
    /// Graph local names contained in this module (v0.2)
    #[serde(default)]
    pub graph_names: Vec<String>,
}

/// Handle for a block including its owning module/version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHandle {
    pub module: String,
    pub version: String,
    pub spec: BlockSpec,
}

// ===== Graph Model (v0.2) =====

/// Graph manifest (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSpec {
    pub schema_version: String,
    pub id: String,
    pub namespace: String,
    pub name: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_visibility")]
    pub visibility: String,
    #[serde(default)]
    pub generics: Vec<GenericParam>,
    #[serde(default)]
    pub requires: Vec<ModuleReq>,
    #[serde(default)]
    pub effects: Vec<String>,
    #[serde(default)]
    pub exports: Vec<Export>,
    pub nodes: Vec<Node>,
    #[serde(default)]
    pub edges: Vec<Edge>,
    pub engine: EngineReq,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrity: Option<Integrity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ValueLiteral>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ValueLiteral>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<ValueLiteral>,
}

fn default_visibility() -> String {
    "public".to_string()
}

/// Module requirement for graphs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleReq {
    pub module: String,
    pub version_req: String,
}

/// Export declaration for subgraphs (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub export_id: String,
    pub from_node: String,
    pub from_port: String,
}

/// Node in a graph (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fq_block: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_req: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concrete_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purity: Option<Purity>,
    #[serde(default)]
    pub effects: Vec<String>,
    #[serde(default)]
    pub generics: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub params: ValueLiteral,
    #[serde(default)]
    pub inputs: Vec<PortDecl>,
    #[serde(default)]
    pub outputs: Vec<PortDecl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ValueLiteral>,
}

/// Node kind (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeKind {
    Block,
    Subgraph,
    Macro,
}

/// Port declaration for nodes (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortDecl {
    pub name: String,
    pub port_id: String,
    pub ty: String,
    pub kind: PortKind,
}

/// Edge in a graph (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Edge {
    pub id: String,
    pub from: Endpoint,
    pub to: Endpoint,
    #[serde(default)]
    pub policy: EdgePolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Endpoint for edges (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Endpoint {
    pub node: String,
    pub port: String,
}

/// Edge policy (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EdgePolicy {
    #[serde(default = "default_adapter")]
    pub adapter: AdapterKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_params: Option<AdapterParams>,
    #[serde(default = "default_backpressure")]
    pub backpressure: Backpressure,
    #[serde(default = "default_ordering")]
    pub ordering: Ordering,
    #[serde(default = "default_timestamp_source")]
    pub timestamp_source: TimestampSource,
}

fn default_adapter() -> AdapterKind {
    AdapterKind::None
}

fn default_backpressure() -> Backpressure {
    Backpressure::Block
}

fn default_ordering() -> Ordering {
    Ordering::Source
}

fn default_timestamp_source() -> TimestampSource {
    TimestampSource::Inherit
}

/// Adapter kind for edges (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
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

/// Backpressure strategy for edges (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Backpressure {
    Block,
    DropOldest,
    DropNewest,
    Expand,
}

/// Ordering strategy for edges (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Ordering {
    Source,
    Timestamp,
    StableKey,
}

/// Timestamp source for edges (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TimestampSource {
    Inherit,
    #[serde(rename = "node")]
    Node(String),
    #[serde(rename = "edge")]
    Edge(String),
}

/// Handle for a graph including its owning module/version (v0.2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphHandle {
    pub module: String,
    pub version: String,
    pub spec: GraphSpec,
}

/// Specific parameter types for different adapters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "adapter")]
pub enum AdapterParams {
    None,
    Map {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        transform: Option<String>,
    },
    Filter {
        predicate: String,
    },
    Buffer {
        capacity: usize,
    },
    Window {
        size: usize,
        slide: Option<usize>,
    },
    Debounce {
        delay_ms: u64,
    },
    Merge {
        strategy: String, // e.g., "round_robin", "priority", "zip"
    },
    Zip {
        // Zip adapter doesn't need additional parameters
    },
    Boundary {
        // Boundary adapter doesn't need additional parameters
    },
}

/// In-memory registry of modules and blocks
#[derive(Debug, Default, Clone)]
pub struct Registry {
    /// module name -> version -> ModuleHandle
    pub modules: BTreeMap<String, BTreeMap<String, ModuleHandle>>,
    /// Fully-qualified key "module@version:block_name" -> BlockHandle
    pub blocks: BTreeMap<String, BlockHandle>,
    /// Fully-qualified key "module@version:graph_name" -> GraphHandle (v0.2)
    pub graphs: BTreeMap<String, GraphHandle>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            blocks: BTreeMap::new(),
            graphs: BTreeMap::new(),
        }
    }

    pub fn insert_module(&mut self, handle: ModuleHandle) {
        let entry = self.modules.entry(handle.name.clone()).or_default();
        entry.insert(handle.version.clone(), handle);
    }
pub fn insert_block(&mut self, handle: BlockHandle) {
    let key = format!("{}@{}:{}", handle.module, handle.version, handle.spec.name);
    self.blocks.insert(key, handle);
}

/// Add a graph to the registry
pub fn insert_graph(&mut self, handle: GraphHandle) {
    let key = format!("{}@{}:{}", handle.module, handle.version, handle.spec.name);
    self.graphs.insert(key, handle);
}

    pub fn module_versions(&self, module: &str) -> Vec<String> {
        self.modules
            .get(module)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn module_block_names(&self, module: &str, version: &str) -> Vec<String> {
        self.modules
            .get(module)
            .and_then(|m| m.get(version))
            .map(|h| h.block_names.clone())
            .unwrap_or_default()
    }

    /// Get graph names for a module
    pub fn module_graph_names(&self, module: &str, version: &str) -> Vec<String> {
        self.modules
            .get(module)
            .and_then(|m| m.get(version))
            .map(|h| h.graph_names.clone())
            .unwrap_or_default()
    }
}

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