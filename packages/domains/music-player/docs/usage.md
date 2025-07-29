# Music Player Module Usage Guide

This guide explains how to use the music player module in your application.

## Module Integration

To integrate the music player module into your application:

1. Add the module to your dependencies:
```toml
[dependencies]
cpc-music-player = { path = "../apps/music-player" }
```

2. Initialize the module in your application:
```rust
use cpc_music_player::web::modular_module::ModularMusicPlayer;
use sqlx::PgPool;

// Create a database pool
let pool = PgPool::connect("postgresql://localhost/music_player").await?;

// Create the module
let mut music_player_module = ModularMusicPlayer::new(pool);

// Enable the module
music_player_module.enable().await?;
```

## Using Services Directly

You can also use the individual services directly without the modular system:

```rust
use cpc_music_player::{
    application::streaming_service::StreamingService,
    infrastructure::database::TrackRepository,
    infrastructure::p2p::P2PStreamManager,
    infrastructure::audio_processor::AudioProcessor,
};
use std::sync::Arc;

// Initialize repositories and infrastructure
let track_repository = Arc::new(TrackRepository::new(db_pool.clone()));
let p2p_manager = Arc::new(P2PStreamManager::new(vec![
    "stun.l.google.com:19302".to_string(),
])?);
let audio_processor = Arc::new(AudioProcessor::new());

// Initialize services
let streaming_service = StreamingService::new(
    track_repository,
    p2p_manager,
    audio_processor,
);

// Use the service
let track = streaming_service.get_track(track_id).await?;
```

## GraphQL Integration

To integrate with the GraphQL schema:

```rust
use cpc_music_player::web::graphql::{
    MusicPlayerQuery, 
    MusicPlayerMutation, 
    MusicPlayerSubscription
};

// Add to your GraphQL schema
let schema = Schema::build(
    MusicPlayerQuery, 
    MusicPlayerMutation, 
    MusicPlayerSubscription
).finish();
```

## REST API Integration

The module automatically provides REST API routes when initialized:

```rust
use cpc_music_player::web::module::initialize;
use axum::Router;

// Initialize the module
let music_player_module = initialize(db_pool);

// Merge the router with your application router
let app = Router::new()
    .merge(music_player_module.router);
```

## Configuration

The module can be configured through environment variables:

- `MUSIC_PLAYER_P2P_STUN_SERVERS` - Comma-separated list of STUN servers
- `MUSIC_PLAYER_STORAGE_LIMIT_MB` - Default storage limit for offline downloads
- `MUSIC_PLAYER_AUTO_DOWNLOAD_WIFI` - Whether to auto-download on WiFi

## Error Handling

All services return `Result` types with specific error variants:

```rust
use cpc_music_player::domain::errors::MusicPlayerError;

match streaming_service.get_track(track_id).await {
    Ok(track) => {
        // Handle successful track retrieval
    }
    Err(MusicPlayerError::TrackNotFound { id }) => {
        // Handle track not found
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Testing

To test the module, run:

```bash
cd apps/music-player
cargo test
```

For integration tests with a real database, you'll need to set up a test database:

```bash
# Set up test database
createdb music_player_test

# Run tests with database
cargo test --features database-tests
```

## Extending the Module

To extend the module with new features:

1. Add new models to `domain/models.rs`
2. Create new services in `application/`
3. Implement repository methods in `infrastructure/database.rs`
4. Add GraphQL types and resolvers in `web/graphql.rs`
5. Add REST endpoints in `web/routes.rs`

## Performance Considerations

- Use connection pooling for database connections
- Cache frequently accessed data
- Use asynchronous operations for I/O bound tasks
- Consider using a CDN for static assets
- Implement proper error handling and logging

## Security Considerations

- Validate all user inputs
- Use prepared statements to prevent SQL injection
- Implement proper authentication and authorization
- Encrypt sensitive data at rest
- Use HTTPS for all network communications
- Implement rate limiting to prevent abuse