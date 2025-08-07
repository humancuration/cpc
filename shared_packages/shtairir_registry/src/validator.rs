use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use semver::{Version, VersionReq};

use crate::model::{
    BlockHandle, BlockSpec, Determinism, ParamSpec, PortSpec, Registry,
    Purity, GenericParam, GraphSpec, Node, Edge, PortKind, EngineReq,
    GraphHandle, CycleInfo, CycleDetectionResult, NodeKind,
};
use crate::types::{Type, GenericBound};
use crate::literal::ValueLiteral;

pub fn validate_registry(reg: &Registry) -> Result<()> {
    // 1) Validate module versions are semver and unique
    let mut seen = std::collections::BTreeSet::new();
    for (module, versions) in &reg.modules {
        for (ver_str, _handle) in versions {
            let ver = Version::parse(ver_str)
                .with_context(|| format!("Module '{}' has invalid semver '{}'", module, ver_str))?;
            let key = format!("{}@{}", module, ver);
            if !seen.insert(key) {
                bail!("Duplicate module entry '{}@{}'", module, ver_str);
            }
            // module name identifier check
            validate_module_name(module)
                .with_context(|| format!("Invalid module name '{}'", module))?;
        }
    }

    // 2) Validate all blocks
    for (key, bh) in &reg.blocks {
        validate_block_handle(bh).with_context(|| format!("Invalid block '{}'", key))?;
    }

    // 3) Validate all graphs (v0.2)
    for (key, gh) in &reg.graphs {
        validate_graph_handle(gh).with_context(|| format!("Invalid graph '{}'", key))?;
    }

    Ok(())
}

fn validate_module_name(name: &str) -> Result<()> {
    // snake_case segments with dots allowed optionally? Requirement says snake_case for module name.
    // Accept segments of [a-z0-9_]+ separated by dots as well to allow namespaces like app.website_builder
    let re = Regex::new(r"^[a-z0-9_]+(?:\.[a-z0-9_]+)*$").unwrap();
    if !re.is_match(name) {
        bail!("Module name '{}' must be snake_case segments separated by '.'", name);
    }
    Ok(())
}

fn validate_block_name(name: &str) -> Result<()> {
    // Accept snake_case with dots for sub-namespaces: e.g., "math.add", "string.format", or "add"
    let re = Regex::new(r"^[a-z0-9_]+(?:\.[a-z0-9_]+)*$").unwrap();
    if !re.is_match(name) {
        bail!("Block name '{}' must be snake_case and may include '.'", name);
    }
    Ok(())
}

fn validate_block_handle(bh: &BlockHandle) -> Result<()> {
    // version is semver string
    Version::parse(&bh.version)
        .with_context(|| format!("Block module '{}' has invalid version '{}'", bh.module, bh.version))?;

    validate_module_name(&bh.module)?;
    validate_block_spec(&bh.spec)
}

fn validate_graph_handle(gh: &GraphHandle) -> Result<()> {
    // version is semver string
    Version::parse(&gh.version)
        .with_context(|| format!("Graph module '{}' has invalid version '{}'", gh.module, gh.version))?;

    validate_module_name(&gh.module)?;
    validate_graph_spec(&gh.spec)
}

fn validate_block_spec(spec: &BlockSpec) -> Result<()> {
    validate_block_name(&spec.name)?;

    // v0.2: validate id format
    validate_id(&spec.id)?;

    // v0.2: validate namespace
    validate_namespace(&spec.namespace)?;

    // v0.2: purity vs effects: pure blocks must have no effects
    if spec.purity == Purity::Pure && !spec.effects.is_empty() {
        bail!("Pure block '{}' must not declare effects", spec.name);
    }

    // determinism vs effects: deterministic should not have effects
    match spec.determinism {
        Determinism::Deterministic => {
            if !spec.effects.is_empty() {
                bail!("Deterministic block '{}' must not declare effects", spec.name);
            }
        }
        Determinism::Nondeterministic => { /* allowed */ }
    }

    // effects format
    for eff in &spec.effects {
        validate_effect_string(eff)
            .with_context(|| format!("Invalid effect '{}'", eff))?;
    }

    // v0.2: validate generics
    for generic in &spec.generics {
        validate_generic_param(generic)?;
    }

    // v0.2: validate engine requirements
    validate_engine_req(&spec.engine)?;

    // ports
    ensure_unique_names(&spec.inputs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), "input")?;
    ensure_unique_names(&spec.outputs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), "output")?;
    ensure_unique_names(&spec.params.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), "param")?;

    if spec.outputs.is_empty() {
        bail!("Block '{}' must have at least one output", spec.name);
    }

    // types and defaults
    for p in &spec.inputs {
        validate_type(&p.ty)
            .with_context(|| format!("Invalid type '{}' for input '{}'", p.ty, p.name))?;
        
        // v0.2: validate port kind if specified
        if let Some(kind) = &p.kind {
            validate_port_kind_for_type(&p.ty, kind)?;
        }
        
        if let Some(d) = &p.default {
            validate_default_compat(d, &p.ty)
                .with_context(|| format!("Default for input '{}' incompatible with type '{}'", p.name, p.ty))?;
        }
    }
    for p in &spec.outputs {
        validate_type(&p.ty)
            .with_context(|| format!("Invalid type '{}' for output '{}'", p.ty, p.name))?;
        
        // v0.2: validate port kind if specified
        if let Some(kind) = &p.kind {
            validate_port_kind_for_type(&p.ty, kind)?;
        }
    }
    for prm in &spec.params {
        validate_type(&prm.ty)
            .with_context(|| format!("Invalid type '{}' for param '{}'", prm.ty, prm.name))?;
        if let Some(d) = &prm.default {
            validate_default_compat(d, &prm.ty)
                .with_context(|| format!("Default for param '{}' incompatible with type '{}'", prm.name, prm.ty))?;
        }
        if let Some(allowed) = &prm.allowed {
            if let Some(en) = &allowed.enum_values {
                for v in en {
                    validate_default_compat(v, &prm.ty)
                        .with_context(|| format!("Allowed enum value incompatible for param '{}'", prm.name))?;
                }
            }
            if let Some((lo, hi)) = &allowed.range {
                // Best-effort: ensure both ends are compatible with the type
                validate_default_compat(lo, &prm.ty)
                    .with_context(|| format!("Allowed range low incompatible for param '{}'", prm.name))?;
                validate_default_compat(hi, &prm.ty)
                    .with_context(|| format!("Allowed range high incompatible for param '{}'", prm.name))?;
            }
        }
    }

    // v0.2: validate integrity if present
    if let Some(integrity) = &spec.integrity {
        validate_integrity_hash(&integrity.content_hash)?;
    }

    Ok(())
}

fn ensure_unique_names(names: &Vec<&str>, kind: &str) -> Result<()> {
    let mut set = std::collections::BTreeSet::new();
    for n in names {
        if !set.insert(*n) {
            bail!("Duplicate {} name '{}'", kind, n);
        }
    }
    Ok(())
}
// Type validation using the enhanced type system
fn validate_type(ty: &str) -> Result<()> {
    // Use our new Type parser to validate the type string
    match Type::parse(ty) {
        Ok(_) => Ok(()),
        Err(e) => bail!("Invalid type '{}': {}", ty, e),
    }
}

// Effects format: segments separated by '.', allow wildcard only at the end
fn validate_effect_string(s: &str) -> Result<()> {
    // examples: fs.read, net.http, time.read, audio.play, storage.kv, device.camera, app.<id>.* , channel.<name>
    let parts: Vec<&str> = s.split('.').collect();
    if parts.is_empty() || parts.iter().any(|p| p.is_empty()) {
        bail!("Effect must be non-empty segments separated by '.'");
    }
    // wildcard only as last entire segment "*"
    for (i, seg) in parts.iter().enumerate() {
        if seg.contains('*') {
            if *seg != "*" || i != parts.len() - 1 {
                bail!("Wildcard '*' allowed only as the last full segment");
            }
        }
        // allow angle bracket placeholders and underscores/digits/lowercase
        let ok = Regex::new(r"^[a-z0-9_]+|<[^>]+>|\*+$").unwrap();
        // Note: our manifests won't be HTML-escaped; accept raw <...> too:
        let ok2 = Regex::new(r"^[a-z0-9_]+|<[^>]+>|\*+$").unwrap();
        if !(ok.is_match(seg) || ok2.is_match(seg)) {
            bail!("Invalid effect segment '{}'", seg);
        }
    }
    Ok(())
}

fn validate_default_compat(v: &ValueLiteral, ty: &str) -> Result<()> {
    // Very basic checks to catch glaring mismatches.
    match ty {
        "i64" => match v {
            ValueLiteral::I64(_) => Ok(()),
            _ => bail!("Expected integer for i64"),
        },
        "f64" => match v {
            ValueLiteral::F64(_) => Ok(()),
            ValueLiteral::I64(_) => Ok(()), // i64 can be converted to f64
            _ => bail!("Expected number for f64"),
        },
        "bool" => match v {
            ValueLiteral::Bool(_) => Ok(()),
            _ => bail!("Expected bool"),
        },
        "string" | "datetime" | "duration" | "uuid" | "decimal" => match v {
            ValueLiteral::String(_) => Ok(()),
            _ => bail!("Expected string for {}", ty),
        },
        "bytes" => match v {
            ValueLiteral::String(_) => Ok(()), // expect base64 string
            _ => bail!("Expected base64 string for bytes"),
        },
        "object" => match v {
            ValueLiteral::Object(_) => Ok(()),
            _ => bail!("Expected object for object type"),
        },
        "array" => match v {
            ValueLiteral::List(_) => Ok(()),
            _ => bail!("Expected array for array type"),
        },
        "null" => match v {
            ValueLiteral::Null => Ok(()),
            _ => bail!("Expected null for null type"),
        },
        _ => {
            // composites
            if let Some(inner) = ty.strip_prefix("option<").and_then(|s| s.strip_suffix('>')) {
                match v {
                    ValueLiteral::List(vec) if vec.is_empty() => Ok(()),
                    ValueLiteral::List(vec) if vec.len() == 1 => {
                        validate_default_compat(&vec[0], inner.trim())
                    },
                    _ => bail!("Expected option value (empty list for None, single-element list for Some)"),
                }
            }
            else if let Some(inner) = ty.strip_prefix("list<").and_then(|s| s.strip_suffix('>')) {
                match v {
                    ValueLiteral::List(arr) => {
                        for it in arr {
                            validate_default_compat(it, inner.trim())?;
                        }
                        Ok(())
                    }
                    _ => bail!("Expected list for list type"),
                }
            }
            else if let Some(rest) = ty.strip_prefix("map<").and_then(|s| s.strip_suffix('>')) {
                let parts: Vec<&str> = rest.split(',').map(|s| s.trim()).collect();
                if parts.len() != 2 {
                    bail!("map expects two type params");
                }
                if parts[0] != "string" {
                    bail!("map key must be string");
                }
                match v {
                    ValueLiteral::Object(obj) => {
                        for (_k, val) in obj {
                            validate_default_compat(val, parts[1])?;
                        }
                        Ok(())
                    }
                    _ => bail!("Expected object for map"),
                }
            }
            else {
                bail!("Unsupported type for default compatibility: {}", ty)
            }
        }
    }
}

// v0.2 validation functions

fn validate_id(id: &str) -> Result<()> {
    // Format: namespace/name@version
    let parts: Vec<&str> = id.split('@').collect();
    if parts.len() != 2 {
        bail!("ID must be in format 'namespace/name@version', got '{}'", id);
    }
    
    let name_part = parts[0];
    let version_part = parts[1];
    
    // Validate version is semver
    Version::parse(version_part)
        .with_context(|| format!("Invalid version in ID: '{}'", version_part))?;
    
    // Validate namespace/name part
    validate_namespace(name_part)?;
    
    Ok(())
}

fn validate_namespace(namespace: &str) -> Result<()> {
    // Format: module.name with snake_case segments
    let re = Regex::new(r"^[a-z0-9_]+(\.[a-z0-9_]+)*$").unwrap();
    if !re.is_match(namespace) {
        bail!("Namespace '{}' must be snake_case segments separated by '.'", namespace);
    }
    Ok(())
}

fn validate_generic_param(generic: &GenericParam) -> Result<()> {
    // Validate generic name
    let re = Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
    if !re.is_match(&generic.name) {
        bail!("Generic parameter name '{}' must start with uppercase letter", generic.name);
    }
    
    // Validate bounds are known capability names
    for bound_str in &generic.bounds {
        let _bound = GenericBound::parse(bound_str)
            .with_context(|| format!("Unknown generic bound '{}'", bound_str))?;
    }
    
    Ok(())
}

fn validate_engine_req(engine: &EngineReq) -> Result<()> {
    // Validate version_req is a valid semver requirement
    VersionReq::parse(&engine.version_req)
        .with_context(|| format!("Invalid engine version requirement: '{}'", engine.version_req))?;
    
    // Validate capability flags
    const KNOWN_CAPABILITIES: &[&str] = &[
        "serde", "pure_values", "streams", "time", "wasm", "net", "fs",
    ];
    
    for flag in &engine.capability_flags {
        if !KNOWN_CAPABILITIES.contains(&flag.as_str()) {
            bail!("Unknown engine capability flag '{}'", flag);
        }
    }
    
    Ok(())
}

fn validate_port_kind_for_type(ty: &str, kind: &PortKind) -> Result<()> {
    match kind {
        PortKind::Value => {
            // Value ports can't be Stream<T> or Event<T>
            if ty.starts_with("Stream<") || ty.starts_with("Event<") {
                bail!("Value port cannot have Stream<T> or Event<T> type");
            }
        }
        PortKind::Stream => {
            // Stream ports must be Stream<T>
            if !ty.starts_with("Stream<") || !ty.ends_with('>') {
                bail!("Stream port must have Stream<T> type");
            }
        }
        PortKind::Event => {
            // Event ports must be Event<T>
            if !ty.starts_with("Event<") || !ty.ends_with('>') {
                bail!("Event port must have Event<T> type");
            }
        }
        PortKind::Composite => {
            // Composite ports must be struct, enum, or tuple
            if !(ty.starts_with("Struct{") || ty.starts_with("Enum{") || ty.starts_with('(')) {
                bail!("Composite port must be Struct{{...}}, Enum{{...}}, or tuple type");
            }
        }
    }
    Ok(())
}

fn validate_integrity_hash(hash: &str) -> Result<()> {
    if !hash.starts_with("sha256:") {
        bail!("Integrity hash must start with 'sha256:'");
    }
    
    let hex_part = &hash[7..];
    if hex_part.len() != 64 {
        bail!("Integrity hash must have 64 hex characters after 'sha256:'");
    }
    
    if !hex_part.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("Integrity hash must contain only hex characters after 'sha256:'");
    }
    
    Ok(())
}

fn validate_graph_spec(graph: &GraphSpec) -> Result<()> {
    // Validate schema version
    if graph.schema_version != "0.2" {
        bail!("Unsupported schema version '{}', expected '0.2'", graph.schema_version);
    }
    
    // Validate ID format
    validate_id(&graph.id)?;
    
    // Validate namespace
    validate_namespace(&graph.namespace)?;
    
    // Validate version is semver
    Version::parse(&graph.version)
        .with_context(|| format!("Graph has invalid semver '{}'", graph.version))?;
    
    // Validate generics
    for generic in &graph.generics {
        validate_generic_param(generic)?;
    }
    
    // Validate module requirements
    for req in &graph.requires {
        validate_namespace(&req.module)?;
        VersionReq::parse(&req.version_req)
            .with_context(|| format!("Invalid version requirement for module '{}': '{}'", req.module, req.version_req))?;
    }
    
    // Validate effects
    for eff in &graph.effects {
        validate_effect_string(eff)
            .with_context(|| format!("Invalid effect '{}'", eff))?;
    }
    
    // Validate exports
    ensure_unique_names(&graph.exports.iter().map(|e| e.export_id.as_str()).collect::<Vec<_>>(), "export")?;
    
    // Validate nodes
    let mut node_ids = std::collections::BTreeSet::new();
    for node in &graph.nodes {
        if !node_ids.insert(&node.id) {
            bail!("Duplicate node ID '{}'", node.id);
        }
        validate_node(node)?;
    }
    
    // Validate edges
    let mut edge_ids = std::collections::BTreeSet::new();
    for edge in &graph.edges {
        if !edge_ids.insert(&edge.id) {
            bail!("Duplicate edge ID '{}'", edge.id);
        }
        validate_edge(edge, &graph.nodes)?;
    }
    
    // Validate graph structure including stream merge policies
    validate_graph_structure(&graph.nodes, &graph.edges)?;
    
    // Validate engine requirements
    validate_engine_req(&graph.engine)?;
    
    // Validate integrity if present
    if let Some(integrity) = &graph.integrity {
        validate_integrity_hash(&integrity.content_hash)?;
    }
    
    Ok(())
}

fn validate_node(node: &Node) -> Result<()> {
    // Validate node ID format
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(&node.id) {
        bail!("Node ID '{}' must be alphanumeric with underscores", node.id);
    }
    
    // Validate based on node kind
    match node.kind {
        crate::model::NodeKind::Block => {
            if node.fq_block.is_none() {
                bail!("Block node must have fq_block specified");
            }
            if node.version_req.is_none() {
                bail!("Block node must have version_req specified");
            }
        }
        crate::model::NodeKind::Subgraph => {
            // Subgraph nodes have similar requirements to block nodes
            if node.fq_block.is_none() {
                bail!("Subgraph node must have fq_block specified");
            }
        }
        crate::model::NodeKind::Macro => {
            // Macro nodes must be lowered before publishing
            bail!("Macro nodes must be lowered to block/subgraph before publishing");
        }
    }
    
    // Validate port declarations
    ensure_unique_names(&node.inputs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), "input")?;
    ensure_unique_names(&node.outputs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), "output")?;
    
    for port in &node.inputs {
        validate_type(&port.ty)
            .with_context(|| format!("Invalid type '{}' for input port '{}'", port.ty, port.name))?;
        validate_port_kind_for_type(&port.ty, &port.kind)?;
    }
    
    for port in &node.outputs {
        validate_type(&port.ty)
            .with_context(|| format!("Invalid type '{}' for output port '{}'", port.ty, port.name))?;
        validate_port_kind_for_type(&port.ty, &port.kind)?;
    }
    
    // Validate effects
    for eff in &node.effects {
        validate_effect_string(eff)
            .with_context(|| format!("Invalid effect '{}'", eff))?;
    }
    
    // Validate purity vs effects
    if let Some(purity) = &node.purity {
        if *purity == Purity::Pure && !node.effects.is_empty() {
            bail!("Pure node '{}' must not declare effects", node.id);
        }
    }
    
    Ok(())
}

// Validate the overall graph structure, including stream merge policies and cycle detection
fn validate_graph_structure(nodes: &[Node], edges: &[Edge]) -> Result<()> {
    // Build a map of input port to incoming edges
    let mut incoming_edges: std::collections::BTreeMap<String, Vec<&Edge>> = std::collections::BTreeMap::new();
    
    for edge in edges {
        let key = format!("{}.{}", edge.to.node, edge.to.port);
        incoming_edges.entry(key).or_insert_with(Vec::new).push(edge);
    }
    
    // Check for multiple producers feeding stream inputs
    for (port_key, edges) in incoming_edges {
        if edges.len() > 1 {
            // Multiple producers - check if this is a stream input
            let (node_id, port_id) = port_key.split_once('.')
                .ok_or_else(|| anyhow!("Invalid port key format: {}", port_key))?;
            
            let node = nodes.iter().find(|n| n.id == node_id)
                .ok_or_else(|| anyhow!("Node '{}' not found", node_id))?;
            
            let port = node.inputs.iter().find(|p| p.port_id == port_id)
                .ok_or_else(|| anyhow!("Input port '{}' not found in node '{}'", port_id, node_id))?;
            
            // Check if this is a stream type
            if let Ok(Type::Stream(_)) = Type::parse(&port.ty) {
                // Multiple producers feeding a stream - validate merge policies
                let has_merge_adapter = edges.iter().any(|e|
                    matches!(e.policy.adapter, crate::model::AdapterKind::Merge));
                
                if !has_merge_adapter {
                    bail!("Multiple producers feed stream input '{}.{}' but no edge has a merge adapter", node_id, port_id);
                }
                
                // Validate that all edges have appropriate adapters
                for edge in edges {
                    match edge.policy.adapter {
                        crate::model::AdapterKind::Merge => Ok(()),
                        crate::model::AdapterKind::None => {
                            bail!("Edge '{}' connects to a stream input with multiple producers but has no merge adapter", edge.id)
                        }
                        _ => {
                            bail!("Edge '{}' connects to a stream input with multiple producers but has an inappropriate adapter", edge.id)
                        }
                    }?;
                }
            }
        }
    }
    
    // Validate cycles in the graph
    validate_cycles_with_details(nodes, edges)
        .with_context(|| "Cycle validation failed")?;
    
    Ok(())
}

fn validate_edge(edge: &Edge, nodes: &[Node]) -> Result<()> {
    // Validate edge ID format
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(&edge.id) {
        bail!("Edge ID '{}' must be alphanumeric with underscores", edge.id);
    }
    
    // Validate endpoints exist
    let from_node = nodes.iter().find(|n| n.id == edge.from.node)
        .ok_or_else(|| anyhow!("From node '{}' not found", edge.from.node))?;
    let to_node = nodes.iter().find(|n| n.id == edge.to.node)
        .ok_or_else(|| anyhow!("To node '{}' not found", edge.to.node))?;
    
    // Validate ports exist
    let from_port = from_node.outputs.iter().find(|p| p.port_id == edge.from.port)
        .ok_or_else(|| anyhow!("From port '{}' not found in node '{}'", edge.from.port, edge.from.node))?;
    let to_port = to_node.inputs.iter().find(|p| p.port_id == edge.to.port)
        .ok_or_else(|| anyhow!("To port '{}' not found in node '{}'", edge.to.port, edge.to.node))?;
    
    // Validate type compatibility between ports
    let from_ty = Type::parse(&from_port.ty)
        .with_context(|| format!("Invalid type '{}' for from port", from_port.ty))?;
    let to_ty = Type::parse(&to_port.ty)
        .with_context(|| format!("Invalid type '{}' for to port", to_port.ty))?;
    
    if !from_ty.is_compatible_with(&to_ty) {
        bail!("Type '{}' is not compatible with '{}' for edge '{}'", from_port.ty, to_port.ty, edge.id);
    }
    
    // Validate stream merge policies when multiple producers feed a stream input
    if let Type::Stream(_) = to_ty {
        // We need to check if there are multiple edges connecting to the same input port
        // This is a simplified check since we don't have direct access to all edges in this function
        // In a complete implementation, we would pass the entire graph to this function
        
        // For now, we'll validate that if an edge has a merge adapter, it's connecting to a stream
        if let crate::model::AdapterKind::Merge = edge.policy.adapter {
            // Ensure we're connecting to a stream type
            if !matches!(to_ty, Type::Stream(_)) {
                bail!("Edge '{}' has a merge adapter but is not connecting to a stream input", edge.id);
            }
        }
        
        // Additional validation for other adapter types
        match edge.policy.adapter {
            crate::model::AdapterKind::Zip => {
                // Zip adapters should only be used with streams
                if !matches!(to_ty, Type::Stream(_)) {
                    bail!("Edge '{}' has a zip adapter but is not connecting to a stream input", edge.id);
                }
            }
            crate::model::AdapterKind::Map => {
                // Map adapters should only be used with streams
                if !matches!(to_ty, Type::Stream(_)) {
                    bail!("Edge '{}' has a map adapter but is not connecting to a stream input", edge.id);
                }
            }
            crate::model::AdapterKind::Filter => {
                // Filter adapters should only be used with streams
                if !matches!(to_ty, Type::Stream(_)) {
                    bail!("Edge '{}' has a filter adapter but is not connecting to a stream input", edge.id);
                }
            }
            crate::model::AdapterKind::None => {
                // No adapter - this is fine for single producer streams
            }
            _ => {
                // Unknown adapter type
                bail!("Edge '{}' has an unknown adapter type", edge.id);
            }
        }
    }
    
    Ok(())
}

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
            inputs: vec![
                crate::model::PortDeclaration {
                    port_id: "in".to_string(),
                    name: "input".to_string(),
                    ty: "i64".to_string(),
                    kind: crate::model::PortKind::Value,
                    description: None,
                }
            ],
            outputs: vec![
                crate::model::PortDeclaration {
                    port_id: "out".to_string(),
                    name: "output".to_string(),
                    ty: "i64".to_string(),
                    kind: crate::model::PortKind::Value,
                    description: None,
                }
            ],
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
            inputs: vec![
                crate::model::PortDeclaration {
                    port_id: "in".to_string(),
                    name: "input".to_string(),
                    ty: "Stream<i64>".to_string(),
                    kind: crate::model::PortKind::Stream,
                    description: None,
                }
            ],
            outputs: vec![
                crate::model::PortDeclaration {
                    port_id: "out".to_string(),
                    name: "output".to_string(),
                    ty: "i64".to_string(),
                    kind: crate::model::PortKind::Value,
                    description: None,
                }
            ],
            effects: vec![],
            ..Default::default()
        }
    }
    
    fn create_test_edge(id: &str, from: &str, to: &str) -> Edge {
        Edge {
            id: id.to_string(),
            from: crate::model::Endpoint { node: from.to_string(), port: "out".to_string() },
            to: crate::model::Endpoint { node: to.to_string(), port: "in".to_string() },
            policy: crate::model::EdgePolicy {
                adapter: crate::model::AdapterKind::None,
                priority: 0,
            },
            ..Default::default()
        }
    }
}

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
    
    // Helper functions for creating test nodes and edges
    fn create_test_node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            kind: NodeKind::Block,
            fq_block: Some("test/simple".to_string()),
            version_req: Some("^0.2".to_string()),
            inputs: vec![
                crate::model::PortDeclaration {
                    port_id: "in".to_string(),
                    name: "input".to_string(),
                    ty: "i64".to_string(),
                    kind: crate::model::PortKind::Value,
                    description: None,
                }
            ],
            outputs: vec![
                crate::model::PortDeclaration {
                    port_id: "out".to_string(),
                    name: "output".to_string(),
                    ty: "i64".to_string(),
                    kind: crate::model::PortKind::Value,
                    description: None,
                }
            ],
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
            inputs: vec![
                crate::model::PortDeclaration {
                    port_id: "in".to_string(),
                    name: "input".to_string(),
                    ty: "Stream<i64>".to_string(),
                    kind: crate::model::PortKind::Stream,
                    description: None,
                }
            ],
            outputs: vec![
                crate::model::PortDeclaration {
                    port_id: "out".to_string(),
                    name: "output".to_string(),
                    ty: "i64".to_string(),
                    kind: crate::model::PortKind::Value,
                    description: None,
                }
            ],
            effects: vec![],
            ..Default::default()
        }
    }
    
    fn create_test_edge(id: &str, from: &str, to: &str) -> Edge {
        Edge {
            id: id.to_string(),
            from: crate::model::Endpoint { node: from.to_string(), port: "out".to_string() },
            to: crate::model::Endpoint { node: to.to_string(), port: "in".to_string() },
            policy: crate::model::EdgePolicy {
                adapter: crate::model::AdapterKind::None,
                priority: 0,
            },
            ..Default::default()
        }
    }
}