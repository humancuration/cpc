## Offline-First Data Patterns

### Conflict Resolution Implementation
All offline-capable features must implement conflict resolution strategies following:
1. Vector clock or timestamp-based resolution
2. Explicit resolution interface implementation
3. Last-write-wins as minimum viable strategy

Example implementation:
```rust
trait ConflictResolution {
    fn resolve_conflict(&self, local: &DataType, remote: &DataType) -> Resolution;
}
```

### Adapter Selection Strategy
Implement composite adapters that automatically select between online/offline implementations:

```rust
enum UserPreferencesImpl {
    Online(GrpcUserPreferences),
    Offline(SledUserPreferences),
}

impl UserPreferences for UserPreferencesImpl {
    // Delegates to appropriate implementation
}
```

### Sync Queue Requirements
All offline features must:
1. Track sync state in storage model
2. Implement explicit sync queue
3. Provide background sync worker