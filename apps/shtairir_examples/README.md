Shtairir Examples
=================

A tiny, end-to-end example that shows how to ship Shtairir block specs in a module and load/validate/resolve them with the shared shtairir_registry crate.

What this contains
- MODULE.toml defining a module named "examples.shtairir" at version 1.0.0
- Two deterministic blocks with valid specs:
  - blocks/math.add.toml (i64 + i64 [+ optional bias i64] -> i64 result)
  - blocks/string.concat.toml (string + string with optional separator: option<string> -> string combined)
- A demo binary that loads the registry from this app and exercises find_block with a caret (^) semver requirement.
- Unit tests that cover identifiers, type/default compatibility, determinism/effects, version resolution, and uniqueness/output presence.

Layout
apps/
  shtairir_examples/
    MODULE.toml
    blocks/
      math.add.toml
      string.concat.toml
    src/
      bin/
        demo.rs
      lib.rs
    tests/
      registry_example_tests.rs
    Cargo.toml

How to run
1) Ensure the workspace includes shared_packages/shtairir_registry (it already does in this repo).
2) Build and run the demo:
   - cargo run -p shtairir_examples --bin demo
   - Expected output includes:
     - Discovered modules: ["examples.shtairir"]
     - Blocks in examples.shtairir: ["math.add", "string.concat"]
     - Resolved lines for each block with version 1.0.0
3) Run tests:
   - cargo test -p shtairir_examples

Notes
- Registry::load recursively scans for MODULE.toml under the given roots; here we pass apps/shtairir_examples.
- Deterministic blocks must not declare effects; both example specs have effects = [].
- Types are restricted by the current validator whitelist:
  - Scalars: i64, f64, bool, string, bytes, decimal, datetime, duration, uuid, json
  - Composites: list<...>, map<string,...>, option<...> (nested OK)
- Defaults must be JSON-compatible with declared types. For option<T>, use null to represent None.

References
- shared_packages/shtairir_registry/ (crate)
- docs/announcements/2025-08-shtairir_registry-announcement.md
- docs/shtairir/01-overview.md
- docs/shtairir/02-blocks-and-modules.md
- docs/shtairir/03-types-and-signatures.md
- docs/shtairir/04-versioning-and-compatibility.md
- docs/shtairir/05-ir.md