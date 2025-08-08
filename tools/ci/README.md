# CI Utilities

This directory contains utilities for continuous integration checks in the CPC project.

## Available Commands

### Documentation Consistency Check
```bash
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency
```

This command checks that key documentation files contain required substrings to ensure discoverability. The rules are defined in `tools/ci/needles.txt`.

### Schema Guardrails
```bash
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
```

This command checks GraphQL schema consistency. Add `--write-snapshot` to update the schema snapshot.

## Configuration

- `needles.txt` - Defines documentation consistency rules
- `src/main.rs` - Main implementation of CI checks

## Documentation

For more information about these checks, see:
- `docs/dev/docs-consistency-checks.md` - Documentation consistency checks
- `docs/dev/schema-checks.md` - Schema guardrails
- `docs/dev/schema-guardrails-architecture.md` - Schema guardrails architecture