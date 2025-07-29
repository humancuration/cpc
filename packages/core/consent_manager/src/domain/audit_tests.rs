//! Tests for audit domain logic.

#[cfg(test)]
mod tests {
    use super::super::audit::*;
    use super::super::consent::{Domain, DataSharingLevel};

    #[test]
    fn test_audit_event_creation() {
        let event = AuditEvent::new(
            "user123".to_string(),
            Domain::FinancialData,
            ConsentAction::Granted,
            None,
            DataSharingLevel::Standard,
            Actor::User("admin456".to_string()),
        );
        
        assert_eq!(event.user_id, "user123");
        assert_eq!(event.domain, Domain::FinancialData);
        assert_eq!(event.action, ConsentAction::Granted);
        assert_eq!(event.previous_level, None);
        assert_eq!(event.new_level, DataSharingLevel::Standard);
        assert_eq!(event.actor, Actor::User("admin456".to_string()));
        assert!(!event.id.is_empty()); // Should have a generated UUID
    }

    #[test]
    fn test_audit_event_with_previous_level() {
        let event = AuditEvent::new(
            "user123".to_string(),
            Domain::FinancialData,
            ConsentAction::Modified,
            Some(DataSharingLevel::Minimal),
            DataSharingLevel::Standard,
            Actor::User("admin456".to_string()),
        );
        
        assert_eq!(event.previous_level, Some(DataSharingLevel::Minimal));
        assert_eq!(event.new_level, DataSharingLevel::Standard);
    }

    #[test]
    fn test_actor_variants() {
        let user_actor = Actor::User("user123".to_string());
        let service_actor = Actor::Service("payment_service".to_string());
        let admin_actor = Actor::Admin("admin456".to_string());
        
        assert_ne!(user_actor, service_actor);
        assert_ne!(service_actor, admin_actor);
        assert_ne!(admin_actor, user_actor);
    }

    #[test]
    fn test_consent_action_variants() {
        assert_ne!(ConsentAction::Granted, ConsentAction::Revoked);
        assert_ne!(ConsentAction::Revoked, ConsentAction::Modified);
        assert_ne!(ConsentAction::Modified, ConsentAction::Granted);
    }
}