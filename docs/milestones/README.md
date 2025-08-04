Schema Guardrails (GraphQL) — Completed

- Status: Completed
- CI job: schema_guardrails (non-blocking; continue-on-error: true)
- Quick commands: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema; add --write-snapshot to update
- Snapshot: docs/api_server/schema.graphql (override via SCHEMA_SNAPSHOT_PATH)
- Links: rollout summary (docs/rollouts/schema-guardrails.md), dev snippet (docs/dev/schema-checks-ci-snippet.md)
- Links: triage playbook (docs/playbooks/schema-guardrails-triage.md), QA checklist (docs/qa/schema-guardrails-checklist.md)
- CI utilities: tools/ci/README.md