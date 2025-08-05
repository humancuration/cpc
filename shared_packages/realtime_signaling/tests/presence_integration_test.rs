#[cfg(test)]
mod tests {
    use realtime_signaling::message::{PresenceUpdate, PresenceSummary, PresenceUser, PresenceStatus};
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_presence_update_creation() {
        let update = PresenceUpdate {
            document_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            cursor: None,
            selection: None,
            is_typing: false,
            avatar_url: Some("https://example.com/avatar.png".to_string()),
            color: "#ff0000".to_string(),
            last_active: Utc::now(),
            timestamp: Utc::now(),
        };

        assert_eq!(update.color, "#ff0000");
        assert!(update.avatar_url.is_some());
    }

    #[test]
    fn test_presence_summary_creation() {
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

        assert_eq!(summary.users.len(), 1);
        assert!(summary.users.contains_key(&user_id));
    }
}