//! Community celebration framework
//!
//! This module provides functionality for highlighting community achievements,
//! celebrating validation outcomes, and recognizing early adopters.

use tracing::info;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Community celebration framework
pub struct CommunityCelebration {
    achievements: Vec<CommunityAchievement>,
    celebrations: Vec<CelebrationEvent>,
    recognition_program: RecognitionProgram,
}

impl CommunityCelebration {
    /// Create a new community celebration framework
    pub fn new() -> Self {
        Self {
            achievements: Vec::new(),
            celebrations: Vec::new(),
            recognition_program: RecognitionProgram::new(),
        }
    }
    
    /// Record a community achievement
    pub fn record_achievement(&mut self, achievement: CommunityAchievement) {
        self.achievements.push(achievement);
        info!("Recorded community achievement: {}", achievement.title);
    }
    
    /// Create a celebration event
    pub fn create_celebration(&mut self, celebration: CelebrationEvent) -> Uuid {
        let celebration_id = celebration.id;
        self.celebrations.push(celebration);
        info!("Created celebration event: {}", celebration_id);
        celebration_id
    }
    
    /// Get recent achievements
    pub fn get_recent_achievements(&self, days: i64) -> Vec<&CommunityAchievement> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.achievements.iter()
            .filter(|achievement| achievement.achieved_at > cutoff)
            .collect()
    }
    
    /// Get upcoming celebrations
    pub fn get_upcoming_celebrations(&self) -> Vec<&CelebrationEvent> {
        let now = Utc::now();
        self.celebrations.iter()
            .filter(|celebration| celebration.scheduled_for > now && !celebration.completed)
            .collect()
    }
    
    /// Mark a celebration as completed
    pub fn complete_celebration(&mut self, celebration_id: Uuid) -> bool {
        if let Some(celebration) = self.celebrations.iter_mut()
            .find(|c| c.id == celebration_id) {
            celebration.completed = true;
            celebration.completed_at = Some(Utc::now());
            info!("Marked celebration as completed: {}", celebration_id);
            true
        } else {
            false
        }
    }
    
    /// Add recognition to the program
    pub fn add_recognition(&mut self, recognition: CommunityRecognition) {
        self.recognition_program.add_recognition(recognition);
    }
    
    /// Get recognition program
    pub fn get_recognition_program(&self) -> &RecognitionProgram {
        &self.recognition_program
    }
    
    /// Generate celebration report
    pub fn generate_celebration_report(&self) -> CelebrationReport {
        let total_achievements = self.achievements.len();
        let total_celebrations = self.celebrations.len();
        let completed_celebrations = self.celebrations.iter()
            .filter(|c| c.completed)
            .count();
        
        let recent_achievements = self.get_recent_achievements(30); // Last 30 days
        let upcoming_celebrations = self.get_upcoming_celebrations();
        
        CelebrationReport {
            generated_at: Utc::now(),
            total_achievements,
            total_celebrations,
            completed_celebrations,
            recent_achievements: recent_achievements.len(),
            upcoming_celebrations: upcoming_celebrations.len(),
        }
    }
}

impl Default for CommunityCelebration {
    fn default() -> Self {
        Self::new()
    }
}

/// Community achievement record
#[derive(Debug, Clone)]
pub struct CommunityAchievement {
    /// Unique identifier for the achievement
    pub id: Uuid,
    
    /// Title of the achievement
    pub title: String,
    
    /// Description of the achievement
    pub description: String,
    
    /// Type of achievement
    pub achievement_type: AchievementType,
    
    /// Community members involved (if applicable)
    pub participants: Option<Vec<String>>,
    
    /// Evidence or supporting information
    pub evidence: Option<String>,
    
    /// When the achievement was achieved
    pub achieved_at: DateTime<Utc>,
    
    /// Whether the achievement has been celebrated
    pub celebrated: bool,
}

impl CommunityAchievement {
    /// Create a new community achievement
    pub fn new(
        title: String,
        description: String,
        achievement_type: AchievementType,
        participants: Option<Vec<String>>,
        evidence: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            achievement_type,
            participants,
            evidence,
            achieved_at: Utc::now(),
            celebrated: false,
        }
    }
}

/// Types of community achievements
#[derive(Debug, Clone)]
pub enum AchievementType {
    /// First validation outcome
    FirstValidation,
    
    /// Milestone adoption rate
    AdoptionMilestone,
    
    /// Significant impact story contribution
    ImpactStory,
    
    /// Community transformation moment
    Transformation,
    
    /// Facilitator training completion
    FacilitatorTraining,
    
    /// Technical milestone
    TechnicalMilestone,
}

/// Celebration event
#[derive(Debug, Clone)]
pub struct CelebrationEvent {
    /// Unique identifier for the celebration
    pub id: Uuid,
    
    /// Title of the celebration
    pub title: String,
    
    /// Description of the celebration
    pub description: String,
    
    /// Type of celebration
    pub celebration_type: CelebrationType,
    
    /// When the celebration is scheduled for
    pub scheduled_for: DateTime<Utc>,
    
    /// Whether the celebration has been completed
    pub completed: bool,
    
    /// When the celebration was completed
    pub completed_at: Option<DateTime<Utc>>,
    
    /// Community members to be recognized
    pub honorees: Vec<String>,
    
    /// Supporting materials or documentation
    pub materials: Vec<CelebrationMaterial>,
}

impl CelebrationEvent {
    /// Create a new celebration event
    pub fn new(
        title: String,
        description: String,
        celebration_type: CelebrationType,
        scheduled_for: DateTime<Utc>,
        honorees: Vec<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            celebration_type,
            scheduled_for,
            completed: false,
            completed_at: None,
            honorees,
            materials: Vec::new(),
        }
    }
}

/// Types of celebration events
#[derive(Debug, Clone)]
pub enum CelebrationType {
    /// Community-wide celebration
    CommunityWide,
    
    /// Recognition ceremony
    Recognition,
    
    /// Milestone achievement
    Milestone,
    
    /// Transformation celebration
    Transformation,
}

/// Celebration material (supporting documentation, media, etc.)
#[derive(Debug, Clone)]
pub struct CelebrationMaterial {
    /// Title or description of the material
    pub title: String,
    
    /// Type of material
    pub material_type: MaterialType,
    
    /// Content or reference to the material
    pub content: String,
    
    /// When the material was created
    pub created_at: DateTime<Utc>,
}

/// Types of celebration materials
#[derive(Debug, Clone)]
pub enum MaterialType {
    /// Photo or image
    Photo,
    
    /// Video recording
    Video,
    
    /// Document or story
    Document,
    
    /// Social media post
    SocialMedia,
}

/// Recognition program for community members
pub struct RecognitionProgram {
    recognitions: HashMap<String, Vec<CommunityRecognition>>,
}

impl RecognitionProgram {
    /// Create a new recognition program
    pub fn new() -> Self {
        Self {
            recognitions: HashMap::new(),
        }
    }
    
    /// Add recognition for a community member
    pub fn add_recognition(&mut self, recognition: CommunityRecognition) {
        let user_id = recognition.recognized_user.clone();
        self.recognitions.entry(user_id)
            .or_insert_with(Vec::new)
            .push(recognition);
    }
    
    /// Get recognitions for a specific user
    pub fn get_user_recognitions(&self, user_id: &str) -> Option<&Vec<CommunityRecognition>> {
        self.recognitions.get(user_id)
    }
    
    /// Get all recognitions
    pub fn get_all_recognitions(&self) -> Vec<&CommunityRecognition> {
        self.recognitions.values()
            .flatten()
            .collect()
    }
    
    /// Get top recognized users
    pub fn get_top_recognized_users(&self, limit: usize) -> Vec<(&String, usize)> {
        let mut user_recognition_counts: Vec<(&String, usize)> = self.recognitions.iter()
            .map(|(user_id, recognitions)| (user_id, recognitions.len()))
            .collect();
        
        user_recognition_counts.sort_by(|a, b| b.1.cmp(&a.1));
        user_recognition_counts.truncate(limit);
        
        user_recognition_counts
    }
}

/// Community recognition record
#[derive(Debug, Clone)]
pub struct CommunityRecognition {
    /// Unique identifier for the recognition
    pub id: Uuid,
    
    /// User being recognized
    pub recognized_user: String,
    
    /// Type of recognition
    pub recognition_type: RecognitionType,
    
    /// Reason for recognition
    pub reason: String,
    
    /// Who provided the recognition
    pub recognized_by: String,
    
    /// When the recognition was given
    pub recognized_at: DateTime<Utc>,
    
    /// Supporting evidence or message
    pub evidence: Option<String>,
}

impl CommunityRecognition {
    /// Create a new community recognition
    pub fn new(
        recognized_user: String,
        recognition_type: RecognitionType,
        reason: String,
        recognized_by: String,
        evidence: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            recognized_user,
            recognition_type,
            reason,
            recognized_by,
            recognized_at: Utc::now(),
            evidence,
        }
    }
}

/// Types of community recognition
#[derive(Debug, Clone)]
pub enum RecognitionType {
    /// Early adopter recognition
    EarlyAdopter,
    
    /// Active participant recognition
    ActiveParticipant,
    
    /// Innovation recognition
    Innovation,
    
    /// Leadership recognition
    Leadership,
    
    /// Support recognition
    Support,
}

/// Celebration report
#[derive(Debug, Clone)]
pub struct CelebrationReport {
    /// When the report was generated
    pub generated_at: DateTime<Utc>,
    
    /// Total number of achievements recorded
    pub total_achievements: usize,
    
    /// Total number of celebrations planned
    pub total_celebrations: usize,
    
    /// Number of celebrations completed
    pub completed_celebrations: usize,
    
    /// Number of recent achievements (last 30 days)
    pub recent_achievements: usize,
    
    /// Number of upcoming celebrations
    pub upcoming_celebrations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    
    #[test]
    fn test_record_achievement() {
        let mut celebration = CommunityCelebration::new();
        let achievement = CommunityAchievement::new(
            "First Validation".to_string(),
            "Community completed first validation session".to_string(),
            AchievementType::FirstValidation,
            Some(vec!["user123".to_string(), "user456".to_string()]),
            Some("Session notes and outcomes".to_string()),
        );
        
        celebration.record_achievement(achievement);
        assert_eq!(celebration.achievements.len(), 1);
    }
    
    #[test]
    fn test_create_celebration() {
        let mut celebration = CommunityCelebration::new();
        let celebration_event = CelebrationEvent::new(
            "Launch Celebration".to_string(),
            "Celebrate dashboard launch".to_string(),
            CelebrationType::CommunityWide,
            Utc::now() + Duration::days(7),
            vec!["user123".to_string(), "user456".to_string()],
        );
        
        let celebration_id = celebration.create_celebration(celebration_event);
        assert_eq!(celebration.celebrations.len(), 1);
        
        // Test completion
        assert!(celebration.complete_celebration(celebration_id));
        assert!(celebration.celebrations[0].completed);
    }
    
    #[test]
    fn test_recognition_program() {
        let mut celebration = CommunityCelebration::new();
        let recognition = CommunityRecognition::new(
            "user123".to_string(),
            RecognitionType::EarlyAdopter,
            "First to complete onboarding".to_string(),
            "system".to_string(),
            Some("Onboarding completion timestamp".to_string()),
        );
        
        celebration.add_recognition(recognition);
        assert!(celebration.get_recognition_program()
            .get_user_recognitions("user123")
            .is_some());
    }
    
    #[test]
    fn test_get_recent_achievements() {
        let mut celebration = CommunityCelebration::new();
        let recent_achievement = CommunityAchievement::new(
            "Recent Achievement".to_string(),
            "Achieved recently".to_string(),
            AchievementType::TechnicalMilestone,
            None,
            None,
        );
        
        let old_achievement = CommunityAchievement {
            id: Uuid::new_v4(),
            title: "Old Achievement".to_string(),
            description: "Achieved 45 days ago".to_string(),
            achievement_type: AchievementType::Transformation,
            participants: None,
            evidence: None,
            achieved_at: Utc::now() - Duration::days(45),
            celebrated: false,
        };
        
        celebration.record_achievement(recent_achievement);
        celebration.achievements.push(old_achievement);
        
        let recent = celebration.get_recent_achievements(30);
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].title, "Recent Achievement");
    }
    
    #[test]
    fn test_generate_report() {
        let mut celebration = CommunityCelebration::new();
        let achievement = CommunityAchievement::new(
            "Test Achievement".to_string(),
            "Test description".to_string(),
            AchievementType::ImpactStory,
            None,
            None,
        );
        
        celebration.record_achievement(achievement);
        
        let celebration_event = CelebrationEvent::new(
            "Test Celebration".to_string(),
            "Test description".to_string(),
            CelebrationType::Milestone,
            Utc::now() + Duration::days(1),
            vec!["user123".to_string()],
        );
        
        celebration.create_celebration(celebration_event);
        
        let report = celebration.generate_celebration_report();
        assert_eq!(report.total_achievements, 1);
        assert_eq!(report.total_celebrations, 1);
        assert_eq!(report.completed_celebrations, 0);
    }
}