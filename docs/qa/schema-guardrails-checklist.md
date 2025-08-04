Schema Guardrails (GraphQL) — Quick QA Checklist

- Run locally before/with PR:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- If schema change is intentional, refresh snapshot:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot
- Snapshot path: docs/api_server/schema.graphql (override with SCHEMA_SNAPSHOT_PATH)
- Ensure schema-affecting stubs/toggles are OFF (see ADR 0009); tool clears known toggles in CI
- Commit the updated snapshot with code changes
- In PR description add one-liner:
  “Schema updated intentionally; snapshot refreshed” OR “No schema changes intended”
- If unintended diff: fix apps/api_server/src/graphql/* and re-run locally
- See triage playbook: docs/playbooks/schema-guardrails-triage.md
- Dev snippet: docs/dev/schema-checks-ci-snippet.md