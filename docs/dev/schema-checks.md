Schema drift guardrails: Local checks and troubleshooting

Summary
Use this check to prevent accidental schema drift. Run the same command locally and in CI to compare the generated GraphQL schema with the committed snapshot. If they differ, update the snapshot intentionally or bring the code and docs back into alignment. Keep the source-of-truth path consistent across docs and CI.

Local command (copyable)
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
Note: This is the exact command CI uses (confirm).

Source of truth
- Snapshot path: docs/api_server/schema.graphql (confirm and keep consistent in docs + CI). If your repo uses a different snapshot path, replace this with [SOURCE_OF_TRUTH_PATH] and coordinate with CI.
- What counts as drift: any difference between the generated schema and the snapshot, including:
  - Added/removed/renamed types and fields
  - Enum value changes
  - Description changes
  - Nullability/input/output shape changes
  - Ordering if the checker treats it as meaningful (confirm tool behavior)

Common failures and quick fixes
1) Forgot to regenerate after resolver/type change
   - Fix: Regenerate the schema, re-run the check, and commit the updated snapshot.
2) Docs reference removed/renamed fields
   - Fix: Update docs and snapshot together in the same PR.
3) Case/path mismatches across OSes
   - Fix: Use forward slashes (/) and correct casing everywhere; fix imports/links accordingly.
4) Stale/uncommitted snapshot
   - Fix: Commit the updated snapshot file (docs/api_server/schema.graphql).
5) CRLF/LF line ending noise
   - Fix: Normalize line endings (e.g., git config core.autocrlf or editor setting) and re-run the check.
6) Partial updates (e.g., added type but missed input/enum/descriptions)
   - Fix: Complete all related schema elements and update docs references; re-run the check.
7) Snapshot moved but docs/CI not updated
   - Fix: Update the snapshot path in this doc and CI in the same PR.

Platform notes
- Windows:
  - Use forward slashes (/) in paths (not backslashes).
  - String comparisons are case-sensitive and whitespace-sensitive.
  - Normalize line endings if diffs only show CRLF/LF changes.
- All platforms:
  - Ensure your working tree is clean; commit snapshot changes with a clear message.

Examples (before/after)
Example 1: Renamed field userName → username
Before (snapshot)
type User {
  id: ID!
  userName: String!
}
After (generated)
type User {
  id: ID!
  username: String!
}
Fix
- Update resolvers/types to match the intended final name.
- Regenerate schema, update snapshot, and commit.

Example 2: Removed enum value; update docs and snapshot
Before (snapshot)
enum Status {
  ACTIVE
  INACTIVE
  PENDING
}
After (generated)
enum Status {
  ACTIVE
  INACTIVE
}
Fix
- Confirm removal is intentional.
- Update docs referencing PENDING.
- Regenerate schema, update snapshot, and commit.

Example 3: Added type Project but missing input/description
Before (generated)
type Project {
  id: ID!
  name: String!
}
After (completed)
"""A collaborative project."""
type Project {
  id: ID!
  name: String!
  description: String
}
input ProjectInput {
  name: String!
  description: String
}
Fix
- Add missing input/description and any required wiring.
- Regenerate schema, update snapshot, and commit.

Reviewer hygiene
- Ask the author to paste the command output if drift persists.
- Confirm the snapshot path matches this doc (docs/api_server/schema.graphql, or the confirmed [SOURCE_OF_TRUTH_PATH]).
- Suggest normalizing line endings before re-running if diffs look purely formatting-related.

Troubleshooting
- Schema check fails but code seems unchanged: ensure you’re on the latest main and re-run the command after a clean build (cargo clean).
- “tools/ci not found” or command fails immediately: verify tools/ci/README.md exists and that you’re running from the repo root. Use the exact command shown above.
- Snapshot mismatch is intended: open your PR with a brief note explaining the intended change and update the snapshot path’s file (docs/api_server/schema.graphql) as part of the same PR.
- Snapshot mismatch is not intended: fix the code or schema to match the current snapshot; don’t update the snapshot file.
- Still stuck: link your PR to a short comment describing the error output and tag a reviewer.

Future CI hook
CI will invoke the same command and fail on drift:
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
The default snapshot path is docs/api_server/schema.graphql; any app‑specific override must be declared in that app’s README and referenced from this document. For architecture and alignment details, see docs/dev/schema-guardrails-architecture.md.

Examples: see docs/dev/schema-checks-examples.md
See also: docs/dev/guardrails-index.md