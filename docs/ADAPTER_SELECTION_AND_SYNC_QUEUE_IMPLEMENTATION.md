# Adapter Selection and Sync Queue Implementation

This document describes the implementation of the adapter selection and sync queue infrastructure for the CPC platform.

## Overview

The implementation provides two key features:

1. **Composite Adapter Pattern**: Runtime selection between online/offline implementations of user preferences
2. **Sync Queue Infrastructure**: Dedicated system for managing offline operations and synchronizing them when connectivity is restored

## Composite Adapter Pattern

The composite adapter pattern allows the application to seamlessly switch between online and offline modes based on network connectivity.

### Implementation

The `UserPreferencesImpl` enum in `packages/infra/core/adapters/composite.rs` provides two variants:

- `Online(GrpcUserPreferences)`: Uses gRPC to communicate with the backend
- `Offline(SledUserPreferences)`: Uses Sled (embedded database) for local storage

The adapter implements the `UserPreferences` trait, providing a unified interface regardless of the underlying implementation.

### Fallback Logic

When in online mode, if a network operation fails, the system automatically falls back to storing the data locally in the offline database. This ensures that user preferences are never lost due to network issues.

## Sync Queue Infrastructure

The sync queue infrastructure manages offline operations and synchronizes them when connectivity is restored.

### Components

1. **SyncQueue**: Manages a queue of operations that need to be synchronized
2. **SyncWorker**: Background worker that processes the sync queue
3. **NetworkStatusMonitor**: Monitors network connectivity changes

### Operation Flow

1. When offline, operations are stored in the local Sled database
2. When online, operations are sent to the backend via gRPC
3. If an operation fails, it's retried with exponential backoff
4. After a maximum number of retries, operations are marked as permanently failed

## Factory Pattern

The `UserPreferencesFactory` in `packages/infra/core/factories/user_preferences_factory.rs` provides a clean way to create the appropriate user preferences implementation based on network status.

## Android Integration

The Android integration in `apps/android/rust/lib.rs` initializes the sync system with a Tokio runtime for handling asynchronous operations.

## Testing

Unit tests are provided for all major components to ensure correct behavior in both online and offline modes.

## Dependencies

The implementation uses the following key dependencies:

- **Sled**: Embedded database for local storage
- **Tonic**: gRPC client for backend communication
- **Tokio**: Async runtime for handling concurrent operations
- **Serde/Bincode**: Serialization for storing data in the database