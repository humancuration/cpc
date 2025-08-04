CPC CI Utilities (tools/ci)

Purpose
This crate hosts small CI utilities used in both local workflows and GitHub Actions.

Commands
- Schema drift check (GraphQL):
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
  Flags:
  --write-snapshot   Overwrite the snapshot with the newly generated SDL.

- Docs consistency checker (legacy but kept):
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency

Non-blocking GraphQL schema guardrails CI job (schema_guardrails)
- CI job name: schema_guardrails (see .github/workflows/ci.yml).
- Behavior: non-blocking (continue-on-error: true) to give early visibility without failing the entire workflow.
- Snapshot path: docs/api_server/schema.graphql by default (can be overridden with SCHEMA_SNAPSHOT_PATH env var if ever needed).
- Local command (same as CI):
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Update snapshot when changes are intentional:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot
  Then review and commit docs/api_server/schema.graphql.
- If changes are not intended, fix schema-affecting code under apps/api_server/src/graphql/* and re-run.
- Toolchain note: CI currently uses actions-rs/toolchain@v1 stable; we may switch to dtolnay/rust-toolchain later.

Docs consistency checker details
Needles config (tools/ci/needles.txt)
Each non-empty, non-comment line defines one rule:
<target_file>|<required_substring>

Example:
apps/api_server/README.md|docs/api_server/volunteer_reputation_stub.md
docs/README.md|docs/adr/0009-bootstrap-stub-toggles.md

More info
- Schema guardrails: see docs/dev/schema-checks-ci-snippet.md for contributor-facing quickstart.
- Docs consistency philosophy: see docs/dev/docs-consistency-checks.md.

Micro-maintenance cadence (needles)
- When you add a new index/entry doc (e.g., docs/README.md, docs/dev/* index, or apps/*/README.md), add a matching needle the same day.
- When you rename/move a doc already referenced in tools/ci/needles.txt, update or retire that needle in the same commit.
- If a needle is flaky or noisy, either narrow the target_file to the closest index doc or broaden the required_substring to a stable directory path (e.g., docs/adr/). Verify locally before pushing.
- Once a month, prune duplicates and remove needles for deprecated docs; keep relationships high-signal.
- Prefer stable substrings (directories like docs/adr/) over volatile filenames unless pointing to a canonical doc is intentional.