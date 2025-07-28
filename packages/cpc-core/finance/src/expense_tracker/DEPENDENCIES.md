# Expense Tracker Module Dependencies

This document outlines the dependencies required for the Expense Tracker module and how they fit into the existing CPC Core package structure.

## Overview

The Expense Tracker module leverages existing dependencies from the CPC Core package and follows the same dependency management patterns as other modules in the system.

## Required Dependencies

All dependencies needed by the Expense Tracker module are already included in the `cpc-core` package's `Cargo.toml` file. No additional dependencies need to be added.

### Core Dependencies (Already in cpc-core)

1. **Database**: `sqlx` - Used for PostgreSQL database operations
2. **Serialization**: `serde`, `serde_json` - For data serialization/deserialization
3. **Time Handling**: `chrono` - For date/time operations
4. **UUID Generation**: `uuid` - For unique identifier generation
5. **Error Handling**: `thiserror`, `anyhow` - For consistent error handling
6. **Async Traits**: `async-trait` - For async repository interfaces
7. **Financial Operations**: `rust_decimal`, `rust_decimal_macros` - For precise monetary calculations
8. **p2p Communication**: `p2panda`, `p2panda-core`, `p2panda-net`, `p2panda-store` - For secure data sharing
9. **Graphics/Visualization**: `bevy` - For receipt scanning UI and visualization
10. **Logging**: `tracing` - For consistent logging

## Feature Flags

The Expense Tracker module uses the existing `finance` feature flag, which is already defined in the `cpc-core` package.

To enable the expense tracker functionality, compile with the finance feature:

```toml
cpc-core = { path = "../cpc-core", features = ["finance"] }
```

## Optional Dependencies Considerations

### OCR Processing

For production OCR processing, you might want to consider adding an OCR library like `tesseract-rust`:

```toml
# Optional OCR dependency (not included by default)
tesseract-rust = { version = "0.5", optional = true }
```

This would be added as an optional dependency and enabled through a feature flag if needed.

### Image Processing

For advanced image processing, you might consider:

```toml
# Optional image processing dependency (not included by default)
image = { version = "0.24", optional = true }
```

## Dependency Management Strategy

The Expense Tracker follows the same dependency management strategy as other modules in the CPC system:

1. **Reuse Existing Dependencies**: All required dependencies are already part of cpc-core
2. **Feature-Gated Compilation**: Functionality is enabled through feature flags
3. **Vertical Slice Isolation**: The module is self-contained with clear boundaries
4. **Standard Error Handling**: Uses the existing FinanceError pattern
5. **Consistent Serialization**: Uses the same serde configuration as other modules

## Version Compatibility

The Expense Tracker module is compatible with all dependency versions specified in the current `cpc-core/Cargo.toml` file. No version conflicts should occur when integrating this module.

## Build Configuration

No special build configuration is required for the Expense Tracker module. It will build with the standard cpc-core package using the finance feature flag.

## Testing Dependencies

The module uses the same testing dependencies as the rest of the cpc-core package:

- `tokio` with testing features
- Standard Rust testing framework

## Integration with Workspace Dependencies

The Expense Tracker module integrates seamlessly with the workspace dependencies as defined in the root `Cargo.toml` file of the CPC project. All dependencies are referenced through the workspace to ensure version consistency across the entire project.

## Future Dependency Considerations

If additional functionality is added in the future, dependencies might include:

1. **Machine Learning**: For advanced expense categorization
2. **Advanced OCR**: More sophisticated text recognition capabilities
3. **Image Enhancement**: For better receipt image processing
4. **Data Analytics**: For expense pattern analysis

These would be added as optional dependencies with corresponding feature flags to maintain the modular nature of the system.