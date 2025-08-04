# ADR 0008: Collaborative Workspace

## Status
Proposed

## Date
2025-08-03

## Context
To support knowledge sharing and volunteer coordination, we need collaborative workspace features including real-time document editing, project management boards, virtual whiteboarding, file sharing with version history, and video meetings. This will enable teams to work together seamlessly across the CPC ecosystem.

## Decision

### 1. Database Schema
New tables to support collaborative features:

```sql
-- Collaborative documents (CRDT-based)
CREATE TABLE collaborative_documents (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    current_state BYTEA NOT NULL, -- CRDT state
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Project boards
CREATE TABLE project_boards (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Project columns
CREATE TABLE project_columns (
    id UUID PRIMARY KEY,
    board_id UUID NOT NULL REFERENCES project_boards(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    position INT NOT NULL
);

-- Project tasks
CREATE TABLE project_tasks (
    id UUID PRIMARY KEY,
    column_id UUID NOT NULL REFERENCES project_columns(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    description TEXT,
    position INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- File versions
CREATE TABLE file_versions (
    id UUID PRIMARY KEY,
    file_id UUID NOT NULL,
    version INT NOT NULL,
    content BYTEA NOT NULL,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(file_id, version)
);

-- Meeting rooms
CREATE TABLE meeting_rooms (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    owner_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);
```

### 2. Hexagonal Architecture Structure
New `collaborative_workspace` package structure:

```
src/
├── domain/
│   ├── models.rs          # Document, ProjectBoard, Whiteboard, etc.
│   ├── repository.rs      # Trait definitions
│   └── service.rs         # Service traits
├── application/
│   ├── document_service.rs
│   ├── project_service.rs
│   ├── whiteboard_service.rs
│   ├── file_service.rs
│   ├── meeting_service.rs
│   └── notification_integration.rs
├── infrastructure/
│   ├── postgres_repository.rs
│   ├── in_memory_repository.rs
│   ├── event_bus.rs       # Extends social_interactions event bus
│   └── webrtc.rs          # WebRTC implementation
└── lib.rs
```

### 3. Core Components

#### Document Service
- `apply_operation(document_id, operation)` - CRDT-based updates
- `get_document_state(document_id)`
- `create_document(title, initial_content)`

#### Project Service
- `create_board(title, owner_id)`
- `add_column(board_id, title, position)`
- `move_task(task_id, new_column_id, position)`

#### Whiteboard Service
- `create_whiteboard(title)`
- `add_whiteboard_element(whiteboard_id, element)` 
- `update_whiteboard_element(whiteboard_id, element_id, new_data)`

#### File Service
- `upload_file(file_data, owner_id)`
- `create_version(file_id, content)`
- `get_file_version(file_id, version)`

#### Meeting Service
- `create_meeting(title, owner_id)`
- `generate_webrtc_offer(meeting_id, user_id)`
- `add_ice_candidate(meeting_id, candidate)`

#### Notification Integration
Extend `notification_core` with new categories:
- `NotificationCategory::DocumentUpdated`
- `NotificationCategory::TaskAssigned`
- `NotificationCategory::WhiteboardUpdated`
- `NotificationCategory::MeetingStarted`

### 4. API Endpoints (GraphQL)
New `apps/api_server/src/graphql/collaborative_workspace.rs` with:

```rust
#[Object]
impl CollaborativeWorkspaceMutations {
    async fn create_document(&self, title: String) -> Result<Document> { ... }
    async fn apply_document_operation(&self, document_id: ID, operation: DocumentOperation) -> Result<Document> { ... }
    async fn create_project_board(&self, title: String) -> Result<ProjectBoard> { ... }
    // Other mutations...
}

#[Object]
impl CollaborativeWorkspaceQueries {
    async fn get_document(&self, document_id: ID) -> Result<Document> { ... }
    async fn get_project_board(&self, board_id: ID) -> Result<ProjectBoard> { ... }
    // Other queries...
}
```

### 5. Event Bus Integration
Extend `SocialEventBus` from `social_interactions` with new events:
- `DocumentUpdated`
- `TaskMoved`
- `WhiteboardModified`
- `MeetingStarted`

## Consequences

### Positive
- Unified collaboration experience across apps
- Real-time synchronization using CRDTs
- Seamless integration with existing social features
- Version history for audit trails
- WebRTC enables low-latency video meetings

### Negative
- Increased database storage requirements
- Complexity of CRDT conflict resolution
- WebRTC requires STUN/TURN server infrastructure
- Additional load on event bus

### Neutral
- Requires updates to GraphQL schema
- New dependencies: CRDT libraries, WebRTC
- Additional monitoring for real-time services

## Implementation Details

### Core Integration Points
1. **Event Bus**: Reuse and extend `SocialEventBus` for collaboration events
2. **Notifications**: Integrate with `notification_core` for collaboration alerts
3. **Auth**: Reuse existing authentication middleware
4. **Storage**: Use PostgreSQL for metadata, object storage for files
5. **Realtime**: WebSockets for operational transforms, WebRTC for video

### Testing Strategy
- Property-based testing for CRDT operations
- Integration tests for collaboration workflows
- Load testing for real-time editing
- Browser compatibility testing for WebRTC
- Failure injection testing for network partitions

## Security Considerations
- Authorization checks for all collaborative resources
- End-to-end encryption for document contents
- Sanitization of HTML/markdown content
- Rate limiting for operational transforms
- WebRTC security: DTLS, SRTP
- Permission models for shared workspaces

## Future Considerations
- Offline editing support
- Conflict resolution UI
- Collaborative code editing
- Meeting recording with transcription
- Integration with p2panda for decentralized collaboration