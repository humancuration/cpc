# Schema Checks — Examples and Diffs

Purpose
This page shows concrete, copyable workflows for intentional GraphQL schema changes using our local schema check. It complements schema-checks.md by moving from “what and why” to “do this now,” focusing on diffs and one-sentence PR rationales.

Before you start
- You can run the local check: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- You know where snapshots live: docs/api_server/schema.graphql

## Example 1: Add a field to an existing type (happy path)

Preconditions
- You are adding a non-breaking field (nullable or with safe defaults).
- The new field has a resolver or a static value where appropriate.

Steps
1) Edit schema or resolvers
   Make a minimal change to add a field to an existing type in your schema or the associated resolvers. Prefer nullable for forward compatibility.

2) Run local schema check
   Command:
   cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
   This regenerates and compares the schema snapshot.

3) Review snapshot diff
   Open docs/api_server/schema.graphql and confirm the new field appears under the correct type with the intended nullability and description (if relevant).

4) Update tests/resolvers as needed
   Ensure resolvers return a value (or null) and that no other paths are broken.

5) Re-run local schema check
   Command:
   cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
   Confirms snapshot matches your intent and shows no unexpected drift.

6) Stage the snapshot
   Add docs/api_server/schema.graphql to your commit; the snapshot is the source of truth for review.

7) Write a one-sentence PR rationale
   Copy/paste from the template below and replace the bracketed text.

Result
- docs/api_server/schema.graphql shows the new field under the target type.
- PR rationale (one sentence):
  “Schema change: add [Type.field] to support [brief reason]. Local check run; snapshot updated. No additional API behavior changes.”

### What changes you’ll see (Diff narrative template)
- Code: Added field [Type.field: Type] (nullable recommended) to schema/resolver.
- Local check: run cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Snapshot: docs/api_server/schema.graphql should show [field added] under [Type].
- PR rationale (copy/paste):
  “Schema change: [brief reason]. Local check run; snapshot updated. No additional API behavior changes.”

## Example 2: Rename a field (requires careful review)

Steps (summary)
1) Prefer additive path: add the new field first, backfill resolvers, and deprecate the old field.
2) If you must rename: update schema and every resolver/selector using the old field.
3) Run local schema check:
   cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
4) Review snapshot carefully: expect one removal and one addition with the names and types you intended—no collateral changes.
5) Backward compatibility: if clients still use the old name, choose “add new + deprecate old” instead of a direct rename.
6) Only update the snapshot if the rename is intentional, reviewed, and compatible with consumers; otherwise decline the snapshot update and fix code to retain compatibility.

Notes on backward compatibility
- Renames can break clients. Prefer additive changes + deprecation.
- Decline snapshot updates when the diff shows unintended removals; fix and re-run the check.

## Example 3 (stub): Remove a field (breaking change)

Notes
- This is a breaking change. Provide a strong rationale and a migration plan (clients impacted, timeline, deprecation period if applicable).
- Link to the review checklist in schema-checks.md.
- Run the local check and ensure the snapshot shows only the intended removal.

## Links
- schema-checks.md
- schema-guardrails-architecture.md
- CONTRIBUTING.md