Shtairir v0.2 — Visual Graph Model (Editor Canonical)
Purpose
Defines the canonical JSON/TOML graph model used by the editor and tools. This model maps 1:1 to the v0.2 language spec and is directly serializable via serde in Rust. It is non-ambiguous and excludes any semantics not present in the language spec.

Design Tenets
- 1:1 mapping to language concepts: nodes = blocks/subgraphs; ports = typed inputs/outputs; edges = typed connections.
- Determinism-first: policies that affect scheduling/backpressure must be explicit in node/edge metadata.
- Separation of concerns: semantic fields vs non-semantic layout/annotations.
- Stable IDs and provenance to enable diffs and undo/redo.

Schema (JSON conceptual; TOML equivalent allowed)
Top-level
{
  "schema_version": "0.2",
  "id": "graph:examples/counter@0.2.0",
  "namespace": "examples",
  "name": "counter",
  "version": "0.2.0",
  "visibility": "public",             // "public" | "internal"
  "effects": ["time.read"],           // union of node effects, computed or stored after validation
  "requires": [
    {"module": "std.stream", "version_req": "^0.2"},
    {"module": "std.math", "version_req": "^0.2"}
  ],
  "generics": [
    // For subgraphs with type parameters
    {"name": "T", "bounds": ["Serializable", "Eq"]},
    {"name": "Acc", "bounds": ["Default", "Add"]}
  ],
  "exports": [
    // Exported ports for subgraph use
    {"export_id": "out", "from_node": "fold1", "from_port": "out"}
  ],
  "nodes": [...],
  "edges": [...],
  "annotations": {...},
  "provenance": {...},
  "integrity": {...}
}

Node
{
  "id": "fold1",                            // stable within graph
  "kind": "block" | "subgraph" | "macro",   // macro = editor expansion shorthand; must lower to block/subgraph
  "fq_block": "std.stream/fold",            // required for kind=block
  "version_req": "^0.2",                    // semver query; frozen concrete version stored post-resolve
  "concrete_version": "0.2.1",              // optional cache; validator may populate
  "title": "Fold",                          // non-semantic label
  "purity": "pure" | "effect",              // copy from block manifest to aid editor decisions (redundant but helpful)
  "effects": [],                            // from block manifest (redundant)
  "generics": {                             // optional instantiation overrides; often inferred from edges
    "T": "i64",
    "Acc": "i64"
  },
  "params": {                               // block parameter values (serde-serializable)
    "init": 0,
    "step": {"ref": "std.math/add_i64"}     // reference to a pure step node or built-in op
  },
  "inputs": [
    {"name": "stream", "port_id": "in", "ty": "Stream<i64>", "kind": "stream"}
  ],
  "outputs": [
    {"name": "out", "port_id": "out", "ty": "Stream<i64>", "kind": "stream"}
  ],
  "layout": {...},                          // non-semantic
  "meta": {...}                             // reserved for tool-specific semantic-free metadata
}

Port (embedded in node.inputs/outputs)
{
  "name": "values",
  "port_id": "values",
  "ty": "list<i64> | Stream<T> | Event<T> | (i64,string) | Struct{...} | Enum{...}",
  "kind": "value" | "stream" | "event" | "composite"
}

Edge
{
  "id": "e1",
  "from": {"node": "src1", "port": "out"},
  "to": {"node": "fold1", "port": "in"},
  "policy": {
    // Explicit adapter/backpressure/merge policies
    "adapter": "none | map | filter | buffer | window | debounce | merge | zip",
    "adapter_params": {"fn": "std.math/add_i64", "window": {"kind": "count", "n": 10}},
    "backpressure": "block | drop_oldest | drop_newest | expand",
    "ordering": "source | timestamp | stable_key",
    "timestamp_source": "inherit | node:<id> | edge:<id>"
  },
  "notes": "optional"
}

Annotations (non-semantic)
{
  "title": "My Counter Graph",
  "comments": [
    {"id": "c1", "text": "Counts clicks", "target": {"node": "fold1"}}
  ],
  "layout": {
    "nodes": {
      "fold1": {"x": 480, "y": 320, "w": 160, "h": 80, "collapsed": false, "group": "g1"},
      "src1": {"x": 240, "y": 320}
    },
    "groups": {
      "g1": {"title": "Accumulation", "color": "#445", "collapsed": false}
    },
    "edges": {
      "e1": {"waypoints": [{"x":300,"y":320},{"x":450,"y":320}]}
    },
    "viewport": {"x":0,"y":0,"zoom":1.0}
  },
  "tags": ["demo", "counter"]
}

Subgraphs and Macros
- Subgraph: a graph that can be used as a node. It declares exports mapping internal node ports to external ports and may declare generics and bounds.
- Macro: editor-time shorthand that expands into a subgraph pattern; must be lowered to concrete nodes/edges before save/publish. Macros never change semantics; they are sugar.

Editor UX Derived from Schema
- Snapping and grouping:
  - Layout.groups and node.group enable semantic-free grouping; snap-to-grid enabled by coordinates; groups are collapsible (collapsed flag).
- Collapse/expand:
  - Nodes and groups support collapse; subgraphs can be entered/exited via breadcrumb; exports are visible as external ports.
- Inspector panels:
  - Edits params, version_req, generics, and policies. Live validation overlays display diagnostics linked to node/edge ids.
- Node palette taxonomy:
  - Palette organizes by namespace/module and tags. Fuzzy search queries name, tags, effects, and types. Results respect capability constraints (only compatible with current selection).
- Quick-actions:
  - Auto-wire suggests compatible ports based on type unification and kind (value/stream/event).
  - Auto-insert adapters: when connecting Stream<T> to T, suggest map, fold with init, or latest-to-value boundary (effectful if needed).
- Live validation overlays and fix-its:
  - Surface diagnostic code, message, and fix-it. Clicking fix-it may insert a policy entry or adapter node.

Provenance and Versioning
- Change sets:
  - Editor emits semantic events: NodeAdded, NodeRemoved, PortRenamed, EdgeAdded, EdgePolicyChanged, ParamChanged, LayoutChanged, etc., referencing ids.
- Graph diffs:
  - Diffs are lists of semantic changes against ids; used for reviews and merges.
- Versioning:
  - Graph.version follows semver. On breaking shape/type changes, bump major. Provenance records:
    {"created_by":"user@time","engine":"shtairir@0.2.0","history":[...]}.

Validation Tiers
- Live: runs per keystroke/drag; fast checks (types, edges, missing policy).
- Preflight: full type inference, effect safety, cycle detection, version resolution; must pass before running.
- Publish: includes integrity hashing, dependency freeze (resolve version_req to concrete versions), and policy checks against registry/org policy.

Registry Mapping
- Storage:
  - Graphs stored as JSON (editor) and optionally normalized JSON/TOML for registry. Registry records content_hash (sha256 of canonical form), metadata, and dependencies.
- Reference:
  - Graphs referenced by "graph:namespace/name@version" and content_hash.
- Metadata fields:
  - authors, description, tags, engine_version_req, capability_flags.

Minimal Examples
1) Node JSON
{
  "id":"add1",
  "kind":"block",
  "fq_block":"std.math/add",
  "version_req":"^0.2",
  "params":{"bias":0},
  "inputs":[
    {"name":"a","port_id":"a","ty":"i64","kind":"value"},
    {"name":"b","port_id":"b","ty":"i64","kind":"value"}
  ],
  "outputs":[{"name":"out","port_id":"out","ty":"i64","kind":"value"}]
}

2) Edge JSON with policy
{
  "id":"e_add_to_map",
  "from":{"node":"src","port":"out"},
  "to":{"node":"map1","port":"in"},
  "policy":{"adapter":"map","adapter_params":{"fn":"std.math/add_i64"},"backpressure":"block","ordering":"source"}
}

3) Layout metadata (excerpt)
{
  "annotations":{
    "layout":{
      "nodes":{"add1":{"x":120,"y":200,"w":140,"h":60}},
      "viewport":{"x":0,"y":0,"zoom":1.0}
    }
  }
}

4) Subgraph with generics (JSON)
{
  "schema_version":"0.2",
  "id":"graph:std/stream_sum@0.2.0",
  "namespace":"std",
  "name":"stream_sum",
  "version":"0.2.0",
  "visibility":"public",
  "generics":[{"name":"T","bounds":["Add","Default","Serializable"]}],
  "nodes":[
    {
      "id":"fold1",
      "kind":"block",
      "fq_block":"std.stream/fold",
      "version_req":"^0.2",
      "generics":{"T":"$T","Acc":"$T"},
      "params":{"init":"$Default(T)","step":{"ref":"std.math/add"}} ,
      "inputs":[{"name":"in","port_id":"in","ty":"Stream<$T>","kind":"stream"}],
      "outputs":[{"name":"out","port_id":"out","ty":"Stream<$T>","kind":"stream"}]
    }
  ],
  "edges":[],
  "exports":[{"export_id":"out","from_node":"fold1","from_port":"out"}]
}

TOML equivalents (minimal)
# Node TOML
[id="add1"]
kind = "block"
fq_block = "std.math/add"
version_req = "^0.2"

[params]
bias = 0

[[inputs]]
name = "a"
port_id = "a"
ty = "i64"
kind = "value"

[[inputs]]
name = "b"
port_id = "b"
ty = "i64"
kind = "value"

[[outputs]]
name = "out"
port_id = "out"
ty = "i64"
kind = "value"

Open Questions
- Exact canonicalization order for graph JSON hashing.
- Whether Event<T> is distinct in storage or a stream with annotation kind="event".
- Standard set of adapter identifiers and their parameter schemas (map, filter, buffer, window, debounce, merge, zip) to lock for v0.2.