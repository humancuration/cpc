# Music Player Module

Full-featured music streaming platform with social features including timestamped comments, visualizers, and offline playback.

## Overview

This module implements a complete music streaming solution with:

- P2P content delivery using p2panda/iRoh
- Social features (likes, reposts, timestamped comments)
- Visualizer support with customizable presets
- Offline playback with intelligent caching
- GraphQL API for frontend integration
- REST API for direct media access

## Features

### Core Streaming
- Content-addressed storage for all media
- Progressive streaming with adaptive bitrate
- P2P delivery with central fallback
- Royalty-free codec support (AV1/Opus/WebM)

### Social Features
- Timestamped comments at specific points in tracks
- Likes and reposts with optional comments
- Artist following system
- Social feed with friend activity

### Visualizers
- Waveform visualization data generation
- Frequency analysis for advanced visualizers
- Customizable visualizer presets
- Bevy integration for rich visualizations

### Offline Capabilities
- Intelligent caching with LRU eviction
- Configurable storage limits
- Encrypted offline content
- Download progress tracking

## Architecture

This module follows a hexagonal architecture pattern with clear separation of concerns:

```
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
│   ├── audio_processor.rs  # ffmpeg.wasm integration
│   └── media_store.rs      # Content-addressed storage
└── web/                    # Adapter layer
    ├── routes.rs           # REST API routes
    ├── graphql.rs          # GraphQL definitions
    ├── module.rs           # Module initialization & wiring
    └── types.rs            # GraphQL input/output types
```

## API Endpoints

### GraphQL

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed GraphQL schema.

### REST

- `GET /api/music-player/tracks/:track_id/stream` - Stream a track
- `GET /api/music-player/tracks/:track_id/visualizer-data` - Get visualizer data
- `GET /api/music-player/tracks/search` - Search tracks
- `POST /api/music-player/tracks/:track_id/comments` - Add comment
- `GET /api/music-player/tracks/:track_id/comments` - Get comments
- `POST /api/music-player/tracks/:track_id/like` - Like track
- `POST /api/music-player/tracks/:track_id/repost` - Repost track
- `POST /api/music-player/artists/:artist_id/follow` - Follow artist
- `POST /api/music-player/tracks/:track_id/download` - Initiate download
- `GET /api/music-player/tracks/:track_id/download-status` - Get download status
- `GET /api/music-player/offline/tracks` - List offline tracks

## Database Schema

See [migrations/20250727000000_music_player_schema.sql](migrations/20250727000000_music_player_schema.sql) for the complete schema.

## Dependencies

- `cpc-core` - Access to cooperative member models and authentication
- `cpc-net` - p2panda integration for distributed streaming
- `cpc-protos` - Shared gRPC definitions for worker communication
- `ffmpeg-wasm` - Client-side audio processing with royalty-free codecs
- `plotters` - Waveform visualization data generation
- `sqlx` - Database access
- `tracing` - Structured logging

## Privacy and Cooperative Values

For more information about our privacy policies and consent management, see our [Privacy Policy](../../docs/privacy_policy.md).

For information about our Android integration architecture, see [Android Architecture](../../docs/android_architecture.md).

This module was designed with cooperative values in mind:

- All data collection is opt-in with clear consent
- Anonymized usage data only collected with explicit permission
- No third-party tracking
- Full offline playback support
- Intelligent caching without excessive storage use
- User-generated playlists can be shared publicly
- Transparent recommendation algorithms
- All content stored in open, royalty-free formats
- Easy export of user data

## Integration

This module integrates with:

- **Backend**: Dynamic module system via module registry
- **Frontend**: GraphQL API for web/mobile clients
- **P2P Network**: p2panda for content distribution
- **Media Processing**: ffmpeg.wasm for audio processing

## Development

To build and run:

```bash
cd apps/music-player
cargo build
cargo run
```

To test:

```bash
cargo test
```

## License

This module is licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

Free Palestine! ✊