# Shtairir Blocks and Modules

This document defines the Block and Module specification used by Shtairir’s textual and visual front-ends and shared IR. It focuses on strong typing, explicit capabilities, determinism flags, lifecycle hooks, and error handling. Examples at the end illustrate complete block specs.

Core Concepts
- Block: A typed, reusable unit of computation with explicit effects. Blocks are the building bricks of programs.
- Module: A versioned collection of blocks plus shared metadata, docs, and capability surface declarations.

Identifiers
- Module ID: reverse-DNS style or scoped path: org.cpc.std, org.cpc.net, app.website_builder
- Block ID: module-local path, e.g., math.add, string.format. Fully-qualified block id is module_id/block_id (e.g., org.cpc.std/math.add)

Required Metadata
For Modules:
- module_id: string (unique in registry)
- name: human name
- version: SemVer (MAJOR.MINOR.PATCH)
- description: short text
- authors: list of strings
- license: omitted here; governed by CPC policy elsewhere
- capabilities: list of capability identifiers this module’s blocks may request
- tags: optional list for discovery
- compatibility:
  - min_host: minimum host runtime version
  - wasm: true/false support
  - win64: true/false support

For Blocks:
- block_id: string (module-local)
- name: human name
- version: SemVer
- description: short text
- inputs: list of typed input ports
- outputs: list of typed output ports
- params: list of typed, defaultable parameters (immutable per invocation)
- effects: declared side effects and capability requirements
- determinism: enum { pure, time_dependent, entropy_dependent, io_dependent }
- lifecycle: supported hooks
- errors: declared error domains and codes
- examples: example invocations and expected results
- docs: optional extended docs (markdown)
- stability: enum { stable, experimental, deprecated }

Signatures
Inputs/Outputs/Params
- Each port has:
  - name: string
  - type: Shtairir type reference (see 03-types)
  - default: optional literal or expression for params only
  - multiplicity: single | variadic
  - optional: bool (for inputs/params)
- Outputs may represent:
  - data: typed outputs
  - control: optional “done”, “error” control outputs in visual graphs (mapped to Result in textual IR)

Effects
- Capabilities declare required permissions (e.g., net.http, fs.read, time.now, audio.play, app.<app_id>.*).
- Effect entries:
  - capability: string
  - scope: optional string (e.g., domain/host/path/channel)
  - mode: read | write | execute | subscribe | publish
  - notes: optional rationale

Determinism
- pure: same inputs => same outputs, no effects
- time_dependent: reads clock, schedule, or time zone
- entropy_dependent: uses RNG or non-deterministic IDs
- io_dependent: external IO (network, file system, device sensors)
The host can enforce policies (allow/deny/mock) per determinism class.

Lifecycle Hooks
- init(context): before first execution; bind resources; may create ephemeral state
- start(context): after init; subscriptions or timers can be registered here
- tick(context): optional periodic callback (host-driven)
- message(context, msg): handle incoming events/messages
- stop(context): graceful stop; flush state
- dispose(context): release resources; finalization
Notes:
- Blocks may implement none or many hooks.
- Pure blocks typically use only the compute step implied by data arrival and do not rely on hooks.
- Hooks that cause effects must declare capabilities.

Error Handling
- Blocks must declare their error domain(s):
  - domain: e.g., net, parse, timeout, capability_denied
  - codes: structured enumeration (e.g., ECONN, ETIMEOUT, EFORMAT)
- Errors are values: represented via Result[T, E] outputs in textual IR and separate “error” output pins in visual IR.
- Host-level policy:
  - retry: exponential backoff with jitter (configurable)
  - fallback: optional param-provided fallback outputs or alternate paths
  - telemetry: tracing with deterministic correlation IDs

Validation Rules (IR/Registry)
- Every effect requires a declared capability present in the module and allowed by host policy.
- A block marked pure must not declare effects; validation will reject otherwise.
- Types must be resolvable and compatible (see 03-types).
- Version constraints:
  - Blocks can only depend on types and capabilities compatible with their declared versions.

Capabilities Model
- System namespaces:
  - fs.read, fs.write
  - net.http, net.ws
  - time.now, time.schedule
  - audio.play, audio.record
  - storage.kv, storage.blob
  - device.camera, device.microphone
- App-scoped:
  - app.<app_id>.* (e.g., app.website_builder.publish)
- Custom scopes:
  - channel.<name>.publish / .subscribe
- Capability checks occur pre-execution (static) and at runtime (dynamic) when effects are invoked.

Complete Example Block Specs

1) math.add (pure)
- module: org.cpc.std
- block_id: math.add
- version: 1.0.0
- description: Adds two numbers with Decimal precision.
- determinism: pure
- inputs:
  - a: Decimal
  - b: Decimal
- outputs:
  - sum: Decimal
- params: none
- effects: none
- errors: none
- examples:
  - inputs: a=1.2, b=3.4 → sum=4.6

2) string.format (pure)
- module: org.cpc.std
- block_id: string.format
- version: 1.0.0
- description: Interpolates a template with named values.
- determinism: pure
- inputs:
  - template: String
  - values: Map[String, Any] (coercions allowed to String where needed)
- outputs:
  - text: String
- params:
  - strict: Bool (default: false) — require all placeholders to be provided
- effects: none
- errors:
  - domain: format
    - E_MISSING_KEY, E_INVALID_PLACEHOLDER
- examples:
  - template: "Hello {name}", values: { name: "Ada" } → "Hello Ada"

3) time.now (time-dependent)
- module: org.cpc.std
- block_id: time.now
- version: 1.0.0
- description: Emits the current UTC timestamp.
- determinism: time_dependent
- inputs: none
- outputs:
  - now: Timestamp
- params:
  - monotonic: Bool (default: false)
- effects:
  - capability: time.now
- errors:
  - domain: time
    - E_CLOCK_UNAVAILABLE
- examples:
  - now → 2025-01-23T12:34:56.789Z

4) event.on_channel (IO-dependent subscription)
- module: org.cpc.std
- block_id: event.on_channel
- version: 1.0.0
- description: Subscribes to a logical event channel and emits payloads as they arrive.
- determinism: io_dependent
- inputs: none
- outputs:
  - event: Any
  - channel: String
- params:
  - channel: String (required)
  - buffer: UInt (default: 64) — internal buffer size
- effects:
  - capability: channel.<param:channel>.subscribe
  - mode: subscribe
- lifecycle:
  - start: subscribe(channel)
  - message: push event payloads to output
  - stop: unsubscribe(channel)
- errors:
  - domain: channel
    - E_SUBSCRIBE_DENIED, E_CHANNEL_NOT_FOUND, E_OVERFLOW
- examples:
  - channel="notifications" → emits incoming payloads

5) net.http_get (IO-dependent)
- module: org.cpc.net
- block_id: net.http_get
- version: 1.0.0
- description: Performs an HTTP GET request and returns status, headers, and body.
- determinism: io_dependent
- inputs: none
- outputs:
  - ok: { status: UInt, headers: Map[String, String], body: Bytes }
  - error: { domain: String, code: String, message: String }
- params:
  - url: Url (required)
  - headers: Map[String, String] (default: {})
  - timeout_ms: UInt (default: 10000)
  - accept_mime: String (optional)
- effects:
  - capability: net.http
  - scope: <param:url.host>
  - mode: read
- lifecycle:
  - compute on trigger (edge-activated or explicit invoke)
- errors:
  - domain: net
    - ETIMEOUT, ECONN, EDNS, EHTTP_4XX, EHTTP_5XX
- examples:
  - url="https://example.org" → ok.status=200, body=...

Module Example (org.cpc.std)
- module_id: org.cpc.std
- version: 1.0.0
- name: CPC Standard Library
- capabilities: [time.now, channel.*]
- blocks: [math.add, string.format, time.now, event.on_channel]

Validation Examples
- math.add cannot declare effects; IR validator rejects if effects present.
- net.http_get requires net.http; host denies if capability is not pregranted.
- event.on_channel requires channel.<name>.subscribe; name is resolved from param at bind time.

Notes for Implementers
- Visual editors expose params as inspector fields and inputs/outputs as pins.
- Textual syntax compiles via pest into IR; visual edits serialize to the same IR.
Rust hosts implement capability providers behind traits; WASM and Win64 hosts provide different backends.
 
 Practical example
 For a minimal, ready-to-run module and block set, see apps/shtairir_examples. Feel free to copy and adapt it to your domain; contributions adding more examples are welcome.
 For an illustration of declaring effects on non-deterministic blocks, see docs/shtairir/examples/blocks/io.fetch_url.toml (documentation-only).
 
 Authoring checklist
 - Identifiers
   - Use snake_case segments; compose namespaces with dots (e.g., math.add, util.join_kv_pairs).
   - Block IDs are unique within a module; fully-qualified ID is module_id/block_id.
 - Determinism vs effects
   - If determinism is pure (deterministic), effects must be empty; registry validation will reject otherwise.
 - Types and defaults
   - option<T> default: JSON null.
   - list<T> default: JSON [].
   - map<K,V> default: JSON {}.
   - Scalars like i64/string: provide concrete JSON-compatible values.
 - Outputs
   - Declare at least one output port; empty outputs are invalid.
 - Version pinning (when consuming blocks/modules)
   - Caret ^ for compatible major (e.g., ^1.2 selects highest 1.x ≥ 1.2).
   - Tilde ~ for patch-level stability (e.g., ~1.2.3 selects 1.2.z).
 
 How a host might execute blocks (conceptual)
 - Resolve the block:
   - Use the registry to resolve module and block by name/version constraint (e.g., Registry::find_block(...)).
 - Validate inputs and params:
   - Check input schema, optionality, and apply defaults for params.
 - Bind and call:
   - Map inputs/params to the block’s runtime entrypoint (pure compute or effectful with capability checks).
 - Produce outputs:
   - Return typed outputs (or Result in textual IR) and emit telemetry with module/block versions.
 
 Pseudo-Rust sketch (conceptual shape only)
 trait Host {
     fn execute_block(&self, module: &str, block: &str, req: &str) -> Result<String, String>;
     // Typical steps inside:
     // 1) let handle = Registry::find_block(module, block, Some("^1"));
     // 2) validator.ensure_inputs_defaults(&handle.spec, req);
     // 3) let result = self.backend.invoke(&handle, req);
     // 4) return result;
 }
 
 See also
 - Examples in apps/shtairir_examples:
   - string.concat (option<string>)
   - math.sum_list (list<i64>)
   - util.join_kv_pairs (map<string,string>)
 - Registry overview: shared_packages/shtairir_registry/README.md