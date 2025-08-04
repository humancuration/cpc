# Contributing to CPC

Thank you for your interest in contributing to the Cooperative Public Code (CPC) project!

## Getting Started

### Prerequisites

Before you can run tests or contribute code, you'll need to set up your development environment:

1. **Rust**: Install the latest stable version of Rust from [rust-lang.org](https://www.rust-lang.org/)
2. **PostgreSQL**: Install PostgreSQL for database integration tests
3. **Git**: For version control

### Setting up the Development Environment

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd cpc
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Running Tests

### Unit Tests

Run unit tests with:
```bash
cargo test
```

### Database Integration Tests

The database integration tests require a running PostgreSQL instance. Set the `DATABASE_URL` environment variable:

```bash
export DATABASE_URL=postgresql://username:password@localhost/database_name
```

Then run tests:
```bash
cargo test
```

The database integration tests use the `sqlx::test` attribute which automatically:
- Creates temporary databases for testing
- Runs necessary migrations
- Cleans up after tests

For more details about database tests, see [DATABASE_TESTS.md](packages/social_integration/DATABASE_TESTS.md).

Please review docs/dev/docs-consistency-checks.md for the lightweight docs discoverability checks used in CI.
See docs/adr/ for our Architecture Decision Records (ADRs) and rationale behind key decisions.

### Troubleshooting docs consistency (required for contributors and reviewers)

Before or during reviews, verify our docs stay consistent across entry/index pages. Run locally:
cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency

See the troubleshooting guide: docs/dev/docs-consistency-checks.md

Notes:
- On Windows, use forward slashes in paths.
- required_substring matches are case-sensitive.
- When adding new entry or index docs, update tools/ci/needles.txt accordingly.

### Test Organization

Tests are organized in the same modules as the code they test, with a `_test` suffix:
- `postgres_unified_post_repository.rs` has tests in `postgres_unified_post_repository_test.rs`

## Code Style

We follow standard Rust conventions:
- Use `rustfmt` to format code
- Follow clippy suggestions for best practices
- Write documentation for public APIs

Format code with:
```bash
cargo fmt
```

Check for linting issues with:
```bash
cargo clippy
```

## Pull Request Process

### Pre‑PR checklist (docs consistency)

- [ ] Docs consistency: I ran cargo run -q --manifest-path tools/ci/Cargo.toml -- check-docs-consistency and resolved findings (see docs/dev/docs-consistency-checks.md). If I added new entry/index docs, I also updated tools/ci/needles.txt.
- Run the docs-consistency checker locally before pushing.
- If you add a new index or entry doc, update tools/ci/needles.txt accordingly.
- CI will fail if discoverability checks break. See docs/dev/docs-consistency-checks.md

### Pre‑PR checklist (schema drift)
- [ ] Schema check passes locally:
      cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
      Source of truth: docs/api_server/schema.graphql (or [SOURCE_OF_TRUTH_PATH] if different). See docs/dev/schema-checks.md.

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

## Reporting Issues

Please use the issue tracker to report bugs or suggest features. Include as much detail as possible, including:
- Steps to reproduce
- Expected behavior
- Actual behavior
- Environment information

## See also

- docs/dev/schema-checks.md — Procedure for local schema checks and troubleshooting
- docs/dev/schema-guardrails-architecture.md — Principles, responsibilities, and forward-compatibility notes

Reviewer quick reference — schema guardrails
- Confirm contributor ran: cargo run -q --manifest-path tools/ci/Cargo.toml -- check-schema
- Compare to snapshot: docs/api_server/schema.graphql
- If diffs exist: request rationale + alignment with docs/dev/schema-checks.md
- Cross-check intent with docs/dev/schema-guardrails-architecture.md
- Ensure no drift surprises remain before merge

## Code of Conduct

All contributors are expected to follow our Code of Conduct, which promotes a respectful and inclusive community.