# Media Workshop Domain Models

## Core Entities

### MediaAsset
```mermaid
classDiagram
    class MediaAsset {
        +String id
        +String title
        +MediaType type
        +String owner_id
        +DateTime created_at
        +DateTime updated_at
        +List~Permission~ permissions
        +MediaMetadata metadata
        +StorageLocation storage
    }
```

### MediaMetadata
```mermaid
classDiagram
    class MediaMetadata {
        +Format format
        +Duration duration
        +Dimensions resolution
        +String codec
        +Int file_size
        +Map~String, String~ exif_data
    }
```

### CollaborationSession
```mermaid
classDiagram
    class CollaborationSession {
        +String session_id
        +String asset_id
        +List~Participant~ participants
        +DateTime started_at
        +DateTime? ended_at
        +List~EditOperation~ edit_history
    }
```

## Relationships
```mermaid
erDiagram
    USER ||--o{ MEDIA_ASSET : owns
    MEDIA_ASSET ||--o{ COLLABORATION_SESSION : "has sessions"
    COLLABORATION_SESSION ||--o{ USER : "has participants"
    MEDIA_ASSET }o--|| SOCIAL_POST : "shared as"
    
    USER {
        string id PK
        string username
    }
    MEDIA_ASSET {
        string id PK
        string owner_id FK
    }
    COLLABORATION_SESSION {
        string id PK
        string asset_id FK
    }
    SOCIAL_POST {
        string id PK
        string media_asset_id FK
    }
```

## Enums

### MediaType
```rust
pub enum MediaType {
    Photo,
    Video,
    AudioTrack,
    AudioProject,
    ArtProject,
    Document,
    Other(String),
}
```

### PermissionLevel
```rust
pub enum PermissionLevel {
    Owner,
    Editor,
    Commenter,
    Viewer,
}
```

### Format
```rust
pub enum VideoFormat {
    WebM,
    // Other formats supported by ffmpeg.wasm
}

pub enum AudioFormat {
    Opus,
    // Other royalty-free formats
}

pub enum ImageFormat {
    Avif,
    WebP,
}
```

## Value Objects

### Dimensions
```rust
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}
```

### StorageLocation
```rust
pub struct StorageLocation {
    pub node_id: String, // P2P node identifier
    pub path: String,
    pub is_cached: bool,
}
```

### EditOperation
```rust
pub struct EditOperation {
    pub user_id: String,
    pub timestamp: DateTime,
    pub operation_type: EditType,
    pub parameters: Value, // JSON object
}