# Music Player Module Architecture

This document outlines the architecture for the music-player module, implementing the feature described in planned_apps.md (line 9). The design follows our hexagonal architecture principles as documented in modular_architecture_v2.md and aligns with the successful implementation pattern demonstrated in the website-builder module.

## 1. Module Structure

```
apps/music-player/
├── MODULE.toml
├── migrations/
│   └── 20250727000000_music_player_schema.sql
├── Cargo.toml
└── src/
    ├── lib.rs                  # Main crate entry, exports the module
    ├── domain/                 # Core business logic
    │   ├── models.rs           # Primary entities
    │   ├── value_objects.rs    # Domain-specific types
    │   └── errors.rs           # Custom error types
    ├── application/            # Business logic services
    │   ├── streaming_service.rs # Core streaming operations
    │   ├── social_service.rs   # Likes, reposts, comments
    │   ├── visualizer_service.rs # Visualizer data generation
    │   └── cache_service.rs    # Offline download management
    ├── infrastructure/         # External implementations
    │   ├── database.rs         # SQLx repository implementations
    │   ├── p2p.rs              # p2panda integration for streaming
    │   └── media_store.rs      # Content-addressed storage
    └── web/                    # Adapter layer
        ├── routes.rs           # REST API routes
        ├── graphql.rs          # GraphQL definitions
        ├── module.rs           # Module initialization & wiring
        └── types.rs            # GraphQL input/output types
```

## 2. Core Domain Models

### Primary Entities

```rust
// domain/models.rs

/// A musical track with metadata
pub struct Track {
    pub id: Uuid,
    pub artist_id: Uuid,
    pub album_id: Option<Uuid>,
    pub title: String,
    pub duration_ms: u64,
    pub media_cid: String,  // Content ID for p2p storage
    pub waveform_data_cid: Option<String>, // For visualizer
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A timestamped comment on a track
pub struct TimestampedComment {
    pub id: Uuid,
    pub track_id: Uuid,
    pub user_id: Uuid,
    pub timestamp_ms: u64,  // Position in track where comment applies
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// A user's interaction with a track
pub enum TrackInteraction {
    Like { track_id: Uuid, created_at: DateTime<Utc> },
    Repost { track_id: Uuid, comment: Option<String>, created_at: DateTime<Utc> },
    FollowArtist { artist_id: Uuid, created_at: DateTime<Utc> },
}

/// Visualizer configuration and metadata
pub struct VisualizerPreset {
    pub id: Uuid,
    pub name: String,
    pub config: serde_json::Value,  // Custom visualizer settings
    pub is_default: bool,
}

/// Playlist containing tracks
pub struct Playlist {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub track_positions: Vec<PlaylistTrack>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Position of a track within a playlist
pub struct PlaylistTrack {
    pub track_id: Uuid,
    pub position: u32,
}
```

## 3. Key Application Services

### StreamingService
- `get_stream_url(track_id: Uuid) -> Result<String>`: Generates streaming URL using p2panda
- `get_visualizer_data(track_id: Uuid) -> Result<VisualizerData>`: Provides waveform/frequency data
- `prepare_offline_download(track_id: Uuid) -> Result<DownloadManifest>`: Prepares content for offline use
- `get_recommended_tracks(user_id: Option<Uuid>) -> Result<Vec<Track>>`: Personalized recommendations

### SocialService
- `add_timestamped_comment(track_id: Uuid, timestamp_ms: u64, content: String) -> Result<TimestampedComment>`
- `like_track(track_id: Uuid, user_id: Uuid) -> Result<()>`
- `repost_track(track_id: Uuid, user_id: Uuid, comment: Option<String>) -> Result<()>`
- `follow_artist(artist_id: Uuid, user_id: Uuid) -> Result<()>`
- `get_comments_for_track(track_id: Uuid, range: Option<(u64, u64)>) -> Result<Vec<TimestampedComment>>`
- `get_social_feed(user_id: Uuid, cursor: Option<DateTime<Utc>>) -> Result<SocialFeed>`

### VisualizerService
- `generate_waveform_data(track_id: Uuid) -> Result<WaveformData>`: Processes audio to extract waveform
- `get_visualizer_presets() -> Result<Vec<VisualizerPreset>>`: Lists available visualizer configurations
- `apply_visualizer_preset(track_id: Uuid, preset_id: Uuid) -> Result<()>`: Sets default preset

### CacheService
- `initiate_download(track_id: Uuid, user_id: Uuid) -> Result<DownloadStatus>`
- `get_download_status(track_id: Uuid, user_id: Uuid) -> Result<DownloadStatus>`
- `list_available_offline_tracks(user_id: Uuid) -> Result<Vec<Track>>`
- `purge_old_downloads(user_id: Uuid) -> Result<()>`: Manages storage space

## 4. Key Design Decisions

### Timestamped Comments Implementation
We store timestamped comments in a dedicated table with the following schema:

```sql
CREATE TABLE track_comments (
    id UUID PRIMARY KEY,
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    timestamp_ms BIGINT NOT NULL CHECK (timestamp_ms >= 0),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_track_comments_track ON track_comments(track_id);
CREATE INDEX idx_track_comments_timestamp ON track_comments(track_id, timestamp_ms);
```

This design:
- Ensures efficient querying of comments within specific time ranges
- Supports social features like "reposting with comment at timestamp"
- Allows for opt-in anonymized data collection for improving recommendations

### P2P Streaming Architecture
We implement a hybrid streaming approach using p2panda:

1. **Content Addressing**: Each track is stored using content addressing via p2panda/iRoh
2. **Streaming Protocol**: 
   - Initial segment served from central node for reliability
   - Subsequent segments streamed peer-to-peer when available
   - Fallback to central node if peer connection fails

3. **Implementation**:
```rust
// infrastructure/p2p.rs

pub struct P2PStreamManager {
    node: Arc<p2panda::Node>,
    stun_servers: Vec<String>,
}

impl P2PStreamManager {
    pub async fn get_stream_segment(
        &self, 
        track_cid: &str, 
        segment_id: u32
    ) -> Result<Bytes> {
        // Try peer-to-peer first
        if let Ok(data) = self.try_p2p_fetch(track_cid, segment_id).await {
            return Ok(data);
        }
        
        // Fall back to central storage
        self.fallback_to_central(track_cid, segment_id).await
    }
}
```

### Audio Processing Pipeline

1. **Client-side Processing**:
   - WebAssembly module for audio processing
   - Only AV1 for video, Opus for audio, WebM container
   - No proprietary codec dependencies

2. **Workflow**:
   - Upload: User uploads audio in any format
   - Processing: Server converts to WebM (AV1/Opus) via ffmpeg.wasm
   - Storage: Content-addressed storage using p2panda
   - Streaming: Progressive streaming with adaptive bitrate

3. **Visualizer Data Generation**:
   - Extract waveform data during processing
   - Store as separate content-addressed resource
   - Provides amplitude data at regular intervals for visualization

### Visualizer Data Requirements
To support the Bevy-based visualizer, the backend must provide:

1. **Waveform Data**:
```json
{
  "sample_rate": 100,
  "duration_ms": 180000,
  "amplitudes": [0.1, 0.15, 0.2, ...]
}
```

2. **Frequency Analysis** (optional for advanced visualizers):
```json
{
  "sample_rate": 24,
  "bands": ["bass", "mid", "treble"],
  "frames": [
    {"timestamp_ms": 0, "values": [0.3, 0.2, 0.1]},
    {"timestamp_ms": 1000, "values": [0.4, 0.3, 0.2]},
    ...
  ]
}
```

3. **Metadata Endpoint**:
```
GET /api/music-player/tracks/{id}/visualizer-data
```

### Download/Caching Strategy
We implement an intelligent offline caching system:

1. **Content Manifest**:
   - Tracks are downloaded as content-addressed packages
   - Manifest includes track metadata, audio, and visualizer data
   - Manifests are signed for integrity verification

2. **Storage Management**:
   - LRU (Least Recently Used) eviction policy
   - Configurable storage limits per user
   - Automatic cleanup of unused content

3. **Privacy Considerations**:
   - All offline content is encrypted with user-specific key
   - No tracking of offline listening habits without explicit consent
   - Clear opt-in for usage data collection

## 5. GraphQL API Integration

The module integrates with our existing GraphQL API through these endpoints:

### Queries
- `track(id: ID!): Track` - Get track details
- `tracksByArtist(artistId: ID!): [Track!]!` - List tracks by artist
- `commentsForTrack(trackId: ID!, timestampRange: TimeRangeInput): [TimestampedComment!]!`
- `visualizerPresets: [VisualizerPreset!]!` - List available presets
- `offlineTracks: [Track!]!` - List available offline tracks

### Mutations
- `playTrack(trackId: ID!, positionMs: Int = 0): PlaySession!`
- `addTimestampedComment(trackId: ID!, timestampMs: Int!, content: String!): TimestampedComment!`
- `likeTrack(trackId: ID!): Boolean!`
- `repostTrack(trackId: ID!, comment: String): Boolean!`
- `prepareOfflineDownload(trackId: ID!): DownloadManifest!`
- `applyVisualizerPreset(trackId: ID!, presetId: ID!): Boolean!`

### Subscriptions
- `trackPlayed(trackId: ID!): TrackPlayedEvent` - For social feed updates
- `commentAdded(trackId: ID!): TimestampedComment` - Real-time comments
- `downloadProgress(trackId: ID!): DownloadProgress` - Offline download status

The implementation follows the module wiring pattern:

```rust
// web/module.rs

pub struct MusicPlayerModule {
    pub router: Router,
    pub query: MusicPlayerQuery,
    pub mutation: MusicPlayerMutation,
    pub subscription: MusicPlayerSubscription,
}

pub fn initialize(db_pool: PgPool) -> MusicPlayerModule {
    let streaming_service = Arc::new(StreamingService::new(db_pool.clone()));
    let social_service = Arc::new(SocialService::new(db_pool.clone()));
    let visualizer_service = Arc::new(VisualizerService::new());
    let cache_service = Arc::new(CacheService::new(db_pool.clone()));
    
    MusicPlayerModule {
        router: create_music_player_router(
            streaming_service.clone(),
            social_service.clone(),
            cache_service.clone()
        ),
        query: MusicPlayerQuery::new(
            streaming_service.clone(),
            social_service.clone(),
            visualizer_service.clone()
        ),
        mutation: MusicPlayerMutation::new(
            streaming_service.clone(),
            social_service.clone(),
            visualizer_service.clone(),
            cache_service.clone()
        ),
        subscription: MusicPlayerSubscription::new(
            social_service.clone(),
            cache_service.clone()
        ),
    }
}
```

## 6. Database Schema

### Primary Tables

```sql
CREATE TABLE tracks (
    id UUID PRIMARY KEY,
    artist_id UUID NOT NULL REFERENCES cooperative_members(id),
    album_id UUID REFERENCES albums(id),
    title VARCHAR(255) NOT NULL,
    duration_ms BIGINT NOT NULL CHECK (duration_ms > 0),
    media_cid VARCHAR(100) NOT NULL,
    waveform_data_cid VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE track_comments (
    id UUID PRIMARY KEY,
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    timestamp_ms BIGINT NOT NULL CHECK (timestamp_ms >= 0),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_track_comments_track ON track_comments(track_id);
CREATE INDEX idx_track_comments_timestamp ON track_comments(track_id, timestamp_ms);

CREATE TABLE track_likes (
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (track_id, user_id)
);

CREATE TABLE artist_follows (
    artist_id UUID NOT NULL REFERENCES cooperative_members(id),
    follower_id UUID NOT NULL REFERENCES cooperative_members(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (artist_id, follower_id)
);

CREATE TABLE playlists (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES cooperative_members(id),
    title VARCHAR(100) NOT NULL,
    description TEXT,
    is_public BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE playlist_tracks (
    playlist_id UUID NOT NULL REFERENCES playlists(id),
    track_id UUID NOT NULL REFERENCES tracks(id),
    position INTEGER NOT NULL,
    PRIMARY KEY (playlist_id, track_id)
);

CREATE TABLE visualizer_presets (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    config JSONB NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE offline_downloads (
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    download_manifest JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    PRIMARY KEY (track_id, user_id)
);
```

## 7. Dependencies

| Dependency | Purpose |
|------------|---------|
| `cpc-core` | Access to cooperative member models and authentication |
| `cpc-net` | p2panda integration for distributed streaming |
| `cpc-protos` | Shared gRPC definitions for worker communication |
| `ffmpeg-wasm` | Client-side audio processing with royalty-free codecs |
| `plotters` | Waveform visualization data generation |
| `sqlx` | Database access (already in backend dependencies) |
| `tracing` | Structured logging |

## 8. Integration with Other Modules

### Website Builder Integration
The music player integrates with the website builder module through:

1. **Embeddable Player Component**:
   - Provides a React/Vue component for embedding in websites
   - Configurable appearance and behavior
   - Tracks interactions for analytics

2. **API Endpoints**:
   ```
   GET /api/website-builder/embed/music-player
   Content-Type: application/javascript
   
   // Returns embeddable player script
   ```

3. **Template Integration**:
   - Music player blocks can be added to website templates
   - Artists can showcase their work directly on their sites

### Social Module Integration
- Track likes and reposts appear in social feeds
- Following artists creates social connections
- Timestamped comments can be shared to the main feed

## 9. Privacy and Cooperative Values Implementation

For more information about our privacy policies and consent management, see our [Privacy Policy](../../docs/privacy_policy.md).

For information about our Android integration architecture, see [Android Architecture](../../docs/android_architecture.md).

This module was designed with our cooperative values in mind:

1. **User Privacy**:
   - All data collection is opt-in with clear consent
   - Anonymized usage data only collected with explicit permission
   - No third-party tracking

2. **Offline Capabilities**:
   - Full offline playback support
   - Intelligent caching without excessive storage use
   - Works in low-connectivity environments

3. **Community Curation**:
   - User-generated playlists can be shared publicly
   - Artist verification process managed by community
   - Transparent recommendation algorithms

4. **Freedom from Vendor Lock-in**:
   - All content stored in open, royalty-free formats
   - Easy export of user data
   - No proprietary formats or DRM

This architecture provides a robust foundation for a music streaming platform that respects user autonomy while delivering rich functionality. The hexagonal structure ensures core business logic remains independent of implementation details, allowing for flexible evolution of the system while maintaining compatibility with our modular architecture.

Free Palestine! ✊