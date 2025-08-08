//! Dashboard governance documentation for the Unified Community Impact Dashboard
//!
//! This module provides comprehensive governance framework documentation,
//! including principles, roles, responsibilities, and decision-making processes.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Dashboard governance framework
pub struct GovernanceFramework {
    principles: Vec<GovernancePrinciple>,
    roles: HashMap<String, GovernanceRole>,
    responsibilities: HashMap<String, Vec<Responsibility>>,
    decision_making_processes: HashMap<String, DecisionMakingProcess>,
    governance_documents: Vec<GovernanceDocument>,
    governance_history: Vec<GovernanceChange>,
}

impl GovernanceFramework {
    /// Create a new governance framework
    pub fn new() -> Self {
        Self {
            principles: Vec::new(),
            roles: HashMap::new(),
            responsibilities: HashMap::new(),
            decision_making_processes: HashMap::new(),
            governance_documents: Vec::new(),
            governance_history: Vec::new(),
        }
    }

    /// Add a governance principle
    pub fn add_principle(&mut self, principle: GovernancePrinciple) {
        self.principles.push(principle);
        info!("Added governance principle: {}", principle.name);
    }

    /// Define a governance role
    pub fn define_role(&mut self, role: GovernanceRole) -> String {
        let role_id = role.id.clone();
        self.roles.insert(role_id.clone(), role);
        info!("Defined governance role: {}", role_id);
        role_id
    }

    /// Assign responsibilities to a role
    pub fn assign_responsibilities(&mut self, role_id: &str, responsibilities: Vec<Responsibility>) -> Result<(), GovernanceError> {
        if !self.roles.contains_key(role_id) {
            return Err(GovernanceError::RoleNotFound(role_id.to_string()));
        }
        
        self.responsibilities.insert(role_id.to_string(), responsibilities);
        info!("Assigned responsibilities to role: {}", role_id);
        Ok(())
    }

    /// Define a decision-making process
    pub fn define_decision_process(&mut self, process: DecisionMakingProcess) -> String {
        let process_id = process.id.clone();
        self.decision_making_processes.insert(process_id.clone(), process);
        info!("Defined decision-making process: {}", process_id);
        process_id
    }

    /// Create a governance document
    pub fn create_document(&mut self, document: GovernanceDocument) -> Uuid {
        let document_id = document.id;
        self.governance_documents.push(document);
        info!("Created governance document: {}", document_id);
        document_id
    }

    /// Get governance principles
    pub fn get_principles(&self) -> &Vec<GovernancePrinciple> {
        &self.principles
    }

    /// Get governance roles
    pub fn get_roles(&self) -> &HashMap<String, GovernanceRole> {
        &self.roles
    }

    /// Get responsibilities for a role
    pub fn get_role_responsibilities(&self, role_id: &str) -> Option<&Vec<Responsibility>> {
        self.responsibilities.get(role_id)
    }

    /// Get decision-making processes
    pub fn get_decision_processes(&self) -> &HashMap<String, DecisionMakingProcess> {
        &self.decision_making_processes
    }

    /// Get governance documents by category
    pub fn get_documents_by_category(&self, category: GovernanceDocumentCategory) -> Vec<&GovernanceDocument> {
        self.governance_documents.iter()
            .filter(|doc| doc.category == category)
            .collect()
    }

    /// Get governance documents by status
    pub fn get_documents_by_status(&self, status: DocumentStatus) -> Vec<&GovernanceDocument> {
        self.governance_documents.iter()
            .filter(|doc| doc.status == status)
            .collect()
    }

    /// Update a governance document
    pub fn update_document(&mut self, document_id: Uuid, updates: DocumentUpdate) -> Result<(), GovernanceError> {
        let document = self.governance_documents.iter_mut()
            .find(|d| d.id == document_id)
            .ok_or(GovernanceError::DocumentNotFound(document_id))?;

        if let Some(title) = updates.title {
            document.title = title;
        }
        
        if let Some(content) = updates.content {
            document.content = content;
        }
        
        if let Some(version) = updates.version {
            document.version = version;
        }
        
        document.last_updated = Utc::now();
        
        // Record the change in governance history
        let change = GovernanceChange::new(
            format!("Updated document: {}", document.title),
            GovernanceChangeType::DocumentUpdate,
            Some(document_id),
        );
        self.governance_history.push(change);
        
        info!("Updated governance document: {}", document_id);
        Ok(())
    }

    /// Record a governance change
    pub fn record_change(&mut self, change: GovernanceChange) {
        self.governance_history.push(change);
        info!("Recorded governance change");
    }

    /// Get recent governance changes
    pub fn get_recent_changes(&self, days: i64) -> Vec<&GovernanceChange> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.governance_history.iter()
            .filter(|change| change.timestamp > cutoff)
            .collect()
    }

    /// Get governance statistics
    pub fn get_statistics(&self) -> GovernanceStatistics {
        let total_principles = self.principles.len();
        let total_roles = self.roles.len();
        let total_processes = self.decision_making_processes.len();
        let total_documents = self.governance_documents.len();
        let total_changes = self.governance_history.len();
        
        GovernanceStatistics {
            total_principles,
            total_roles,
            total_processes,
            total_documents,
            total_changes,
        }
    }

    /// Generate governance report
    pub fn generate_governance_report(&self) -> GovernanceReport {
        let stats = self.get_statistics();
        let recent_changes = self.get_recent_changes(30); // Last 30 days
        let active_documents = self.get_documents_by_status(DocumentStatus::Active);
        
        GovernanceReport {
            generated_at: Utc::now(),
            statistics: stats,
            recent_changes: recent_changes.len(),
            active_documents: active_documents.len(),
        }
    }

    /// Get a specific governance document
    pub fn get_document(&self, document_id: Uuid) -> Option<&GovernanceDocument> {
        self.governance_documents.iter().find(|d| d.id == document_id)
    }

    /// Search governance documents by keyword
    pub fn search_documents(&self, keyword: &str) -> Vec<&GovernanceDocument> {
        self.governance_documents.iter()
            .filter(|doc| {
                doc.title.to_lowercase().contains(&keyword.to_lowercase()) ||
                doc.content.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }
}

/// Governance principle guiding community decisions
#[derive(Debug, Clone)]
pub struct GovernancePrinciple {
    pub id: String,
    pub name: String,
    pub description: String,
    pub priority: PrinciplePriority,
    pub related_documents: Vec<Uuid>,
}

impl GovernancePrinciple {
    /// Create a new governance principle
    pub fn new(id: String, name: String, description: String, priority: PrinciplePriority) -> Self {
        Self {
            id,
            name,
            description,
            priority,
            related_documents: Vec::new(),
        }
    }

    /// Add related documents
    pub fn add_related_documents(&mut self, documents: Vec<Uuid>) {
        self.related_documents.extend(documents);
    }
}

/// Governance role with specific responsibilities
#[derive(Debug, Clone)]
pub struct GovernanceRole {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

impl GovernanceRole {
    /// Create a new governance role
    pub fn new(id: String, name: String, description: String, permissions: Vec<Permission>) -> Self {
        Self {
            id,
            name,
            description,
            permissions,
            created_at: Utc::now(),
            is_active: true,
        }
    }
}

/// Responsibility associated with a governance role
#[derive(Debug, Clone)]
pub struct Responsibility {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: ResponsibilityCategory,
    pub required_skills: Vec<String>,
}

impl Responsibility {
    /// Create a new responsibility
    pub fn new(id: String, name: String, description: String, category: ResponsibilityCategory) -> Self {
        Self {
            id,
            name,
            description,
            category,
            required_skills: Vec::new(),
        }
    }

    /// Add required skills
    pub fn add_required_skills(&mut self, skills: Vec<String>) {
        self.required_skills.extend(skills);
    }
}

/// Decision-making process for governance
#[derive(Debug, Clone)]
pub struct DecisionMakingProcess {
    pub id: String,
    pub name: String,
    pub description: String,
    pub process_type: ProcessType,
    pub participants: Vec<String>, // Role IDs
    pub required_quorum: Option<usize>,
    pub voting_method: Option<VotingMethod>,
    pub timeline: Option<DecisionTimeline>,
    pub escalation_path: Option<String>, // Role ID for escalation
}

impl DecisionMakingProcess {
    /// Create a new decision-making process
    pub fn new(
        id: String,
        name: String,
        description: String,
        process_type: ProcessType,
        participants: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            process_type,
            participants,
            required_quorum: None,
            voting_method: None,
            timeline: None,
            escalation_path: None,
        }
    }

    /// Set required quorum
    pub fn set_quorum(&mut self, quorum: usize) {
        self.required_quorum = Some(quorum);
    }

    /// Set voting method
    pub fn set_voting_method(&mut self, method: VotingMethod) {
        self.voting_method = Some(method);
    }

    /// Set timeline
    pub fn set_timeline(&mut self, timeline: DecisionTimeline) {
        self.timeline = Some(timeline);
    }

    /// Set escalation path
    pub fn set_escalation_path(&mut self, role_id: String) {
        self.escalation_path = Some(role_id);
    }
}

/// Governance document
#[derive(Debug, Clone)]
pub struct GovernanceDocument {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: GovernanceDocumentCategory,
    pub version: String,
    pub status: DocumentStatus,
    pub authors: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub approval_status: ApprovalStatus,
    pub related_documents: Vec<Uuid>,
}

impl GovernanceDocument {
    /// Create a new governance document
    pub fn new(
        title: String,
        content: String,
        category: GovernanceDocumentCategory,
        version: String,
        authors: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            content,
            category,
            version,
            status: DocumentStatus::Draft,
            authors,
            created_at: now,
            last_updated: now,
            approval_status: ApprovalStatus::Pending,
            related_documents: Vec::new(),
        }
    }

    /// Add related documents
    pub fn add_related_documents(&mut self, documents: Vec<Uuid>) {
        self.related_documents.extend(documents);
    }

    /// Publish the document
    pub fn publish(&mut self) {
        self.status = DocumentStatus::Active;
        self.last_updated = Utc::now();
    }

    /// Archive the document
    pub fn archive(&mut self) {
        self.status = DocumentStatus::Archived;
        self.last_updated = Utc::now();
    }
}

/// Updates for a governance document
#[derive(Debug, Clone)]
pub struct DocumentUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub version: Option<String>,
}

/// Governance change record
#[derive(Debug, Clone)]
pub struct GovernanceChange {
    pub id: Uuid,
    pub description: String,
    pub change_type: GovernanceChangeType,
    pub affected_entity: Option<Uuid>, // Document ID or other entity
    pub timestamp: DateTime<Utc>,
    pub made_by: Option<String>, // Role ID or user ID
}

impl GovernanceChange {
    /// Create a new governance change record
    pub fn new(description: String, change_type: GovernanceChangeType, affected_entity: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            change_type,
            affected_entity,
            timestamp: Utc::now(),
            made_by: None,
        }
    }

    /// Set who made the change
    pub fn set_made_by(&mut self, made_by: String) {
        self.made_by = Some(made_by);
    }
}

/// Statistics about governance framework
#[derive(Debug, Clone)]
pub struct GovernanceStatistics {
    pub total_principles: usize,
    pub total_roles: usize,
    pub total_processes: usize,
    pub total_documents: usize,
    pub total_changes: usize,
}

/// Governance report
#[derive(Debug, Clone)]
pub struct GovernanceReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: GovernanceStatistics,
    pub recent_changes: usize,
    pub active_documents: usize,
}

/// Types of governance principles
#[derive(Debug, Clone)]
pub enum PrinciplePriority {
    Fundamental,
    Important,
    Advisory,
}

/// Types of permissions
#[derive(Debug, Clone)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
    Vote,
    Propose,
    Review,
}

/// Categories of responsibilities
#[derive(Debug, Clone)]
pub enum ResponsibilityCategory {
    Technical,
    Community,
    Governance,
    Documentation,
    Support,
    Translation,
}

/// Types of decision-making processes
#[derive(Debug, Clone)]
pub enum ProcessType {
    Consensus,
    Voting,
    Proposal,
    Discussion,
    Delegation,
}

/// Voting methods for decisions
#[derive(Debug, Clone)]
pub enum VotingMethod {
    SimpleMajority,
    SuperMajority,
    Unanimous,
    RankedChoice,
    Quadratic,
}

/// Timeline for decision-making
#[derive(Debug, Clone)]
pub struct DecisionTimeline {
    pub proposal_period: Option<i64>, // Hours
    pub discussion_period: Option<i64>, // Hours
    pub voting_period: Option<i64>, // Hours
    pub implementation_period: Option<i64>, // Hours
}

/// Categories of governance documents
#[derive(Debug, Clone, PartialEq)]
pub enum GovernanceDocumentCategory {
    Principles,
    Roles,
    Processes,
    Policies,
    Procedures,
    Guidelines,
    MeetingNotes,
    Agreements,
}

/// Status of governance documents
#[derive(Debug, Clone, PartialEq)]
pub enum DocumentStatus {
    Draft,
    Review,
    Active,
    Archived,
}

/// Approval status for documents
#[derive(Debug, Clone, PartialEq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    RequiresChanges,
}

/// Types of governance changes
#[derive(Debug, Clone)]
pub enum GovernanceChangeType {
    PrincipleAdded,
    RoleDefined,
    ResponsibilityAssigned,
    ProcessDefined,
    DocumentCreated,
    DocumentUpdate,
    DocumentPublished,
    DocumentArchived,
}

/// Error types for governance framework
#[derive(Debug)]
pub enum GovernanceError {
    RoleNotFound(String),
    DocumentNotFound(Uuid),
    ProcessNotFound(String),
    UpdateError(String),
}

impl std::fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GovernanceError::RoleNotFound(id) => write!(f, "Role not found: {}", id),
            GovernanceError::DocumentNotFound(id) => write!(f, "Document not found: {}", id),
            GovernanceError::ProcessNotFound(id) => write!(f, "Process not found: {}", id),
            GovernanceError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for GovernanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_framework_initialization() {
        let framework = GovernanceFramework::new();
        assert!(framework.principles.is_empty());
        assert!(framework.roles.is_empty());
    }

    #[test]
    fn test_add_principle() {
        let mut framework = GovernanceFramework::new();
        let principle = GovernancePrinciple::new(
            "community_benefit".to_string(),
            "Community Benefit".to_string(),
            "All decisions should prioritize community benefit".to_string(),
            PrinciplePriority::Fundamental,
        );
        
        framework.add_principle(principle);
        assert_eq!(framework.principles.len(), 1);
        assert_eq!(framework.principles[0].id, "community_benefit");
    }

    #[test]
    fn test_define_role() {
        let mut framework = GovernanceFramework::new();
        let role = GovernanceRole::new(
            "governance_lead".to_string(),
            "Governance Lead".to_string(),
            "Lead for governance activities".to_string(),
            vec![Permission::Read, Permission::Write, Permission::Propose],
        );
        
        let role_id = framework.define_role(role);
        assert_eq!(role_id, "governance_lead");
        assert!(framework.roles.contains_key("governance_lead"));
    }

    #[test]
    fn test_assign_responsibilities() {
        let mut framework = GovernanceFramework::new();
        let role = GovernanceRole::new(
            "governance_lead".to_string(),
            "Governance Lead".to_string(),
            "Lead for governance activities".to_string(),
            vec![Permission::Read, Permission::Write, Permission::Propose],
        );
        
        framework.define_role(role);
        
        let responsibilities = vec![
            Responsibility::new(
                "governance_oversight".to_string(),
                "Governance Oversight".to_string(),
                "Oversight of governance processes".to_string(),
                ResponsibilityCategory::Governance,
            )
        ];
        
        let result = framework.assign_responsibilities("governance_lead", responsibilities);
        assert!(result.is_ok());
        assert!(framework.responsibilities.contains_key("governance_lead"));
    }

    #[test]
    fn test_define_decision_process() {
        let mut framework = GovernanceFramework::new();
        let process = DecisionMakingProcess::new(
            "feature_voting".to_string(),
            "Feature Voting Process".to_string(),
            "Process for voting on new features".to_string(),
            ProcessType::Voting,
            vec!["community_member".to_string()],
        );
        
        let process_id = framework.define_decision_process(process);
        assert_eq!(process_id, "feature_voting");
        assert!(framework.decision_making_processes.contains_key("feature_voting"));
    }

    #[test]
    fn test_create_document() {
        let mut framework = GovernanceFramework::new();
        let document = GovernanceDocument::new(
            "Governance Principles".to_string(),
            "Core principles for community governance".to_string(),
            GovernanceDocumentCategory::Principles,
            "1.0".to_string(),
            vec!["governance_team".to_string()],
        );
        
        let document_id = framework.create_document(document);
        assert!(!document_id.is_nil());
        assert_eq!(framework.governance_documents.len(), 1);
    }

    #[test]
    fn test_get_documents_by_category() {
        let mut framework = GovernanceFramework::new();
        let document = GovernanceDocument::new(
            "Governance Principles".to_string(),
            "Core principles for community governance".to_string(),
            GovernanceDocumentCategory::Principles,
            "1.0".to_string(),
            vec!["governance_team".to_string()],
        );
        
        framework.create_document(document);
        let principles_docs = framework.get_documents_by_category(GovernanceDocumentCategory::Principles);
        assert_eq!(principles_docs.len(), 1);
    }

    #[test]
    fn test_update_document() {
        let mut framework = GovernanceFramework::new();
        let document = GovernanceDocument::new(
            "Governance Principles".to_string(),
            "Core principles for community governance".to_string(),
            GovernanceDocumentCategory::Principles,
            "1.0".to_string(),
            vec!["governance_team".to_string()],
        );
        
        let document_id = framework.create_document(document);
        let updates = DocumentUpdate {
            title: Some("Updated Governance Principles".to_string()),
            content: Some("Updated core principles for community governance".to_string()),
            version: Some("1.1".to_string()),
        };
        
        let result = framework.update_document(document_id, updates);
        assert!(result.is_ok());
        
        let updated_document = framework.get_document(document_id).unwrap();
        assert_eq!(updated_document.title, "Updated Governance Principles");
        assert_eq!(updated_document.version, "1.1");
    }

    #[test]
    fn test_record_change() {
        let mut framework = GovernanceFramework::new();
        let change = GovernanceChange::new(
            "Added new principle".to_string(),
            GovernanceChangeType::PrincipleAdded,
            None,
        );
        
        framework.record_change(change);
        assert_eq!(framework.governance_history.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut framework = GovernanceFramework::new();
        
        // Add some data
        let principle = GovernancePrinciple::new(
            "community_benefit".to_string(),
            "Community Benefit".to_string(),
            "All decisions should prioritize community benefit".to_string(),
            PrinciplePriority::Fundamental,
        );
        framework.add_principle(principle);
        
        let role = GovernanceRole::new(
            "governance_lead".to_string(),
            "Governance Lead".to_string(),
            "Lead for governance activities".to_string(),
            vec![Permission::Read, Permission::Write, Permission::Propose],
        );
        framework.define_role(role);
        
        let document = GovernanceDocument::new(
            "Governance Principles".to_string(),
            "Core principles for community governance".to_string(),
            GovernanceDocumentCategory::Principles,
            "1.0".to_string(),
            vec!["governance_team".to_string()],
        );
        framework.create_document(document);
        
        let stats = framework.get_statistics();
        assert_eq!(stats.total_principles, 1);
        assert_eq!(stats.total_roles, 1);
        assert_eq!(stats.total_documents, 1);
    }

    #[test]
    fn test_search_documents() {
        let mut framework = GovernanceFramework::new();
        let document = GovernanceDocument::new(
            "Governance Principles".to_string(),
            "Core principles for community governance".to_string(),
            GovernanceDocumentCategory::Principles,
            "1.0".to_string(),
            vec!["governance_team".to_string()],
        );
        
        framework.create_document(document);
        let results = framework.search_documents("principles");
        assert_eq!(results.len(), 1);
    }
}