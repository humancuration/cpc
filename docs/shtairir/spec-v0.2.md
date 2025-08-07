Shtairir v0.2 — Language and Execution Model Spec
Purpose
Single source of truth for implementers of the engine, editor, and registry. Rust-first, hexagonal architecture; vertical slices; screaming architecture. Keep semantics deterministic and explicit; effects and async are modeled, typed, and validated.

Rust-idiomatic stance
Shtairir deliberately mirrors Rust mental models:
- Types: ADTs (struct/enum), tuples, options, results, slices/lists, maps; serde-friendly.
- Generics and trait-like bounds: T: Add + Default + Serialize mirrors Rust trait bounds.
- Ownership and purity: “pure” blocks behave like & functions without side effects; “effect” blocks are like functions that use capabilities/traits to access side effects.
- Async/streams: Stream<T> resembles async streams with explicit backpressure and scheduling; Event<T> is a specialized stream.
- Determinism: explicit seeds and policies, no hidden global state, reproducible by design.

Language Pillars
1) Strong static typing with inference
- Types: scalars (i64, f64, bool, string, bytes, decimal, datetime, duration, uuid, json), tuples (T1, T2, ...), structs {field: T}, enums Enum{Variant(T?)}, options option<T>, lists list<T>, maps map<K,V> (K ∈ {string}).
- Inference: local type inference for block wiring and let-bindings; generics resolved via constraints; flows must be monomorphic at each node after inference.
- Type aliases allowed within modules.

2) Algebraic data types
- Struct: named fields; immutable; value-semantic.
- Enum: tagged unions with optional payloads.
- Tuple: positional aggregate; either named types or inferred.
- Pattern projection via composite ports in the visual graph (see Visual Graph spec).

3) Generics and constraints
- Type variables: T, U, Acc. Written as generic params on blocks and subgraphs.
- Trait-like constraints (Rust-style): T: Add + Eq + Serialize; Acc: Default + Add<Output=Acc>.
- Named capability traits map to engine-provided Rust traits; where possible, use Rust traits directly in native blocks.
- Bounds syntax (conceptual): generics = [T: Add + Serialize, Acc: Default + Add<Output=Acc>].
- Constraints drive adapter insertion (e.g., map/reduce) and validation.

4) Traits/typeclasses-like reuse
- Blocks may declare they require certain capabilities on generic types. Registry validation checks bounds are satisfiable using known engine capabilities or block-provided impls.
- Capabilities are engine-known names (e.g., Add, Default, Ord, Serialize, RandomSeed).

5) Purity model
- Block purity: pure | effect.
- Pure: referentially transparent, no effects, no time dependence.
- Effectful: declare effects = ["domain.resource", "domain.*"]. Allowed in graphs subject to effect safety rules.
- Effects are first-class in type metadata, not values. They partition scheduling and validation.

6) Async/stream primitives
- Value ports: single T values (pull-at-start or on-demand). Think fn(T) -> U.
- Stream<T>: ordered sequence of T with monotonic logical time (akin to futures::Stream<Item = T>).
- Event<T>: sparse, instantaneous occurrences; represent as Stream<T> with event-kind; same typing rules.
- Combinators (Rust-flavored): map, filter, fold, reduce, window, debounce, merge, zip. Step functions are pure closures or pure subgraphs.
- Time windows: count(n) or duration(d). Duration uses a clock capability; deterministic when policy is explicit or a simulated clock is provided.

7) Deterministic dataflow
- Graphs are directed acyclic for pure portions; controlled feedback only via validated patterns (see Cycle rules).
- Determinism guaranteed unless explicitly using nondeterministic blocks (e.g., random, now). Such blocks must expose seed or clock policy for reproducibility.

Modules and Packaging
- Namespaces: module.name (snake_case dot-separated).
- Blocks and graphs live under modules; fully-qualified: module.name/block_name; subgraphs export public names.
- Visibility: public (exported), internal (not exported); editor shows only public by default.
- Imports: graph manifests declare required module@versions; resolution via registry.
- Versioning: semver; breaking changes increment major. Compatible changes: add optional input with default, widen generic constraints safely, add outputs only if marked additive and not required by dependents.
- Block/Graph SemVer compatibility:
  - Major increment on: remove/rename port, change type incompatibly, tighten constraints, change determinism/purity.
  - Minor increment on: add optional ports with defaults, add new effect domains (if purity already effectful and caps allow), performance changes with identical semantics.
  - Patch: docs, fixes without shape/type changes.

Interop
- Rust FFI for native blocks:
  - Block trait concept (pseudocode): trait Block { type In; type Out; fn eval(&self, input: Self::In, ctx: &mut Ctx) -> Self::Out; }
  - Async/streams (pseudocode): trait StreamBlock { type In; type Out; type S: Stream<Item = Self::Out>; fn spawn(&self, input: Self::In, ctx: &mut Ctx) -> Self::S; }
  - Types map 1:1 to serde-friendly Rust types; use #[derive(Serialize, Deserialize)] for ADTs used across the boundary.
  - Capability traits bound by generic constraints (e.g., Add) map to Rust traits provided by engine; prefer standard traits where applicable (Add, Default, Ord).
- WASM target:
  - Blocks compiled to WASM must only use allowed host functions. Effects only through capabilities injected by the engine consistent with manifest effects.
  - Determinism: disallow ambient nondeterminism; require explicit seeds/clock policy. Prefer deterministic RNGs (e.g., ChaCha seeded).

Serialization Formats
- Canonical block manifest: TOML (registry-schema-v0.2.md defines exact schema).
- Graph manifest: JSON or TOML canonicalization rules; prefer JSON for editor saves, TOML for human-authored small graphs.
- Stable IDs:
  - Node IDs: UUID v4 or content-address-based deterministic IDs; editor can regenerate but must preserve on round-trip.
  - Block IDs: module/name@version with stable id field. Registry may compute content-hash sha256 of manifest normalized form.
- Content addressing:
  - content_hash: sha256 of canonical form (sorted keys, UTF-8, line endings normalized); stored on manifests for integrity.

Validation Rules
Type checking
- All edges must connect compatible types; implicit upcast only if declared by adapters; no implicit numeric widening unless rule exists in engine policy.
- Generic instantiation: substitute concrete types for type variables satisfying all constraints; report error if unsatisfied.
- Composite ports:
  - Struct/tuple destructure: connecting from composite output to multiple field/element inputs is allowed via composite out ports.
  - Enum variant ports must select a specific variant via Variant ports or pattern-matcher node.

Cycle detection and feedback
- Pure subgraph must be DAG. Feedback allowed only when:
  - There’s an explicit stateful node (fold/reduce/accumulator) that breaks the cycle with initial state.
  - Or when bridging via Event feedback with explicit debounce/window and well-defined initial emission.
- Validation rejects cycles not broken by allowed state nodes.

Effect safety
- Pure nodes cannot depend on effectful nodes unless mediated by a boundary node (e.g., to_value with capture semantics) that freezes values and declares determinism loss; such boundary is effectful.
- Effect domains caps:
  - Graph must declare cumulative effects = union of nodes’ effects.
  - Publish policy can restrict allowed domains; editor surfaces violations.
- Crossing rules:
  - value -> effectful ok, effectful -> pure not allowed directly; requires boundary node which is effectful.

Stream/Event contracts
- Stream arity: any number of producers may merge if merge policy specified (ordered merge, stable key merge, interleave); defaults must be explicit in graph or node config.
- fold:
  - type: fold<Stream<T>, Acc>(init: Acc, step: (Acc, T) -> Acc) -> Stream<Acc>.
  - Deterministic if input ordering is deterministic and step is pure.
- reduce:
  - like fold but emits on window close; requires associative operator or explicit order preservation.
- window:
  - policies: count(n), time(duration, clock=logical|wall); wall requires effect domain time.read.

Execution Model
Scheduler
- Topological order for ready pure nodes per tick.
- Tick: minimal unit when new input values or stream events arrive. Streams processed in event-time order per source; merge policy defines cross-source ordering.
- Async:
  - Effectful IO nodes run on async runtime; outputs enqueue events with timestamps.
  - Buffering: backpressure strategies must be explicit on edges or nodes: drop_oldest, drop_newest, block, expand (bounded by policy).
- Concurrency:
  - Parallel execution allowed when no dependencies and resource caps permit; determinism preserved by strict ordering of event commit per timestamp and per-edge sequence numbers.

Determinism guarantees
- Given same inputs, seeds, and event-time schedule, outputs are bit-for-bit identical.
- Randomness:
  - Explicit Random node: params include seed: u64; if omitted, graph must provide seed via a seed port; otherwise nondeterministic and must be marked nondeterministic with policy disallowing publish by default.
- Time:
  - now() requires effect domain time.read. Determinism only if a simulated clock is provided by test harness or graph config.

Errors and Diagnostics
- Diagnostic structure:
  - severity: error | warning | info
  - code: short stable string (e.g., ST1001 TypeMismatch, ST2001 CycleDetected, ST3001 EffectBoundaryViolation)
  - message: human-readable
  - spans: array of {file_or_asset, node_id, port_id, range?}
  - hints: array of quick-fix suggestions (e.g., "Insert map adapter", "Add fold with init: 0")
- Typical codes:
  - ST1001 TypeMismatch
  - ST1002 GenericUnsatisfied
  - ST1100 UnknownType
  - ST1200 PortNotFound
  - ST1300 MissingDefaultForNewInput
  - ST2001 CycleDetected
  - ST2100 WindowPolicyRequired
  - ST2200 StreamMergePolicyMissing
  - ST3001 EffectBoundaryViolation
  - ST3002 DisallowedEffectDomain
  - ST4000 NonDeterminismNotSeeded

Motivating Examples
1) Generic add with constraint
- Block: add<T: Add + Default>(a: T, b: T, bias?: T=default()) -> T.
- Wiring two i64 outputs infers T = i64.

2) Struct projection
- Block emits User { id: uuid, name: string }. Downstream nodes can attach to .id and .name composite output ports.

3) Enum variant handling
- ParseResult = Ok(T) | Err(string).
- Visual graph uses Variant Ok(T) port to handle success, merge later.

4) Async stream with fold (worked example)
- Inputs:
  - clicks: Stream<i64> (Event)
- Nodes:
  - map to 1 per click
  - fold with init=0, step(acc, x) => acc + x
- Policies:
  - merge not needed (single source)
  - buffering: block
- Output:
  - count: Stream<i64> deterministically increases with each click.

Worked example shape (ASCII)
[clicks: Event<i64>] --map(+1)--> [fold(init=0, step=+)] --> [count Stream<i64>]

Rust-analogy for worked example (pseudocode)
fn count_clicks(clicks: impl Stream<Item = i64>) -> impl Stream<Item = i64> {
    clicks.map(|_| 1).scan(0, |acc, x| { *acc += x; Some(*acc) })
}

Minimal TOML for fold node config (graph manifest excerpt)
[nodes.fold1]
block = "std.stream.fold"
version = "^0.2"
params.init = 0
params.step = "add_i64" # reference to pure step subgraph or builtin

Open Questions
- Exact set of built-in capability trait names and mapping to Rust traits.
- Canonical JSON normalization rules: specify full key ordering and whitespace?
- Standard library blocks inventory and reserved names for adapters.
- Precise semantics for Event vs Stream in editor visuals (one concept or two kinds with mapping?).