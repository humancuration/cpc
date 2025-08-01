# Conflict Resolution System

## Overview
This document specifies the conflict resolution system for real-time collaborative editing. The system uses Operational Transformation (OT) as the primary conflict resolution mechanism, with Conflict-free Replicated Data Types (CRDT) as a fallback. It integrates with Presence Manager for priority resolution and Version Manager for historical tracking.

## Key Components
1. **Conflict Detector**: Identifies conflicts by checking operation ranges
2. **OT Transformer**: Implements transformation functions for different operation types
3. **Resolution Strategies**:
   - Timestamp-based ordering
   - User priority (using Presence QoS)
   - Operational merging
   - Manual resolution
4. **Event Integration**: Publishes ConflictDetected and ConflictResolved events

## API Definitions

### ConflictResolver
```rust
struct ConflictResolver {
    document_id: Uuid,
    conflicts: HashMap<Uuid, Conflict>,
    document_content: String,
    // ...
}

impl ConflictResolver {
    fn detect_conflicts(&self, operations: &[Operation]) -> Vec<Conflict>;
    fn resolve_conflict(&mut self, conflict_id: Uuid) -> Result<(), CollaborationError>;
    fn transform_operation(&mut self, op1: &Operation, op2: &Operation) -> Result<Operation, CollaborationError>;
    fn get_user_priority(&self, user_id: Uuid) -> i32;
    fn set_document_content(&mut self, content: String);
}
```

### Transformation Functions
```rust
fn transform_insert_vs_insert(conflict_id: Uuid, ...) -> Operation;
fn transform_insert_vs_delete(conflict_id: Uuid, ...) -> Operation;
fn transform_delete_vs_insert(conflict_id: Uuid, ...) -> Operation;
fn transform_delete_vs_delete(conflict_id: Uuid, ...) -> Operation;
```

## Algorithm Pseudocode

### Operational Transformation
```
function transform_operation(op1, op2):
    if op1 is Insert and op2 is Insert:
        return transform_insert_vs_insert(Uuid::nil(), op1, op2)
    if op1 is Insert and op2 is Delete:
        return transform_insert_vs_delete(Uuid::nil(), op1, op2)
    if op1 is Delete and op2 is Insert:
        return transform_delete_vs_insert(Uuid::nil(), op1, op2)
    if op1 is Delete and op2 is Delete:
        return transform_delete_vs_delete(Uuid::nil(), op1, op2)
```

### Conflict Resolution Workflow
```
detect_conflicts(operations):
    for each operation pair (op1, op2):
        if operation_ranges_overlap(op1, op2):
            create_conflict(op1, op2)

resolve_conflict(conflict):
    case conflict.resolution_strategy:
        TimestampOrder:
            ops = sort_by_timestamp(conflict.operations)
            return apply_transform_chain(ops)
        UserPriority:
            ops = sort_by_user_priority(conflict.operations)
            return apply_transform_chain(ops)
        Merge:
            return merge_operations(conflict.operations)
        Manual:
            queue_for_user_resolution(conflict)
    
    // Create version after resolution
    if version_manager:
        create_version(document_content, resolved_operations, conflict_metadata)
```

### Range Calculation
```
function calculate_range_length(start, end, document_content):
    if start.line == end.line:
        return end.column - start.column
    else:
        line_diff = end.line - start.line
        first_line_chars = len(document_content.lines[start.line]) - start.column
        middle_lines_chars = sum(len(document_content.lines[start.line + i]) for i in 1..line_diff)
        last_line_chars = end.column
        return first_line_chars + middle_lines_chars + last_line_chars
```

### Merge Strategy
```
function resolve_by_merge(operations):
    sorted_ops = sort_by_position(operations)
    for i in 0..len(sorted_ops):
        for j in 0..i:
            sorted_ops[i] = transform_operation(sorted_ops[i], sorted_ops[j])
    return sorted_ops
```

## Test Scenarios

### Scenario 1: Concurrent Inserts at Same Position
```rust
// User A inserts "Hello" at position (0,0)
let op_a = Insert { position: (0,0), text: "Hello", ... };

// User B inserts "Hi" at position (0,0)
let op_b = Insert { position: (0,0), text: "Hi", ... };

// Expected: Transform to "HiHello" or "HelloHi" based on timestamp
```

### Scenario 2: Insert-Delete Conflict
```rust
// User A inserts "World" at (0,5)
let op_a = Insert { position: (0,5), text: "World", ... };

// User B deletes range (0,0) to (0,5)
let op_b = Delete { start: (0,0), end: (0,5), ... };

// Expected: Insert moves to (0,0) after deletion
```

### Scenario 3: Priority-Based Resolution
```rust
// Admin user (QoS 0) and regular user (QoS 2)
// Admin operation should take precedence
```

### Scenario 4: Operation Merging
```rust
// User A inserts "A" at (0,1)
// User B inserts "B" at (0,3)
// Should merge to "A B" without conflict
```

### Scenario 5: Multi-line Range Calculation
```rust
// Document content: "Line 1\nLine 2\nLine 3"
// Calculate range from (0,2) to (2,3)
// Expected: "ne 1\nLine 2\nLin" (length = 13)
```

## Integration Points
1. **Presence Manager**: Provides QoS-based user priorities
2. **Version Manager**: Creates snapshots after conflict resolution
3. **Schema Registry**: Validates conflict-related events
4. **Event Bus**: Publishes ConflictDetected and ConflictResolved events

## Error Handling
- `OperationConflict`: When conflicts cannot be automatically resolved
- `TransformationError`: When operations cannot be transformed
- `ResolutionTimeout`: When manual resolution takes too long

## Performance Considerations
- Operation batching to reduce transformation overhead
- Caching of transformation results
- Limiting conflict resolution depth