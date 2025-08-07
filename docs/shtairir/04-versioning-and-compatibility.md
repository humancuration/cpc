# Versioning and Compatibility

This document defines the semantic versioning policy for Shtairir modules and blocks, IR stability guarantees, registry resolution, conflict handling, and deprecation/shim guidance.

Scope
- Modules: versioned collections of blocks, types, and metadata.
- Blocks: versioned units within modules.
- IR: shared intermediate representation produced by both textual and visual front-ends.
- Registry: resolves module/block versions and their dependencies/capabilities.

Semantic Versioning (SemVer)
- Format: MAJOR.MINOR.PATCH
- Patch (x.y.Z):
  - Bug fixes, performance improvements, documentation.
  - No changes to signatures, types, capabilities, determinism flags, or error domains.
- Minor (x.Y.z):
  - Additive, backward-compatible changes:
    - Add new blocks.
    - Add optional params with defaults.
    - Add optional outputs (Option[T] or Result Ok expansion that remains backward-compatible).
    - Extend error enums with optional variants if callers are not forced to handle them (see notes).
  - Must not:
    - Remove inputs/outputs/params.
    - Change required params or tighten types.
    - Introduce new required capabilities or change determinism from pure to effectful.
- Major (X.y.z):
  - Breaking changes:
    - Change input/output/param types incompatibly.
    - Remove/rename identifiers.
    - Add required capabilities or change determinism to more effectful class.
    - Alter error types in a way that breaks existing handlers.

Block-Level Versioning Rules
- Blocks carry a version within the module.
- A module release pins a specific set of block versions.
- Prefer “new id” for breaking changes (e.g., string.format2) to enable side-by-side migration within the same module major.

Capabilities and Versioning
- Adding a new required capability to a block is breaking (MAJOR).
- Narrowing capability scope (e.g., net.http → net.http with specific host) is breaking unless the previous behavior remains default.
- Adding optional capability-gated behavior is MINOR if default behavior is unchanged when the capability is absent.

IR Stability Guarantees
- IR Major Versions:
  - IR v1: stable schema; new optional fields may be added.
  - Older runtimes must ignore unknown fields.
- Breaking IR changes (field removals, meaning changes) require IR major bump.
- Determinism annotations and capability semantics are part of IR; semantic changes require major bump.

Registry Resolution
- Constraints syntax:
  - ^1.2 (highest 1.x ≥1.2), ~1.2.3 (patches only), =2.0.0 (pin exact)
- Resolution algorithm:
  1) Collect constraints from the program and all transitive module references.
  2) Select the highest compatible version per module.
  3) If conflicts remain, prefer versions closest to explicit program constraints.
  4) If unresolved, fail with a diagnostic listing the conflicting constraints and suggest alternatives.
- Deduplication:
  - A program uses one version of a module at a time.
  - Advanced (opt-in): allow namespace-isolated side-by-side versions if fully qualified with version tags in the program lockfile; discouraged for general use.

Conflict Handling
- Host options:
  - Suggest compatible upgrade/downgrade sets that satisfy all constraints.
  - Allow per-program pinning via lockfile overrides.
  - Abort with diagnostics if no solution.
- Content-addressed caching:
  - Manifests and block specs cached by hash for reproducibility.

Deprecation and Shims
- Deprecation:
  - Mark blocks as deprecated with message and since version; keep at least one minor release before removal in next major.
- Shims/Adapters:
  - Minor releases may include adapters that wrap old signatures to new ones if safe (e.g., inject new optional params with defaults).
  - Shims must not add capabilities or change determinism.
  - Shims are versioned and removable in the next major.
- Migration guidance:
  - Provide examples and automated fix-it hints where feasible (e.g., rename refactor, param insertion).

Program Reproducibility
- Lockfiles:
  - Program lockfile records exact module versions and IR content hash.
  - Ensures consistent resolution across hosts (WASM, Win64).
- Determinism policy:
  - Hosts may require an explicit “allow non-determinism” flag for programs using time/IO/entropy blocks.
  - Seedable randomness and time mocking enable deterministic CI/test runs.

Error Compatibility
- Adding new error variants:
  - Minor if errors are surfaced as Result[T, Enum{...}] and callers handle as opaque with default paths; otherwise major.
- Renaming error codes or domains:
  - Major. Prefer adding aliases and deprecating old names first.

Breaking/Additive Change Examples
- Additive (MINOR):
  - string.format adds optional param strict: Bool = false.
  - net.http_get adds optional output redirect_chain: Option[List[Url]].
  - event.on_channel adds optional param buffer: UInt = 64.
- Breaking (MAJOR):
  - math.add changes Decimal inputs to Float.
  - event.on_channel changes event output from Any to Struct{payload:Bytes} without adapter.
  - time.now adds required param timezone.

Runtime Enforcement
- Preflight validation:
  - Verify module versions resolved, capabilities allowed, types compatible, determinism policies satisfied, and platform flags (wasm/win64) supported.
- Execution-time checks:
  - Capability calls are mediated by host; denied calls yield capability_denied errors.
  - IR major mismatch between program and runtime aborts with guidance to upgrade runtime or recompile IR.

Registry and Policy Examples
- Example resolve:
  - Program requests org.cpc.std ^1.2 and org.cpc.net ~2.1.3.
  - Registry selects org.cpc.std 1.9.0 and org.cpc.net 2.1.7 if available.
- Conflict example:
  - A requires org.cpc.std ^1.4; B requires org.cpc.std ^2.0.
  - Host suggests upgrading A to 2.x or pinning two isolated namespaces (discouraged).
  - If not possible, abort with diagnostics.

Audit and Telemetry
- Each block invocation includes module/block version and TypeIDs in tracing metadata (tracing crate).
- Hosts may collect deprecation usage stats to guide cleanup and adapter retention.

Summary
- Use SemVer strictly; prefer additive changes with defaults.
- Keep IR stable within major; ignore unknown fields for forward-compat.
- Resolve to a single version per module; pin via lockfiles for reproducibility.
- Deprecate before removing; provide shims when safe to smooth migrations.