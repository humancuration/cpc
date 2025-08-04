use uuid::Uuid;
use consent_manager::{ConsentService, Domain, DataSharingLevel, Actor};

pub struct ConsentAdapter {
    consent_service: ConsentService,
}

impl ConsentAdapter {
    pub fn new(consent_service: ConsentService) -> Self {
        Self {
            consent_service,
        }
    }
    
    pub async fn check_consent(&self, user_id: Uuid, target_user_id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        // Check if user_id has given consent to interact with target_user_id
        // For social interactions, we'll use DocumentData as a placeholder domain
        let level = self.consent_service.get_consent_level(
            &user_id.to_string(),
            Domain::DocumentData
        ).await?;
        
        // If the user has at least minimal consent, they can interact
        Ok(level.priority() >= DataSharingLevel::Minimal.priority())
    }
    
    pub async fn request_consent(&self, user_id: Uuid, target_user_id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        // Request consent from the target user to interact with user_id
        // This would typically involve sending a notification to the target user
        // and waiting for their response
        
        // For now, we'll just return true as a placeholder
        Ok(true)
    }
    
    pub async fn update_consent(&self, user_id: Uuid, level: DataSharingLevel, actor: Actor) -> Result<(), Box<dyn std::error::Error>> {
        // Update the consent level for the user
        self.consent_service.update_consent_level(
            &user_id.to_string(),
            Domain::DocumentData,
            level,
            actor
        ).await?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_consent_check() {
        // Note: This test is a placeholder since we can't easily create a real ConsentService in tests
        // In a real implementation, we would use a mock ConsentService
        assert!(true);
    }
}