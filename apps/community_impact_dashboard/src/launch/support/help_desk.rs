//! Community help desk system with peer support for the Unified Community Impact Dashboard
//!
//! This module provides a community-centered help desk system that leverages
//! peer support, community expertise, and collaborative problem-solving.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Community help desk system
pub struct CommunityHelpDesk {
    support_requests: Vec<SupportRequest>,
    peer_supporters: HashMap<String, PeerSupporter>,
    support_groups: HashMap<String, SupportGroup>,
    resolved_requests: Vec<SupportRequest>,
    support_statistics: SupportStatistics,
}

impl CommunityHelpDesk {
    /// Create a new community help desk system
    pub fn new() -> Self {
        Self {
            support_requests: Vec::new(),
            peer_supporters: HashMap::new(),
            support_groups: HashMap::new(),
            resolved_requests: Vec::new(),
            support_statistics: SupportStatistics::new(),
        }
    }

    /// Submit a support request
    pub fn submit_support_request(&mut self, request: SupportRequest) -> Uuid {
        let request_id = request.id;
        self.support_requests.push(request);
        self.support_statistics.total_requests += 1;
        info!("Submitted support request: {}", request_id);
        request_id
    }

    /// Register a peer supporter
    pub fn register_peer_supporter(&mut self, supporter: PeerSupporter) {
        self.peer_supporters.insert(supporter.user_id.clone(), supporter);
        self.support_statistics.registered_supporters += 1;
        info!("Registered peer supporter: {}", supporter.user_id);
    }

    /// Create a support group
    pub fn create_support_group(&mut self, group: SupportGroup) -> String {
        let group_id = group.id.clone();
        self.support_groups.insert(group_id.clone(), group);
        self.support_statistics.support_groups += 1;
        info!("Created support group: {}", group_id);
        group_id
    }

    /// Assign a support request to a peer supporter
    pub fn assign_request(&mut self, request_id: Uuid, supporter_id: String) -> Result<(), HelpDeskError> {
        let request = self.support_requests.iter_mut()
            .find(|r| r.id == request_id)
            .ok_or(HelpDeskError::RequestNotFound(request_id))?;

        request.assigned_to = Some(supporter_id.clone());
        request.status = SupportStatus::Assigned;
        request.assigned_at = Some(Utc::now());

        // Update supporter stats
        if let Some(supporter) = self.peer_supporters.get_mut(&supporter_id) {
            supporter.current_assignments += 1;
        }

        info!("Assigned request {} to supporter {}", request_id, supporter_id);
        Ok(())
    }

    /// Resolve a support request
    pub fn resolve_request(&mut self, request_id: Uuid, resolution: String) -> Result<(), HelpDeskError> {
        let request_index = self.support_requests.iter()
            .position(|r| r.id == request_id)
            .ok_or(HelpDeskError::RequestNotFound(request_id))?;

        let mut request = self.support_requests.remove(request_index);
        request.status = SupportStatus::Resolved;
        request.resolved_at = Some(Utc::now());
        request.resolution = Some(resolution);

        self.resolved_requests.push(request);
        self.support_statistics.resolved_requests += 1;

        // Update supporter stats if assigned
        if let Some(supporter_id) = &request.assigned_to {
            if let Some(supporter) = self.peer_supporters.get_mut(supporter_id) {
                supporter.completed_assignments += 1;
                supporter.current_assignments -= 1;
            }
        }

        info!("Resolved support request: {}", request_id);
        Ok(())
    }

    /// Get pending support requests
    pub fn get_pending_requests(&self) -> Vec<&SupportRequest> {
        self.support_requests.iter()
            .filter(|r| r.status == SupportStatus::Pending)
            .collect()
    }

    /// Get assigned support requests for a supporter
    pub fn get_assigned_requests(&self, supporter_id: &str) -> Vec<&SupportRequest> {
        self.support_requests.iter()
            .filter(|r| r.assigned_to.as_deref() == Some(supporter_id))
            .collect()
    }

    /// Get support requests by category
    pub fn get_requests_by_category(&self, category: SupportCategory) -> Vec<&SupportRequest> {
        self.support_requests.iter()
            .filter(|r| r.category == category)
            .collect()
    }

    /// Add a comment to a support request
    pub fn add_comment(&mut self, request_id: Uuid, comment: SupportComment) -> Result<(), HelpDeskError> {
        let request = self.support_requests.iter_mut()
            .find(|r| r.id == request_id)
            .ok_or(HelpDeskError::RequestNotFound(request_id))?;

        request.comments.push(comment);
        info!("Added comment to request: {}", request_id);
        Ok(())
    }

    /// Escalate a support request
    pub fn escalate_request(&mut self, request_id: Uuid, reason: String) -> Result<(), HelpDeskError> {
        let request = self.support_requests.iter_mut()
            .find(|r| r.id == request_id)
            .ok_or(HelpDeskError::RequestNotFound(request_id))?;

        request.escalated = true;
        request.escalation_reason = Some(reason);
        request.escalated_at = Some(Utc::now());
        self.support_statistics.escalated_requests += 1;

        info!("Escalated support request: {}", request_id);
        Ok(())
    }

    /// Get peer supporters by expertise
    pub fn get_supporters_by_expertise(&self, expertise: SupportExpertise) -> Vec<&PeerSupporter> {
        self.peer_supporters.values()
            .filter(|s| s.expertise.contains(&expertise))
            .collect()
    }

    /// Get support statistics
    pub fn get_statistics(&self) -> &SupportStatistics {
        &self.support_statistics
    }

    /// Generate support report
    pub fn generate_support_report(&self) -> SupportReport {
        let pending_requests = self.get_pending_requests().len();
        let assigned_requests: usize = self.peer_supporters.values()
            .map(|s| self.get_assigned_requests(&s.user_id).len())
            .sum();

        SupportReport {
            generated_at: Utc::now(),
            total_requests: self.support_statistics.total_requests,
            resolved_requests: self.support_statistics.resolved_requests,
            pending_requests,
            assigned_requests,
            escalated_requests: self.support_statistics.escalated_requests,
            registered_supporters: self.support_statistics.registered_supporters,
            support_groups: self.support_statistics.support_groups,
        }
    }
}

/// Support request from community members
#[derive(Debug, Clone)]
pub struct SupportRequest {
    pub id: Uuid,
    pub requester: String,
    pub title: String,
    pub description: String,
    pub category: SupportCategory,
    pub priority: SupportPriority,
    pub status: SupportStatus,
    pub submitted_at: DateTime<Utc>,
    pub assigned_to: Option<String>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub escalated: bool,
    pub escalation_reason: Option<String>,
    pub escalated_at: Option<DateTime<Utc>>,
    pub comments: Vec<SupportComment>,
    pub tags: Vec<String>,
}

impl SupportRequest {
    /// Create a new support request
    pub fn new(
        requester: String,
        title: String,
        description: String,
        category: SupportCategory,
        priority: SupportPriority,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            requester,
            title,
            description,
            category,
            priority,
            status: SupportStatus::Pending,
            submitted_at: Utc::now(),
            assigned_to: None,
            assigned_at: None,
            resolved_at: None,
            resolution: None,
            escalated: false,
            escalation_reason: None,
            escalated_at: None,
            comments: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// Add tags to the support request
    pub fn add_tags(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
    }
}

/// Peer supporter in the community
#[derive(Debug, Clone)]
pub struct PeerSupporter {
    pub user_id: String,
    pub name: String,
    pub expertise: Vec<SupportExpertise>,
    pub availability: SupportAvailability,
    pub languages: Vec<String>,
    pub registered_at: DateTime<Utc>,
    pub current_assignments: usize,
    pub completed_assignments: usize,
    pub rating: Option<f64>,
}

impl PeerSupporter {
    /// Create a new peer supporter
    pub fn new(
        user_id: String,
        name: String,
        expertise: Vec<SupportExpertise>,
        languages: Vec<String>,
    ) -> Self {
        Self {
            user_id,
            name,
            expertise,
            availability: SupportAvailability::Available,
            languages,
            registered_at: Utc::now(),
            current_assignments: 0,
            completed_assignments: 0,
            rating: None,
        }
    }
}

/// Support group for collaborative problem-solving
#[derive(Debug, Clone)]
pub struct SupportGroup {
    pub id: String,
    pub name: String,
    pub description: String,
    pub members: Vec<String>,
    pub topics: Vec<SupportCategory>,
    pub created_at: DateTime<Utc>,
    pub active: bool,
}

impl SupportGroup {
    /// Create a new support group
    pub fn new(
        id: String,
        name: String,
        description: String,
        topics: Vec<SupportCategory>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            members: Vec::new(),
            topics,
            created_at: Utc::now(),
            active: true,
        }
    }

    /// Add a member to the support group
    pub fn add_member(&mut self, user_id: String) {
        if !self.members.contains(&user_id) {
            self.members.push(user_id);
        }
    }
}

/// Comment on a support request
#[derive(Debug, Clone)]
pub struct SupportComment {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub posted_at: DateTime<Utc>,
    pub is_solution: bool,
}

impl SupportComment {
    /// Create a new support comment
    pub fn new(author: String, content: String, is_solution: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            author,
            content,
            posted_at: Utc::now(),
            is_solution,
        }
    }
}

/// Categories of support requests
#[derive(Debug, Clone, PartialEq)]
pub enum SupportCategory {
    Technical,
    Onboarding,
    Visualization,
    Validation,
    Storytelling,
    Facilitation,
    Accessibility,
    Translation,
    General,
}

/// Priority levels for support requests
#[derive(Debug, Clone, PartialEq)]
pub enum SupportPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of support requests
#[derive(Debug, Clone, PartialEq)]
pub enum SupportStatus {
    Pending,
    Assigned,
    InProgress,
    Resolved,
    Closed,
}

/// Types of support expertise
#[derive(Debug, Clone, PartialEq)]
pub enum SupportExpertise {
    DashboardNavigation,
    DataVisualization,
    CommunityValidation,
    StoryContributing,
    Facilitation,
    TechnicalIssues,
    Accessibility,
    Translation,
}

/// Availability status for peer supporters
#[derive(Debug, Clone, PartialEq)]
pub enum SupportAvailability {
    Available,
    Busy,
    Offline,
    Away,
}

/// Support statistics tracking
#[derive(Debug, Clone)]
pub struct SupportStatistics {
    pub total_requests: usize,
    pub resolved_requests: usize,
    pub escalated_requests: usize,
    pub registered_supporters: usize,
    pub support_groups: usize,
}

impl SupportStatistics {
    /// Create new support statistics
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            resolved_requests: 0,
            escalated_requests: 0,
            registered_supporters: 0,
            support_groups: 0,
        }
    }
}

/// Support report
#[derive(Debug, Clone)]
pub struct SupportReport {
    pub generated_at: DateTime<Utc>,
    pub total_requests: usize,
    pub resolved_requests: usize,
    pub pending_requests: usize,
    pub assigned_requests: usize,
    pub escalated_requests: usize,
    pub registered_supporters: usize,
    pub support_groups: usize,
}

/// Error types for help desk system
#[derive(Debug)]
pub enum HelpDeskError {
    RequestNotFound(Uuid),
    SupporterNotFound(String),
    GroupNotFound(String),
    AssignmentError(String),
}

impl std::fmt::Display for HelpDeskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HelpDeskError::RequestNotFound(id) => write!(f, "Support request not found: {}", id),
            HelpDeskError::SupporterNotFound(id) => write!(f, "Peer supporter not found: {}", id),
            HelpDeskError::GroupNotFound(id) => write!(f, "Support group not found: {}", id),
            HelpDeskError::AssignmentError(msg) => write!(f, "Assignment error: {}", msg),
        }
    }
}

impl std::error::Error for HelpDeskError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_desk_initialization() {
        let help_desk = CommunityHelpDesk::new();
        assert!(help_desk.support_requests.is_empty());
        assert!(help_desk.peer_supporters.is_empty());
    }

    #[test]
    fn test_submit_support_request() {
        let mut help_desk = CommunityHelpDesk::new();
        let request = SupportRequest::new(
            "user1".to_string(),
            "Help with dashboard".to_string(),
            "I need help navigating the dashboard".to_string(),
            SupportCategory::Technical,
            SupportPriority::Medium,
        );
        
        let request_id = help_desk.submit_support_request(request);
        assert!(!request_id.is_nil());
        assert_eq!(help_desk.support_requests.len(), 1);
    }

    #[test]
    fn test_register_peer_supporter() {
        let mut help_desk = CommunityHelpDesk::new();
        let supporter = PeerSupporter::new(
            "supporter1".to_string(),
            "Jane Doe".to_string(),
            vec![SupportExpertise::DashboardNavigation],
            vec!["en".to_string()],
        );
        
        help_desk.register_peer_supporter(supporter);
        assert_eq!(help_desk.peer_supporters.len(), 1);
        assert!(help_desk.peer_supporters.contains_key("supporter1"));
    }

    #[test]
    fn test_create_support_group() {
        let mut help_desk = CommunityHelpDesk::new();
        let group = SupportGroup::new(
            "tech_help".to_string(),
            "Technical Help Group".to_string(),
            "Group for technical support".to_string(),
            vec![SupportCategory::Technical],
        );
        
        let group_id = help_desk.create_support_group(group);
        assert_eq!(group_id, "tech_help");
        assert!(help_desk.support_groups.contains_key("tech_help"));
    }

    #[test]
    fn test_assign_request() {
        let mut help_desk = CommunityHelpDesk::new();
        
        let request = SupportRequest::new(
            "user1".to_string(),
            "Help with dashboard".to_string(),
            "I need help navigating the dashboard".to_string(),
            SupportCategory::Technical,
            SupportPriority::Medium,
        );
        
        let request_id = help_desk.submit_support_request(request);
        
        let supporter = PeerSupporter::new(
            "supporter1".to_string(),
            "Jane Doe".to_string(),
            vec![SupportExpertise::DashboardNavigation],
            vec!["en".to_string()],
        );
        
        help_desk.register_peer_supporter(supporter);
        
        let result = help_desk.assign_request(request_id, "supporter1".to_string());
        assert!(result.is_ok());
        
        let assigned_request = help_desk.support_requests.iter().find(|r| r.id == request_id).unwrap();
        assert_eq!(assigned_request.status, SupportStatus::Assigned);
        assert_eq!(assigned_request.assigned_to, Some("supporter1".to_string()));
    }

    #[test]
    fn test_resolve_request() {
        let mut help_desk = CommunityHelpDesk::new();
        
        let request = SupportRequest::new(
            "user1".to_string(),
            "Help with dashboard".to_string(),
            "I need help navigating the dashboard".to_string(),
            SupportCategory::Technical,
            SupportPriority::Medium,
        );
        
        let request_id = help_desk.submit_support_request(request);
        
        let result = help_desk.resolve_request(request_id, "Issue resolved".to_string());
        assert!(result.is_ok());
        assert_eq!(help_desk.resolved_requests.len(), 1);
    }
}