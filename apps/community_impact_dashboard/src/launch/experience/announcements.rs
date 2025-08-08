//! Values-centered launch announcements for the Unified Community Impact Dashboard
//!
//! This module provides community-specific, values-aligned announcements that
//! celebrate the launch while honoring community rhythms and preferences.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Values-centered launch announcement system
pub struct LaunchAnnouncement {
    community_values: Vec<String>,
    announcement_templates: HashMap<AnnouncementType, Vec<AnnouncementTemplate>>,
    sent_announcements: Vec<SentAnnouncement>,
}

impl LaunchAnnouncement {
    /// Create a new launch announcement system
    pub fn new(community_values: Vec<String>) -> Self {
        Self {
            community_values,
            announcement_templates: HashMap::new(),
            sent_announcements: Vec::new(),
        }
    }

    /// Initialize announcement templates aligned with community values
    pub fn initialize_templates(&mut self) {
        // Launch announcement templates
        let launch_templates = vec![
            AnnouncementTemplate::new(
                "launch_celebration".to_string(),
                "Celebrating Our Community's Launch!".to_string(),
                "We're excited to announce the launch of our community's Unified Impact Dashboard - a tool that shows how our individual actions connect to create collective transformation.".to_string(),
                AnnouncementType::Launch,
                AnnouncementFormat::CommunityCentered,
            ),
            AnnouncementTemplate::new(
                "values_aligned_launch".to_string(),
                "Launching with Our Values at the Heart".to_string(),
                "This dashboard reflects our community's commitment to collaboration, transparency, and collective benefit. Together, we're not just measuring impact - we're understanding and optimizing it.".to_string(),
                AnnouncementType::Launch,
                AnnouncementFormat::ValuesCentered,
            ),
        ];
        self.announcement_templates.insert(AnnouncementType::Launch, launch_templates);

        // Milestone announcement templates
        let milestone_templates = vec![
            AnnouncementTemplate::new(
                "beta_milestone".to_string(),
                "Beta Phase Milestone Reached!".to_string(),
                "Thank you to our beta testers for your invaluable feedback! Your participation is helping shape a dashboard that truly serves our community.".to_string(),
                AnnouncementType::Milestone,
                AnnouncementFormat::CommunityCentered,
            ),
            AnnouncementTemplate::new(
                "early_adopter_milestone".to_string(),
                "Early Adopters Making a Difference".to_string(),
                "Our early adopters are leading the way in understanding interconnected impact. Your engagement is creating pathways for others to follow.".to_string(),
                AnnouncementType::Milestone,
                AnnouncementFormat::Recognition,
            ),
        ];
        self.announcement_templates.insert(AnnouncementType::Milestone, milestone_templates);

        // Celebration announcement templates
        let celebration_templates = vec![
            AnnouncementTemplate::new(
                "community_achievement".to_string(),
                "Celebrating Our Collective Achievement!".to_string(),
                "Together, we've reached a significant milestone in our journey to understand interconnected impact. This achievement belongs to all of us.".to_string(),
                AnnouncementType::Celebration,
                AnnouncementFormat::CommunityCentered,
            ),
        ];
        self.announcement_templates.insert(AnnouncementType::Celebration, celebration_templates);
    }

    /// Create a personalized announcement for the community
    pub fn create_personalized_announcement(
        &self,
        announcement_type: AnnouncementType,
        community_context: &CommunityContext,
    ) -> Result<PersonalizedAnnouncement, AnnouncementError> {
        let templates = self.announcement_templates.get(&announcement_type)
            .ok_or(AnnouncementError::NoTemplatesForType(format!("{:?}", announcement_type)))?;

        // Find the best matching template based on community values
        let template = self.find_best_template(templates, community_context);

        let personalized = PersonalizedAnnouncement::new(
            template.title.clone(),
            template.content.clone(),
            announcement_type,
            template.format.clone(),
            community_context.name.clone(),
        );

        Ok(personalized)
    }

    /// Find the best template based on community values and context
    fn find_best_template(
        &self,
        templates: &[AnnouncementTemplate],
        community_context: &CommunityContext,
    ) -> &AnnouncementTemplate {
        // Simple matching algorithm - in a real implementation, this would be more sophisticated
        // For now, we'll just return the first template as a placeholder
        &templates[0]
    }

    /// Send an announcement to community members
    pub fn send_announcement(
        &mut self,
        announcement: PersonalizedAnnouncement,
        recipients: Vec<String>,
    ) -> Result<Uuid, AnnouncementError> {
        let sent_announcement = SentAnnouncement::new(
            announcement.id,
            recipients.len(),
            announcement.announcement_type.clone(),
        );

        self.sent_announcements.push(sent_announcement);
        
        info!("Sent {} announcement to {} recipients", 
              announcement.announcement_type, 
              recipients.len());
        
        Ok(announcement.id)
    }

    /// Get announcement statistics
    pub fn get_announcement_stats(&self) -> AnnouncementStats {
        let total_sent = self.sent_announcements.len();
        let by_type: HashMap<AnnouncementType, usize> = self.sent_announcements.iter()
            .fold(HashMap::new(), |mut acc, ann| {
                *acc.entry(ann.announcement_type.clone()).or_insert(0) += 1;
                acc
            });

        AnnouncementStats {
            total_sent,
            sent_by_type: by_type,
            last_sent: self.sent_announcements.last().map(|a| a.sent_at),
        }
    }

    /// Customize announcement language for community context
    pub fn customize_language(&self, announcement: &mut PersonalizedAnnouncement, context: &CommunityContext) {
        // In a real implementation, this would customize the language based on:
        // - Community cultural context
        // - Preferred communication styles
        // - Accessibility needs
        // - Historical communication patterns
        // For now, we'll just log that customization would happen
        info!("Customizing announcement language for community: {}", context.name);
    }
}

/// Community context for announcement personalization
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

/// Types of announcements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AnnouncementType {
    Launch,
    Milestone,
    Celebration,
    Update,
    FeedbackRequest,
}

/// Formats for announcements
#[derive(Debug, Clone)]
pub enum AnnouncementFormat {
    CommunityCentered,
    ValuesCentered,
    Recognition,
    StoryBased,
    Educational,
}

/// Template for announcements
#[derive(Debug, Clone)]
pub struct AnnouncementTemplate {
    pub id: String,
    pub title: String,
    pub content: String,
    pub announcement_type: AnnouncementType,
    pub format: AnnouncementFormat,
    pub created_at: DateTime<Utc>,
}

impl AnnouncementTemplate {
    /// Create a new announcement template
    pub fn new(
        id: String,
        title: String,
        content: String,
        announcement_type: AnnouncementType,
        format: AnnouncementFormat,
    ) -> Self {
        Self {
            id,
            title,
            content,
            announcement_type,
            format,
            created_at: Utc::now(),
        }
    }
}

/// Personalized announcement for a specific community
#[derive(Debug, Clone)]
pub struct PersonalizedAnnouncement {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub announcement_type: AnnouncementType,
    pub format: AnnouncementFormat,
    pub community_name: String,
    pub created_at: DateTime<Utc>,
}

impl PersonalizedAnnouncement {
    /// Create a new personalized announcement
    pub fn new(
        title: String,
        content: String,
        announcement_type: AnnouncementType,
        format: AnnouncementFormat,
        community_name: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            announcement_type,
            format,
            community_name,
            created_at: Utc::now(),
        }
    }
}

/// Record of sent announcement
#[derive(Debug, Clone)]
pub struct SentAnnouncement {
    pub announcement_id: Uuid,
    pub recipient_count: usize,
    pub announcement_type: AnnouncementType,
    pub sent_at: DateTime<Utc>,
}

impl SentAnnouncement {
    /// Create a new sent announcement record
    pub fn new(
        announcement_id: Uuid,
        recipient_count: usize,
        announcement_type: AnnouncementType,
    ) -> Self {
        Self {
            announcement_id,
            recipient_count,
            announcement_type,
            sent_at: Utc::now(),
        }
    }
}

/// Statistics about announcement sending
#[derive(Debug, Clone)]
pub struct AnnouncementStats {
    pub total_sent: usize,
    pub sent_by_type: HashMap<AnnouncementType, usize>,
    pub last_sent: Option<DateTime<Utc>>,
}

/// Error types for announcement system
#[derive(Debug)]
pub enum AnnouncementError {
    NoTemplatesForType(String),
    TemplateNotFound(String),
    SendFailure(String),
}

impl std::fmt::Display for AnnouncementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnnouncementError::NoTemplatesForType(type_name) => {
                write!(f, "No templates found for announcement type: {}", type_name)
            }
            AnnouncementError::TemplateNotFound(id) => {
                write!(f, "Template not found: {}", id)
            }
            AnnouncementError::SendFailure(reason) => {
                write!(f, "Failed to send announcement: {}", reason)
            }
        }
    }
}

impl std::error::Error for AnnouncementError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcement_initialization() {
        let values = vec!["collaboration".to_string(), "transparency".to_string()];
        let mut announcement = LaunchAnnouncement::new(values);
        announcement.initialize_templates();
        
        assert!(!announcement.announcement_templates.is_empty());
        assert!(announcement.announcement_templates.contains_key(&AnnouncementType::Launch));
        assert!(announcement.announcement_templates.contains_key(&AnnouncementType::Milestone));
    }

    #[test]
    fn test_create_personalized_announcement() {
        let values = vec!["collaboration".to_string(), "transparency".to_string()];
        let mut announcement = LaunchAnnouncement::new(values);
        announcement.initialize_templates();
        
        let context = CommunityContext {
            name: "Test Community".to_string(),
            values: vec!["collaboration".to_string()],
            culture: "diverse".to_string(),
            communication_preferences: vec![CommunicationPreference::Textual],
            accessibility_needs: vec![AccessibilityNeed::None],
        };
        
        let result = announcement.create_personalized_announcement(AnnouncementType::Launch, &context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_announcement() {
        let values = vec!["collaboration".to_string(), "transparency".to_string()];
        let mut announcement = LaunchAnnouncement::new(values);
        
        let personalized = PersonalizedAnnouncement::new(
            "Test Announcement".to_string(),
            "This is a test announcement".to_string(),
            AnnouncementType::Launch,
            AnnouncementFormat::CommunityCentered,
            "Test Community".to_string(),
        );
        
        let recipients = vec!["user1".to_string(), "user2".to_string()];
        let result = announcement.send_announcement(personalized, recipients);
        assert!(result.is_ok());
        
        let stats = announcement.get_announcement_stats();
        assert_eq!(stats.total_sent, 1);
    }
}