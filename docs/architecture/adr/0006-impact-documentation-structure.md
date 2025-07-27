# ADR 0006: Impact Service Documentation Structure

## Status
Accepted

## Context
Multiple impact-related documentation files existed across different directories:
- `docs/impact_service_architecture.md`
- `docs/dev/impact-report-impl.md`
- `docs/features/impact-report.md`

This created knowledge silos and violated our single source of truth principle.

## Decision
All impact-related architecture documentation must follow:
- Core service design: `docs/architecture/impact-service.md`
- Feature flags implementation: `docs/feature-flags.md`
- Database design: `docs/database-design.md`
- Deprecated docs will be archived with deprecation notice

## Consequences
- ✅ Clear separation of concerns following Hexagonal Architecture
- ✅ Single authoritative source for impact service architecture
- ✅ Elimination of documentation duplication
- ⚠️ Requires updating all references to point to new location