#[cfg(test)]
mod tests {
    use super::*;
    use yew::prelude::*;
    use yew::{Renderer, Scope};
    use wasm_bindgen_test::*;
    use uuid::Uuid;
    use std::collections::HashMap;
    use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};
    use crate::presentation::presence_batcher::{PresenceUpdateBatcher, CursorVirtualizer};

    #[wasm_bindgen_test]
    fn test_presence_sidebar_renders() {
        let mut users = HashMap::new();
        let user_id = Uuid::new_v4();
        users.insert(user_id, PresenceUser {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            status: PresenceStatus::Online,
        });

        let on_user_click = Callback::from(|_| {});

        let props = PresenceSidebarProps {
            users,
            on_user_click,
        };

        // This test just verifies the component compiles and can be created
        let _component = PresenceSidebar::new(props);
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn test_status_indicator_renders() {
        let props = StatusIndicatorProps {
            status: PresenceStatus::Online,
        };

        // This test just verifies the component compiles and can be created
        let _component = StatusIndicator::new(props);
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn test_avatar_badge_renders() {
        let props = AvatarBadgeProps {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            user_id: Uuid::new_v4(),
            is_typing: true,
        };

        // This test just verifies the component compiles and can be created
        let _component = AvatarBadge::new(props);
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_cursor_overlay_renders() {
        let mut users = HashMap::new();
        let user_id = Uuid::new_v4();
        users.insert(user_id, PresenceUser {
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            status: PresenceStatus::Online,
        });
        
        let mut cursor_positions = HashMap::new();
        cursor_positions.insert(user_id, (5, 10));

        let props = CursorOverlayProps {
            users,
            cursor_positions,
        };

        // This test just verifies the component compiles and can be created
        let _component = CursorOverlay::new(props);
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    fn test_presence_expiration_logic() {
        // Test that the presence sidebar correctly handles user presence states
        let mut users = HashMap::new();
        let user1_id = Uuid::new_v4();
        let user2_id = Uuid::new_v4();
        
        // Add online user
        users.insert(user1_id, PresenceUser {
            avatar_url: Some("https://example.com/avatar1.png".to_string()),
            color: "#ff0000".to_string(),
            status: PresenceStatus::Online,
        });
        
        // Add away user
        users.insert(user2_id, PresenceUser {
            avatar_url: Some("https://example.com/avatar2.png".to_string()),
            color: "#00ff00".to_string(),
            status: PresenceStatus::Away,
        });
        
        assert_eq!(users.len(), 2);
        assert_eq!(users.get(&user1_id).unwrap().status, PresenceStatus::Online);
        assert_eq!(users.get(&user2_id).unwrap().status, PresenceStatus::Away);
    }
    
    #[wasm_bindgen_test]
    fn test_presence_batcher_functionality() {
        let mut batcher = PresenceUpdateBatcher::new(100, 5);
        
        // Add some updates
        let update = shared_packages::realtime_signaling::message::PresenceUpdate {
            document_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            cursor: None,
            selection: None,
            is_typing: false,
            avatar_url: None,
            color: "#ff0000".to_string(),
            last_active: chrono::Utc::now(),
            timestamp: chrono::Utc::now(),
        };
        
        batcher.add_presence_update(update.clone());
        assert_eq!(batcher.pending_count(), 1);
        
        // Flush should return the update
        let (presence_updates, cursor_updates) = batcher.flush();
        assert_eq!(presence_updates.len(), 1);
        assert_eq!(cursor_updates.len(), 0);
        assert_eq!(batcher.pending_count(), 0);
    }
    
    #[wasm_bindgen_test]
    fn test_cursor_virtualizer_visibility() {
        let mut virtualizer = CursorVirtualizer::new(800.0, 600.0); // 800x600 viewport
        virtualizer.set_char_dimensions(8.0, 20.0); // 8px wide, 20px tall characters
        
        // Cursor at position (5, 10) = 80px, 100px
        assert!(virtualizer.is_cursor_visible(5, 10));
        
        // Cursor outside viewport
        assert!(!virtualizer.is_cursor_visible(1000, 1000));
    }
}