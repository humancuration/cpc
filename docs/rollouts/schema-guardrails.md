Schema Guardrails (GraphQL) — Rollout Summary

- What shipped: non-blocking GraphQL schema guardrails CI job (schema_guardrails)
- Where to see it: GitHub Actions → CI workflow → job “schema_guardrails” (continue-on-error: true)
- Local check: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Update snapshot: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot
- Snapshot path (default): docs/api_server/schema.graphql
- Override via env: SCHEMA_SNAPSHOT_PATH=<path> (if a different snapshot location is required)
- Quick refs:
  - Dev snippet: docs/dev/schema-checks-ci-snippet.md
  - Triage playbook: docs/playbooks/schema-guardrails-triage.md
  - QA checklist: docs/qa/schema-guardrails-checklist.md
  - CI utilities: tools/ci/README.md
- Maintainer note: Use the triage playbook when the job reports diffs; require snapshot updates for intentional schema changes
- Status: Complete; future improvements to be tracked via issues