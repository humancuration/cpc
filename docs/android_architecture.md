# Android Architecture for CPC Platform

## Overview

This document details the architecture and protocols for integrating the Rust-based CPC modules with the Android platform. The integration enables a hybrid application model where native Android UI components coexist with high-performance Rust backends.

## Key Components

### 1. JNI Bridge
A foreign function interface (FFI) that allows Kotlin/Java code to call Rust functions and vice-versa.

### 2. Lifecycle Management
A set of hooks to manage the state of the Rust core in alignment with the Android application lifecycle.

### 3. Event Bus
A system for passing events and commands between the UI layer and the Rust backend.

## Integration Pattern for Modules

Each CPC module follows a consistent integration pattern on Android:

```
apps/cpc-platform/android/app/
└── src/main/kotlin/com/cpc/[module-name]/
    ├── [Module]Activity.kt       # Main UI for the module
    ├── RustBridge.kt             # Rust<->Kotlin interface
    └── [Module]View.kt           # Custom views (if needed)
```

### Example: Music Player Module
```
apps/cpc-platform/android/app/
└── src/main/kotlin/com/cpc/musicplayer/
    ├── MusicPlayerActivity.kt    # Main player UI
    ├── RustBridge.kt             # Rust<->Kotlin interface
    └── VisualizerView.kt         # Custom canvas for visualizer
```

## JNI Interface Specification

The JNI bridge exposes several functions to facilitate communication between the Android app and CPC modules.

### Common Functions
| Function Signature (Kotlin) | Description |
|---|---|
| `external fun initializeModule()` | Initializes the module |
| `external fun handleEvent(eventType: String, data: String)` | Sends events to the Rust backend |
| `external fun getState(): String` | Gets the current module state |

## Lifecycle Management

CPC modules mirror the Android application lifecycle to ensure proper resource management and state synchronization.

### Lifecycle Events
- `onCreate` - Initialize the module
- `onPause` - Pause module operations
- `onResume` - Resume module operations
- `onDestroy` - Clean up module resources

## Best Practices

1. All heavy processing should happen in Rust
2. UI rendering should happen in Kotlin/Java
3. Use gRPC streaming for real-time data updates
4. Implement proper error handling for network failures
5. Follow privacy-by-design principles

## Related Documents

- [Music Player Integration](music_player_integration.md) - Details the music player module integration
- [Privacy Policy](privacy_policy.md) - Privacy guidelines for all modules