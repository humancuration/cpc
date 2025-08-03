//! Integration demo showing how the live streaming module integrates with other CPC services

use cpc_live_streaming::{StreamEventService, StreamNotificationService};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Live Streaming Integration Demo");
    println!("==============================");
    
    // Initialize services
    let stream_event_service = StreamEventService::new();
    let stream_notification_service = StreamNotificationService::new();
    
    // Simulate a stream started event
    let user_id = Uuid::new_v4();
    let stream_id = Uuid::new_v4();
    
    println!("Simulating stream started event...");
    stream_event_service.handle_stream_started(user_id, stream_id).await?;
    
    // Simulate sending notifications to followers
    println!("Sending stream started notifications...");
    let results = stream_notification_service
        .send_stream_started_notification(&user_id.to_string(), &stream_id.to_string())
        .await?;
    
    println!("Sent {} notifications", results.len());
    
    // Simulate a viewer joining
    let viewer_id = Uuid::new_v4();
    stream_event_service.handle_viewer_joined(viewer_id, stream_id).await?;
    
    // Simulate a chat message
    let message_id = Uuid::new_v4();
    stream_event_service.handle_chat_message_sent(viewer_id, stream_id, message_id).await?;
    
    // Simulate a subscription
    let channel_id = Uuid::new_v4();
    stream_event_service.handle_subscription_created(
        viewer_id, 
        channel_id, 
        cpc_social_integration::domain::social_event::SubscriptionTier::Tier1
    ).await?;
    
    println!("All integration events simulated successfully!");
    
    Ok(())
}