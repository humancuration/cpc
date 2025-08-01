# Expense Tracker Module - Cargo Integration

This document explains how the Expense Tracker module integrates with the Cargo build system and dependency management.

## Overview

The Expense Tracker module is designed to integrate seamlessly with the existing CPC Core package structure without requiring additional dependencies. It leverages existing dependencies and follows the same feature-flagged compilation patterns as other modules in the system.

## Current Cargo.toml Structure

The Expense Tracker module works with the existing `apps/finance/Cargo.toml` configuration:

```toml
[package]
name = "cpc-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Core Technologies (Already used by Expense Tracker)
bevy = { workspace = true }
p2panda = { workspace = true }
p2panda-core = { workspace = true }
p2panda-net = { workspace = true }
p2panda-store = { workspace = true }
sqlx = { workspace = true }
tracing = { workspace = true }

# GraphQL for Public API
async-graphql = { workspace = true, features = ["chrono", "uuid"] }

# Common dependencies (Already used by Expense Tracker)
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
chrono = { workspace = true }
uuid = { workspace = true }
anyhow = { workspace = true }
thiserror = { workspace = true }
async-trait = { workspace = true }

# Financial operations (Already used by Expense Tracker)
rust_decimal = { version = "1.34", features = ["serde", "std"] }
rust_decimal_macros = "1.34"

# Cryptography (Already used by Expense Tracker)
ed25519-dalek = { version = "2.1", features = ["serde"] }
rand = "0.8"
hex = "0.4"

# HTTP client for OAuth
reqwest = { version = "0.11", features = ["json"] }

# Android support (optional)
jni = { version = "0.21", optional = true }
winit = { version = "0.29", features = ["android-native-activity"], optional = true }

# p2p communication (Already used by Expense Tracker)
libp2p = { version = "0.53", features = ["tcp", "dns", "websocket", "noise", "yamux", "ping", "macros"] }

[features]
default = []
android = ["jni", "winit"]
finance = []  # Used by Expense Tracker
health = []
invoicing = []
p2p = []      # Used by Expense Tracker
visualization = []
web = []
crm = []

[dev-dependencies]
# Testing dependencies (Already used by Expense Tracker)
tokio = { workspace = true, features = ["macros", "rt-multi-thread", "test-util"] }
```

## No Additional Dependencies Required

The Expense Tracker module does not require any additional dependencies beyond what is already specified in the `cpc-core` package. This was a deliberate design choice to:

1. **Minimize Complexity**: Reduce dependency management overhead
2. **Ensure Compatibility**: Maintain version consistency across the project
3. **Reduce Build Times**: Avoid additional compilation steps
4. **Simplify Maintenance**: Fewer dependencies to update and monitor

## Feature Flag Integration

The Expense Tracker module integrates with the existing `finance` feature flag:

```toml
# In Cargo.toml of applications using cpc-core
cpc-core = { path = "../cpc-core", features = ["finance"] }
```

This allows the module to be conditionally compiled based on whether the finance features are needed.

## Conditional Compilation Examples

The module uses conditional compilation in several places:

### 1. Domain Models
```rust
// In domain/mod.rs
#[cfg(feature = "finance")]
pub mod expense_tracker;
```

### 2. Application Services
```rust
// In application/mod.rs
#[cfg(feature = "finance")]
pub mod expense_tracker;
```

### 3. Infrastructure Components
```rust
// In various infrastructure files
#[cfg(feature = "p2p")]
use p2panda::ratchet::DoubleRatchet;
```

## Workspace Integration

The Expense Tracker module integrates with the CPC workspace through:

1. **Workspace Dependencies**: All dependencies reference the workspace
2. **Consistent Versioning**: Shared dependency versions across all packages
3. **Unified Build Process**: Single build command for entire project
4. **Shared Configuration**: Common rustfmt, clippy, and other tooling

## Build Profiles

The module works with standard Cargo build profiles:

### Development
```bash
# Fast compilation with debug information
cargo build
```

### Release
```bash
# Optimized compilation for production
cargo build --release
```

### Testing
```bash
# Run all tests for the expense tracker
cargo test -p cpc-core --lib expense_tracker
```

## Cross-Compilation Support

The module supports cross-compilation to different targets:

### Android
```bash
# Build for Android with required features
cargo build --target aarch64-linux-android --features "finance,android"
```

### WebAssembly
```bash
# Build for WebAssembly
cargo build --target wasm32-unknown-unknown --features "finance,web"
```

## Testing Integration

The module integrates with the existing testing infrastructure:

### Unit Tests
```rust
// In expense_tracker_test.rs
#[cfg(test)]
mod tests {
    // Unit tests using existing dev-dependencies
}
```

### Integration Tests
```rust
// In expense_tracker_integration_test.rs
#[cfg(test)]
mod integration_tests {
    // Integration tests using existing dev-dependencies
}
```

## Documentation Integration

The module integrates with the existing documentation tooling:

### Rustdoc
```bash
# Generate documentation for the expense tracker
cargo doc --package cpc-core --lib finance::expense_tracker
```

### Examples
```bash
# Run examples
cargo run --example expense_tracker_usage
```

## Linting and Code Quality

The module works with existing linting tools:

### Clippy
```bash
# Run clippy on the expense tracker module
cargo clippy --package cpc-core --lib finance::expense_tracker
```

### Rustfmt
```bash
# Format the expense tracker code
cargo fmt --package cpc-core -- finance/src/expense_tracker
```

## Continuous Integration

The module integrates with existing CI pipelines:

### GitHub Actions
```yaml
# No changes needed to existing CI configuration
- name: Test
  run: cargo test --workspace
```

### GitLab CI
```yaml
# No changes needed to existing CI configuration
script:
  - cargo test --all-features
```

## Performance Monitoring

The module integrates with existing performance monitoring:

### Criterion.rs Benchmarks
```rust
// In benches/expense_tracker.rs (if created)
use criterion::{criterion_group, criterion_main, Criterion};

fn expense_creation_benchmark(c: &mut Criterion) {
    // Benchmark expense creation performance
}
```

## Security Auditing

The module works with existing security auditing tools:

### Cargo Audit
```bash
# Audit dependencies for security vulnerabilities
cargo audit
```

## Future Dependency Considerations

If additional functionality is needed in the future, dependencies would be added as optional dependencies with corresponding feature flags:

```toml
# Example of optional OCR dependency
tesseract-rust = { version = "0.5", optional = true }

[features]
default = []
finance = []
expense_tracker_ocr = ["tesseract-rust"]  # Optional OCR feature
```

## Conclusion

The Expense Tracker module integrates seamlessly with the existing Cargo build system and dependency management without requiring any changes to the `cpc-core/Cargo.toml` file. It leverages existing dependencies, follows established patterns, and maintains compatibility with all existing build, test, and deployment processes.

This approach ensures:
- ✅ No additional dependency management overhead
- ✅ Consistent versioning across the project
- ✅ Minimal impact on build times
- ✅ Easy maintenance and updates
- ✅ Seamless integration with existing tooling
- ✅ Compatibility with all target platforms
- ✅ Support for all existing development workflows