# Naming Convention Standardization

## Decision
- Standardized term: `ContentItem`
- Retired term: `FeedItem` (wherever it appears)

## Rationale
The term `ContentItem` better reflects the generalized nature of content in our system, while `FeedItem` implies a specific presentation context. All instances of `FeedItem` in code and documentation should be replaced with `ContentItem`.

## Implementation Plan
- [ ] Update all code references
- [ ] Update documentation files
- [ ] Deprecate `FeedItem` type if it exists