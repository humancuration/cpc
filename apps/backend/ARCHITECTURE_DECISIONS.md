# CPC Backend Architecture Decisions

## Decision: Module Dependency Management
**Date**: 2025-07-26  
**Status**: Accepted  
**Context**:  
As our system grows with more modules, we need a robust dependency management system that:
- Ensures modules initialize in correct order
- Prevents circular dependencies
- Handles optional features gracefully
- Provides clear error messages for misconfigurations

**Decision**:  
We implement a dependency management system based on directed acyclic graphs (DAGs) with the following characteristics:

1. **Topological Sorting for Enablement Order**  
   Uses Kahn's algorithm to determine safe enablement sequence, processing modules with zero dependencies first.

2. **Semantic Versioning Constraints**  
   Supports standard semver ranges (`>=`, `==`, `<`, etc.) for dependency version specification.

3. **Optional Dependencies with Fallbacks**  
   Modules marked with `?` are treated as optional - features depending on them are gracefully degraded.

4. **Database-Backed State Tracking**  
   The `module_registry` table stores both enabled state AND dependency relationships for cross-restart consistency.

**Rationale**:  
This approach provides:
- Strong initialization guarantees (dependencies always enabled first)
- Clear upgrade paths through version constraints
- Resilience through optional dependencies
- Recovery from failed states via database persistence
- Compatibility with our existing async/await architecture

**Consequences**:  
- Increased startup time for large module sets (mitigated by caching)
- Requires careful version management across modules
- Adds complexity to module enable/disable operations

---

## Decision: GraphQL Schema Composition
**Date**: 2025-07-26  
**Status**: Accepted  
**Context**:  
Our modular architecture requires that:
- Each module can independently contribute to the GraphQL schema
- Type conflicts between modules are impossible
- Modules can safely extend types from other modules
- Schema remains valid at all times

**Decision**:  
We implement schema composition through:

1. **Explicit Type Ownership**  
   Each GraphQL type belongs to exactly one module (enforced at registration)

2. **Interface-Based Extension**  
   Modules extend types from dependencies using Apollo Federation-like interfaces:
   ```rust
   #[Object(extends)]
   impl BaseModuleType {
       async fn extension_field(&self) -> String { /* ... */ }
   }
   ```

3. **Automatic Namespace Prefixing**  
   Module names are used as prefixes for types (configurable per module)

4. **Dependency-Ordered Registration**  
   Schema building occurs in topological dependency order

**Rationale**:  
This approach ensures:
- Type safety through single ownership
- Composability via interface extensions
- Conflict prevention using namespaces
- Compatibility with async_graphql's schema builder
- Clear error messages for schema conflicts

**Consequences**:  
- Slight verbosity in type names (mitigated by automatic prefixing)
- Requires discipline in module design
- Initial complexity in schema registration
- Better long-term maintainability