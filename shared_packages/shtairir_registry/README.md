Shtairir Registry (in-memory)
=============================

Purpose
- Discover MODULE.toml manifests under given roots
- Load module metadata and referenced block spec TOMLs
- Validate versions, identifiers, types, effects, defaults
- Query modules/blocks and resolve versions using semver constraints

Status: in-memory only; no persistence or global state.

Manifest formats

1) Module manifest (MODULE.toml at module root)
Example:
name = "sb_std"
version = "0.1.0"
title = "Shtairir Standard Blocks"
description = "Standard/pure blocks for math/string/time"
authors = ["CPC Coop"]
categories = ["standard"]
min_shtairir_version = "0.1.0"
blocks = [
  "blocks/math.add.toml",
  "blocks/string.format.toml",
]

2) Block spec TOML (e.g., blocks/math.add.toml)
Example:
name = "math.add"
title = "Add"
description = "Add two integers"
determinism = "Deterministic"   # or "Nondeterministic"
effects = []                     # e.g., ["net.http"]
inputs = [
  { name = "a", ty = "i64" },
  { name = "b", ty = "i64" },
]
outputs = [
  { name = "out", ty = "i64" },
]
params = []
examples = ["out = add(a:1,b:2)"]

Types (whitelist enforced)
Scalars: i64, f64, bool, string, bytes, decimal, datetime, duration, uuid, json
Composites: list<...>, map<string,...>, option<...> with nested validation
v0.2 Enhanced Types: Struct{...}, Enum{...}, Stream<T>, Event<T>, tuples, generics with bounds

Effects (format)
Segments separated by '.', optional wildcard only as the last segment:
- fs.read, net.http, time.read, audio.play, storage.kv, device.camera
- app.<id>.*, channel.<name>
- Wildcard '*' allowed only at end (e.g., "app.website_builder.*")

Quick Start

Add the crate to your workspace members (if needed), then:

use std::path::PathBuf;
use shtairir_registry::model::BlockHandle;
use shtairir_registry::model::Registry;

fn main() -> anyhow::Result<()> {
    // Scan these roots recursively for MODULE.toml
    let roots = vec![
        PathBuf::from("apps"),
        PathBuf::from("shared_packages"),
    ];

    let reg = Registry::load(&roots)?;

    println!("Modules: {:?}", reg.list_modules());
    println!("Blocks in sb_std: {:?}", reg.list_blocks("sb_std"));

    // Find block with version resolution:
    if let Some(BlockHandle { module, version, spec }) =
        reg.find_block("sb_std", "math.add", Some("^0.1"))
    {
        println!("Resolved {} {} block {}", module, version, spec.name);
    }

    Ok(())
}

Validation Summary
- Module name: snake_case segments separated by '.' (e.g., "app.website_builder")
- Block name: snake_case with optional dots (e.g., "math.add")
- Module version: SemVer
- Uniqueness: module@version unique; unique port names per block; unique param names; at least one output
- Types: must be in whitelist or valid composite
- Defaults: basic JSON type compatibility check
- Determinism: Deterministic blocks must declare no effects
- Effects strings: format validated; wildcard only at end
- v0.2 Graph Validation:
  * Type compatibility checking between connected ports
  * Stream merge policy validation for multiple producers
  * Generic bounds validation
  * Port kind validation (Value, Stream, Event, Composite)

Design Notes
- Rust 2021 edition
- serde for TOML/JSON
- walkdir for discovery
- semver for versioning and resolution
- No global singletons; pass Registry around
- v0.2 Features:
  * Enhanced type system with ADTs (structs, enums)
  * Type compatibility checking using AST-based validation
  * Graph structure validation with stream merge policy enforcement
  * Generic parameter bounds validation

Folder Structure

shared_packages/shtairir_registry/
- Cargo.toml
- README.md
- src/
  - lib.rs
  - model.rs
  - loader.rs
  - validator.rs
  - types.rs (v0.2 type system)
  - integrity.rs (v0.2 content integrity)

Example modules and blocks

Looking for a complete, working scaffold you can copy? See the Shtairir Examples app:
- apps/shtairir_examples/README.md — overview and how to run
- apps/shtairir_examples/MODULE.toml — example module manifest
- apps/shtairir_examples/blocks/* — example block specs (deterministic, no effects)
- apps/shtairir_examples/src/bin/demo.rs — minimal registry loader and caret (^) version resolution demo

The example demonstrates:
- Deterministic blocks with no effects
- Identifier rules and uniqueness constraints
- Type/default validation against the registry whitelist
- Caret semver (^) version resolution via Registry::find_block
- v0.2 Features:
  * Graph validation with type compatibility checking
  * Stream merge policy validation for multiple producers
  * Enhanced type system with structs and enums

We welcome improvements and additional example blocks—PRs are encouraged!

License
- Governed by CPC cooperative policy for this repository (no license headers in code as per project rules).