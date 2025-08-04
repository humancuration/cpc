Architecture note: GraphQL schema guardrails (non-blocking)

Intent
- Provide gentle, non-blocking guardrails that surface GraphQL schema drift early in CI and locally, without blocking merges.

Contract and assumptions
- Single canonical snapshot file at docs/api_server/schema.graphql; override via SCHEMA_SNAPSHOT_PATH when needed.
- Deterministic schema generation via apps/api_server/src/graphql/mod.rs::ci_schema (build_schema_for_ci). CI must use this path.
- ADR 0009 stub toggles are OFF in CI; the tool clears known schema-affecting toggles to keep SDL stable.
- CI job name: schema_guardrails in .github/workflows/ci.yml with continue-on-error: true (advisory only).
- CLI: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema; add --write-snapshot to update the snapshot.

Why non-blocking
- Encourages contribution and reduces friction while still providing high-signal visibility; can be flipped to blocking later once stable.

Extensibility guidance
- When adding schema-affecting flags/modules, update tools/ci to clear the new env toggles for determinism and amend ADR 0009 and this note. Ensure ci_schema includes new roots/types consistent with production composition.

Links
- Triage playbook: docs/playbooks/schema-guardrails-triage.md
- QA checklist: docs/qa/schema-guardrails-checklist.md
- Rollout summary: docs/rollouts/schema-guardrails.md
- CI utilities/usage: tools/ci/README.md