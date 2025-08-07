# Shtairir Toolchain (docs only)

Shtairir is a unified scripting system with textual and visual front-ends that compile to a shared Intermediate Representation (IR) and execute on Rust hosts for Web (WASM) and Windows 64-bit. This folder documents the upcoming toolchain; code will be added in phases.

Targets
- wasm32-unknown-unknown (Web): Visual editor (Yew) and textual parser (pest) compiled to WASM.
- x86_64-pc-windows-msvc (Win64): Native CLI/GUI tools for parsing, validation, packaging, and runtime integration.

Planned Components
- Parser (pest):
  - Text → AST → IR v1.
  - Uses the skeleton grammar in docs/shtairir/06-text-grammar.md.
- Visual Graph Compiler:
  - Visual nodes/wires → IR v1 (round-trippable with textual front-end).
- IR and Validator:
  - IR v1 as described in docs/shtairir/05-ir.md.
  - Validation for types, kinds (event/data), port compatibility, params, capabilities, determinism, and resource handle hygiene.
- Registry Client:
  - Resolves module.block@version to block specs, types, effects (see docs/shtairir/02-blocks-and-modules.md and 04-versioning-and-compatibility.md).
- Runtime Host Bindings:
  - Capability traits (e.g., time.now, net.http, storage.kv).
  - Determinism enforcement and capability gating per host policy.

Execution Model Summary
- Event-driven dataflow graph of typed nodes (blocks).
- Events are push streams; data edges can buffer (latest or bounded queue).
- Subgraphs provide function-like scopes with their own inputs/outputs.
- Nodes declare determinism and effects; hosts enforce policies.

Example: Text → IR Sketch

Text (pseudo):
  use org.cpc.std as std;
  let now = std.time.now();
  let add = std.math.add(a: 5, b: 0);
  now.now -> add.b;
  export sum = add.sum;

IR (JSON-like):
{
  "ir_version": 1,
  "program_id": "example.add_now",
  "inputs": [],
  "outputs": [{ "id": "out_sum", "name": "sum", "type": "Int", "kind": "data" }],
  "consts": [{ "const_id": "c1", "type": "Int", "value": 5 }],
  "nodes": [
    { "instance_id": "n_now", "ref": "org.cpc.std/time.now@1.0.0",
      "params": { "monotonic": false }, "determinism": "time_dependent",
      "effects": [{ "capability": "time.now", "mode": "read" }] },
    { "instance_id": "n_add", "ref": "org.cpc.std/math.add@1.0.0",
      "params": {}, "determinism": "pure", "effects": [] }
  ],
  "edges": [
    { "source": { "const_id": "c1", "port": "value" }, "target": { "node_id": "n_add", "port": "a" }, "kind": "data", "buffer": "latest" },
    { "source": { "node_id": "n_now", "port": "now" }, "target": { "node_id": "n_add", "port": "b" }, "kind": "data", "buffer": "latest" }
  ],
  "subgraphs": []
}

Development Roadmap
- Phase 1 (this commit): IR v1, pest grammar skeleton, architecture outline.
- Phase 2: AST + IR serializer/deserializer (serde), validator scaffolding, registry mock.
- Phase 3: Visual editor export/import and textual/visual round-trip.
- Phase 4: Host bindings and capability traits; standard library coverage.

Alignment with CPC Principles
- Hexagonal architecture, screaming architecture, vertical slices.
- Rust-first, serde for serialization, pest for parsing.
- Federation-oriented with explicit capabilities and determinism flags.