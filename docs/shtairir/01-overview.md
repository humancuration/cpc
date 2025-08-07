# Shtairir Overview

Shtairir is a unified scripting system with both textual and visual front-ends that compile to a shared intermediate representation (IR) and execute in Rust hosts (Web via WASM and Windows 64-bit). It is designed for modularity, composability, and safety across a federation of apps that emphasize connectivity, collaboration, and volunteerism.

Goals
- One language, two front-ends: a friendly visual block editor and a concise textual syntax, producing the same IR.
- Strong typing and explicit capabilities for safe composition and predictable execution.
- Deterministic-by-default for reproducibility; opt-in non-determinism for real-world IO and time.
- Portable execution across WASM (web) and native Windows hosts using a capability-gated runtime.
- Extensible registry of modules/blocks with semantic versioning and compatibility guarantees.
- Hexagonal and screaming architecture: clear boundaries, domain-first vertical slices, Rust-first implementation.

Principles
- Safety by declaration: blocks declare inputs, outputs, parameters, effects, and required capabilities.
- Least privilege runtime: execution requires explicit capability grants by host or user policy.
- Async and event-driven: programs react to events, message passing, and timers using async execution.
- Determinism-first: pure blocks are strongly preferred; effects are explicit and sandboxed.
- Reuse and composability: common blocks form a standard library; domain apps add their own modules.
- Federation-oriented: blocks can communicate via events, channels, and capability-scoped networking.

Execution Model
- Event-driven graph of blocks:
  - Blocks run in a directed graph where outputs feed inputs.
  - Events trigger block execution; dataflow and control edges are represented in the IR.
- Async Rust runtime:
  - Each program runs on an async executor (Tokio where available; WASM-compatible futures on web).
  - Blocking IO is forbidden; all effects are async and capability-gated.
- Determinism flags:
  - Blocks declare determinism: pure (deterministic), time/entropy-dependent, IO-dependent.
  - The host can enforce policies (e.g., deny non-deterministic blocks for reproducible runs).
- Scheduling:
  - Cooperative scheduling via futures; the host orchestrates ticks and event loop integration.
  - Message queues per program, optional priority lanes for UI responsiveness.
- State:
  - Blocks are stateless by default; local state is allowed when declared via lifecycle hooks.
  - Persistent state requires storage capabilities (e.g., storage.kv) and is explicit in effects.

Textual and Visual Front-Ends
- Textual syntax:
  - Parsed with pest into an AST which is normalized into the shared IR.
  - Human-readable modules and block invocations with typed signatures and capability annotations.
- Visual blocks:
  - Nodes and wires represent blocks and connections; editor exports/imports a canonical IR.
- Shared IR:
  - Both front-ends compile to the same IR for validation, type checking, capability analysis, and execution.
  - IR is host-independent; host-specific bindings provide capability implementations.
  - IR serialization supports content hashing for caching and reproducibility.

Host Integration (Rust-first)
- Web (WASM):
  - Runs in a sandboxed WASM runtime; capabilities are implemented via host shims using web-sys and async glue.
  - Networking and storage are limited by browser policies and user grants.
- Windows (x64):
  - Native async runtime with broader capability surface; policies configurable by the host client.
- Capability Gate:
  - All non-pure operations go through a capability interface. Hosts decide if a program is allowed to use them, possibly prompting the user.
  - Capability declarations are compiled into IR metadata for pre-execution checks.

Roadmap
- This document (01-overview): goals and model.
- 02-blocks-and-modules: block/module spec with examples and capabilities.
- 03-types-and-signatures: type system, generics, hashing/versioning of types.
- 04-versioning-and-compatibility: SemVer policy, IR stability, registry resolution.
- Next: IR schema, registry format, compiler bindings (Rust traits, pest grammar, Yew-based visual editor).