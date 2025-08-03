//! Data models for social enhancements

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Represents an achievement that a user can earn
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Achievement {
    /// Unique identifier for the achievement
    pub id: Uuid,
    
    /// User who earned the achievement
    pub user_id: Uuid,
    
    /// Type of achievement
    pub achievement_type: AchievementType,
    
    /// Title of the achievement
    pub title: String,
    
    /// Description of the achievement
    pub description: String,
    
    /// Optional image URL for the achievement badge
    pub image_url: Option<String>,
    
    /// Timestamp when the achievement was earned
    pub earned_at: DateTime<Utc>,
}

impl Achievement {
    /// Create a new achievement
    pub fn new(user_id: Uuid, achievement_type: AchievementType, title: String, description: String, image_url: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            achievement_type,
            title,
            description,
            image_url,
            earned_at: Utc::now(),
        }
    }
}

/// Types of achievements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AchievementType {
    /// Volunteer hour milestone
    VolunteerHours(VolunteerMilestone),
    
    /// Skill mastery achievement
    SkillMaster,
    
    /// Community challenge completion
    ChallengeCompleted,
    
    /// Social engagement milestone
    SocialEngagement,
}

/// Volunteer hour milestones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VolunteerMilestone {
    /// 10 hours volunteered
    TenHours,
    
    /// 50 hours volunteered
    FiftyHours,
    
    /// 100 hours volunteered
    HundredHours,
    
    /// 500 hours volunteered
    FiveHundredHours,
}

/// Represents a group challenge for community engagement
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupChallenge {
    /// Unique identifier for the challenge
    pub id: Uuid,
    
    /// Title of the challenge
    pub title: String,
    
    /// Description of the challenge
    pub description: String,
    
    /// Type of challenge
    pub challenge_type: ChallengeType,
    
    /// Target value for the challenge
    pub target_value: Decimal,
    
    /// Start time of the challenge
    pub start_time: DateTime<Utc>,
    
    /// End time of the challenge
    pub end_time: DateTime<Utc>,
    
    /// Whether the challenge is active
    pub is_active: bool,
}

impl GroupChallenge {
    /// Create a new group challenge
    pub fn new(title: String, description: String, challenge_type: ChallengeType, target_value: Decimal, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            challenge_type,
            target_value,
            start_time,
            end_time,
            is_active: true,
        }
    }
}

/// Types of group challenges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChallengeType {
    /// Volunteer hour challenge
    VolunteerHours,
    
    /// Skill exchange challenge
    SkillExchanges,
}

/// Progress tracking for group challenges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChallengeProgress {
    /// Unique identifier for the progress record
    pub id: Uuid,
    
    /// ID of the challenge
    pub challenge_id: Uuid,
    
    /// User participating in the challenge
    pub user_id: Uuid,
    
    /// Current progress value
    pub progress: Decimal,
    
    /// Timestamp of last update
    pub updated_at: DateTime<Utc>,
}

impl ChallengeProgress {
    /// Create a new challenge progress record
    pub fn new(challenge_id: Uuid, user_id: Uuid, progress: Decimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            challenge_id,
            user_id,
            progress,
            updated_at: Utc::now(),
        }
    }
    
    /// Update progress
    pub fn update_progress(&mut self, progress: Decimal) {
        self.progress = progress;
        self.updated_at = Utc::now();
    }
}