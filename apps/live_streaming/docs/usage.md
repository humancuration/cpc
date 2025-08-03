# Usage Guide

This document provides guidance on how to use the Live Streaming module.

## Getting Started

To use the Live Streaming module, you need to:

1. Set up a PostgreSQL database
2. Run the database migrations
3. Configure the module with your database connection
4. Initialize the services you need

## Channel Management

### Creating a Channel

```rust
use cpc_live_streaming::channel::manager::ChannelManager;
use sqlx::PgPool;
use uuid::Uuid;

async fn create_channel_example() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgresql://localhost/live_streaming").await?;
    let mut channel_manager = ChannelManager::new(pool);
    
    let owner_id = Uuid::new_v4();
    let channel = channel_manager.create_channel(
        owner_id,
        "my_channel".to_string(),
        Some("My awesome streaming channel".to_string())
    ).await?;
    
    println!("Created channel: {}", channel.name);
    Ok(())
}
```

### Updating Channel Information

```rust
async fn update_channel_example(channel_manager: &mut ChannelManager, channel_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
    channel_manager.update_channel_info(
        channel_id,
        Some("new_channel_name".to_string()),
        Some("Updated channel description".to_string())
    ).await?;
    
    println!("Channel updated successfully");
    Ok(())
}
```

## Streaming

### Starting a Stream

```rust
use cpc_live_streaming::streaming::broadcaster::Broadcaster;

fn start_stream_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut broadcaster = Broadcaster::new();
    
    let stream = broadcaster.start_stream(
        channel_id,
        "unique_stream_key".to_string(),
        "Playing a cool game".to_string(),
        "Gaming".to_string(),
        cpc_live_streaming::streaming::broadcaster::StreamMetadata {
            resolution: "1920x1080".to_string(),
            bitrate: 6000,
            fps: 60,
            hardware_encoding: true,
        },
    );
    
    println!("Started stream: {}", stream.title);
    Ok(())
}
```

### Viewing a Stream

```rust
use cpc_live_streaming::streaming::viewer::Viewer;
use uuid::Uuid;

fn view_stream_example() -> Result<(), Box<dyn std::error::Error>> {
    let viewer_id = Uuid::new_v4();
    let mut viewer = Viewer::new(viewer_id);
    
    // Start watching a stream
    viewer.start_watching(stream_id);
    
    println!("Viewer is now watching stream");
    Ok(())
}
```

## Social Features

### Following a Channel

```rust
use cpc_live_streaming::social::follow::FollowService;
use sqlx::PgPool;
use uuid::Uuid;

async fn follow_channel_example() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPool::connect("postgresql://localhost/live_streaming").await?;
    let follow_service = FollowService::new(pool);
    
    follow_service.follow_channel(follower_id, channel_owner_id).await?;
    
    println!("Successfully followed channel");
    Ok(())
}
```

### Subscribing to a Channel

```rust
use cpc_live_streaming::social::subscription::{SubscriptionService, SubscriptionBenefits};

fn subscribe_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut subscription_service = SubscriptionService::new();
    
    // Create subscription benefits
    let benefits = SubscriptionBenefits {
        subscriber_emotes: true,
        ad_free: true,
        higher_quality: true,
        custom_badges: true,
        subscriber_chat: true,
        special_badge: true,
        custom_benefits: vec!["Early access to VODs".to_string()],
    };
    
    // Create a subscription tier
    let tier = subscription_service.create_tier(
        channel_id,
        "Tier 1".to_string(),
        "Basic subscription".to_string(),
        499, // $4.99
        1,
        benefits,
    );
    
    // Subscribe user to channel
    let subscription = subscription_service.subscribe_user(
        subscriber_id,
        channel_owner_id,
        tier.id,
        false,
        None,
    )?;
    
    println!("User subscribed to channel with tier: {}", tier.name);
    Ok(())
}
```

## Media Processing

### Transcoding a Stream

```rust
use cpc_live_streaming::media_processing::transcoder::Transcoder;
use cpc_live_streaming::media_processing::transcoder::MediaFormat;

fn transcode_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut transcoder = Transcoder::new();
    
    let input_format = MediaFormat {
        codec: "h264".to_string(),
        width: 1920,
        height: 1080,
        fps: 30.0,
        bitrate_kbps: 6000,
        audio_codec: "aac".to_string(),
        audio_bitrate_kbps: 128,
        audio_sample_rate: 44100,
    };
    
    let output_format = Transcoder::create_webm_av1_format(1920, 1080, 6000);
    
    let job = transcoder.start_transcoding_job(
        "stream_key_123".to_string(),
        input_format,
        output_format,
    );
    
    println!("Started transcoding job: {}", job.id);
    Ok(())
}
```

## Web Integration

### Setting up HTTP Routes

```rust
use cpc_live_streaming::web::routes::create_live_streaming_router;
use axum::Router;

fn setup_routes_example() -> Router {
    let router = create_live_streaming_router();
    router
}
```

### GraphQL Integration

```rust
use cpc_live_streaming::web::graphql::create_schema;
use async_graphql::Schema;

fn setup_graphql_example() -> Schema<impl async_graphql::ObjectType, impl async_graphql::ObjectType, impl async_graphql::SubscriptionType> {
    let schema = create_schema();
    schema
}
```

## Testing

To run the tests for the Live Streaming module:

```bash
cd apps/live_streaming
cargo test
```

To run a specific test:

```bash
cargo test test_channel_creation
```

## Examples

The module includes several examples in the `examples/` directory:

- `basic_usage.rs`: Basic usage of the module
- `streaming_example.rs`: Example of streaming functionality
- `social_features.rs`: Example of social features

To run an example:

```bash
cd apps/live_streaming
cargo run --example basic_usage