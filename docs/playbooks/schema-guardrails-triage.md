Schema Guardrails Triage (GraphQL)

When
- CI job “schema_guardrails” shows “diff detected” (non-blocking advisory).

Quick triage
- 1) Re-run CI: Actions → CI → job “schema_guardrails” to rule out flake.
- 2) Reproduce locally:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- 3) Review the diff in CI/local output (snapshot: docs/api_server/schema.graphql).
- 4) If intentional change:
  - Update snapshot:
    cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot
  - Commit the updated docs/api_server/schema.graphql.
- 5) If NOT intentional:
  - Fix/revert schema-affecting code in apps/api_server/src/graphql/* and re-run the check.
- 6) If environment-specific:
  - Optionally set SCHEMA_SNAPSHOT_PATH to match your layout.
  - Ensure schema-affecting stubs/toggles are OFF per ADR 0009; the tool clears known toggles in CI.

Notes
- Job is non-blocking by design; treat as advisory. Request fixes before merge if schema changed unintentionally.

See also
- docs/dev/schema-checks-ci-snippet.md (how-to)
- tools/ci/README.md (CI utilities)
- docs/maintainers.md (maintainer tips)