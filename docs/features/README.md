# CPC Features Documentation

This directory contains documentation for various features implemented in the CPC software ecosystem.

## Real-time Presence Indicators

The real-time presence indicators feature allows users to see who else is collaborating on a document and their current status.

### Key Components

1. **Enhanced Signaling Messages** - Additional fields in presence updates for avatar, color, and activity information
2. **Presence Expiration** - Automatic cleanup of stale presence information
3. **Differential Updates** - Efficient broadcasting of presence information
4. **UI Components** - Visual indicators for user presence and activity

### Documentation

- [Presence Indicators Design Document](presence_indicators.md) - Comprehensive overview of the architecture and implementation

### Implementation Files

- `shared_packages/realtime_signaling/src/message.rs` - Enhanced message types
- `shared_packages/realtime_signaling/src/redis_signaling.rs` - Presence handling with expiration logic
- `apps/document_editor/src/presentation/presence_indicators.rs` - UI components
- `apps/document_editor/src/presentation/editor_with_presence.rs` - Example integration

### Tests

- `shared_packages/realtime_signaling/tests/presence_integration_test.rs` - Basic presence functionality tests
- `shared_packages/realtime_signaling/tests/presence_with_signaling_test.rs` - Integration with signaling service
- `apps/document_editor/src/presentation/presence_indicators_test.rs` - UI component tests
- `apps/document_editor/src/presentation/editor_with_presence_test.rs` - Integration tests

### Examples

- `apps/document_editor/examples/presence_indicators_example.rs` - Standalone example of presence indicators