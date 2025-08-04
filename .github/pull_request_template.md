# Pull Request

Describe your change briefly. Keep it focused and link related issues.

## Checklist (optional)

- [ ] Tests added/updated (if applicable)
- [ ] Docs updated (if applicable)

## Schema Guardrails (GraphQL) — optional

- [ ] If this PR changes GraphQL schema, I ran locally:
      cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- [ ] If intentional schema updates, I updated the snapshot:
      cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema --write-snapshot

Reference: docs/dev/schema-checks-ci-snippet.md
QA checklist: docs/qa/schema-guardrails-checklist.md