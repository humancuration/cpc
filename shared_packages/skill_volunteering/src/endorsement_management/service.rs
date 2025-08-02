//! Service for managing skill endorsements

use crate::endorsement_management::models::SkillEndorsement;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum EndorsementServiceError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Endorsement not found")]
    NotFound,
    
    #[error("User not authorized to create endorsement for this opportunity")]
    Unauthorized,
    
    #[error("Internal server error: {0}")]
    Internal(String),
}

/// Repository trait for endorsement persistence
#[async_trait::async_trait]
pub trait EndorsementRepository: Send + Sync {
    /// Save an endorsement
    async fn save(&self, endorsement: &SkillEndorsement) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find endorsements by recipient user ID
    async fn find_by_recipient(&self, recipient_id: Uuid) -> Result<Vec<SkillEndorsement>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find endorsements by recipient and skill
    async fn find_by_recipient_and_skill(
        &self, 
        recipient_id: Uuid, 
        skill_id: Uuid
    ) -> Result<Vec<SkillEndorsement>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Find an endorsement by opportunity, endorser, and recipient
    async fn find_existing(
        &self,
        opportunity_id: Uuid,
        skill_id: Uuid,
        endorser_id: Uuid,
        recipient_id: Uuid,
    ) -> Result<Option<SkillEndorsement>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Service for managing skill endorsements
pub struct EndorsementService {
    repository: Arc<dyn EndorsementRepository>,
}

impl EndorsementService {
    /// Create a new endorsement service
    pub fn new(repository: Arc<dyn EndorsementRepository>) -> Self {
        Self { repository }
    }
    
    /// Record a new skill endorsement
    pub async fn record_endorsement(
        &self,
        opportunity_id: Uuid,
        skill_id: Uuid,
        endorser_id: Uuid,
        recipient_id: Uuid,
        comment: Option<String>,
        rating: u32,
    ) -> Result<SkillEndorsement, EndorsementServiceError> {
        // Validate rating range
        if rating == 0 || rating > 5 {
            return Err(EndorsementServiceError::InvalidInput(
                "Rating must be between 1 and 5".to_string()
            ));
        }
        
        // Check if user is trying to endorse themselves
        if endorser_id == recipient_id {
            return Err(EndorsementServiceError::InvalidInput(
                "Cannot endorse yourself".to_string()
            ));
        }
        
        // Check if endorsement already exists
        if let Some(existing) = self.repository
            .find_existing(opportunity_id, skill_id, endorser_id, recipient_id)
            .await
            .map_err(|e| EndorsementServiceError::Internal(e.to_string()))?
        {
            return Err(EndorsementServiceError::InvalidInput(
                "You have already endorsed this user for this skill in this opportunity".to_string()
            ));
        }
        
        // Create new endorsement
        let endorsement = SkillEndorsement::new(
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            comment,
            rating,
        );
        
        // Save the endorsement
        self.repository
            .save(&endorsement)
            .await
            .map_err(|e| EndorsementServiceError::Internal(e.to_string()))?;
        
        Ok(endorsement)
    }
    
    /// Get all endorsements for a specific user
    pub async fn get_endorsements_for_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<SkillEndorsement>, EndorsementServiceError> {
        self.repository
            .find_by_recipient(user_id)
            .await
            .map_err(|e| EndorsementServiceError::Internal(e.to_string()))
    }
    
    /// Get endorsements for a specific user and skill
    pub async fn get_endorsements_for_user_skill(
        &self,
        user_id: Uuid,
        skill_id: Uuid,
    ) -> Result<Vec<SkillEndorsement>, EndorsementServiceError> {
        self.repository
            .find_by_recipient_and_skill(user_id, skill_id)
            .await
            .map_err(|e| EndorsementServiceError::Internal(e.to_string()))
    }
}

/// In-memory implementation of the endorsement repository for testing
#[derive(Default)]
pub struct InMemoryEndorsementRepository {
    endorsements: Mutex<Vec<SkillEndorsement>>,
}

#[async_trait::async_trait]
impl EndorsementRepository for InMemoryEndorsementRepository {
    async fn save(&self, endorsement: &SkillEndorsement) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut endorsements = self.endorsements.lock().await;
        endorsements.push(endorsement.clone());
        Ok(())
    }
    
    async fn find_by_recipient(&self, recipient_id: Uuid) -> Result<Vec<SkillEndorsement>, Box<dyn std::error::Error + Send + Sync>> {
        let endorsements = self.endorsements.lock().await;
        Ok(endorsements
            .iter()
            .filter(|e| e.recipient_id == recipient_id)
            .cloned()
            .collect())
    }
    
    async fn find_by_recipient_and_skill(
        &self, 
        recipient_id: Uuid, 
        skill_id: Uuid
    ) -> Result<Vec<SkillEndorsement>, Box<dyn std::error::Error + Send + Sync>> {
        let endorsements = self.endorsements.lock().await;
        Ok(endorsements
            .iter()
            .filter(|e| e.recipient_id == recipient_id && e.skill_id == skill_id)
            .cloned()
            .collect())
    }
    
    async fn find_existing(
        &self,
        opportunity_id: Uuid,
        skill_id: Uuid,
        endorser_id: Uuid,
        recipient_id: Uuid,
    ) -> Result<Option<SkillEndorsement>, Box<dyn std::error::Error + Send + Sync>> {
        let endorsements = self.endorsements.lock().await;
        Ok(endorsements
            .iter()
            .find(|e| {
                e.opportunity_id == opportunity_id &&
                e.skill_id == skill_id &&
                e.endorser_id == endorser_id &&
                e.recipient_id == recipient_id
            })
            .cloned())
    }
}