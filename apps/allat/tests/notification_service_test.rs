#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::notification_service::NotificationServiceImpl;
    use crate::domain::notification_events::NotificationEvent;
    use std::sync::Arc;
    
    // Mock notification core service
    struct MockNotificationCoreService;
    
    #[async_trait::async_trait]
    impl crate::application::notification_service::NotificationCoreService for MockNotificationCoreService {
        async fn send_notification(&self, _notification: crate::application::notification_service::CoreNotification) -> Result<(), crate::application::error::ApplicationError> {
            // Mock implementation
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_handle_post_reply_notification() {
        let core_service = Arc::new(MockNotificationCoreService);
        let service = NotificationServiceImpl::new(core_service);
        
        let event = NotificationEvent::PostReply {
            post_id: Uuid::new_v4(),
            post_title: "Test Post".to_string(),
            replier_id: Uuid::new_v4(),
            replier_name: "Test User".to_string(),
            community_id: Uuid::new_v4(),
            community_name: "Test Community".to_string(),
        };
        
        // This test would verify that the notification is properly formatted and sent
        // In a real implementation, we would mock the core service and verify the call
        let result = service.handle_event(event).await;
        // Assertions would go here
    }
}