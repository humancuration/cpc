# ADR 0009: Bootstrap Composition via Environment Toggles for Test Stubs

## Status
Proposed

## Date
2025-08-04

## Context
We need deterministic, opt-in test doubles for local development and tests without changing production defaults. Developers must be able to enable a stubbed adapter at the composition root while keeping the default production behavior unchanged. This supports faster iteration and reliable tests for features like Volunteer Reputation verification.

## Decision
- Composition location
  - Place composition code under `apps/api_server/src/bootstrap/`.
- Environment toggle
  - Read an environment variable (e.g., `VOLUNTEER_REPUTATION=stub`) to select a stub adapter; otherwise pass `None` to disable.
- Logging
  - Log enablement with `tracing::info!` indicating whether the stub is enabled or disabled.
- GraphQL test helper
  - Provide a `#[cfg(test)]` GraphQL schema helper in `apps/api_server/src/graphql/mod.rs` to inject the composed service via `Schema::data` for tests (e.g., `test_helpers::build_vc_schema_with_service`).
- Backward-compatible field aliasing
  - Prefer #[graphql(name = "...")] for GraphQL field aliasing to preserve compatibility when required (e.g., verificationRef as an alias of verifiedBy).

## Schema determinism note
The following environment toggles can affect the GraphQL schema and MUST be OFF (unset) when generating the snapshot and in CI:
- VOLUNTEER_REPUTATION
If additional schema-affecting toggles are introduced, extend this list in tools/ci (ensure_stub_envs_off_for_schema) and update this ADR accordingly.
See docs/dev/schema-checks.md for the determinism checklist and commands.

## Consequences
- Local enablement is simple (set an env var); production default remains off and unchanged.
- Tests standardize on a single helper for schema construction, reducing drift and coupling in test setup.
- Future stubs (event bus, audit log, etc.) can replicate the same pattern with minimal boilerplate and clear logging.

## References
- `apps/api_server/src/bootstrap/volunteer.rs`
- `apps/api_server/src/graphql/mod.rs` (`test_helpers::build_vc_schema_with_service`)
- `apps/api_server/src/graphql/volunteer_coordination.rs` (verificationRef alias)
- `docs/api_server/volunteer_reputation_stub.md`