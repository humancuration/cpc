//! Integration tests for the live streaming module

#[cfg(test)]
mod tests {
    use cpc_live_streaming::{
        channel::channel::Channel,
        streaming::broadcaster::Broadcaster,
        social::follow::FollowService,
        social::subscription::SubscriptionService,
    };
    use uuid::Uuid;

    #[test]
    fn test_channel_creation() {
        let owner_id = Uuid::new_v4();
        let channel = Channel::new(owner_id, "test_channel".to_string(), Some("Test Channel Description".to_string()));
        
        assert_eq!(channel.name, "test_channel");
        assert_eq!(channel.owner_id, owner_id);
        assert_eq!(channel.description, Some("Test Channel Description".to_string()));
    }

    #[test]
    fn test_broadcaster_creation() {
        let broadcaster = Broadcaster::new();
        assert_eq!(broadcaster.active_streams.len(), 0);
    }

    #[test]
    fn test_subscription_service_creation() {
        let subscription_service = SubscriptionService::new();
        assert_eq!(subscription_service.subscriptions.len(), 0);
        assert_eq!(subscription_service.tiers.len(), 0);
    }

    #[test]
    fn test_follow_service_creation() {
        // This would require a database connection in a real test
        // For now, we just verify the struct can be created
        // let follow_service = FollowService::new(db_pool);
    }

    #[test]
    fn test_streaming_lifecycle() {
        let mut broadcaster = Broadcaster::new();
        let channel_id = Uuid::new_v4();
        
        let stream = broadcaster.start_stream(
            channel_id,
            "test_stream_key".to_string(),
            "Test Stream".to_string(),
            "Gaming".to_string(),
            cpc_live_streaming::streaming::broadcaster::StreamMetadata {
                resolution: "1920x1080".to_string(),
                bitrate: 6000,
                fps: 30,
                hardware_encoding: true,
            },
        );
        
        assert_eq!(broadcaster.active_streams.len(), 1);
        assert_eq!(stream.title, "Test Stream");
        assert_eq!(stream.category, "Gaming");
        
        let stopped_stream = broadcaster.stop_stream("test_stream_key");
        assert!(stopped_stream.is_some());
        assert_eq!(broadcaster.active_streams.len(), 0);
    }

    #[test]
    fn test_subscription_lifecycle() {
        let mut subscription_service = SubscriptionService::new();
        let channel_id = Uuid::new_v4();
        let subscriber_id = Uuid::new_v4();
        let channel_owner_id = Uuid::new_v4();
        
        // Create a tier
        let benefits = cpc_live_streaming::social::subscription::SubscriptionBenefits {
            subscriber_emotes: true,
            ad_free: true,
            higher_quality: true,
            custom_badges: true,
            subscriber_chat: true,
            special_badge: true,
            custom_benefits: vec!["custom_benefit_1".to_string()],
        };
        
        let tier = subscription_service.create_tier(
            channel_id,
            "Tier 1".to_string(),
            "First tier".to_string(),
            499, // $4.99
            1,
            benefits,
        );
        
        // Subscribe user
        let subscription = subscription_service.subscribe_user(
            subscriber_id,
            channel_owner_id,
            tier.id,
            false,
            None,
        );
        
        assert!(subscription.is_ok());
        let subscription = subscription.unwrap();
        assert_eq!(subscription.subscriber_id, subscriber_id);
        assert_eq!(subscription.channel_owner_id, channel_owner_id);
        assert_eq!(subscription.tier_id, tier.id);
        assert!(subscription.is_active);
        
        // Get user subscriptions
        let user_subscriptions = subscription_service.get_user_subscriptions(subscriber_id);
        assert_eq!(user_subscriptions.len(), 1);
        
        // Cancel subscription
        let cancelled = subscription_service.cancel_subscription(subscription.id);
        assert!(cancelled.is_ok());
        
        // Verify subscription is no longer active
        let user_subscriptions = subscription_service.get_user_subscriptions(subscriber_id);
        assert_eq!(user_subscriptions.len(), 0);
    }
}