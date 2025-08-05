# Collaborative Documents Integration Architecture

## 1. App Integration Points

### Initialization Sequence
```mermaid
sequenceDiagram
    participant App as Client App
    participant DocSvc as Document Service
    participant Prov as Content Provider
    participant Reg as Social Registry
    
    App->>DocSvc: Initialize(store, access_checker)
    App->>Prov: new(DocSvc)
    App->>Reg: register(Prov, metadata)
    Reg->>Consent: Wrap provider
    Reg->>Deps: Validate dependencies
    Reg-->>App: Provider ID
```

### Registration Locations
- **Desktop Apps**: During main window initialization
- **Web Apps**: In application bootstrapping
- **Mobile**: In app startup sequence

## 2. Consent Flow Extension

### Visibility Settings
```mermaid
stateDiagram-v2
    [*] --> Private
    Private --> FriendsOnly: User updates
    Private --> Public
    FriendsOnly --> Public
    Public --> Private
```

### UI Integration Points
- Document properties panel
- Share dialog
- New document creation flow

### Consent Management Flow
```mermaid
sequenceDiagram
    participant User
    participant UI as Document UI
    participant Consent as Consent Service
    participant Social as Social Graph
    
    User->>UI: Open document properties
    UI->>Consent: Get current visibility settings
    Consent->>Social: Fetch relationships
    Social-->>Consent: Relationship data
    Consent-->>UI: Current visibility + options
    UI-->>User: Show consent dialog
    User->>UI: Select new visibility (Public/Friends/Private)
    UI->>Consent: Update visibility settings
    Consent->>Social: Notify visibility change
    Social->>Feed: Update document visibility
```

### Consent UI Wireframe
```
+-------------------------------+
| Document Visibility Settings  |
|                               |
| [●] Private                   |
| [ ] Friends Only              |
| [ ] Public                    |
|                               |
| [ ] Allow comments            |
| [✓] Allow editing             |
|                               |
| [Save]      [Cancel]          |
+-------------------------------+
```

### Consent Enforcement
```rust
fn enforce_consent(item: &ContentItem, user: &User) -> bool {
    match item.visibility {
        Visibility::Public => true,
        Visibility::FriendsOnly => social_graph.are_friends(user, item.owner),
        Visibility::Private => user.id == item.owner_id,
    }
}
```

## 3. Feed Integration

### Document Previews
```mermaid
classDiagram
    class FeedItem {
        +id: Uuid
        +title: String
        +preview: String
        +owner: UserRef
        +visibility: Visibility
        +open_action()
        +share_action()
    }
```

### Interaction Patterns
1. Feed renders document card with:
   - Title
   - First 100 characters of content
   - Owner avatar
   - Visibility indicator
2. Tap/click opens document in editor
3. Long-press shows sharing options

### Document Preview Flow
```mermaid
sequenceDiagram
    participant User
    participant Feed
    participant Preview as Preview Service
    participant DocSvc as Document Service
    
    User->>Feed: Scroll through feed
    Feed->>Preview: Request previews
    Preview->>DocSvc: Get document metadata
    DocSvc-->>Preview: Title + snippet
    Preview-->>Feed: Rendered preview card
    Feed-->>User: Display preview card
    User->>Feed: Tap on document card
    Feed->>DocSvc: Open document request
    DocSvc-->>Feed: Document content
    Feed-->>User: Show document editor
```

### Document Preview Wireframe
```
+-------------------------------+
| Planning Meeting Notes        |
|                               |
| Let's discuss Q3 goals...     |
| We need to finalize the...    |
|                               |
| [👤] Sarah Chen               |
| 🔒 Friends Only   📅 2h ago   |
+-------------------------------+
```

## 4. Cross-App Coordination

### Change Propagation
```mermaid
sequenceDiagram
    participant App1 as App Instance 1
    participant DocSvc as Document Service
    participant App2 as App Instance 2
    participant Feed as Social Feed
    
    App1->>DocSvc: Update document
    DocSvc->>Feed: Notify content change
    Feed->>App2: Push update
    App2->>Feed: Refresh preview
```

### Conflict Resolution
- CRDT-based merging for content
- Last-write-wins for metadata
- UI shows conflict resolution dialog when:
  - Simultaneous edits detected
  - Version mismatch occurs

### Real-time Signaling
```rust
enum CollaborationSignal {
    CursorPosition { user: Uuid, position: usize },
    SelectionRange { start: usize, end: usize },
    PresenceUpdate { user: Uuid, active: bool },
}
```

## Component Integration
```mermaid
flowchart TD
    A[Document Editor] -->|Content| B[Content Provider]
    A -->|Signals| RTC[Real-time Service]
    B --> C[Social Registry]
    C --> D[Consent Middleware]
    D --> E[Social Graph]
    E --> F[Relationship Repo]
    B -->|Notifications| G[Feed Service]
    G --> P[Preview Service]
    P -->|Metadata| B
    
    style RTC fill:#f9f,stroke:#333
    style P fill:#9f9,stroke:#333
```

### Integration Points
1. **Real-time Service (RTC)**: Handles collaboration signals between app instances
2. **Preview Service**: Generates document previews for social feeds
3. **Consent Middleware**: Enforces visibility rules for all document access
4. **Feed Service**: Distributes document updates to user feeds

## Next Steps
1. Implement UI components for consent management
2. Add feed preview rendering logic
3. Integrate real-time signaling service
4. Update documentation with examples