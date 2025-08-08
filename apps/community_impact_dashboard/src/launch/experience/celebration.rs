//! Launch celebration features for the Unified Community Impact Dashboard
//!
//! This module provides features to highlight community achievements,
//! celebrate validation outcomes, and recognize early adopters during the launch.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Launch celebration system
pub struct LaunchCelebration {
    celebration_events: Vec<CelebrationEvent>,
    community_recognitions: Vec<CommunityRecognition>,
    achievement_tracking: HashMap<String, Vec<CommunityAchievement>>,
    celebration_materials: Vec<CelebrationMaterial>,
}

impl LaunchCelebration {
    /// Create a new launch celebration system
    pub fn new() -> Self {
        Self {
            celebration_events: Vec::new(),
            community_recognitions: Vec::new(),
            achievement_tracking: HashMap::new(),
            celebration_materials: Vec::new(),
        }
    }

    /// Create a launch celebration event
    pub fn create_launch_celebration(
        &mut self,
        title: String,
        description: String,
        honorees: Vec<String>,
    ) -> Uuid {
        let celebration = CelebrationEvent::new(
            title,
            description,
            CelebrationType::CommunityWide,
            Utc::now(),
            honorees,
        );
        
        let celebration_id = celebration.id;
        self.celebration_events.push(celebration);
        
        info!("Created launch celebration event: {}", title);
        celebration_id
    }

    /// Record a community achievement during launch
    pub fn record_launch_achievement(&mut self, achievement: CommunityAchievement) {
        let user_id = achievement.recipient.clone().unwrap_or("community".to_string());
        
        self.achievement_tracking
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(achievement);
        
        info!("Recorded launch achievement: {}", achievement.title);
    }

    /// Add recognition for community members
    pub fn add_recognition(&mut self, recognition: CommunityRecognition) -> Uuid {
        let recognition_id = recognition.id;
        self.community_recognitions.push(recognition);
        info!("Added community recognition");
        recognition_id
    }

    /// Create a recognition for early adopters
    pub fn recognize_early_adopters(&mut self, adopters: Vec<String>, reason: String) -> Vec<Uuid> {
        let mut recognition_ids = Vec::new();
        
        for adopter in adopters {
            let recognition = CommunityRecognition::new(
                adopter.clone(),
                RecognitionType::EarlyAdopter,
                reason.clone(),
                "launch_system".to_string(),
                Some("Recognized for early participation in dashboard launch".to_string()),
            );
            
            let recognition_id = self.add_recognition(recognition);
            recognition_ids.push(recognition_id);
        }
        
        info!("Recognized {} early adopters", recognition_ids.len());
        recognition_ids
    }

    /// Create a recognition for active participants
    pub fn recognize_active_participants(&mut self, participants: Vec<String>, reason: String) -> Vec<Uuid> {
        let mut recognition_ids = Vec::new();
        
        for participant in participants {
            let recognition = CommunityRecognition::new(
                participant.clone(),
                RecognitionType::ActiveParticipant,
                reason.clone(),
                "launch_system".to_string(),
                Some("Recognized for active participation in dashboard activities".to_string()),
            );
            
            let recognition_id = self.add_recognition(recognition);
            recognition_ids.push(recognition_id);
        }
        
        info!("Recognized {} active participants", recognition_ids.len());
        recognition_ids
    }

    /// Create celebration materials for sharing
    pub fn create_celebration_material(&mut self, material: CelebrationMaterial) -> Uuid {
        let material_id = material.id;
        self.celebration_materials.push(material);
        info!("Created celebration material");
        material_id
    }

    /// Get upcoming celebration events
    pub fn get_upcoming_celebrations(&self) -> Vec<&CelebrationEvent> {
        let now = Utc::now();
        self.celebration_events.iter()
            .filter(|event| event.scheduled_for > now && !event.completed)
            .collect()
    }

    /// Get recent achievements
    pub fn get_recent_achievements(&self, days: i64) -> Vec<&CommunityAchievement> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.achievement_tracking.values()
            .flatten()
            .filter(|achievement| achievement.achieved_at > cutoff)
            .collect()
    }

    /// Get recognitions for a specific user
    pub fn get_user_recognitions(&self, user_id: &str) -> Vec<&CommunityRecognition> {
        self.community_recognitions.iter()
            .filter(|recognition| recognition.recognized_user == user_id)
            .collect()
    }

    /// Complete a celebration event
    pub fn complete_celebration(&mut self, celebration_id: Uuid) -> bool {
        if let Some(celebration) = self.celebration_events.iter_mut()
            .find(|c| c.id == celebration_id) {
            celebration.completed = true;
            celebration.completed_at = Some(Utc::now());
            info!("Marked celebration as completed: {}", celebration_id);
            true
        } else {
            false
        }
    }

    /// Generate celebration report
    pub fn generate_celebration_report(&self) -> CelebrationReport {
        let total_achievements: usize = self.achievement_tracking.values().map(|v| v.len()).sum();
        let total_celebrations = self.celebration_events.len();
        let completed_celebrations = self.celebration_events.iter()
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

    /// Create a transformation celebration
    pub fn celebrate_transformation(
        &mut self,
        title: String,
        description: String,
        participants: Vec<String>,
    ) -> Uuid {
        let celebration = CelebrationEvent::new(
            title,
            description,
            CelebrationType::Transformation,
            Utc::now(),
            participants,
        );
        
        let celebration_id = celebration.id;
        self.celebration_events.push(celebration);
        celebration_id
    }
}

/// Community achievement during launch
#[derive(Debug, Clone)]
pub struct CommunityAchievement {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub achievement_type: AchievementType,
    pub recipient: Option<String>, // User ID or None for community achievement
    pub participants: Option<Vec<String>>,
    pub evidence: Option<String>,
    pub achieved_at: DateTime<Utc>,
    pub celebrated: bool,
}

impl CommunityAchievement {
    /// Create a new community achievement
    pub fn new(
        title: String,
        description: String,
        achievement_type: AchievementType,
        recipient: Option<String>,
        participants: Option<Vec<String>>,
        evidence: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            achievement_type,
            recipient,
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
    FirstValidation,
    AdoptionMilestone,
    ImpactStory,
    Transformation,
    FacilitatorTraining,
    TechnicalMilestone,
    CommunityEngagement,
}

/// Celebration event
#[derive(Debug, Clone)]
pub struct CelebrationEvent {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub celebration_type: CelebrationType,
    pub scheduled_for: DateTime<Utc>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub honorees: Vec<String>,
    pub materials: Vec<Uuid>, // References to celebration materials
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
    CommunityWide,
    Recognition,
    Milestone,
    Transformation,
    Launch,
}

/// Community recognition
#[derive(Debug, Clone)]
pub struct CommunityRecognition {
    pub id: Uuid,
    pub recognized_user: String,
    pub recognition_type: RecognitionType,
    pub reason: String,
    pub recognized_by: String,
    pub recognized_at: DateTime<Utc>,
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
    EarlyAdopter,
    ActiveParticipant,
    Innovation,
    Leadership,
    Support,
    CommunityBuilder,
}

/// Celebration material for sharing
#[derive(Debug, Clone)]
pub struct CelebrationMaterial {
    pub id: Uuid,
    pub title: String,
    pub material_type: MaterialType,
    pub content: String, // Could be text, URL, or file path
    pub associated_event: Option<Uuid>,
    pub associated_achievement: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl CelebrationMaterial {
    /// Create a new celebration material
    pub fn new(
        title: String,
        material_type: MaterialType,
        content: String,
        associated_event: Option<Uuid>,
        associated_achievement: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            material_type,
            content,
            associated_event,
            associated_achievement,
            created_at: Utc::now(),
        }
    }
}

/// Types of celebration materials
#[derive(Debug, Clone)]
pub enum MaterialType {
    Photo,
    Video,
    Document,
    SocialMedia,
    Story,
    Testimonial,
}

/// Celebration report
#[derive(Debug, Clone)]
pub struct CelebrationReport {
    pub generated_at: DateTime<Utc>,
    pub total_achievements: usize,
    pub total_celebrations: usize,
    pub completed_celebrations: usize,
    pub recent_achievements: usize,
    pub upcoming_celebrations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_launch_celebration_initialization() {
        let celebration = LaunchCelebration::new();
        assert!(celebration.celebration_events.is_empty());
        assert!(celebration.community_recognitions.is_empty());
    }

    #[test]
    fn test_create_launch_celebration() {
        let mut celebration = LaunchCelebration::new();
        let celebration_id = celebration.create_launch_celebration(
            "Dashboard Launch".to_string(),
            "Celebrating our dashboard launch!".to_string(),
            vec!["user1".to_string(), "user2".to_string()],
        );
        
        assert!(!celebration_id.is_nil());
        assert_eq!(celebration.celebration_events.len(), 1);
    }

    #[test]
    fn test_record_launch_achievement() {
        let mut celebration = LaunchCelebration::new();
        let achievement = CommunityAchievement::new(
            "First Validation".to_string(),
            "Completed first community validation session".to_string(),
            AchievementType::FirstValidation,
            Some("user1".to_string()),
            Some(vec!["user1".to_string(), "user2".to_string()]),
            Some("Session notes and outcomes".to_string()),
        );
        
        celebration.record_launch_achievement(achievement);
        assert_eq!(celebration.achievement_tracking.len(), 1);
        assert!(celebration.achievement_tracking.contains_key("user1"));
    }

    #[test]
    fn test_recognize_early_adopters() {
        let mut celebration = LaunchCelebration::new();
        let adopters = vec!["user1".to_string(), "user2".to_string()];
        let recognition_ids = celebration.recognize_early_adopters(
            adopters,
            "Early participation in dashboard launch".to_string()
        );
        
        assert_eq!(recognition_ids.len(), 2);
        assert_eq!(celebration.community_recognitions.len(), 2);
    }

    #[test]
    fn test_get_upcoming_celebrations() {
        let mut celebration = LaunchCelebration::new();
        celebration.create_launch_celebration(
            "Future Celebration".to_string(),
            "A celebration in the future".to_string(),
            vec!["user1".to_string()],
        );
        
        let upcoming = celebration.get_upcoming_celebrations();
        assert_eq!(upcoming.len(), 1);
    }

    #[test]
    fn test_complete_celebration() {
        let mut celebration = LaunchCelebration::new();
        let celebration_id = celebration.create_launch_celebration(
            "Test Celebration".to_string(),
            "A test celebration".to_string(),
            vec!["user1".to_string()],
        );
        
        let result = celebration.complete_celebration(celebration_id);
        assert!(result);
        
        let celebration_event = celebration.celebration_events.iter()
            .find(|c| c.id == celebration_id)
            .unwrap();
        assert!(celebration_event.completed);
    }
}