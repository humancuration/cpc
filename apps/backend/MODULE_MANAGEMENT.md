# Module Management in CPC Backend

This document explains how to manage modules in the CPC backend system.

## Overview

The CPC backend uses a modular architecture that allows modules to be enabled or disabled at runtime without requiring a restart. This provides flexibility for users to customize their installation based on their needs.

## Available Modules

Currently, the following modules are available:

1. `website-builder` - Dynamic website builder with templates and analytics

## API Endpoints

### Enable a Module

```
POST /api/modules/enable
Content-Type: application/json

{
  "module_name": "website-builder"
}
```

### Disable a Module

```
POST /api/modules/disable
Content-Type: application/json

{
  "module_name": "website-builder"
}
```

### List Available Modules

```
GET /api/modules/available
```

## Module Structure

Each module follows a specific structure:

```
apps/
└── [module-name]/
    ├── Cargo.toml
    ├── MODULE.toml
    ├── migrations/
    │   └── *.sql
    └── src/
        ├── lib.rs
        ├── domain/
        ├── application/
        ├── infrastructure/
        └── web/
            ├── routes.rs
            ├── graphql.rs
            ├── module.rs
            └── modular_module.rs
```

### MODULE.toml

Each module must include a `MODULE.toml` file at the crate root with metadata:

```toml
name = "module-name"
description = "Module description"
version = "0.1.0"
## Module Dependencies
Our modular architecture supports explicit declaration of dependencies between modules to ensure proper initialization order and feature availability. Dependencies are declared in the MODULE.toml file and processed during module registration and enablement. Current implementation supports semantic versioning constraints and optional dependencies.

### Dependency Declaration Format
Modules declare dependencies using semantic versioning constraints with optional indicators:

```toml
dependencies = [
  # "required-module>=1.0.0",    # Minimum version required
  # "core-module==2.3.1",        # Exact version required
  # "analytics-module>=1.2.0,<2.0.0", # Version range
  # "optional-module?"           # Optional dependency (enabled if available)
  # Uncomment and modify these examples when adding actual dependencies
]
```

> **Note**: Actual dependencies are declared using this format, but in practice they appear uncommented when needed. See `apps/website-builder/MODULE.toml` for a real-world example.

### Dependency Resolution
When enabling a module, the system performs the following steps:

1. **Dependency Collection**: Recursively gather all required dependencies
2. **Version Validation**: Verify all version constraints are satisfied
3. **Topological Sorting**: Order modules using Kahn's algorithm to ensure dependencies are enabled before dependents
4. **State Transition**: Enable modules in the calculated order

Example resolution flow:
```
User enables "reporting-module"
↓
System discovers dependencies: ["analytics-module>=1.5.0", "core-module?"]
↓
Check if analytics-module v1.7.0 is available and meets constraint
↓
Check if core-module is available (optional, so proceed if missing)
↓
Calculate enablement order: [analytics-module, reporting-module]
↓
Enable modules in sequence
```

### Circular Dependency Prevention
The system detects circular dependencies through topological sorting:

1. Build dependency graph from all enabled modules
2. Apply Kahn's algorithm:
   - Count incoming edges (dependencies) for each node
   - Process nodes with zero incoming edges
   - Remove processed nodes and their outgoing edges
   - Repeat until all nodes processed or cycle detected
3. If nodes remain unprocessed after algorithm completion, a circular dependency exists

Error example:
```
Circular dependency detected: [apps/website-builder@1.0.0] -> [apps/analytics-module@1.2.0] -> [apps/website-builder@1.0.0]

> **Note**: This is a runtime validation error that occurs during module enablement (not at build time).
```

## GraphQL Schema Merging
Modules contribute to the global GraphQL schema through a structured merging process that ensures type safety and prevents conflicts.

### Schema Composition Rules
1. **Type Uniqueness**: Each GraphQL type must be defined by exactly one module
2. **Extension Mechanism**: Modules may extend types from other modules using interfaces:
   ```rust
   // In analytics-module
   #[Object]
   impl AnalyticsMetrics {
       async fn page_views(&self) -> i32 { /* ... */ }
   }

   // In reporting-module
   #[Object(extends)]
   impl AnalyticsMetrics {
       async fn report_generated(&self) -> bool { /* ... */ }
      }
      ```
   
   ### Current Schema Composition Rules
   1. **Type Ownership**: Each GraphQL type must be defined by exactly one module (enforced at registration time)
   2. **No Interface Extensions Yet**: The Apollo Federation-like interface extension pattern is planned but not yet implemented
   3. **Namespace Handling**: Module-specific types currently require manual prefixing in module implementations
   
   ### Planned Future Improvements
   These features are planned for future implementation:
   
   1. **Automatic Namespace Prefixing**:
      - Will automatically prefix types with module names (e.g., `WebsiteBuilder_Page`)
      - Configuration option per module to disable/modify prefixing
   
   2. **Interface Extension Support**:
      ```rust
      // In analytics-module
      #[Object(extends)]
      impl WebsitePage {
          async fn analytics_metrics(&self) -> AnalyticsMetrics { /* ... */ }
      }
      ```
   
   ### Current Merging Process
   During schema construction:

1. Modules are processed in dependency order (dependents after dependencies)
2. Each module registers its types via `register_schema()`
3. Schema builder performs:
   - Type conflict detection
   - Interface extension validation
   - Directive composition
4. Final schema is validated for consistency

Example merging sequence:
```
1. core-module defines User type
2. analytics-module extends User with tracking fields
3. reporting-module adds report-related queries
4. Final schema contains merged User type with all extensions
```
### Error Handling
The system provides clear error messages for conflicts during module operations:

**Type Ownership Conflict Example**:
```
ERROR: duplicate key value violates unique constraint "types_pkey"
DETAIL: Key (module_id, type_name)=(apps/website-builder@1.0.0, Page) already exists.
```

> **Note**: This database constraint violation occurs because type ownership is enforced at the database level via unique constraints on `(module_id, type_name)`. The system propagates these SQLx errors directly without custom formatting.

**Circular Dependency Example**:
```
Error: Circular dependency detected

website-builder → cms-module → website-builder

Resolution: Break the circular dependency by removing one dependency link.
```


### Implementation Notes
- All dependency errors generated in `src/module_registry/registry.rs` (lines 250-252, 325-327)
- Type conflicts enforced via SQLx unique constraints (see database schema)
- Errors propagate directly from database layer - no custom formatting

### Migrations

Each module has its own migrations directory at `apps/[module-name]/migrations/`. These migrations are automatically run when the module is enabled.

## Adding a New Module

To add a new module:

1. Create a new directory under `apps/` with your module name
2. Implement the module following the hexagonal architecture pattern
3. Create a `MODULE.toml` file with module metadata
4. Implement the `Module` trait in a `modular_module.rs` file
5. Add any necessary database migrations to the `migrations/` directory
6. Register the module in `apps/backend/src/main.rs`

## Technical Implementation

The module system is implemented using:

1. `ModuleRegistry` - Tracks available and enabled modules
2. `MigrationSystem` - Handles module-specific database migrations
3. `SchemaBuilder` - Dynamically builds GraphQL schemas based on enabled modules

Modules implement the `Module` trait which defines the interface for module management.

## Current Implementation Status

### Implemented Features
- ✅ Module dependency management with semantic versioning
- ✅ Topological sorting for proper enablement order
- ✅ Circular dependency detection
- ✅ Basic GraphQL schema merging in dependency order
- ✅ Type ownership enforcement (single module owns each type)

### Planned Features
- ⏳ Automatic namespace prefixing (Future Improvement #2)
- ⏳ Apollo Federation-like interface extensions (Future Improvement #3)
- ⏳ Advanced version constraint validation

See IMPLEMENTATION_SUMMARY.md for full details on current implementation and future roadmap.