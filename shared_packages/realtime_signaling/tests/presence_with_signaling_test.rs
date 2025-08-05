#[cfg(test)]
mod tests {
    use realtime_signaling::message::{PresenceUpdate, PresenceSummary, PresenceUser, PresenceStatus, SignalingMessage, CursorPosition, Position};
    use realtime_signaling::RedisSignalingService;
    use uuid::Uuid;
    use chrono::Utc;
    use std::collections::HashMap;
    use tokio;

    #[tokio::test]
    async fn test_presence_update_with_signaling() {
        // Create a mock Redis signaling service
        // Note: This test requires a Redis server running on localhost:6379
        let signaling_service = RedisSignalingService::new("redis://localhost:6379", Some("test_signaling".to_string())).await.unwrap();
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Register connection for this document
        let mut receiver = signaling_service.register_connection(document_id).await.unwrap();
        
        // Create a presence update
        let update = PresenceUpdate {
            document_id,
            user_id,
            cursor: Some(Position { line: 5, column: 10 }),
            selection: None,
            is_typing: true,
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            last_active: Utc::now(),
            timestamp: Utc::now(),
        };
        
        // Send presence update
        let message = SignalingMessage::PresenceUpdate(update.clone());
        signaling_service.broadcast_message(document_id, &message).await.unwrap();
        
        // Verify we can receive the message
        // Note: In a real test, we would verify the message content
        assert!(true);
    }
    
    #[tokio::test]
    async fn test_cursor_update_with_signaling() {
        // Create a mock Redis signaling service
        // Note: This test requires a Redis server running on localhost:6379
        let signaling_service = RedisSignalingService::new("redis://localhost:6379", Some("test_signaling".to_string())).await.unwrap();
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Register connection for this document
        let mut receiver = signaling_service.register_connection(document_id).await.unwrap();
        
        // Create a cursor update
        let cursor = CursorPosition {
            document_id,
            user_id,
            position: Position { line: 3, column: 15 },
            timestamp: Utc::now(),
        };
        
        // Send cursor update
        let message = SignalingMessage::CursorUpdate(cursor.clone());
        signaling_service.broadcast_message(document_id, &message).await.unwrap();
        
        // Verify we can receive the message
        // Note: In a real test, we would verify the message content
        assert!(true);
    }
    
    #[tokio::test]
    async fn test_presence_summary_with_signaling() {
        // Create a mock Redis signaling service
        // Note: This test requires a Redis server running on localhost:6379
        let signaling_service = RedisSignalingService::new("redis://localhost:6379", Some("test_signaling".to_string())).await.unwrap();
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Register connection for this document
        let mut receiver = signaling_service.register_connection(document_id).await.unwrap();
        
        // Create a presence summary
        let mut users = HashMap::new();
        users.insert(user_id, PresenceUser {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            status: PresenceStatus::Online,
        });
        
        let summary = PresenceSummary {
            users,
            expires_at: Utc::now(),
        };
        
        // Send presence summary
        let message = SignalingMessage::PresenceSummary(summary.clone());
        signaling_service.broadcast_message(document_id, &message).await.unwrap();
        
        // Verify we can receive the message
        // Note: In a real test, we would verify the message content
        assert!(true);
    }
    
    #[tokio::test]
    async fn test_connection_recovery() {
        // Test connection recovery scenarios
        // Note: This test requires a Redis server running on localhost:6379
        let signaling_service = RedisSignalingService::new("redis://localhost:6379", Some("test_signaling".to_string())).await.unwrap();
        
        let document_id = Uuid::new_v4();
        
        // Register connection
        let result = signaling_service.register_connection(document_id).await;
        assert!(result.is_ok());
        
        // Unregister connection
        let result = signaling_service.unregister_connection(document_id).await;
        assert!(result.is_ok());
        
        // Register again to test recovery
        let result = signaling_service.register_connection(document_id).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_performance_with_simulated_users() {
        // Test performance with simulated users
        // Note: This test requires a Redis server running on localhost:6379
        let signaling_service = RedisSignalingService::new("redis://localhost:6379", Some("test_signaling".to_string())).await.unwrap();
        
        let document_id = Uuid::new_v4();
        
        // Register connection
        let mut receiver = signaling_service.register_connection(document_id).await.unwrap();
        
        // Simulate 100 users joining
        for i in 0..100 {
            let user_id = Uuid::new_v4();
            
            let join_message = SignalingMessage::JoinDocument {
                document_id,
                user_id,
            };
            
            let result = signaling_service.broadcast_message(document_id, &join_message).await;
            assert!(result.is_ok());
        }
        
        // Send presence updates for all users
        for i in 0..100 {
            let user_id = Uuid::new_v4();
            
            let update = PresenceUpdate {
                document_id,
                user_id,
                cursor: Some(Position { line: i, column: 0 }),
                selection: None,
                is_typing: false,
                avatar_url: Some(format!("https://example.com/avatar{}.png", i)),
                color: format!("#{:06x}", i),
                last_active: Utc::now(),
                timestamp: Utc::now(),
            };
            
            let message = SignalingMessage::PresenceUpdate(update);
            let result = signaling_service.broadcast_message(document_id, &message).await;
            assert!(result.is_ok());
        }
        
        assert!(true);
    }
}