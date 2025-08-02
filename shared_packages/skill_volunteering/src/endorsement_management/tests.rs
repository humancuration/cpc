#[cfg(test)]
mod tests {
    use super::*;
    use crate::endorsement_management::{models::SkillEndorsement, service::{EndorsementService, InMemoryEndorsementRepository}};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_record_endorsement_success() {
        let repo = Arc::new(InMemoryEndorsementRepository::default());
        let service = EndorsementService::new(repo);
        
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let endorser_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            Some("Great work!".to_string()),
            5,
        ).await;
        
        assert!(result.is_ok());
        let endorsement = result.unwrap();
        assert_eq!(endorsement.opportunity_id, opportunity_id);
        assert_eq!(endorsement.skill_id, skill_id);
        assert_eq!(endorsement.endorser_id, endorser_id);
        assert_eq!(endorsement.recipient_id, recipient_id);
        assert_eq!(endorsement.comment, Some("Great work!".to_string()));
        assert_eq!(endorsement.rating, 5);
    }

    #[tokio::test]
    async fn test_record_endorsement_self_endorsement_fails() {
        let repo = Arc::new(InMemoryEndorsementRepository::default());
        let service = EndorsementService::new(repo);
        
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            user_id,
            user_id, // Same user
            None,
            4,
        ).await;
        
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            EndorsementServiceError::InvalidInput(_)
        ));
    }

    #[tokio::test]
    async fn test_record_endorsement_invalid_rating_fails() {
        let repo = Arc::new(InMemoryEndorsementRepository::default());
        let service = EndorsementService::new(repo);
        
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let endorser_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        
        // Test rating 0
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            None,
            0,
        ).await;
        
        assert!(result.is_err());
        
        // Test rating 6
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            None,
            6,
        ).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_record_duplicate_endorsement_fails() {
        let repo = Arc::new(InMemoryEndorsementRepository::default());
        let service = EndorsementService::new(repo);
        
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        let endorser_id = Uuid::new_v4();
        let recipient_id = Uuid::new_v4();
        
        // First endorsement should succeed
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            None,
            4,
        ).await;
        assert!(result.is_ok());
        
        // Duplicate should fail
        let result = service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            None,
            5,
        ).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_endorsements_for_user() {
        let repo = Arc::new(InMemoryEndorsementRepository::default());
        let service = EndorsementService::new(repo);
        
        let recipient_id = Uuid::new_v4();
        let endorser_id1 = Uuid::new_v4();
        let endorser_id2 = Uuid::new_v4();
        let opportunity_id = Uuid::new_v4();
        let skill_id = Uuid::new_v4();
        
        // Add two endorsements for the same recipient
        service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id1,
            recipient_id,
            Some("Good work".to_string()),
            4,
        ).await.unwrap();
        
        service.record_endorsement(
            opportunity_id,
            skill_id,
            endorser_id2,
            recipient_id,
            Some("Excellent".to_string()),
            5,
        ).await.unwrap();
        
        // Get endorsements for user
        let endorsements = service.get_endorsements_for_user(recipient_id).await.unwrap();
        assert_eq!(endorsements.len(), 2);
        
        // Test filtering by skill
        let skill_endorsements = service.get_endorsements_for_user_skill(recipient_id, skill_id).await.unwrap();
        assert_eq!(skill_endorsements.len(), 2);
    }
}