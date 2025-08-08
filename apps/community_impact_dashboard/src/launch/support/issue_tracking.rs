//! Real-time issue tracking and resolution for the Unified Community Impact Dashboard
//!
//! This module provides real-time monitoring of technical issues, community
//! challenges, and system performance problems during the launch period.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Real-time issue tracking system
pub struct IssueTracker {
    issues: Vec<Issue>,
    issue_assignments: HashMap<Uuid, String>,
    resolution_times: Vec<ResolutionTime>,
    issue_statistics: IssueStatistics,
    issue_categories: HashMap<IssueCategory, Vec<Uuid>>,
}

impl IssueTracker {
    /// Create a new issue tracking system
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            issue_assignments: HashMap::new(),
            resolution_times: Vec::new(),
            issue_statistics: IssueStatistics::new(),
            issue_categories: HashMap::new(),
        }
    }

    /// Report a new issue
    pub fn report_issue(&mut self, issue: Issue) -> Uuid {
        let issue_id = issue.id;
        
        // Add to category tracking
        self.issue_categories
            .entry(issue.category.clone())
            .or_insert_with(Vec::new)
            .push(issue_id);
        
        self.issues.push(issue);
        self.issue_statistics.total_reported += 1;
        
        info!("Reported new issue: {}", issue_id);
        issue_id
    }

    /// Assign an issue to a resolver
    pub fn assign_issue(&mut self, issue_id: Uuid, assignee: String) -> Result<(), IssueTrackingError> {
        let issue = self.issues.iter_mut()
            .find(|i| i.id == issue_id)
            .ok_or(IssueTrackingError::IssueNotFound(issue_id))?;

        issue.status = IssueStatus::Assigned;
        issue.assigned_to = Some(assignee.clone());
        issue.assigned_at = Some(Utc::now());
        
        self.issue_assignments.insert(issue_id, assignee);
        
        info!("Assigned issue {} to {}", issue_id, assignee);
        Ok(())
    }

    /// Update issue status
    pub fn update_issue_status(&mut self, issue_id: Uuid, status: IssueStatus) -> Result<(), IssueTrackingError> {
        let issue = self.issues.iter_mut()
            .find(|i| i.id == issue_id)
            .ok_or(IssueTrackingError::IssueNotFound(issue_id))?;

        let old_status = issue.status.clone();
        issue.status = status;
        
        match &status {
            IssueStatus::InProgress => {
                issue.started_at = Some(Utc::now());
            }
            IssueStatus::Resolved => {
                let resolved_at = Utc::now();
                issue.resolved_at = Some(resolved_at);
                
                // Calculate and store resolution time
                if let Some(submitted_at) = issue.submitted_at {
                    let duration = resolved_at.signed_duration_since(submitted_at);
                    let resolution_time = ResolutionTime {
                        issue_id,
                        duration: duration.num_seconds() as f64,
                        resolved_at,
                    };
                    self.resolution_times.push(resolution_time);
                }
                
                self.issue_statistics.resolved += 1;
            }
            IssueStatus::Closed => {
                issue.closed_at = Some(Utc::now());
                self.issue_statistics.closed += 1;
            }
            _ => {}
        }
        
        info!("Updated issue {} status from {:?} to {:?}", issue_id, old_status, status);
        Ok(())
    }

    /// Add a comment to an issue
    pub fn add_comment(&mut self, issue_id: Uuid, comment: IssueComment) -> Result<(), IssueTrackingError> {
        let issue = self.issues.iter_mut()
            .find(|i| i.id == issue_id)
            .ok_or(IssueTrackingError::IssueNotFound(issue_id))?;

        issue.comments.push(comment);
        info!("Added comment to issue: {}", issue_id);
        Ok(())
    }

    /// Escalate an issue
    pub fn escalate_issue(&mut self, issue_id: Uuid, reason: String) -> Result<(), IssueTrackingError> {
        let issue = self.issues.iter_mut()
            .find(|i| i.id == issue_id)
            .ok_or(IssueTrackingError::IssueNotFound(issue_id))?;

        issue.escalated = true;
        issue.escalation_reason = Some(reason);
        issue.escalated_at = Some(Utc::now());
        self.issue_statistics.escalated += 1;
        
        info!("Escalated issue: {}", issue_id);
        Ok(())
    }

    /// Get issues by status
    pub fn get_issues_by_status(&self, status: IssueStatus) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| i.status == status)
            .collect()
    }

    /// Get issues by category
    pub fn get_issues_by_category(&self, category: &IssueCategory) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| &i.category == category)
            .collect()
    }

    /// Get issues by priority
    pub fn get_issues_by_priority(&self, priority: IssuePriority) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| i.priority == priority)
            .collect()
    }

    /// Get assigned issues for a resolver
    pub fn get_assigned_issues(&self, assignee: &str) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| i.assigned_to.as_deref() == Some(assignee))
            .collect()
    }

    /// Get unresolved issues (not resolved or closed)
    pub fn get_unresolved_issues(&self) -> Vec<&Issue> {
        self.issues.iter()
            .filter(|i| i.status != IssueStatus::Resolved && i.status != IssueStatus::Closed)
            .collect()
    }

    /// Get statistics about issue resolution times
    pub fn get_resolution_statistics(&self) -> ResolutionStatistics {
        if self.resolution_times.is_empty() {
            return ResolutionStatistics {
                average_resolution_time: 0.0,
                median_resolution_time: 0.0,
                fastest_resolution: 0.0,
                slowest_resolution: 0.0,
                total_resolved: 0,
            };
        }

        let mut times: Vec<f64> = self.resolution_times.iter()
            .map(|rt| rt.duration)
            .collect();
        
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let average = times.iter().sum::<f64>() / times.len() as f64;
        let median = if times.len() % 2 == 0 {
            (times[times.len() / 2 - 1] + times[times.len() / 2]) / 2.0
        } else {
            times[times.len() / 2]
        };
        
        ResolutionStatistics {
            average_resolution_time: average,
            median_resolution_time: median,
            fastest_resolution: *times.first().unwrap(),
            slowest_resolution: *times.last().unwrap(),
            total_resolved: times.len(),
        }
    }

    /// Get overall issue statistics
    pub fn get_statistics(&self) -> &IssueStatistics {
        &self.issue_statistics
    }

    /// Generate issue tracking report
    pub fn generate_issue_report(&self) -> IssueReport {
        let unresolved_issues = self.get_unresolved_issues().len();
        let by_status: HashMap<IssueStatus, usize> = self.issues.iter()
            .fold(HashMap::new(), |mut acc, issue| {
                *acc.entry(issue.status.clone()).or_insert(0) += 1;
                acc
            });
        
        let resolution_stats = self.get_resolution_statistics();
        
        IssueReport {
            generated_at: Utc::now(),
            total_issues: self.issues.len(),
            unresolved_issues,
            resolved_issues: self.issue_statistics.resolved,
            closed_issues: self.issue_statistics.closed,
            escalated_issues: self.issue_statistics.escalated,
            issues_by_status: by_status,
            resolution_statistics: resolution_stats,
        }
    }

    /// Get issues that are taking too long to resolve
    pub fn get_overdue_issues(&self, max_resolution_time_hours: i64) -> Vec<&Issue> {
        let cutoff = Utc::now() - chrono::Duration::hours(max_resolution_time_hours);
        
        self.issues.iter()
            .filter(|issue| {
                issue.submitted_at.map_or(false, |submitted| submitted < cutoff) &&
                (issue.status == IssueStatus::Pending || issue.status == IssueStatus::Assigned || issue.status == IssueStatus::InProgress)
            })
            .collect()
    }
}

/// Issue tracking record
#[derive(Debug, Clone)]
pub struct Issue {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: IssueCategory,
    pub priority: IssuePriority,
    pub status: IssueStatus,
    pub reporter: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub assigned_to: Option<String>,
    pub assigned_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub escalated: bool,
    pub escalation_reason: Option<String>,
    pub escalated_at: Option<DateTime<Utc>>,
    pub comments: Vec<IssueComment>,
    pub tags: Vec<String>,
    pub affected_users: Option<usize>,
}

impl Issue {
    /// Create a new issue
    pub fn new(
        title: String,
        description: String,
        category: IssueCategory,
        priority: IssuePriority,
        reporter: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            category,
            priority,
            status: IssueStatus::Pending,
            reporter,
            submitted_at: Some(Utc::now()),
            assigned_to: None,
            assigned_at: None,
            started_at: None,
            resolved_at: None,
            closed_at: None,
            resolution: None,
            escalated: false,
            escalation_reason: None,
            escalated_at: None,
            comments: Vec::new(),
            tags: Vec::new(),
            affected_users: None,
        }
    }

    /// Add tags to the issue
    pub fn add_tags(&mut self, tags: Vec<String>) {
        self.tags.extend(tags);
    }

    /// Set affected users count
    pub fn set_affected_users(&mut self, count: usize) {
        self.affected_users = Some(count);
    }
}

/// Comment on an issue
#[derive(Debug, Clone)]
pub struct IssueComment {
    pub id: Uuid,
    pub author: String,
    pub content: String,
    pub posted_at: DateTime<Utc>,
    pub is_resolution: bool,
}

impl IssueComment {
    /// Create a new issue comment
    pub fn new(author: String, content: String, is_resolution: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            author,
            content,
            posted_at: Utc::now(),
            is_resolution,
        }
    }
}

/// Resolution time tracking
#[derive(Debug, Clone)]
pub struct ResolutionTime {
    pub issue_id: Uuid,
    pub duration: f64, // in seconds
    pub resolved_at: DateTime<Utc>,
}

/// Categories of issues
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IssueCategory {
    Technical,
    Performance,
    Usability,
    Accessibility,
    Data,
    Security,
    Community,
    Onboarding,
    Validation,
    Storytelling,
}

/// Priority levels for issues
#[derive(Debug, Clone, PartialEq)]
pub enum IssuePriority {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of issues
#[derive(Debug, Clone, PartialEq)]
pub enum IssueStatus {
    Pending,
    Assigned,
    InProgress,
    Resolved,
    Closed,
}

/// Statistics about issue tracking
#[derive(Debug, Clone)]
pub struct IssueStatistics {
    pub total_reported: usize,
    pub resolved: usize,
    pub closed: usize,
    pub escalated: usize,
}

impl IssueStatistics {
    /// Create new issue statistics
    pub fn new() -> Self {
        Self {
            total_reported: 0,
            resolved: 0,
            closed: 0,
            escalated: 0,
        }
    }
}

/// Statistics about resolution times
#[derive(Debug, Clone)]
pub struct ResolutionStatistics {
    pub average_resolution_time: f64,
    pub median_resolution_time: f64,
    pub fastest_resolution: f64,
    pub slowest_resolution: f64,
    pub total_resolved: usize,
}

/// Issue tracking report
#[derive(Debug, Clone)]
pub struct IssueReport {
    pub generated_at: DateTime<Utc>,
    pub total_issues: usize,
    pub unresolved_issues: usize,
    pub resolved_issues: usize,
    pub closed_issues: usize,
    pub escalated_issues: usize,
    pub issues_by_status: HashMap<IssueStatus, usize>,
    pub resolution_statistics: ResolutionStatistics,
}

/// Error types for issue tracking
#[derive(Debug)]
pub enum IssueTrackingError {
    IssueNotFound(Uuid),
    AssignmentError(String),
    StatusUpdateError(String),
}

impl std::fmt::Display for IssueTrackingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueTrackingError::IssueNotFound(id) => write!(f, "Issue not found: {}", id),
            IssueTrackingError::AssignmentError(msg) => write!(f, "Assignment error: {}", msg),
            IssueTrackingError::StatusUpdateError(msg) => write!(f, "Status update error: {}", msg),
        }
    }
}

impl std::error::Error for IssueTrackingError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_issue_tracker_initialization() {
        let tracker = IssueTracker::new();
        assert!(tracker.issues.is_empty());
        assert!(tracker.issue_assignments.is_empty());
    }

    #[test]
    fn test_report_issue() {
        let mut tracker = IssueTracker::new();
        let issue = Issue::new(
            "Dashboard Loading Issue".to_string(),
            "Dashboard takes too long to load".to_string(),
            IssueCategory::Performance,
            IssuePriority::High,
            "user1".to_string(),
        );
        
        let issue_id = tracker.report_issue(issue);
        assert!(!issue_id.is_nil());
        assert_eq!(tracker.issues.len(), 1);
        assert!(tracker.issue_categories.contains_key(&IssueCategory::Performance));
    }

    #[test]
    fn test_assign_issue() {
        let mut tracker = IssueTracker::new();
        let issue = Issue::new(
            "Dashboard Loading Issue".to_string(),
            "Dashboard takes too long to load".to_string(),
            IssueCategory::Performance,
            IssuePriority::High,
            "user1".to_string(),
        );
        
        let issue_id = tracker.report_issue(issue);
        let result = tracker.assign_issue(issue_id, "tech_support".to_string());
        assert!(result.is_ok());
        
        let assigned_issue = tracker.issues.iter().find(|i| i.id == issue_id).unwrap();
        assert_eq!(assigned_issue.status, IssueStatus::Assigned);
        assert_eq!(assigned_issue.assigned_to, Some("tech_support".to_string()));
    }

    #[test]
    fn test_update_issue_status() {
        let mut tracker = IssueTracker::new();
        let issue = Issue::new(
            "Dashboard Loading Issue".to_string(),
            "Dashboard takes too long to load".to_string(),
            IssueCategory::Performance,
            IssuePriority::High,
            "user1".to_string(),
        );
        
        let issue_id = tracker.report_issue(issue);
        let result = tracker.update_issue_status(issue_id, IssueStatus::InProgress);
        assert!(result.is_ok());
        
        let updated_issue = tracker.issues.iter().find(|i| i.id == issue_id).unwrap();
        assert_eq!(updated_issue.status, IssueStatus::InProgress);
    }

    #[test]
    fn test_get_issues_by_status() {
        let mut tracker = IssueTracker::new();
        let issue = Issue::new(
            "Dashboard Loading Issue".to_string(),
            "Dashboard takes too long to load".to_string(),
            IssueCategory::Performance,
            IssuePriority::High,
            "user1".to_string(),
        );
        
        let issue_id = tracker.report_issue(issue);
        tracker.update_issue_status(issue_id, IssueStatus::InProgress).unwrap();
        
        let in_progress_issues = tracker.get_issues_by_status(IssueStatus::InProgress);
        assert_eq!(in_progress_issues.len(), 1);
    }

    #[test]
    fn test_get_resolution_statistics() {
        let mut tracker = IssueTracker::new();
        
        // Create and resolve an issue
        let issue = Issue::new(
            "Test Issue".to_string(),
            "Test description".to_string(),
            IssueCategory::Technical,
            IssuePriority::Medium,
            "user1".to_string(),
        );
        
        let issue_id = tracker.report_issue(issue);
        tracker.update_issue_status(issue_id, IssueStatus::Resolved).unwrap();
        
        let stats = tracker.get_resolution_statistics();
        assert_eq!(stats.total_resolved, 1);
        assert!(stats.average_resolution_time > 0.0);
    }
}