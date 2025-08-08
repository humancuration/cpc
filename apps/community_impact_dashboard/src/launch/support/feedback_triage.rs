//! Community feedback triage and response system for the Unified Community Impact Dashboard
//!
//! This module provides systematic triage of community feedback, routing issues
//! to appropriate responders, and ensuring community input is acknowledged and acted upon.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Community feedback triage system
pub struct FeedbackTriage {
    feedback_items: Vec<FeedbackItem>,
    triage_rules: Vec<TriageRule>,
    response_templates: HashMap<FeedbackCategory, Vec<ResponseTemplate>>,
    feedback_assignments: HashMap<Uuid, String>,
    response_times: Vec<ResponseTime>,
    triage_statistics: TriageStatistics,
}

impl FeedbackTriage {
    /// Create a new feedback triage system
    pub fn new() -> Self {
        Self {
            feedback_items: Vec::new(),
            triage_rules: Vec::new(),
            response_templates: HashMap::new(),
            feedback_assignments: HashMap::new(),
            response_times: Vec::new(),
            triage_statistics: TriageStatistics::new(),
        }
    }

    /// Add a feedback item for triage
    pub fn add_feedback(&mut self, feedback: FeedbackItem) -> Uuid {
        let feedback_id = feedback.id;
        self.feedback_items.push(feedback);
        self.triage_statistics.total_feedback += 1;
        info!("Added feedback for triage: {}", feedback_id);
        feedback_id
    }

    /// Add a triage rule
    pub fn add_triage_rule(&mut self, rule: TriageRule) {
        self.triage_rules.push(rule);
        info!("Added triage rule: {}", rule.id);
    }

    /// Add a response template
    pub fn add_response_template(&mut self, template: ResponseTemplate) {
        self.response_templates
            .entry(template.category.clone())
            .or_insert_with(Vec::new)
            .push(template);
        info!("Added response template for category: {:?}", template.category);
    }

    /// Automatically triage feedback items based on rules
    pub fn auto_triage_feedback(&mut self) -> Result<usize, TriageError> {
        let mut triaged_count = 0;
        
        for feedback in self.feedback_items.iter_mut() {
            if feedback.status == FeedbackStatus::Pending {
                // Apply triage rules
                for rule in &self.triage_rules {
                    if rule.matches_feedback(feedback) {
                        feedback.category = rule.target_category.clone();
                        feedback.priority = rule.target_priority.clone();
                        feedback.status = FeedbackStatus::Triaged;
                        feedback.triaged_at = Some(Utc::now());
                        
                        // Assign to responder if specified
                        if let Some(responder) = &rule.auto_assign_to {
                            self.assign_feedback(feedback.id, responder.clone())?;
                        }
                        
                        triaged_count += 1;
                        self.triage_statistics.auto_triaged += 1;
                        break;
                    }
                }
            }
        }
        
        info!("Auto-triaged {} feedback items", triaged_count);
        Ok(triaged_count)
    }

    /// Manually triage a feedback item
    pub fn manual_triage(
        &mut self,
        feedback_id: Uuid,
        category: FeedbackCategory,
        priority: FeedbackPriority,
        assign_to: Option<String>,
    ) -> Result<(), TriageError> {
        let feedback = self.feedback_items.iter_mut()
            .find(|f| f.id == feedback_id)
            .ok_or(TriageError::FeedbackNotFound(feedback_id))?;

        feedback.category = category;
        feedback.priority = priority;
        feedback.status = FeedbackStatus::Triaged;
        feedback.triaged_at = Some(Utc::now());
        
        if let Some(assignee) = assign_to {
            self.assign_feedback(feedback_id, assignee)?;
        }
        
        self.triage_statistics.manually_triaged += 1;
        info!("Manually triaged feedback: {}", feedback_id);
        Ok(())
    }

    /// Assign feedback to a responder
    pub fn assign_feedback(&mut self, feedback_id: Uuid, assignee: String) -> Result<(), TriageError> {
        let feedback = self.feedback_items.iter_mut()
            .find(|f| f.id == feedback_id)
            .ok_or(TriageError::FeedbackNotFound(feedback_id))?;

        feedback.assigned_to = Some(assignee.clone());
        feedback.assigned_at = Some(Utc::now());
        self.feedback_assignments.insert(feedback_id, assignee);
        
        info!("Assigned feedback {} to {}", feedback_id, feedback.assigned_to.as_ref().unwrap());
        Ok(())
    }

    /// Send a response to feedback
    pub fn send_response(&mut self, response: FeedbackResponse) -> Result<Uuid, TriageError> {
        let feedback = self.feedback_items.iter_mut()
            .find(|f| f.id == response.feedback_id)
            .ok_or(TriageError::FeedbackNotFound(response.feedback_id))?;

        // Record response time
        if let Some(submitted_at) = feedback.submitted_at {
            let response_time = ResponseTime {
                feedback_id: response.feedback_id,
                duration: Utc::now().signed_duration_since(submitted_at).num_seconds() as f64,
                responded_at: Utc::now(),
            };
            self.response_times.push(response_time);
        }

        feedback.responses.push(response.clone());
        feedback.status = FeedbackStatus::Responded;
        feedback.last_updated = Utc::now();
        
        self.triage_statistics.responded += 1;
        info!("Sent response to feedback: {}", response.feedback_id);
        Ok(response.id)
    }

    /// Get feedback items by status
    pub fn get_feedback_by_status(&self, status: FeedbackStatus) -> Vec<&FeedbackItem> {
        self.feedback_items.iter()
            .filter(|f| f.status == status)
            .collect()
    }

    /// Get feedback items by category
    pub fn get_feedback_by_category(&self, category: FeedbackCategory) -> Vec<&FeedbackItem> {
        self.feedback_items.iter()
            .filter(|f| f.category == category)
            .collect()
    }

    /// Get feedback items by priority
    pub fn get_feedback_by_priority(&self, priority: FeedbackPriority) -> Vec<&FeedbackItem> {
        self.feedback_items.iter()
            .filter(|f| f.priority == priority)
            .collect()
    }

    /// Get assigned feedback for a responder
    pub fn get_assigned_feedback(&self, assignee: &str) -> Vec<&FeedbackItem> {
        self.feedback_items.iter()
            .filter(|f| f.assigned_to.as_deref() == Some(assignee))
            .collect()
    }

    /// Get pending feedback (not yet triaged)
    pub fn get_pending_feedback(&self) -> Vec<&FeedbackItem> {
        self.feedback_items.iter()
            .filter(|f| f.status == FeedbackStatus::Pending)
            .collect()
    }

    /// Escalate feedback that needs higher attention
    pub fn escalate_feedback(&mut self, feedback_id: Uuid, reason: String) -> Result<(), TriageError> {
        let feedback = self.feedback_items.iter_mut()
            .find(|f| f.id == feedback_id)
            .ok_or(TriageError::FeedbackNotFound(feedback_id))?;

        feedback.escalated = true;
        feedback.escalation_reason = Some(reason);
        feedback.escalated_at = Some(Utc::now());
        self.triage_statistics.escalated += 1;
        
        info!("Escalated feedback: {}", feedback_id);
        Ok(())
    }

    /// Get response time statistics
    pub fn get_response_time_statistics(&self) -> ResponseTimeStatistics {
        if self.response_times.is_empty() {
            return ResponseTimeStatistics {
                average_response_time: 0.0,
                median_response_time: 0.0,
                fastest_response: 0.0,
                slowest_response: 0.0,
                total_responded: 0,
            };
        }

        let mut times: Vec<f64> = self.response_times.iter()
            .map(|rt| rt.duration)
            .collect();
        
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let average = times.iter().sum::<f64>() / times.len() as f64;
        let median = if times.len() % 2 == 0 {
            (times[times.len() / 2 - 1] + times[times.len() / 2]) / 2.0
        } else {
            times[times.len() / 2]
        };
        
        ResponseTimeStatistics {
            average_response_time: average,
            median_response_time: median,
            fastest_response: *times.first().unwrap(),
            slowest_response: *times.last().unwrap(),
            total_responded: times.len(),
        }
    }

    /// Get triage statistics
    pub fn get_statistics(&self) -> &TriageStatistics {
        &self.triage_statistics
    }

    /// Generate feedback triage report
    pub fn generate_triage_report(&self) -> TriageReport {
        let pending_feedback = self.get_pending_feedback().len();
        let by_category: HashMap<FeedbackCategory, usize> = self.feedback_items.iter()
            .fold(HashMap::new(), |mut acc, feedback| {
                *acc.entry(feedback.category.clone()).or_insert(0) += 1;
                acc
            });
        
        let response_stats = self.get_response_time_statistics();
        
        TriageReport {
            generated_at: Utc::now(),
            total_feedback: self.feedback_items.len(),
            pending_feedback,
            triaged_feedback: self.triage_statistics.auto_triaged + self.triage_statistics.manually_triaged,
            responded_feedback: self.triage_statistics.responded,
            escalated_feedback: self.triage_statistics.escalated,
            feedback_by_category: by_category,
            response_time_statistics: response_stats,
        }
    }

    /// Get appropriate response template for a feedback category
    pub fn get_response_template(&self, category: FeedbackCategory) -> Option<&ResponseTemplate> {
        self.response_templates.get(&category)
            .and_then(|templates| templates.first())
    }

    /// Close feedback as resolved
    pub fn close_feedback(&mut self, feedback_id: Uuid, resolution: String) -> Result<(), TriageError> {
        let feedback = self.feedback_items.iter_mut()
            .find(|f| f.id == feedback_id)
            .ok_or(TriageError::FeedbackNotFound(feedback_id))?;

        feedback.status = FeedbackStatus::Closed;
        feedback.resolution = Some(resolution);
        feedback.closed_at = Some(Utc::now());
        self.triage_statistics.closed += 1;
        
        info!("Closed feedback: {}", feedback_id);
        Ok(())
    }
}

/// Feedback item from community members
#[derive(Debug, Clone)]
pub struct FeedbackItem {
    pub id: Uuid,
    pub submitter: String,
    pub content: String,
    pub category: FeedbackCategory,
    pub priority: FeedbackPriority,
    pub status: FeedbackStatus,
    pub submitted_at: DateTime<Utc>,
    pub triaged_at: Option<DateTime<Utc>>,
    pub assigned_to: Option<String>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub responses: Vec<FeedbackResponse>,
    pub escalated: bool,
    pub escalation_reason: Option<String>,
    pub escalated_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub closed_at: Option<DateTime<Utc>>,
    pub last_updated: DateTime<Utc>,
    pub tags: Vec<String>,
}

impl FeedbackItem {
    /// Create a new feedback item
    pub fn new(submitter: String, content: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            submitter,
            content,
            category: FeedbackCategory::General,
            priority: FeedbackPriority::Medium,
            status: FeedbackStatus::Pending,
            submitted_at: now,
            triaged_at: None,
            assigned_to: None,
            assigned_at: None,
            responses: Vec::new(),
            escalated: false,
            escalation_reason: None,
            escalated_at: None,
            resolution: None,
            closed_at: None,
            last_updated: now,
            tags: Vec::new(),
        }
    }

    /// Add tags to the feedback item
    pub fn add_tags(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
        self.last_updated = Utc::now();
    }
}

/// Response to feedback
#[derive(Debug, Clone)]
pub struct FeedbackResponse {
    pub id: Uuid,
    pub feedback_id: Uuid,
    pub responder: String,
    pub content: String,
    pub response_type: ResponseType,
    pub sent_at: DateTime<Utc>,
    pub is_automated: bool,
}

impl FeedbackResponse {
    /// Create a new feedback response
    pub fn new(feedback_id: Uuid, responder: String, content: String, response_type: ResponseType) -> Self {
        Self {
            id: Uuid::new_v4(),
            feedback_id,
            responder,
            content,
            response_type,
            sent_at: Utc::now(),
            is_automated: false,
        }
    }
}

/// Triage rule for automatic feedback categorization
#[derive(Debug, Clone)]
pub struct TriageRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub target_category: FeedbackCategory,
    pub target_priority: FeedbackPriority,
    pub auto_assign_to: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl TriageRule {
    /// Create a new triage rule
    pub fn new(
        id: String,
        name: String,
        description: String,
        keywords: Vec<String>,
        target_category: FeedbackCategory,
        target_priority: FeedbackPriority,
        auto_assign_to: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            keywords,
            target_category,
            target_priority,
            auto_assign_to,
            created_at: Utc::now(),
        }
    }

    /// Check if this rule matches a feedback item
    pub fn matches_feedback(&self, feedback: &FeedbackItem) -> bool {
        self.keywords.iter().any(|keyword| {
            feedback.content.to_lowercase().contains(&keyword.to_lowercase())
        })
    }
}

/// Response template for common feedback categories
#[derive(Debug, Clone)]
pub struct ResponseTemplate {
    pub id: String,
    pub name: String,
    pub category: FeedbackCategory,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ResponseTemplate {
    /// Create a new response template
    pub fn new(id: String, name: String, category: FeedbackCategory, content: String) -> Self {
        Self {
            id,
            name,
            category,
            content,
            created_at: Utc::now(),
        }
    }
}

/// Response time tracking
#[derive(Debug, Clone)]
pub struct ResponseTime {
    pub feedback_id: Uuid,
    pub duration: f64, // in seconds
    pub responded_at: DateTime<Utc>,
}

/// Categories of feedback
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeedbackCategory {
    Technical,
    Usability,
    Content,
    Performance,
    Accessibility,
    FeatureRequest,
    BugReport,
    Onboarding,
    Validation,
    Storytelling,
    General,
}

/// Priority levels for feedback
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of feedback items
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackStatus {
    Pending,
    Triaged,
    Assigned,
    Responded,
    Escalated,
    Closed,
}

/// Types of responses
#[derive(Debug, Clone)]
pub enum ResponseType {
    Acknowledgment,
    Solution,
    FollowUp,
    Escalation,
    Closure,
}

/// Statistics about feedback triage
#[derive(Debug, Clone)]
pub struct TriageStatistics {
    pub total_feedback: usize,
    pub auto_triaged: usize,
    pub manually_triaged: usize,
    pub responded: usize,
    pub escalated: usize,
    pub closed: usize,
}

impl TriageStatistics {
    /// Create new triage statistics
    pub fn new() -> Self {
        Self {
            total_feedback: 0,
            auto_triaged: 0,
            manually_triaged: 0,
            responded: 0,
            escalated: 0,
            closed: 0,
        }
    }
}

/// Statistics about response times
#[derive(Debug, Clone)]
pub struct ResponseTimeStatistics {
    pub average_response_time: f64,
    pub median_response_time: f64,
    pub fastest_response: f64,
    pub slowest_response: f64,
    pub total_responded: usize,
}

/// Feedback triage report
#[derive(Debug, Clone)]
pub struct TriageReport {
    pub generated_at: DateTime<Utc>,
    pub total_feedback: usize,
    pub pending_feedback: usize,
    pub triaged_feedback: usize,
    pub responded_feedback: usize,
    pub escalated_feedback: usize,
    pub feedback_by_category: HashMap<FeedbackCategory, usize>,
    pub response_time_statistics: ResponseTimeStatistics,
}

/// Error types for feedback triage system
#[derive(Debug)]
pub enum TriageError {
    FeedbackNotFound(Uuid),
    AssignmentError(String),
    ResponseError(String),
    RuleError(String),
}

impl std::fmt::Display for TriageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TriageError::FeedbackNotFound(id) => write!(f, "Feedback not found: {}", id),
            TriageError::AssignmentError(msg) => write!(f, "Assignment error: {}", msg),
            TriageError::ResponseError(msg) => write!(f, "Response error: {}", msg),
            TriageError::RuleError(msg) => write!(f, "Rule error: {}", msg),
        }
    }
}

impl std::error::Error for TriageError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_triage_initialization() {
        let triage = FeedbackTriage::new();
        assert!(triage.feedback_items.is_empty());
        assert!(triage.triage_rules.is_empty());
    }

    #[test]
    fn test_add_feedback() {
        let mut triage = FeedbackTriage::new();
        let feedback = FeedbackItem::new(
            "user1".to_string(),
            "I'm having trouble with the dashboard navigation".to_string(),
        );
        
        let feedback_id = triage.add_feedback(feedback);
        assert!(!feedback_id.is_nil());
        assert_eq!(triage.feedback_items.len(), 1);
    }

    #[test]
    fn test_add_triage_rule() {
        let mut triage = FeedbackTriage::new();
        let rule = TriageRule::new(
            "navigation_issues".to_string(),
            "Navigation Issues".to_string(),
            "Rule for navigation-related feedback".to_string(),
            vec!["navigation".to_string(), "dashboard".to_string()],
            FeedbackCategory::Usability,
            FeedbackPriority::Medium,
            None,
        );
        
        triage.add_triage_rule(rule);
        assert_eq!(triage.triage_rules.len(), 1);
    }

    #[test]
    fn test_add_response_template() {
        let mut triage = FeedbackTriage::new();
        let template = ResponseTemplate::new(
            "usability_template".to_string(),
            "Usability Feedback Response".to_string(),
            FeedbackCategory::Usability,
            "Thank you for your usability feedback. We're looking into this issue.".to_string(),
        );
        
        triage.add_response_template(template);
        assert!(triage.response_templates.contains_key(&FeedbackCategory::Usability));
        assert_eq!(triage.response_templates[&FeedbackCategory::Usability].len(), 1);
    }

    #[test]
    fn test_manual_triage() {
        let mut triage = FeedbackTriage::new();
        let feedback = FeedbackItem::new(
            "user1".to_string(),
            "I'm having trouble with the dashboard navigation".to_string(),
        );
        
        let feedback_id = triage.add_feedback(feedback);
        let result = triage.manual_triage(
            feedback_id,
            FeedbackCategory::Usability,
            FeedbackPriority::High,
            Some("support_team".to_string()),
        );
        
        assert!(result.is_ok());
        
        let triaged_feedback = triage.feedback_items.iter().find(|f| f.id == feedback_id).unwrap();
        assert_eq!(triaged_feedback.status, FeedbackStatus::Triaged);
        assert_eq!(triaged_feedback.category, FeedbackCategory::Usability);
        assert_eq!(triaged_feedback.priority, FeedbackPriority::High);
        assert_eq!(triaged_feedback.assigned_to, Some("support_team".to_string()));
    }

    #[test]
    fn test_auto_triage_feedback() {
        let mut triage = FeedbackTriage::new();
        
        // Add a rule
        let rule = TriageRule::new(
            "navigation_issues".to_string(),
            "Navigation Issues".to_string(),
            "Rule for navigation-related feedback".to_string(),
            vec!["navigation".to_string()],
            FeedbackCategory::Usability,
            FeedbackPriority::Medium,
            None,
        );
        triage.add_triage_rule(rule);
        
        // Add feedback that matches the rule
        let feedback = FeedbackItem::new(
            "user1".to_string(),
            "I'm having trouble with navigation".to_string(),
        );
        triage.add_feedback(feedback);
        
        // Run auto-triage
        let result = triage.auto_triage_feedback();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
        
        let pending_feedback = triage.get_pending_feedback();
        assert_eq!(pending_feedback.len(), 0);
    }

    #[test]
    fn test_send_response() {
        let mut triage = FeedbackTriage::new();
        let feedback = FeedbackItem::new(
            "user1".to_string(),
            "I'm having trouble with the dashboard navigation".to_string(),
        );
        
        let feedback_id = triage.add_feedback(feedback);
        let response = FeedbackResponse::new(
            feedback_id,
            "support_agent".to_string(),
            "We're working on this issue".to_string(),
            ResponseType::Solution,
        );
        
        let result = triage.send_response(response);
        assert!(result.is_ok());
        
        let responded_feedback = triage.feedback_items.iter().find(|f| f.id == feedback_id).unwrap();
        assert_eq!(responded_feedback.status, FeedbackStatus::Responded);
        assert_eq!(responded_feedback.responses.len(), 1);
    }

    #[test]
    fn test_get_feedback_by_category() {
        let mut triage = FeedbackTriage::new();
        let mut feedback = FeedbackItem::new(
            "user1".to_string(),
            "I'm having trouble with the dashboard navigation".to_string(),
        );
        feedback.category = FeedbackCategory::Usability;
        
        triage.add_feedback(feedback);
        let usability_feedback = triage.get_feedback_by_category(FeedbackCategory::Usability);
        assert_eq!(usability_feedback.len(), 1);
    }

    #[test]
    fn test_escalate_feedback() {
        let mut triage = FeedbackTriage::new();
        let feedback = FeedbackItem::new(
            "user1".to_string(),
            "Critical issue with data loss".to_string(),
        );
        
        let feedback_id = triage.add_feedback(feedback);
        let result = triage.escalate_feedback(feedback_id, "Critical data issue requiring immediate attention".to_string());
        assert!(result.is_ok());
        
        let escalated_feedback = triage.feedback_items.iter().find(|f| f.id == feedback_id).unwrap();
        assert!(escalated_feedback.escalated);
        assert_eq!(escalated_feedback.escalation_reason, Some("Critical data issue requiring immediate attention".to_string()));
    }

    #[test]
    fn test_close_feedback() {
        let mut triage = FeedbackTriage::new();
        let feedback = FeedbackItem::new(
            "user1".to_string(),
            "Minor UI suggestion".to_string(),
        );
        
        let feedback_id = triage.add_feedback(feedback);
        let result = triage.close_feedback(feedback_id, "Implemented in next release".to_string());
        assert!(result.is_ok());
        
        let closed_feedback = triage.feedback_items.iter().find(|f| f.id == feedback_id).unwrap();
        assert_eq!(closed_feedback.status, FeedbackStatus::Closed);
        assert_eq!(closed_feedback.resolution, Some("Implemented in next release".to_string()));
    }
}