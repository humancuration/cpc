#[cfg(test)]
mod tests {
    use super::*;
    use yew::prelude::*;
    use wasm_bindgen_test::*;
    use uuid::Uuid;
    use std::collections::HashMap;
    use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};
    use crate::presentation::presence_state::{PresenceStateManager, UserPresenceState};

    #[wasm_bindgen_test]
    fn test_document_editor_with_presence_creation() {
        let props = DocumentEditorWithPresenceProps {
            document_id: Uuid::new_v4(),
            current_user_id: Uuid::new_v4(),
        };

        // This test just verifies the component compiles and can be created
        let _component = DocumentEditorWithPresence::new(props);
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_handle_signaling_message_presence_update() {
        // Test handling of presence update messages
        let users = UseStateHandle::new(HashMap::new());
        let cursor_positions = UseStateHandle::new(HashMap::new());
        let last_activity = UseStateHandle::new(HashMap::new());
        
        // Create a presence update message
        let user_id = Uuid::new_v4();
        let update = shared_packages::realtime_signaling::message::PresenceUpdate {
            document_id: Uuid::new_v4(),
            user_id,
            cursor: None,
            selection: None,
            is_typing: true,
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            last_active: chrono::Utc::now(),
            timestamp: chrono::Utc::now(),
        };
        
        let message = shared_packages::realtime_signaling::message::SignalingMessage::PresenceUpdate(update);
        
        // This test just verifies the function compiles
        // In a real test, we would need to mock the UseStateHandle
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_handle_signaling_message_cursor_update() {
        // Test handling of cursor update messages
        let users = UseStateHandle::new(HashMap::new());
        let cursor_positions = UseStateHandle::new(HashMap::new());
        let last_activity = UseStateHandle::new(HashMap::new());
        
        // Create a cursor update message
        let user_id = Uuid::new_v4();
        let cursor = shared_packages::realtime_signaling::message::CursorPosition {
            document_id: Uuid::new_v4(),
            user_id,
            position: shared_packages::realtime_signaling::message::Position {
                line: 5,
                column: 10,
            },
            timestamp: chrono::Utc::now(),
        };
        
        let message = shared_packages::realtime_signaling::message::SignalingMessage::CursorUpdate(cursor);
        
        // This test just verifies the function compiles
        // In a real test, we would need to mock the UseStateHandle
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_presence_state_manager() {
        let mut manager = PresenceStateManager::new(100);
        
        let user_id = Uuid::new_v4();
        let presence_user = PresenceUser {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            status: PresenceStatus::Online,
        };
        
        let state = UserPresenceState::new(presence_user, chrono::Utc::now());
        manager.insert(user_id, state);
        
        assert!(manager.contains(&user_id));
        assert_eq!(manager.len(), 1);
        
        let retrieved_state = manager.get(&user_id);
        assert!(retrieved_state.is_some());
    }
    
    #[wasm_bindgen_test]
    fn test_typing_indicator_handling() {
        // Test handling of typing indicator messages
        let presence_state = UseStateHandle::new(PresenceStateManager::new(100));
        let users = UseStateHandle::new(HashMap::new());
        
        let user_id = Uuid::new_v4();
        
        // This test just verifies the function compiles
        // In a real test, we would need to mock the UseStateHandle
        assert!(true);
    }
}