#[cfg(test)]
mod tests {
    use realtime_signaling::message::{
        SignalingMessage, PresenceUpdate, PresenceSummary, PresenceUser, PresenceStatus
    };
    use realtime_signaling::redis_signaling::RedisSignalingService;
    use uuid::Uuid;
    use chrono::Utc;
    use tokio;

    // Note: These tests require a running Redis instance
    // They are marked as ignored by default to avoid requiring Redis for regular tests
    #[tokio::test]
    #[ignore]
    async fn test_redis_signaling_handles_presence_messages() {
        let service = RedisSignalingService::new("redis://127.0.0.1/", None).await.unwrap();
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Register connection
        let _receiver = service.register_connection(document_id).await.unwrap();
        
        // Create presence update
        let update = PresenceUpdate {
            document_id,
            user_id,
            cursor: None,
            selection: None,
            is_typing: false,
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#00ff00".to_string(),
            last_active: Utc::now(),
            timestamp: Utc::now(),
        };
        
        let message = SignalingMessage::PresenceUpdate(update);
        
        // Handle the message
        service.handle_message(document_id, message).await.unwrap();
        
        // Test presence summary creation
        let summary = service.create_presence_summary(document_id).await;
        assert!(summary.users.contains_key(&user_id));
    }

    #[tokio::test]
    #[ignore]
    async fn test_redis_signaling_presence_summary_broadcast() {
        let service = RedisSignalingService::new("redis://127.0.0.1/", None).await.unwrap();
        let document_id = Uuid::new_v4();
        
        // Register connection
        let mut receiver = service.register_connection(document_id).await.unwrap();
        
        // Create presence summary
        let mut users = std::collections::HashMap::new();
        let user_id = Uuid::new_v4();
        users.insert(user_id, PresenceUser {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            status: PresenceStatus::Online,
        });

        let summary = PresenceSummary {
            users,
            expires_at: Utc::now(),
        };
        
        let message = SignalingMessage::PresenceSummary(summary);
        
        // Broadcast the message
        service.broadcast_message(document_id, &message).await.unwrap();
        
        // Try to receive the message
        let received = tokio::time::timeout(tokio::time::Duration::from_millis(100), receiver.recv()).await;
        assert!(received.is_ok());
    }
}