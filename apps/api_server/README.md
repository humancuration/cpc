# API Server

The API server for the CPC platform provides GraphQL endpoints for all core features including social, volunteer, and skill exchange functionality.

## Features

- GraphQL API for volunteer tracking
- GraphQL API for skill exchange marketplace
- Integration with wallet, notification, and social systems
- Real-time subscriptions for volunteer and skill events

## Developer Docs

- Volunteer Coordination reputation stub toggle: see docs/api_server/volunteer_reputation_stub.md for how to enable VOLUNTEER_REPUTATION=stub locally, bootstrap composition, and GraphQL schema DI details.
- Overview and index: see docs/api_server/ for API server documentation

### Status: Volunteer Reputation Stub

- Composition helper, GraphQL alias (verificationRef), and tests have been standardized.
- Local toggle: VOLUNTEER_REPUTATION=stub; default is disabled.
- See docs/api_server/volunteer_reputation_stub.md for behavior, env usage, and test helper details.

#### Quickstart (local)
- PowerShell: $env:VOLUNTEER_REPUTATION="stub"; cargo run -p api_server
- cmd.exe: set VOLUNTEER_REPUTATION=stub && cargo run -p api_server
- bash/zsh: VOLUNTEER_REPUTATION=stub cargo run -p api_server

## GraphQL Schema

The API server exposes the following GraphQL schema:

### Volunteer Features

- `logVolunteerHours` - Log volunteer hours for a user
- `verifyVolunteerHours` - Verify volunteer hours (organization admin only)
- `convertToDabloons` - Convert verified volunteer hours to Dabloons
- `myVolunteerActivities` - Get all volunteer activities for the current user
- `myVerifiedVolunteerActivities` - Get verified volunteer activities for the current user

### Skill Exchange Features

- `createSkillListing` - Create a new skill listing
- `updateSkillListing` - Update an existing skill listing
- `deactivateSkillListing` - Deactivate a skill listing
- `claimSkillListing` - Claim a skill listing
- `acceptClaim` - Accept a claim on a skill listing
- `rejectClaim` - Reject a claim on a skill listing
- `completeSkillExchange` - Complete a skill exchange
- `searchSkillListings` - Search for active skill listings
- `mySkillListings` - Get skill listings by provider
- `mySkillClaims` - Get claims by claimant

## Testing

The API server includes comprehensive unit and integration tests:

```bash
cargo test
```

### Schema guardrails
Schema changes are checked locally and in CI to prevent drift. Run the schema check before you open a PR and compare against the snapshot.
- How-to: see docs/dev/schema-checks.md
- Architecture: see docs/dev/schema-guardrails-architecture.md
- Examples: see docs/dev/schema-checks-examples.md
Command (verbatim):
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
Snapshot (verbatim):
docs/api_server/schema.graphql

## License

This software is licensed under the CPC License.
See also: docs/dev/guardrails-index.md