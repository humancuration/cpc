//! Integration tests for the Messenger application

#[cfg(test)]
mod tests {
    use messenger_domain::{
        models::{Conversation, Participant},
        errors::MessengerError,
    };
    use uuid::Uuid;
    
    #[test]
    fn test_create_conversation() {
        let participant1 = Participant::new(Uuid::new_v4());
        let participant2 = Participant::new(Uuid::new_v4());
        let participants = vec![participant1, participant2];
        
        let conversation = Conversation::new_1to1(participants);
        
        assert!(!conversation.is_group);
        assert_eq!(conversation.participants.len(), 2);
    }
    
    #[test]
    fn test_create_group_conversation() {
        let participant1 = Participant::new(Uuid::new_v4());
        let participant2 = Participant::new(Uuid::new_v4());
        let participant3 = Participant::new(Uuid::new_v4());
        let participants = vec![participant1, participant2, participant3];
        
        let group_name = "Test Group".to_string();
        let conversation = Conversation::new_group(participants, group_name.clone());
        
        assert!(conversation.is_group);
        assert_eq!(conversation.group_name, Some(group_name));
        assert_eq!(conversation.participants.len(), 3);
    }
    
    #[test]
    fn test_message_status_updates() {
        let mut message = messenger_domain::models::Message::new_text(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "Hello, World!".to_string(),
        );
        
        assert!(matches!(message.delivery_status, messenger_domain::models::DeliveryStatus::Pending));
        
        message.mark_sent();
        assert!(matches!(message.delivery_status, messenger_domain::models::DeliveryStatus::Sent(_)));
        
        message.mark_delivered();
        assert!(matches!(message.delivery_status, messenger_domain::models::DeliveryStatus::Delivered(_)));
        
        message.mark_read();
        assert!(matches!(message.delivery_status, messenger_domain::models::DeliveryStatus::Read(_)));
    }
}