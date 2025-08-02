#[cfg(test)]
mod social_integration_tests {
    use super::*;
    use crate::skill_volunteering::{
        endorsement_management::service::{EndorsementService, InMemoryEndorsementRepository},
        social_integration::SkillVolunteeringSocialIntegration,
        SkillVolunteeringServiceImpl,
    };
    use shared_packages::social_integration::domain::social_event::SocialEvent;
    use uuid::Uuid;

    #[test]
    fn test_create_opportunity_shared_event() {
        let user_id = Uuid::new_v4();
        let opportunity_id = Uuid::new_v4();
        
        let event = SkillVolunteeringSocialIntegration::create_opportunity_shared_event(
            user_id,
            opportunity_id,
        );
        
        match event {
            SocialEvent::OpportunityShared { user_id: uid, opportunity_id: oid, .. } => {
                assert_eq!(uid, user_id);
                assert_eq!(oid, opportunity_id);
            },
            _ => panic!("Expected OpportunityShared event"),
        }
    }

    #[test]
    fn test_create_volunteered_event() {
        let user_id = Uuid::new_v4();
        let opportunity_id = Uuid::new_v4();
        let hours_contributed = 5.5;
        
        let event = SkillVolunteeringSocialIntegration::create_volunteered_event(
            user_id,
            opportunity_id,
            hours_contributed,
        );
        
        match event {
            SocialEvent::Volunteered { user_id: uid, opportunity_id: oid, hours_contributed: hours, .. } => {
                assert_eq!(uid, user_id);
                assert_eq!(oid, opportunity_id);
                assert_eq!(hours, hours_contributed);
            },
            _ => panic!("Expected Volunteered event"),
        }
    }

    #[tokio::test]
    async fn test_endorsement_flow_with_social_integration() {
        // Setup in-memory repository
        let repo = Arc::new(InMemoryEndorsementRepository::default());
        let endorsement_service = EndorsementService::new(repo);
        
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let endorser_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        
        // Record an endorsement
        let endorsement = endorsement_service
            .record_endorsement(
                opportunity_id,
                skill_id,
                endorser_id,
                recipient_id,
                Some("Excellent work on this project!".to_string()),
                5,
            )
            .await
            .unwrap();
        
        // Verify endorsement was created
        assert_eq!(endorsement.rating, 5);
        assert_eq!(endorsement.comment, Some("Excellent work on this project!".to_string()));
        
        // Get endorsements for recipient
        let endorsements = endorsement_service
            .get_endorsements_for_user(recipient_id)
            .await
            .unwrap();
        
        assert_eq!(endorsements.len(), 1);
        assert_eq!(endorsements[0].id, endorsement.id);
        
        // Test filtering by skill
        let skill_endorsements = endorsement_service
            .get_endorsements_for_user_skill(recipient_id, skill_id)
            .await
            .unwrap();
        
        assert_eq!(skill_endorsements.len(), 1);
    }

    #[tokio::test]
    async fn test_social_event_integration() {
        use shared_packages::social_integration::{
            domain::{post::AppSource, social_event::SocialEvent},
            application::social_integration_service::{SocialIntegrationService, MockUnifiedPostRepository},
        };
        
        // Create mock social integration service
        let mock_repo = Box::new(MockUnifiedPostRepository::new());
        let social_service = SocialIntegrationService::new(mock_repo);
        
        // Test event handling
        let event = SocialEvent::OpportunityShared {
            user_id: Uuid::new_v4(),
            opportunity_id: Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
        };
        
        let result = social_service.handle_social_event(event).await;
        assert!(result.is_ok());
    }
}