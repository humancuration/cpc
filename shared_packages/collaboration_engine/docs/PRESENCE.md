# Presence Management

## Overview
Tracks user presence in collaborative documents including:
- Cursor positions
- Text selections
- Typing status

## Integration
```rust
// Create presence manager
let mut presence = PresenceManager::new(document_id);
presence.set_event_bus(event_bus);

// Update presence
presence.update_presence(
    user_id, 
    Some(Position { line: 5, column: 10 }), 
    None, 
    true
)?;
```

## Events
- `PresenceUpdated`: Published on any presence change