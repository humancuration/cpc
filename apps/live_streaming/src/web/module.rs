//! Module initialization and wiring for the live streaming service

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

use crate::channel::manager::ChannelManager;
use crate::chat::chat_service::ChatService;
use crate::social::follow::FollowService;
use crate::social::subscription::SubscriptionService;
use crate::streaming::broadcaster::Broadcaster;
use crate::web::routes::create_live_streaming_router;

// External service dependencies
use cpc_social_integration::StreamEventService;
use cpc_notification_core::StreamNotificationService;

/// This struct holds all the pieces the backend needs from this module
pub struct LiveStreamingModule {
    pub router: Router,
    pub stream_event_service: Arc<StreamEventService>,
    pub stream_notification_service: Arc<StreamNotificationService>,
    // In a real implementation, we would also have GraphQL schema components
}

/// This function initializes the module and its dependencies
pub fn initialize(db_pool: PgPool) -> LiveStreamingModule {
    // Initialize services
    let channel_manager = Arc::new(ChannelManager::new(db_pool.clone()));
    let chat_service = Arc::new(ChatService::new(db_pool.clone()));
    let follow_service = Arc::new(FollowService::new(db_pool.clone()));
    let subscription_service = Arc::new(SubscriptionService::new());
    let broadcaster = Arc::new(Broadcaster::new());
    
    // Initialize external services
    let stream_event_service = Arc::new(StreamEventService::new());
    let stream_notification_service = Arc::new(StreamNotificationService::new());
    
    // Initialize web components
    let router = create_live_streaming_router();
    
    LiveStreamingModule {
        router,
        stream_event_service,
        stream_notification_service,
    }
}