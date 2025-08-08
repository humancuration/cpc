//! Community-specific welcome experiences for the Unified Community Impact Dashboard
//!
//! This module provides personalized welcome experiences that honor community
//! values, rhythms, and preferences while gently introducing dashboard concepts.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Community-specific welcome experience
pub struct WelcomeExperience {
    community_context: CommunityContext,
    welcome_sequences: HashMap<String, WelcomeSequence>,
    user_welcome_history: HashMap<String, UserWelcomeProgress>,
}

impl WelcomeExperience {
    /// Create a new welcome experience system
    pub fn new(community_context: CommunityContext) -> Self {
        Self {
            community_context,
            welcome_sequences: HashMap::new(),
            user_welcome_history: HashMap::new(),
        }
    }

    /// Initialize welcome sequences for different user types
    pub fn initialize_welcome_sequences(&mut self) {
        // Beta tester welcome sequence
        let beta_sequence = WelcomeSequence::new(
            "beta_welcome".to_string(),
            "Beta Tester Welcome Experience".to_string(),
            vec![
                WelcomeStep::new(
                    "introduction".to_string(),
                    "Welcome to the Beta Testing Phase!".to_string(),
                    WelcomeContentType::Text("Thank you for being part of our beta testing community. Your feedback will help shape the dashboard experience for everyone.".to_string()),
                    1,
                ),
                WelcomeStep::new(
                    "dashboard_preview".to_string(),
                    "Dashboard Preview".to_string(),
                    WelcomeContentType::Interactive,
                    2,
                ),
                WelcomeStep::new(
                    "feedback_mechanism".to_string(),
                    "Your Feedback Matters".to_string(),
                    WelcomeContentType::Text("We've created special feedback channels for beta testers. Please share your thoughts as you explore!".to_string()),
                    3,
                ),
            ],
        );
        self.welcome_sequences.insert("beta_welcome".to_string(), beta_sequence);

        // Early adopter welcome sequence
        let early_adopter_sequence = WelcomeSequence::new(
            "early_adopter_welcome".to_string(),
            "Early Adopter Welcome Experience".to_string(),
            vec![
                WelcomeStep::new(
                    "community_welcome".to_string(),
                    "Welcome Pioneer!".to_string(),
                    WelcomeContentType::Text("As an early adopter, you're helping us build understanding of interconnected impact in our community.".to_string()),
                    1,
                ),
                WelcomeStep::new(
                    "values_alignment".to_string(),
                    "Our Shared Values".to_string(),
                    WelcomeContentType::Text("This dashboard is built on principles of community benefit, reciprocity, transparency, and inclusivity.".to_string()),
                    2,
                ),
                WelcomeStep::new(
                    "getting_started".to_string(),
                    "Getting Started Guide".to_string(),
                    WelcomeContentType::Interactive,
                    3,
                ),
            ],
        );
        self.welcome_sequences.insert("early_adopter_welcome".to_string(), early_adopter_sequence);

        // General community member welcome sequence
        let community_sequence = WelcomeSequence::new(
            "community_welcome".to_string(),
            "Community Member Welcome Experience".to_string(),
            vec![
                WelcomeStep::new(
                    "belonging".to_string(),
                    "You Belong Here".to_string(),
                    WelcomeContentType::Text("Welcome to our community's interconnected impact journey. Your participation makes our collective understanding richer.".to_string()),
                    1,
                ),
                WelcomeStep::new(
                    "community_impact".to_string(),
                    "Our Community's Impact Story".to_string(),
                    WelcomeContentType::Story,
                    2,
                ),
                WelcomeStep::new(
                    "exploration_invitation".to_string(),
                    "Explore at Your Pace".to_string(),
                    WelcomeContentType::Text("Take your time to explore. There's no rush - understanding interconnected impact is a journey.".to_string()),
                    3,
                ),
            ],
        );
        self.welcome_sequences.insert("community_welcome".to_string(), community_sequence);
    }

    /// Start welcome experience for a user
    pub fn start_welcome_experience(&mut self, user_id: String, user_type: UserType) -> Result<Uuid, WelcomeError> {
        let sequence_key = match user_type {
            UserType::BetaTester => "beta_welcome",
            UserType::EarlyAdopter => "early_adopter_welcome",
            UserType::CommunityMember => "community_welcome",
            UserType::Facilitator => "early_adopter_welcome", // Facilitators get early adopter experience
            UserType::Admin => "early_adopter_welcome", // Admins get early adopter experience
        };

        let sequence = self.welcome_sequences.get(sequence_key)
            .ok_or(WelcomeError::SequenceNotFound(sequence_key.to_string()))?;

        let welcome_progress = UserWelcomeProgress::new(
            user_id.clone(),
            sequence.id.clone(),
            sequence.steps.len(),
        );

        self.user_welcome_history.insert(user_id.clone(), welcome_progress);
        
        info!("Started welcome experience for user {}: {}", user_id, sequence_key);
        Ok(sequence.id)
    }

    /// Get next welcome step for a user
    pub fn get_next_welcome_step(&self, user_id: &str) -> Result<Option<&WelcomeStep>, WelcomeError> {
        let progress = self.user_welcome_history.get(user_id)
            .ok_or(WelcomeError::UserNotInWelcomeProcess(user_id.to_string()))?;

        let sequence = self.welcome_sequences.get(&progress.sequence_id)
            .ok_or(WelcomeError::SequenceNotFound(progress.sequence_id.to_string()))?;

        if progress.current_step >= sequence.steps.len() {
            return Ok(None); // Welcome experience completed
        }

        Ok(Some(&sequence.steps[progress.current_step]))
    }

    /// Complete current welcome step for a user
    pub fn complete_welcome_step(&mut self, user_id: &str) -> Result<(), WelcomeError> {
        let progress = self.user_welcome_history.get_mut(user_id)
            .ok_or(WelcomeError::UserNotInWelcomeProcess(user_id.to_string()))?;

        progress.current_step += 1;
        progress.last_updated = Utc::now();

        if progress.current_step >= progress.total_steps {
            progress.completed = true;
            info!("Welcome experience completed for user: {}", user_id);
        }

        Ok(())
    }

    /// Get welcome progress for a user
    pub fn get_welcome_progress(&self, user_id: &str) -> Result<&UserWelcomeProgress, WelcomeError> {
        self.user_welcome_history.get(user_id)
            .ok_or(WelcomeError::UserNotInWelcomeProcess(user_id.to_string()))
    }

    /// Customize welcome experience for community context
    pub fn customize_for_community(&mut self, context: CommunityContext) {
        self.community_context = context;
        // In a real implementation, this would customize the welcome sequences
        // based on community-specific values, culture, and preferences
    }
}

/// Community context for personalization
#[derive(Debug, Clone)]
pub struct CommunityContext {
    pub name: String,
    pub values: Vec<String>,
    pub culture: String,
    pub communication_preferences: Vec<CommunicationPreference>,
    pub accessibility_needs: Vec<AccessibilityNeed>,
}

/// Communication preferences for community members
#[derive(Debug, Clone)]
pub enum CommunicationPreference {
    Visual,
    Textual,
    Interactive,
    StoryBased,
}

/// Accessibility needs for inclusive design
#[derive(Debug, Clone)]
pub enum AccessibilityNeed {
    Visual,
    Auditory,
    Motor,
    Cognitive,
    None,
}

/// User types in the welcome system
#[derive(Debug, Clone)]
pub enum UserType {
    Admin,
    Facilitator,
    BetaTester,
    EarlyAdopter,
    CommunityMember,
}

/// Welcome sequence containing multiple steps
#[derive(Debug, Clone)]
pub struct WelcomeSequence {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub steps: Vec<WelcomeStep>,
    pub created_at: DateTime<Utc>,
}

impl WelcomeSequence {
    /// Create a new welcome sequence
    pub fn new(name: String, description: String, steps: Vec<WelcomeStep>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            steps,
            created_at: Utc::now(),
        }
    }
}

/// Individual welcome step
#[derive(Debug, Clone)]
pub struct WelcomeStep {
    pub id: String,
    pub title: String,
    pub content_type: WelcomeContentType,
    pub step_number: usize,
    pub created_at: DateTime<Utc>,
}

impl WelcomeStep {
    /// Create a new welcome step
    pub fn new(id: String, title: String, content_type: WelcomeContentType, step_number: usize) -> Self {
        Self {
            id,
            title,
            content_type,
            step_number,
            created_at: Utc::now(),
        }
    }
}

/// Types of welcome content
#[derive(Debug, Clone)]
pub enum WelcomeContentType {
    Text(String),
    Interactive,
    Story,
    Video(String), // URL to video
    Audio(String), // URL to audio
}

/// User's progress through welcome experience
#[derive(Debug, Clone)]
pub struct UserWelcomeProgress {
    pub user_id: String,
    pub sequence_id: Uuid,
    pub current_step: usize,
    pub total_steps: usize,
    pub completed: bool,
    pub started_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

impl UserWelcomeProgress {
    /// Create new user welcome progress
    pub fn new(user_id: String, sequence_id: Uuid, total_steps: usize) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            sequence_id,
            current_step: 0,
            total_steps,
            completed: false,
            started_at: now,
            last_updated: now,
        }
    }
}

/// Error types for welcome experience
#[derive(Debug)]
pub enum WelcomeError {
    SequenceNotFound(String),
    UserNotInWelcomeProcess(String),
    WelcomeAlreadyCompleted(String),
}

impl std::fmt::Display for WelcomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WelcomeError::SequenceNotFound(id) => write!(f, "Welcome sequence not found: {}", id),
            WelcomeError::UserNotInWelcomeProcess(id) => write!(f, "User not in welcome process: {}", id),
            WelcomeError::WelcomeAlreadyCompleted(id) => write!(f, "Welcome already completed for user: {}", id),
        }
    }
}

impl std::error::Error for WelcomeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welcome_experience_initialization() {
        let context = CommunityContext {
            name: "Test Community".to_string(),
            values: vec!["collaboration".to_string(), "transparency".to_string()],
            culture: "diverse".to_string(),
            communication_preferences: vec![CommunicationPreference::StoryBased],
            accessibility_needs: vec![AccessibilityNeed::None],
        };
        
        let mut welcome = WelcomeExperience::new(context);
        welcome.initialize_welcome_sequences();
        
        assert!(!welcome.welcome_sequences.is_empty());
        assert!(welcome.welcome_sequences.contains_key("beta_welcome"));
        assert!(welcome.welcome_sequences.contains_key("early_adopter_welcome"));
        assert!(welcome.welcome_sequences.contains_key("community_welcome"));
    }

    #[test]
    fn test_start_welcome_experience() {
        let context = CommunityContext {
            name: "Test Community".to_string(),
            values: vec!["collaboration".to_string(), "transparency".to_string()],
            culture: "diverse".to_string(),
            communication_preferences: vec![CommunicationPreference::StoryBased],
            accessibility_needs: vec![AccessibilityNeed::None],
        };
        
        let mut welcome = WelcomeExperience::new(context);
        welcome.initialize_welcome_sequences();
        
        let result = welcome.start_welcome_experience("user123".to_string(), UserType::BetaTester);
        assert!(result.is_ok());
    }

    #[test]
    fn test_welcome_step_progression() {
        let context = CommunityContext {
            name: "Test Community".to_string(),
            values: vec!["collaboration".to_string(), "transparency".to_string()],
            culture: "diverse".to_string(),
            communication_preferences: vec![CommunicationPreference::StoryBased],
            accessibility_needs: vec![AccessibilityNeed::None],
        };
        
        let mut welcome = WelcomeExperience::new(context);
        welcome.initialize_welcome_sequences();
        
        let _ = welcome.start_welcome_experience("user123".to_string(), UserType::BetaTester);
        
        let step1 = welcome.get_next_welcome_step("user123").unwrap();
        assert!(step1.is_some());
        assert_eq!(step1.unwrap().title, "Welcome to the Beta Testing Phase!");
        
        let _ = welcome.complete_welcome_step("user123");
        
        let step2 = welcome.get_next_welcome_step("user123").unwrap();
        assert!(step2.is_some());
        assert_eq!(step2.unwrap().title, "Dashboard Preview");
    }
}