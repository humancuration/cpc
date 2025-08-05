# Real-time Presence Indicators

## Architecture Overview

The presence indicators feature provides real-time visualization of user activity in collaborative documents. The system consists of:

1. Enhanced signaling messages with additional presence information
2. Server-side presence management with expiration logic
3. Client-side UI components for visualizing user presence

```
[Client] ←→ [Signaling Server] ←→ [Redis] ←→ [Other Clients]
    ↓
[UI Components: Sidebar, Cursors, Status Indicators]
```

## Message Types

### Updated PresenceUpdate struct

```rust
/// Presence update message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceUpdate {
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub cursor: Option<Position>,
    pub selection: Option<SelectionRange>,
    pub is_typing: bool,
    pub avatar_url: Option<String>,
    pub color: String,
    pub last_active: DateTime<Utc>,
    pub timestamp: DateTime<Utc>,
}

/// Presence summary message for efficient broadcasting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceSummary {
    pub users: HashMap<Uuid, PresenceUser>,
    pub expires_at: DateTime<Utc>,
}

/// Individual user presence information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresenceUser {
    pub avatar_url: Option<String>,
    pub color: String,
    pub status: PresenceStatus,
}

/// User presence status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PresenceStatus {
    Online,
    Away,
    Busy,
    Offline,
}
```

## Data Flow

1. **Client sends presence updates**
   - When a user joins, leaves, moves cursor, or starts typing
   - Updates include avatar, color, and activity information

2. **Server aggregates and expires stale presences**
   - Maintains presence state for each document
   - Automatically expires users after 30 seconds of inactivity
   - Sends periodic summaries every 5 seconds

3. **Selective broadcasts to relevant clients**
   - Individual updates for immediate feedback
   - Periodic summaries for efficient state synchronization

4. **UI components render presence state**
   - PresenceSidebar shows all users in the document
   - CursorOverlay visualizes cursor positions
   - StatusIndicator shows user availability
   - AvatarBadge displays user identity

## UI Components

### `PresenceSidebar`

A vertical sidebar showing all users currently in the document with their status and avatar.

### `CursorOverlay`

An overlay that displays other users' cursor positions with colored indicators.

### `StatusIndicator`

A small component showing a user's current status (Online, Away, Busy, Offline).

### `AvatarBadge`

A component displaying a user's avatar or colored initial with visual feedback for typing activity.

## Implementation Details

### Presence Expiration

Users are automatically marked as "Away" after 5 seconds of inactivity and removed after 30 seconds.

### Differential Updates

To reduce network traffic, the system sends:
- Individual presence updates for immediate feedback
- Periodic summaries every 5 seconds for state synchronization

### Color Coding

Each user is assigned a unique color for their cursor and presence indicators to distinguish between users.