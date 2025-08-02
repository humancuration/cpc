//! Models for skill endorsement system

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a skill endorsement given by one user to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEndorsement {
    /// Unique identifier for the endorsement
    pub id: Uuid,
    
    /// ID of the opportunity this endorsement relates to
    pub opportunity_id: Uuid,
    
    /// ID of the skill being endorsed
    pub skill_id: Uuid,
    
    /// ID of the user giving the endorsement
    pub endorser_id: Uuid,
    
    /// ID of the user receiving the endorsement
    pub recipient_id: Uuid,
    
    /// Optional comment about the endorsement
    pub comment: Option<String>,
    
    /// Rating given (1-5 scale)
    pub rating: u32,
    
    /// When the endorsement was created
    pub created_at: DateTime<Utc>,
}

impl SkillEndorsement {
    /// Create a new skill endorsement
    pub fn new(
        opportunity_id: Uuid,
        skill_id: Uuid,
        endorser_id: Uuid,
        recipient_id: Uuid,
        comment: Option<String>,
        rating: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            opportunity_id,
            skill_id,
            endorser_id,
            recipient_id,
            comment,
            rating,
            created_at: Utc::now(),
        }
    }
}