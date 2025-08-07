Shtairir v0.2 — Registry Manifest Schema
Purpose
Defines block and graph manifests for the registry. These TOML/JSON documents are validated via serde in Rust and are the basis for discovery, versioning, integrity, and dependency resolution.

Scope
- Block manifest (TOML recommended)
- Graph manifest (JSON or TOML; editor saves JSON, registry accepts both)
- Dependency resolution and security policy
- Concrete examples: math.add, string.concat, math.sum_list (v0.2 style)

Common Conventions
- semver used for version fields and requirements
- ids are stable strings; content_hash is sha256 over canonical form (sorted keys, UTF-8, normalized newlines)
- capability_flags enumerate engine features used (e.g., "streams", "time", "wasm", "serde_json")
- effects must follow dot.segments with optional wildcard only at tail

Block Manifest Schema (TOML)
Top-level keys
id                string (stable identifier; recommended "module/name@version")
namespace         string (snake_case dot-separated module name; e.g., "examples.shtairir")
name              string (block short name; e.g., "math.add")
version           string (semver; e.g., "0.2.0")
title             string
description       string
authors           array<string>
license           string (placeholder text allowed)
tags              array<string>
purity            "pure" | "effect"
effects           array<string> (empty if purity="pure")
determinism       "Deterministic" | "Nondeterministic" (explicit)
generics          array<object> (optional)
  - name: string (e.g., "T", "Acc")
  - bounds: array<string> (e.g., ["Add", "Default", "Serializable"])
inputs            array<Port>
outputs           array<Port>
params            array<Param> (optional)
examples          array<string> (optional; brief usage hints)
tests             array<TestRef> (optional; references to fixtures or inline)
engine            object
  - version_req: string (e.g., "^0.2")
  - capability_flags: array<string>
integrity         object (optional)
  - content_hash: string (hex sha256)
  - signature: string (optional; detached sig)
metadata          object (optional free-form; serde_json::Value)

Port
name              string
ty                string (e.g., "i64", "option<string>", "list<i64>", "Struct{...}", "Stream<T>")
default           any (optional; must be JSON-compatible with ty)

Param
name              string
ty                string
default           any (optional)

TestRef
name              string
kind              "inline_graph" | "graph_ref"
graph             object|string (inline JSON/TOML or "graph:namespace/name@version")
expect            object (optional; e.g., {"diagnostics":0, "output_snapshot":"..."} )

Block Manifest Example (v0.2 style) — math.add
id = "examples.shtairir/math.add@0.2.0"
namespace = "examples.shtairir"
name = "math.add"
version = "0.2.0"
title = "Add"
description = "Adds two 64-bit integers with an optional bias parameter."
authors = ["CPC Coop"]
license = "CPC"
tags = ["math", "pure", "demo"]
purity = "pure"
effects = []
determinism = "Deterministic"

[[generics]]
name = "T"
bounds = ["Add", "Default", "Serializable"]
# For this concrete v0.2 block we also allow concrete i64 usage; generics retained for reuse.

[[inputs]]
name = "a"
ty = "i64"

[[inputs]]
name = "b"
ty = "i64"
default = 0

[[params]]
name = "bias"
ty = "i64"
default = 0

[[outputs]]
name = "result"
ty = "i64"

examples = [
  "result = math.add(a:2, b:3, bias:1) # → 6"
]

[engine]
version_req = "^0.2"
capability_flags = ["serde", "pure_values"]

[integrity]
content_hash = "sha256:REPLACEME"

Block Manifest Example — string.concat
id = "examples.shtairir/string.concat@0.2.0"
namespace = "examples.shtairir"
name = "string.concat"
version = "0.2.0"
title = "Concat"
description = "Concatenates two strings with an optional separator."
authors = ["CPC Coop"]
license = "CPC"
tags = ["string", "pure", "demo"]
purity = "pure"
effects = []
determinism = "Deterministic"

[[inputs]]
name = "a"
ty = "string"

[[inputs]]
name = "b"
ty = "string"

[[params]]
name = "separator"
ty = "option<string>"
default = null

[[outputs]]
name = "combined"
ty = "string"

examples = [
  "combined = string.concat(a:\"foo\", b:\"bar\", separator:null) # → \"foobar\""
]

[engine]
version_req = "^0.2"
capability_flags = ["serde", "pure_values"]

[integrity]
content_hash = "sha256:REPLACEME"

Block Manifest Example — math.sum_list
id = "examples.shtairir/math.sum_list@0.2.0"
namespace = "examples.shtairir"
name = "math.sum_list"
version = "0.2.0"
title = "Sum List"
description = "Sums a list of 64-bit integers."
authors = ["CPC Coop"]
license = "CPC"
tags = ["math", "pure", "list"]
purity = "pure"
effects = []
determinism = "Deterministic"

[[inputs]]
name = "values"
ty = "list<i64>"
default = []

[[outputs]]
name = "total"
ty = "i64"

examples = ["total = math.sum_list(values:[1,2,3,4]) # → 10"]

[engine]
version_req = "^0.2"
capability_flags = ["serde", "pure_values"]

[integrity]
content_hash = "sha256:REPLACEME"

Graph Manifest Schema
Top-level keys
id                string (e.g., "graph:examples/counter@0.2.0")
namespace         string
name              string
version           string (semver)
title             string
description       string
authors           array<string>
tags              array<string>
visibility        "public" | "internal"
generics          array<object> (same structure as block generics)
requires          array<object> [{module, version_req}]
effects           array<string> (union of node effects; optional cache)
exports           array<Export> (for subgraph blocks)
nodes             array<Node>
edges             array<Edge>
engine            object {version_req, capability_flags}
integrity         object {content_hash, signature?}
provenance        object {created_by, created_at, engine, history: array<Change>}
metadata          object (free-form)
annotations       object (non-semantic layout/comments; see visual-graph-model)

Export
export_id         string
from_node         string
from_port         string

Node
id                string
kind              "block" | "subgraph"
fq_block          string (for kind="block", "module/name")
version_req       string (semver req)
concrete_version  string (resolved; optional cache)
purity            "pure" | "effect"
effects           array<string>
generics          object<string,string> (instantiation map; may include "$T" placeholders that resolve at call site)
params            object<string, any> (serde)
inputs            array<PortDecl>
outputs           array<PortDecl>
meta              object (non-semantic)

PortDecl
name              string
port_id           string
ty                string
kind              "value" | "stream" | "event" | "composite"

Edge
id                string
from              object {node, port}
to                object {node, port}
policy            object
  - adapter         "none" | "map" | "filter" | "buffer" | "window" | "debounce" | "merge" | "zip" | "boundary"
  - adapter_params  object (adapter-specific)
  - backpressure    "block" | "drop_oldest" | "drop_newest" | "expand"
  - ordering        "source" | "timestamp" | "stable_key"
  - timestamp_source "inherit" | "node:<id>" | "edge:<id>"

Minimal Graph Example (JSON) — stream fold subgraph
{
  "schema_version":"0.2",
  "id":"graph:std/stream_sum@0.2.0",
  "namespace":"std",
  "name":"stream_sum",
  "version":"0.2.0",
  "title":"Stream Sum",
  "visibility":"public",
  "generics":[{"name":"T","bounds":["Add","Default","Serializable"]}],
  "requires":[
    {"module":"std.stream","version_req":"^0.2"},
    {"module":"std.math","version_req":"^0.2"}
  ],
  "nodes":[
    {
      "id":"fold1",
      "kind":"block",
      "fq_block":"std.stream/fold",
      "version_req":"^0.2",
      "generics":{"T":"$T","Acc":"$T"},
      "params":{"init":"$Default(T)","step":{"ref":"std.math/add"}},
      "inputs":[{"name":"in","port_id":"in","ty":"Stream<$T>","kind":"stream"}],
      "outputs":[{"name":"out","port_id":"out","ty":"Stream<$T>","kind":"stream"}]
    }
  ],
  "edges":[],
  "exports":[{"export_id":"out","from_node":"fold1","from_port":"out"}],
  "engine":{"version_req":"^0.2","capability_flags":["streams","serde"]},
  "integrity":{"content_hash":"sha256:REPLACEME"}
}

Dependency Resolution Rules
- Blocks and graphs declare engine.version_req; registry filters by engine compatibility.
- A node’s version_req is resolved to a concrete_version on Preflight/Publish; this freeze is included in integrity hashing for published artifacts.
- Conflicts:
  - If multiple nodes require incompatible versions of the same module, Preflight fails with a diagnostic (code: RS2100 VersionConflict).
- Resolution algorithm:
  1) For each node, collect candidate versions satisfying version_req and engine.version_req.
  2) Pick highest compatible version.
  3) Record concrete_version; update graph effects = union(effects of concrete blocks).

Security Posture and Policy
- Capability flags:
  - Each manifest states used runtime capabilities (e.g., "time", "net.http", "fs", "wasm"). Org policy can permit/deny.
- Effects caps:
  - Registry and engine enforce per-graph allowed effect domains. Blocks exceeding allowed domains are rejected at Publish (RS3002 DisallowedEffectDomain).
- WASM/native sandbox:
  - Native blocks must be in allow-listed crates/modules; WASM blocks must declare "wasm" in capability_flags and only access host functions corresponding to declared effects.
- Signature:
  - Optional signature may be verified by registry policy; mismatch between signature and content_hash is reject (RS3100 BadSignature).

Validation Summary (Registry)
- IDs: namespace, name formats; id must match "namespace/name@version".
- Types: ports and params must use valid v0.2 types; defaults JSON-compatible.
- Purity/effects: purity="pure" -> effects=[]; determinism consistent with block semantics.
- Ports: unique names within inputs, outputs, params; at least one output.
- Generics: bounds must reference known capability names; instantiation in graphs must satisfy bounds.
- Graph edges: type compatibility; event/stream kinds match or have explicit adapter "boundary"/"map".
- Cycles: as per language spec rules; stateful nodes required to break cycles.

Concrete Manifests (Upgrading Existing Examples to v0.2)
math.add (see example above)
string.concat (see example above)
math.sum_list (see example above)

Note: To align with v0.2, set purity, determinism, engine.version_req, and integrity fields, and introduce id, namespace split. Inputs/outputs/params remain compatible.

Open Questions
- Should id be computed (content-address form) or declared human-readable? Current: declared, plus content_hash.
- Canonicalization exact field order for hash: finalize JSON/TOML normalization guide.
- Policy schema for org-level allow/deny lists: embed here or in separate policy doc?