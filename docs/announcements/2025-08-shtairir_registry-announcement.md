Shtairir Registry is live — discover, validate, and resolve blocks via manifests
One-sentence summary
A new shared crate, shtairir_registry, lets teams discover modules and block specs from TOML manifests, validate naming/types/effects/versioning, and resolve blocks by semantic version constraints — so you can ship reliable block libraries with confidence.

What’s included
New shared crate: shared_packages/shtairir_registry
- Purpose: In-memory discovery, validation, and lookup of Shtairir modules and blocks.
- Status: In-memory only (no persistence). Ready for teams to publish MODULE.toml and block spec TOML files.

Core capabilities
- Loader
  - Scans recursive roots (walkdir) for MODULE.toml.
  - Parses MODULE.toml into ModuleManifest (name, version, title, description, authors, categories, min_shtairir_version, blocks).
  - Loads referenced block spec TOML files relative to the manifest directory.
  - Builds an in-memory Registry keyed by "module@version:block_name" with ModuleHandle summaries.
- Validator
  - SemVer and uniqueness:
    - Module versions must be valid SemVer.
    - Each module@version is unique.
  - Identifier rules:
    - Module name: snake_case with optional dot segments (e.g., app.website_builder).
    - Block name: snake_case with optional dots (e.g., math.add).
  - Type whitelist (with nested validation):
    - Scalars: i64, f64, bool, string, bytes, decimal, datetime, duration, uuid, json.
    - Composites: list<...>, map<string,...>, option<...> (nested OK).
  - Effects format:
    - Dot-separated segments, wildcard only as the final full segment.
    - Accepts common domains like fs.*, net.http, app.<id>.*, channel.<name>.
  - Determinism vs effects:
    - Deterministic blocks must not declare effects.
  - Defaults and uniqueness:
    - Default values must be compatible with declared types.
    - Unique names within inputs, outputs, and params.
    - At least one output is required.

Public API quick look
- Registry::load(paths: &[PathBuf]) -> anyhow::Result<Registry>
  - Recursively loads and validates modules and blocks from the given root paths.
- list_modules() -> Vec<String>
  - Returns all module names present in the registry.
- list_blocks(module: &str) -> Vec<String>
  - Returns deduplicated block names for a module across all versions.
- find_block(module: &str, block: &str, version_spec: Option<&str>) -> Option<BlockHandle>
  - Supports semver::VersionReq including ^ and ~. If None, selects the highest available version containing the block.

Getting started
- Add a MODULE.toml to your module’s root and list your block spec TOML files in the blocks array.
- Place block spec TOMLs alongside your code (relative paths from MODULE.toml are supported).
- See the crate README for manifest examples and a short usage snippet.

Call for contributions
- Add example MODULE.toml manifests and block specs to help new contributors ramp quickly.
- Write tests that exercise identifiers, types/defaults, effects, and version resolution.
- Simple first examples:
  - math.add (pure): a: i64, b: i64 -> out: i64
  - string.concat (pure): a: string, b: string -> out: string
  - Consider composite type coverage (e.g., list<i64>, map<string,string>, option<uuid>).

Next steps at a glance
1) Workspace
   - Add the crate to workspace members in the root Cargo.toml (if not already present).
2) Examples
   - Publish an examples/ or apps/shtairir_examples/ with two blocks and a tiny demo that calls Registry::load and find_block.
3) Validator extensions
   - Extend validation when we finalize expanded TypeSpec (Struct, Enum, Result, Url, UInt, etc.).
4) Tests
   - Add unit tests for identifiers, types/defaults, effects wildcard rules, and version resolution (^, ~).
5) Persistence
   - Plan content-addressed persistence and an on-disk cache in a follow-up.

Gratitude
Huge thanks to everyone who contributed manifests, validator rules, and docs. This unlocks a smoother path for teams to share modules and blocks across the ecosystem. If you’re new, there are plenty of bite-sized issues — claim one and say hi!

References
Crate location and files:
- shared_packages/shtairir_registry/Cargo.toml
- shared_packages/shtairir_registry/src/lib.rs
- shared_packages/shtairir_registry/src/model.rs
- shared_packages/shtairir_registry/src/loader.rs
- shared_packages/shtairir_registry/src/validator.rs
- shared_packages/shtairir_registry/README.md

Shtairir docs (concept alignment):
- docs/shtairir/01-overview.md
- docs/shtairir/02-blocks-and-modules.md
- docs/shtairir/03-types-and-signatures.md
- docs/shtairir/04-versioning-and-compatibility.md
- docs/shtairir/05-ir.md