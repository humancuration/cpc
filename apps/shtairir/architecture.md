# Shtairir Toolchain Architecture (concise)

Purpose
- Provide a Rust-first toolchain that converts textual and visual programs into a shared IR (v1), validates them, resolves registry references, and prepares them for execution on WASM and Win64 hosts.

Principles
- Hexagonal, screaming architecture; vertical slices per feature (parser, IR, registry, runtime).
- Core remains host-agnostic; hosts implement capability providers via traits.
- Determinism-first with explicit capability declarations and effects.

Component Breakdown

1) Parser (pest) → AST → IR
- Input: Textual Shtairir source files.
- Output: IR v1 (serde-friendly structures).
- Responsibilities:
  - Tokenize/parse using grammar in docs/shtairir/06-text-grammar.md.
  - Build AST preserving source spans for diagnostics and IR source maps.
  - Name-resolution for `use` aliases and fully-qualified block refs.
  - Type-check ports/params with registry-provided signatures (see 03-types).
  - Lower to IR:
    - Create nodes (instance_id, ref, params, determinism/effects copied from registry).
    - Create consts for literals feeding input ports.
    - Create edges from wire statements: "->" (data), "~>" (event).
    - Build subgraphs from fn declarations with scoped inputs/outputs.

2) Visual Graph Compiler → IR
- Input: Visual editor model (nodes, pins, wires, canvas metadata).
- Output: IR v1 identical to textual output (round-trippable).
- Responsibilities:
  - Validate block references and port kinds/types against registry.
  - Preserve ui metadata in node.source.ui for editors.
  - Export/import canonical IR JSON for sharing and version control.

3) Registry Client
- Resolves module.block@version to block specs, types, effects (see 02-blocks-and-modules and 04-versioning).
- Responsibilities:
  - Apply version constraints and lockfile rules.
  - Fetch/capture module manifests, block signatures, TypeSpecs.
  - Provide determinism/effects and capability scope metadata (including param-dependent scopes).

4) Validator (IR-level)
- Input: IR v1 graph/subgraphs.
- Responsibilities:
  - Type existence and compatibility checks (03-types).
  - Port compatibility: direction/kind (data vs event), required wiring/defaults.
  - Param presence/defaults; coercions only when allowed.
  - Effects/capabilities presence and policy preflight.
  - Resource handle hygiene: warn on unclosed handles.
  - Identifier rules (snake_case nodes; CamelCase subgraphs recommended).

5) Runtime Host Bindings (Capabilities)
- Traits for capability providers: time.now, net.http, storage.kv, channel.subscribe, audio.play, etc.
- Responsibilities:
  - Enforce determinism policy (pure/time/entropy/io dependent).
  - Mediate capability calls with allow/deny/prompt.
  - Provide async execution environment (Tokio on native; wasm futures on web).
  - Manage scheduling, queues, and backpressure (policy TBD in runtime doc).

Data Flow (bullets)
- Text Source → Parser (pest) → AST → Resolve (Registry) → Validate → IR v1 → Serialize (JSON/CBOR) → Host Runtime
- Visual Model → Visual Compiler → Validate → IR v1 → Serialize → Host Runtime
- Registry → Supplies types/effects/capabilities/determinism metadata to Parser/Visual/Validator

Responsibilities Summary
- Parser/Visual: Build IR; produce clear diagnostics; preserve source maps.
- Registry: Resolve refs; inform types and effects; cache and lock.
- Validator: Enforce IR and type rules; compute determinism summary.
- Host Runtime: Execute IR with capability providers; schedule; enforce policy.

Platform Targets
- wasm32-unknown-unknown (web): Yew editor, wasm-bindgen parser/validator; limited capabilities per browser policies.
- x86_64-pc-windows-msvc: CLI tools and native editor hosting; broader capability surface.

Open Items / TODO
- Define event/data backpressure semantics and queue sizing strategy.
- Lockfile format for module resolution and reproducibility.
- Standard flow/control nodes (select, merge, debounce, throttle) and their canonical IDs.
- Resource handle lints across subgraph boundaries.