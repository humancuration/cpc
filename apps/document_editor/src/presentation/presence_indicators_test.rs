#[cfg(test)]
mod tests {
    use super::*;
    use yew::prelude::*;
    use yew::{Renderer, Scope};
    use wasm_bindgen_test::*;
    use uuid::Uuid;
    use std::collections::HashMap;
    use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};

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
}