//! Basic usage example for the live streaming module

use cpc_live_streaming::{
    channel::channel::Channel,
    channel::manager::ChannelManager,
    streaming::broadcaster::Broadcaster,
    streaming::viewer::Viewer,
    social::follow::FollowService,
    social::subscription::{SubscriptionService, SubscriptionBenefits},
};
use sqlx::PgPool;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Live Streaming Module - Basic Usage Example");
    println!("==========================================");
    
    // Create a channel manager
    // Note: In a real application, you would connect to a real database
    // For this example, we'll just create a mock pool
    let pool = PgPool::connect("postgresql://localhost/test").await?;
    let mut channel_manager = ChannelManager::new(pool);
    
    // Create a broadcaster
    let mut broadcaster = Broadcaster::new();
    
    // Create a subscription service
    let mut subscription_service = SubscriptionService::new();
    
    // Create a follow service
    // let follow_service = FollowService::new(pool);
    
    // 1. Create a channel
    println!("\n1. Creating a channel...");
    let owner_id = Uuid::new_v4();
    let channel = channel_manager.create_channel(
        owner_id,
        "gamer_pro".to_string(),
        Some("Professional gaming content".to_string())
    ).await?;
    
    println!("   Created channel: {} (ID: {})", channel.name, channel.id);
    
    // 2. Create subscription tiers
    println!("\n2. Creating subscription tiers...");
    let tier_benefits = SubscriptionBenefits {
        subscriber_emotes: true,
        ad_free: true,
        higher_quality: true,
        custom_badges: true,
        subscriber_chat: true,
        special_badge: true,
        custom_benefits: vec!["Early access to VODs".to_string()],
    };
    
    let tier1 = subscription_service.create_tier(
        channel.id,
        "Tier 1".to_string(),
        "Basic subscription".to_string(),
        499, // $4.99
        1,
        tier_benefits,
    );
    
    println!("   Created subscription tier: {} (ID: {})", tier1.name, tier1.id);
    
    // 3. Start a stream
    println!("\n3. Starting a stream...");
    let stream = broadcaster.start_stream(
        channel.id,
        "stream_key_12345".to_string(),
        "Playing the latest RPG game".to_string(),
        "RPG".to_string(),
        cpc_live_streaming::streaming::broadcaster::StreamMetadata {
            resolution: "1920x1080".to_string(),
            bitrate: 6000,
            fps: 60,
            hardware_encoding: true,
        },
    );
    
    println!("   Started stream: {} (ID: {})", stream.title, stream.id);
    println!("   Stream key: {}", stream.stream_key);
    println!("   Category: {}", stream.category);
    
    // 4. Create a viewer
    println!("\n4. Creating a viewer...");
    let viewer_id = Uuid::new_v4();
    let mut viewer = Viewer::new(viewer_id);
    
    // Viewer starts watching the stream
    viewer.start_watching(stream.id);
    println!("   Viewer (ID: {}) is now watching stream {}", viewer_id, stream.id);
    
    // 5. Subscribe to the channel
    println!("\n5. Subscribing to channel...");
    let subscription = subscription_service.subscribe_user(
        viewer_id,
        owner_id,
        tier1.id,
        false,
        None,
    )?;
    
    println!("   User {} subscribed to channel {} with tier {}", 
             subscription.subscriber_id, 
             subscription.channel_owner_id, 
             subscription.tier_id);
    
    // 6. Update viewer count
    println!("\n6. Updating viewer count...");
    broadcaster.update_viewer_count(&stream.stream_key, 1)?;
    println!("   Stream now has 1 viewer");
    
    // 7. Stop the stream
    println!("\n7. Stopping the stream...");
    let stopped_stream = broadcaster.stop_stream(&stream.stream_key);
    if let Some(stream) = stopped_stream {
        println!("   Stopped stream: {}", stream.title);
    }
    
    println!("\nExample completed successfully!");
    
    Ok(())
}