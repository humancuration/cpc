# Contributing

Thank you for contributing to CPC. This guide highlights common workflows and links to deeper docs.

## Non-blocking GraphQL schema guardrails CI job (schema_guardrails)

What it is
- A non-blocking GraphQL schema guardrails CI job (`schema_guardrails`) that detects schema drift early and surfaces diffs in CI. It does not fail the overall workflow.

How to find it
- GitHub → Actions → CI workflow → job “schema_guardrails”.
- On a PR, open the Checks tab and select the job named “schema_guardrails”. You can re-run that job manually if needed.

Run locally
- Check:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Intentionally update the snapshot (when changes are expected):
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot

Snapshot location
- Default path: docs/api_server/schema.graphql (can be overridden with SCHEMA_SNAPSHOT_PATH).

Where to fix unintended changes
- Edit GraphQL schema-affecting code under: apps/api_server/src/graphql/*

Stub toggles
- Per ADR 0009, any schema-affecting stubs must be OFF. The CI tool clears known toggles automatically. See docs/adr/0009-bootstrap-stub-toggles.md.

Toolchain
- CI uses actions-rs/toolchain@v1 with the stable toolchain currently.

More details
- See docs/dev/schema-checks-ci-snippet.md for quickstart details.
- See tools/ci/README.md for CI utilities overview.
- Quick QA checklist: docs/qa/schema-guardrails-checklist.md