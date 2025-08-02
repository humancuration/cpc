#[cfg(test)]
mod integration_tests {
    use super::*;
    use skill_volunteering::{
        endorsement_management::service::{EndorsementService, InMemoryEndorsementRepository},
        social_integration::SkillVolunteeringSocialIntegration,
        SkillVolunteeringServiceImpl,
    };
    use shared_packages::social_integration::domain::social_event::SocialEvent;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_full_social_integration_flow() {
        // Setup services
        let endorsement_service = Arc::new(EndorsementService::new(
            Arc::new(InMemoryEndorsementRepository::default())
        ));
        
        // Create test data
        let user_id = Uuid::new_v4();
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        let endorser_id = Uuid::new_v4();
        
        // Step 1: User shares an opportunity
        let share_event = SkillVolunteeringSocialIntegration::create_opportunity_shared_event(
            user_id,
            opportunity_id,
        );
        
        // Verify event creation
        match share_event {
            SocialEvent::OpportunityShared { user_id: uid, opportunity_id: oid, .. } => {
                assert_eq!(uid, user_id);
                assert_eq!(oid, opportunity_id);
            },
            _ => panic!("Expected OpportunityShared event"),
        }
        
        // Step 2: User volunteers for opportunity
        let volunteer_event = SkillVolunteeringSocialIntegration::create_volunteered_event(
            user_id,
            opportunity_id,
            10.5,
        );
        
        // Verify event creation
        match volunteer_event {
            SocialEvent::Volunteered { user_id: uid, opportunity_id: oid, hours_contributed, .. } => {
                assert_eq!(uid, user_id);
                assert_eq!(oid, opportunity_id);
                assert_eq!(hours_contributed, 10.5);
            },
            _ => panic!("Expected Volunteered event"),
        }
        
        // Step 3: Record skill endorsement
        let endorsement = endorsement_service
            .record_endorsement(
                opportunity_id,
                skill_id,
                endorser_id,
                recipient_id,
                Some("Excellent contribution!".to_string()),
                5,
            )
            .await
            .unwrap();
        
        // Verify endorsement
        assert_eq!(endorsement.rating, 5);
        assert_eq!(endorsement.comment, Some("Excellent contribution!".to_string()));
        
        // Step 4: Retrieve endorsements for user
        let endorsements = endorsement_service
            .get_endorsements_for_user(recipient_id)
            .await
            .unwrap();
        
        assert_eq!(endorsements.len(), 1);
        assert_eq!(endorsements[0].id, endorsement.id);
        
        // Step 5: Filter endorsements by skill
        let skill_endorsements = endorsement_service
            .get_endorsements_for_user_skill(recipient_id, skill_id)
            .await
            .unwrap();
        
        assert_eq!(skill_endorsements.len(), 1);
        assert_eq!(skill_endorsements[0].skill_id, skill_id);
    }

    #[test]
    fn test_social_event_types() {
        let user_id = Uuid::new_v4();
        let opportunity_id = Uuid::new_v4();
        
        // Test OpportunityShared event
        let event = SkillVolunteeringSocialIntegration::create_opportunity_shared_event(
            user_id,
            opportunity_id,
        );
        
        assert_eq!(event.user_id(), &user_id);
        
        // Test Volunteered event
        let event = SkillVolunteeringSocialIntegration::create_volunteered_event(
            user_id,
            opportunity_id,
            8.0,
        );
        
        assert_eq!(event.user_id(), &user_id);
    }

    #[tokio::test]
    async fn test_endorsement_validation() {
        let service = EndorsementService::new(
            Arc::new(InMemoryEndorsementRepository::default())
        );
        
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Test self-endorsement prevention
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            user_id,
            user_id,
            None,
            4,
        ).await;
        
        assert!(result.is_err());
        
        // Test invalid rating
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            Uuid::new_v4(),
            user_id,
            None,
            0,
        ).await;
        
        assert!(result.is_err());
        
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            Uuid::new_v4(),
            user_id,
            None,
            6,
        ).await;
        
        assert!(result.is_err());
    }
}