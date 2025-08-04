# Volunteer Reputation Stub Toggle

Purpose
Deterministic local testing of contribution verification in Volunteer Coordination without integrating an external reputation system. When enabled, the application layer composes a stubbed ReputationPort into the VolunteerServiceImpl.

Toggle
Environment variable: VOLUNTEER_REPUTATION
Supported value: "stub"
Any other value or unset: disabled (default; used in production)

Enable locally
Linux/macOS (bash/zsh):
export VOLUNTEER_REPUTATION=stub

Windows (PowerShell):
$env:VOLUNTEER_REPUTATION="stub"

Windows (cmd.exe):
set VOLUNTEER_REPUTATION=stub

Behavior when enabled
The stub drives verification outcomes in a predictable way:
- Hours < 1.0 => verified=false
- Hours ≥ 1.0 => verified=true
- Deliverable => verified=true

Composition and bootstrap
File: apps/api_server/src/bootstrap/volunteer.rs
- build_volunteer_service(...) reads VOLUNTEER_REPUTATION.
- When set to "stub", it injects shared_packages::volunteer_coordination::reputation_stub() into the VolunteerServiceImpl constructor; otherwise passes None.
- Logs enablement state via tracing::info!.

Snippet reference (see file for full code):
- Reads env: std::env::var("VOLUNTEER_REPUTATION") == "stub"
- On stub: info!("Volunteer reputation stub enabled via VOLUNTEER_REPUTATION=stub")
- On disabled: info!("Volunteer reputation integration disabled (no VOLUNTEER_REPUTATION=stub)")

Application layer composition
File: shared_packages/volunteer_coordination/src/application/volunteer_service.rs
- VolunteerServiceImpl holds optional reputation: Option<Arc<dyn ReputationPort + Send + Sync>>.
- verify_contribution(...) consults reputation when present, mapping:
  - kind Hours: amount_hours=Some(hours)
  - other kinds: amount_hours=None
- On error from reputation service, verification defaults to false conservatively.

Reputation port
File: shared_packages/volunteer_coordination/src/application/reputation_port.rs
- Defines async trait ReputationPort for external verification.
- The stub satisfies this trait and is provided by shared_packages::volunteer_coordination::reputation_stub().

GraphQL schema composition and testing DI
File: apps/api_server/src/graphql/mod.rs
- #[cfg(test)] pub mod test_helpers provides build_vc_schema_with_service(...):
  - Accepts repositories, builds Arc<VolunteerServiceImpl> via bootstrap::volunteer::build_volunteer_service(...)
  - Injects user_id and Arc<VolunteerServiceImpl> into async-graphql Schema::data
  - Respects the VOLUNTEER_REPUTATION env toggle during schema construction for tests

GraphQL type alias for verification reference
File: apps/api_server/src/graphql/volunteer_coordination.rs
- VolunteerContribution GraphQL type includes:
  - verified: bool
  - verifiedBy: Option<ID> (mapped from domain verification_ref)
  - verificationRef: Option<ID> with #[graphql(name = "verificationRef")] (alias of verifiedBy)
- Mapping ensures both verifiedBy and verificationRef are populated from the same domain field verification_ref. See From<domain::VolunteerContribution> impl.

Example GraphQL usage

Query example (reading a contribution):
{
  contribution(id: "<id>") {
    id
    verified
    verifiedBy
    verificationRef
  }
}

Expected:
- verifiedBy == verificationRef
- When stub is enabled:
  - For HOURS amount 0.5: verified=false
  - For HOURS amount 1.25: verified=true
  - For DELIVERABLE: verified=true

Mutation example (verify):
mutation Verify($input: VerifyContributionInput!) {
  verifyContribution(input: $input) {
    id
    verified
    verifiedBy
    verificationRef
  }
}

Note: verificationRef is an alias of verifiedBy in GraphQL. Either can be selected by clients and will return the same value.

Default and production guidance
- The stub is disabled by default and should remain disabled in production (do not set VOLUNTEER_REPUTATION=stub).
- Future stubs (e.g., event bus, audit log) should mirror this pattern by adding a small bootstrap module under apps/api_server/src/bootstrap/ and threading optional ports into service constructors, with tracing::info! logging.

Related code references
- apps/api_server/src/bootstrap/volunteer.rs
- apps/api_server/src/graphql/mod.rs (test_helpers::build_vc_schema_with_service)
- apps/api_server/src/graphql/volunteer_coordination.rs (VolunteerContribution with verificationRef alias)
- apps/api_server/src/graphql/volunteer_coordination_test.rs (end-to-end tests including stub behavior)
- shared_packages/volunteer_coordination/src/application/volunteer_service.rs (service composition and verify flow)
- shared_packages/volunteer_coordination/src/application/reputation_port.rs (port definition)

## Adding New Stubs using the Bootstrap Pattern

Placement
- Create a small bootstrap module under apps/api_server/src/bootstrap/ (e.g., audit_log.rs or event_bus.rs) that owns the env toggle read and service wiring.

Composition approach
- Read an environment toggle (e.g., AUDIT_LOG_STUB=stub or EVENT_BUS=stub).
- Build the service with an optional port (Some(stub) vs None) mirroring build_volunteer_service behavior.
- Log enablement via tracing::info! so local dev/test runs clearly indicate stub usage.

GraphQL tests
- Provide a test-only helper in apps/api_server/src/graphql/mod.rs under #[cfg(test)] to construct a Schema that injects the service via Schema::data. Reuse the bootstrap builder so env toggles are honored.
- Ensure tests use the helper consistently to respect env toggles, similar to test_helpers::build_vc_schema_with_service.

GraphQL fields
- If aliasing is needed for backward compatibility, add #[graphql(name = "...")] on the struct field and keep mapping consistent with domain models (see verificationRef alias pattern).

Notes
- Keep defaults disabled in production; enable stubs explicitly in local dev/tests.
- Prefer mirroring naming and structure for discoverability and future maintenance.

## Local CI/dev checklist
- Ensure VOLUNTEER_REPUTATION is unset (default) or set to "stub" explicitly when testing stub behavior.
- Use graphql::test_helpers::build_vc_schema_with_service in GraphQL tests to respect env toggles.
- Verify tests that exercise verification include fields: verified, verifiedBy, verificationRef, and assert verifiedBy == verificationRef.
- Confirm in-memory repos implement the exact trait signatures from shared_packages/volunteer_coordination::domain::repository and satisfy Send + Sync when used behind Arc<dyn Trait>.
- Confirm resolvers fetch Arc<VolunteerServiceImpl> from Schema data (matching what the helper injects).
- When failures occur: align trait imports, attributes like #[graphql(name = "...")], and check for Arc<VolunteerServiceImpl> type mismatches.

References
- apps/api_server/src/bootstrap/volunteer.rs
- apps/api_server/src/graphql/mod.rs (test_helpers::build_vc_schema_with_service)
- apps/api_server/src/graphql/volunteer_coordination.rs (verificationRef pattern)
- docs/api_server/volunteer_reputation_stub.md (this doc)