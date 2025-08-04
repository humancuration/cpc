# Maintainers

## Schema Guardrails (GraphQL)
- Job: non-blocking GraphQL schema guardrails CI job (schema_guardrails), continues on error (continue-on-error: true).
- Local commands:
  - cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
  - cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot
- Snapshot path (default): docs/api_server/schema.graphql (override with SCHEMA_SNAPSHOT_PATH).
- Fix unintended schema changes in: apps/api_server/src/graphql/*
- ADR reference: docs/adr/0009-bootstrap-stub-toggles.md (stubs OFF; tool clears known toggles).
- Links: docs/dev/schema-checks-ci-snippet.md, tools/ci/README.md, docs/qa/schema-guardrails-checklist.md
- Rollout summary: docs/rollouts/schema-guardrails.md
- Maintainer tip: When reviewing PRs that change the schema, ask authors to confirm they ran check-schema and updated the snapshot if the change is intentional.
- Triage playbook: docs/playbooks/schema-guardrails-triage.md
- Milestones: docs/milestones/README.md

### Next ideas (optional)
- Add workflow_dispatch for schema_guardrails to enable manual runs without pushing.
- Consider nightly scheduled run to detect drift proactively on default branch.
- Explore switching to dtolnay/rust-toolchain for consistency when repo standardizes.
- Gate merges with required check in future once ecosystem stabilizes (flip to blocking).