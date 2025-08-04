Non-blocking GraphQL schema guardrails CI job (schema_guardrails)

Purpose
- Provide early visibility of GraphQL schema drift without blocking merges. Results are shown in CI and can be rerun manually.
- Local command matches CI exactly for consistency.

CI job
- Non-blocking GraphQL schema guardrails CI job (schema_guardrails) (see .github/workflows/ci.yml)
- How to find: In a PR’s Checks tab or in Actions → CI workflow runs, locate the job named schema_guardrails.
- How to re-run: Use “Re-run jobs” on the CI workflow and select schema_guardrails.
- Toolchain: Uses actions-rs/toolchain@v1 with stable, minimal profile (we may switch to dtolnay/rust-toolchain later).
- Command executed in CI:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Note: There is also a blocking job named schema-check that runs the same command without continue-on-error for stricter pipelines.

Run locally
- Use the same command:
  cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema

Snapshot
- Default snapshot path: docs/api_server/schema.graphql
- No extra flags needed for the default. The tool also supports SCHEMA_SNAPSHOT_PATH env var to override if ever needed.

Expected outcomes
- Pass: No diff between generated SDL and snapshot.
- Fail: Diff detected; CI job will show a summary diff and exit non-zero locally.

How to fix
1) Inspect the output/diff from CI or your local run.
2) If the schema change is intentional:
   - Update the snapshot with:
     cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot
   - Review and commit docs/api_server/schema.graphql.
3) If the schema change is not intended:
   - Revert or correct schema-affecting code under apps/api_server/src/graphql/*, then re-run the check.

Notes
- Stub toggles that affect schema must be OFF for determinism (see docs/adr/0009-bootstrap-stub-toggles.md). The CI tool clears known toggles automatically.
- This document complements tools/ci/README.md, which includes a short “Schema guardrails” section.

See also
- Triage playbook: docs/playbooks/schema-guardrails-triage.md