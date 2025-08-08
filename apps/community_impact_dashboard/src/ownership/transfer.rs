//! Ownership transfer processes for the Unified Community Impact Dashboard
//!
//! This module manages the formal processes for transferring ownership of dashboard
//! components, features, and governance responsibilities from initial development
//! teams to the community.

use tracing::info;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

/// Ownership transfer system
pub struct OwnershipTransferSystem {
    transfer_requests: HashMap<Uuid, TransferRequest>,
    transfer_processes: HashMap<Uuid, TransferProcess>,
    transfer_records: Vec<TransferRecord>,
    transfer_policies: Vec<TransferPolicy>,
    community_stewards: Vec<CommunitySteward>,
    transfer_history: Vec<TransferHistoryEntry>,
}

impl OwnershipTransferSystem {
    /// Create a new ownership transfer system
    pub fn new() -> Self {
        Self {
            transfer_requests: HashMap::new(),
            transfer_processes: HashMap::new(),
            transfer_records: Vec::new(),
            transfer_policies: vec![
                TransferPolicy::new(
                    "standard_transfer".to_string(),
                    "Standard Ownership Transfer".to_string(),
                    vec![
                        TransferRequirement::DocumentationComplete,
                        TransferRequirement::CommunityReview,
                        TransferRequirement::KnowledgeTransfer,
                        TransferRequirement::SupportAgreement,
                    ],
                    chrono::Duration::weeks(2),
                ),
                TransferPolicy::new(
                    "urgent_transfer".to_string(),
                    "Urgent Ownership Transfer".to_string(),
                    vec![
                        TransferRequirement::DocumentationComplete,
                        TransferRequirement::KnowledgeTransfer,
                    ],
                    chrono::Duration::days(3),
                ),
                TransferPolicy::new(
                    "governance_transfer".to_string(),
                    "Governance Ownership Transfer".to_string(),
                    vec![
                        TransferRequirement::CommunityConsent,
                        TransferRequirement::DocumentationComplete,
                        TransferRequirement::KnowledgeTransfer,
                        TransferRequirement::SupportAgreement,
                        TransferRequirement::TransitionPeriod,
                    ],
                    chrono::Duration::weeks(4),
                ),
            ],
            community_stewards: Vec::new(),
            transfer_history: Vec::new(),
        }
    }

    /// Add a community steward
    pub fn add_community_steward(&mut self, steward: CommunitySteward) {
        self.community_stewards.push(steward);
        info!("Added community steward: {}", &self.community_stewards.last().unwrap().id);
    }

    /// Create a transfer request
    pub fn create_transfer_request(&mut self, request: TransferRequest) -> Uuid {
        let request_id = request.id;
        self.transfer_requests.insert(request_id, request);
        info!("Created transfer request: {}", request_id);
        request_id
    }

    /// Get a transfer request by ID
    pub fn get_transfer_request(&self, request_id: Uuid) -> Option<&TransferRequest> {
        self.transfer_requests.get(&request_id)
    }

    /// Update transfer request status
    pub fn update_request_status(&mut self, request_id: Uuid, status: TransferRequestStatus) -> Result<(), TransferError> {
        let request = self.transfer_requests.get_mut(&request_id)
            .ok_or(TransferError::RequestNotFound(request_id))?;
        
        request.status = status;
        request.updated_at = Utc::now();
        
        info!("Updated transfer request {} status to {:?}", request_id, status);
        Ok(())
    }

    /// Submit transfer request for review
    pub fn submit_request(&mut self, request_id: Uuid) -> Result<(), TransferError> {
        self.update_request_status(request_id, TransferRequestStatus::Submitted)
    }

    /// Approve a transfer request
    pub fn approve_request(&mut self, request_id: Uuid) -> Result<(), TransferError> {
        self.update_request_status(request_id, TransferRequestStatus::Approved)
    }

    /// Reject a transfer request
    pub fn reject_request(&mut self, request_id: Uuid) -> Result<(), TransferError> {
        self.update_request_status(request_id, TransferRequestStatus::Rejected)
    }

    /// Start transfer process
    pub fn start_transfer_process(&mut self, process: TransferProcess) -> Uuid {
        let process_id = process.id;
        self.transfer_processes.insert(process_id, process);
        info!("Started transfer process: {}", process_id);
        process_id
    }

    /// Get a transfer process by ID
    pub fn get_transfer_process(&self, process_id: Uuid) -> Option<&TransferProcess> {
        self.transfer_processes.get(&process_id)
    }

    /// Update transfer process status
    pub fn update_process_status(&mut self, process_id: Uuid, status: TransferProcessStatus) -> Result<(), TransferError> {
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        process.status = status;
        process.updated_at = Utc::now();
        
        info!("Updated transfer process {} status to {:?}", process_id, status);
        Ok(())
    }

    /// Add a requirement to a transfer process
    pub fn add_requirement(&mut self, process_id: Uuid, requirement: TransferRequirementStatus) -> Result<(), TransferError> {
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        process.requirements.push(requirement);
        info!("Added requirement to transfer process: {}", process_id);
        Ok(())
    }

    /// Mark a requirement as completed
    pub fn complete_requirement(&mut self, process_id: Uuid, requirement_type: TransferRequirement) -> Result<(), TransferError> {
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        let requirement = process.requirements.iter_mut()
            .find(|r| r.requirement_type == requirement_type)
            .ok_or(TransferError::RequirementNotFound(requirement_type.clone()))?;
        
        requirement.status = RequirementStatus::Completed;
        requirement.completed_at = Some(Utc::now());
        
        info!("Completed requirement {:?} for transfer process: {}", requirement_type, process_id);
        Ok(())
    }

    /// Check if all requirements are met for a transfer process
    pub fn check_requirements_met(&self, process_id: Uuid) -> Result<bool, TransferError> {
        let process = self.transfer_processes.get(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        // Check if all required requirements are completed
        let all_completed = process.requirements.iter()
            .filter(|r| r.is_required)
            .all(|r| r.status == RequirementStatus::Completed);
        
        Ok(all_completed)
    }

    /// Complete transfer process
    pub fn complete_transfer_process(&mut self, process_id: Uuid) -> Result<(), TransferError> {
        // Check if all requirements are met
        if !self.check_requirements_met(process_id)? {
            return Err(TransferError::RequirementsNotMet);
        }
        
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        process.status = TransferProcessStatus::Completed;
        process.completed_at = Some(Utc::now());
        
        // Create transfer record
        let record = TransferRecord::new(
            process.component_id,
            process.from_owner.clone(),
            process.to_owner.clone(),
            process.transfer_type.clone(),
        );
        self.transfer_records.push(record);
        
        // Record in history
        let history_entry = TransferHistoryEntry::new(
            format!("Completed transfer of component {} from {} to {}", 
                   process.component_id, process.from_owner, process.to_owner),
            process.component_id,
            process.from_owner.clone(),
            process.to_owner.clone(),
            TransferHistoryType::TransferCompleted,
        );
        self.transfer_history.push(history_entry);
        
        info!("Completed transfer process: {}", process_id);
        Ok(())
    }

    /// Record a transfer activity
    pub fn record_transfer(&mut self, record: TransferRecord) {
        self.transfer_records.push(record);
        info!("Recorded transfer");
    }

    /// Get recent transfer activities
    pub fn get_recent_transfers(&self, days: i64) -> Vec<&TransferRecord> {
        let cutoff = Utc::now() - chrono::Duration::days(days);
        self.transfer_records.iter()
            .filter(|record| record.timestamp > cutoff)
            .collect()
    }

    /// Get transfer history
    pub fn get_transfer_history(&self, component_id: Option<Uuid>) -> Vec<&TransferHistoryEntry> {
        if let Some(id) = component_id {
            self.transfer_history.iter()
                .filter(|entry| entry.component_id == Some(id))
                .collect()
        } else {
            self.transfer_history.iter().collect()
        }
    }

    /// Get transfer statistics
    pub fn get_statistics(&self) -> TransferStatistics {
        let total_requests = self.transfer_requests.len();
        let total_processes = self.transfer_processes.len();
        let total_records = self.transfer_records.len();
        let total_stewards = self.community_stewards.len();
        
        // Count requests by status
        let pending_requests = self.transfer_requests.values()
            .filter(|r| r.status == TransferRequestStatus::Pending)
            .count();
        
        let approved_requests = self.transfer_requests.values()
            .filter(|r| r.status == TransferRequestStatus::Approved)
            .count();
        
        let completed_processes = self.transfer_processes.values()
            .filter(|p| p.status == TransferProcessStatus::Completed)
            .count();
        
        TransferStatistics {
            total_requests,
            total_processes,
            total_records,
            total_stewards,
            pending_requests,
            approved_requests,
            completed_processes,
        }
    }

    /// Generate transfer report
    pub fn generate_transfer_report(&self) -> TransferReport {
        let stats = self.get_statistics();
        let recent_transfers = self.get_recent_transfers(30); // Last 30 days
        let recent_history = self.transfer_history.iter()
            .filter(|entry| {
                let cutoff = Utc::now() - chrono::Duration::days(30);
                entry.timestamp > cutoff
            })
            .count();
        
        TransferReport {
            generated_at: Utc::now(),
            statistics: stats,
            recent_transfers: recent_transfers.len(),
            recent_history,
        }
    }

    /// Search transfer requests by keyword
    pub fn search_requests(&self, keyword: &str) -> Vec<&TransferRequest> {
        self.transfer_requests.values()
            .filter(|request| {
                request.reason.to_lowercase().contains(&keyword.to_lowercase()) ||
                request.component_name.to_lowercase().contains(&keyword.to_lowercase())
            })
            .collect()
    }

    /// Get transfer requests by status
    pub fn get_requests_by_status(&self, status: TransferRequestStatus) -> Vec<&TransferRequest> {
        self.transfer_requests.values()
            .filter(|request| request.status == status)
            .collect()
    }

    /// Get transfer processes by status
    pub fn get_processes_by_status(&self, status: TransferProcessStatus) -> Vec<&TransferProcess> {
        self.transfer_processes.values()
            .filter(|process| process.status == status)
            .collect()
    }

    /// Get transfer policies
    pub fn get_policies(&self) -> &Vec<TransferPolicy> {
        &self.transfer_policies
    }

    /// Get community stewards
    pub fn get_community_stewards(&self) -> &Vec<CommunitySteward> {
        &self.community_stewards
    }

    /// Assign steward to transfer process
    pub fn assign_steward(&mut self, process_id: Uuid, steward_id: Uuid) -> Result<(), TransferError> {
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        process.assigned_steward = Some(steward_id);
        process.updated_at = Utc::now();
        
        info!("Assigned steward {} to transfer process: {}", steward_id, process_id);
        Ok(())
    }

    /// Get overdue transfer processes
    pub fn get_overdue_processes(&self) -> Vec<&TransferProcess> {
        let now = Utc::now();
        self.transfer_processes.values()
            .filter(|process| {
                if let Some(deadline) = process.deadline {
                    process.status == TransferProcessStatus::InProgress && now > deadline
                } else {
                    false
                }
            })
            .collect()
    }

    /// Extend transfer process deadline
    pub fn extend_deadline(&mut self, process_id: Uuid, extension: chrono::Duration) -> Result<(), TransferError> {
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        if let Some(deadline) = process.deadline {
            process.deadline = Some(deadline + extension);
            process.updated_at = Utc::now();
            
            info!("Extended deadline for transfer process: {} by {:?}", process_id, extension);
            Ok(())
        } else {
            Err(TransferError::NoDeadlineSet)
        }
    }

    /// Get transfer process by component ID
    pub fn get_process_by_component(&self, component_id: Uuid) -> Option<&TransferProcess> {
        self.transfer_processes.values()
            .find(|process| process.component_id == component_id)
    }

    /// Cancel transfer process
    pub fn cancel_process(&mut self, process_id: Uuid) -> Result<(), TransferError> {
        let process = self.transfer_processes.get_mut(&process_id)
            .ok_or(TransferError::ProcessNotFound(process_id))?;
        
        process.status = TransferProcessStatus::Cancelled;
        process.completed_at = Some(Utc::now());
        
        // Record cancellation in history
        let history_entry = TransferHistoryEntry::new(
            format!("Cancelled transfer process for component {}", process.component_id),
            process.component_id,
            process.from_owner.clone(),
            process.to_owner.clone(),
            TransferHistoryType::TransferCancelled,
        );
        self.transfer_history.push(history_entry);
        
        info!("Cancelled transfer process: {}", process_id);
        Ok(())
    }
}

/// Transfer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    pub id: Uuid,
    pub component_id: Uuid,
    pub component_name: String,
    pub from_owner: String, // User ID or Role ID
    pub to_owner: String, // User ID or Role ID
    pub reason: String,
    pub status: TransferRequestStatus,
    pub priority: PriorityLevel,
    pub requested_by: String, // User ID
    pub requested_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub supporting_documents: Vec<Uuid>, // Document IDs
    pub estimated_impact: Option<String>,
}

impl TransferRequest {
    /// Create a new transfer request
    pub fn new(
        component_id: Uuid,
        component_name: String,
        from_owner: String,
        to_owner: String,
        reason: String,
        requested_by: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            component_id,
            component_name,
            from_owner,
            to_owner,
            reason,
            status: TransferRequestStatus::Pending,
            priority: PriorityLevel::Medium,
            requested_by,
            requested_at: now,
            updated_at: now,
            supporting_documents: Vec::new(),
            estimated_impact: None,
        }
    }

    /// Set priority level
    pub fn set_priority(&mut self, priority: PriorityLevel) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }

    /// Add supporting documents
    pub fn add_supporting_documents(&mut self, documents: Vec<Uuid>) {
        self.supporting_documents.extend(documents);
        self.updated_at = Utc::now();
    }

    /// Set estimated impact
    pub fn set_estimated_impact(&mut self, impact: String) {
        self.estimated_impact = Some(impact);
        self.updated_at = Utc::now();
    }

    /// Submit the request
    pub fn submit(&mut self) {
        self.status = TransferRequestStatus::Submitted;
        self.updated_at = Utc::now();
    }

    /// Approve the request
    pub fn approve(&mut self) {
        self.status = TransferRequestStatus::Approved;
        self.updated_at = Utc::now();
    }

    /// Reject the request
    pub fn reject(&mut self) {
        self.status = TransferRequestStatus::Rejected;
        self.updated_at = Utc::now();
    }
}

/// Transfer process
#[derive(Debug, Clone)]
pub struct TransferProcess {
    pub id: Uuid,
    pub component_id: Uuid,
    pub component_name: String,
    pub from_owner: String, // User ID or Role ID
    pub to_owner: String, // User ID or Role ID
    pub transfer_type: TransferType,
    pub status: TransferProcessStatus,
    pub requirements: Vec<TransferRequirementStatus>,
    pub assigned_steward: Option<Uuid>, // Steward ID
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub progress_notes: Vec<ProgressNote>,
}

impl TransferProcess {
    /// Create a new transfer process
    pub fn new(
        component_id: Uuid,
        component_name: String,
        from_owner: String,
        to_owner: String,
        transfer_type: TransferType,
        policy: &TransferPolicy,
    ) -> Self {
        let now = Utc::now();
        let deadline = Some(now + policy.max_duration);
        
        // Create requirement statuses from policy requirements
        let requirements = policy.requirements.iter()
            .map(|req| TransferRequirementStatus::new(*req, true))
            .collect();
        
        Self {
            id: Uuid::new_v4(),
            component_id,
            component_name,
            from_owner,
            to_owner,
            transfer_type,
            status: TransferProcessStatus::Created,
            requirements,
            assigned_steward: None,
            created_at: now,
            started_at: None,
            updated_at: now,
            completed_at: None,
            deadline,
            progress_notes: Vec::new(),
        }
    }

    /// Start the transfer process
    pub fn start(&mut self) {
        self.status = TransferProcessStatus::InProgress;
        self.started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Add a progress note
    pub fn add_progress_note(&mut self, note: ProgressNote) {
        self.progress_notes.push(note);
        self.updated_at = Utc::now();
    }

    /// Get progress percentage
    pub fn get_progress(&self) -> f64 {
        if self.requirements.is_empty() {
            return 0.0;
        }
        
        let completed = self.requirements.iter()
            .filter(|r| r.status == RequirementStatus::Completed)
            .count();
        
        (completed as f64 / self.requirements.len() as f64) * 100.0
    }

    /// Get pending requirements
    pub fn get_pending_requirements(&self) -> Vec<&TransferRequirementStatus> {
        self.requirements.iter()
            .filter(|r| r.status != RequirementStatus::Completed)
            .collect()
    }

    /// Get completed requirements
    pub fn get_completed_requirements(&self) -> Vec<&TransferRequirementStatus> {
        self.requirements.iter()
            .filter(|r| r.status == RequirementStatus::Completed)
            .collect()
    }

    /// Check if process is overdue
    pub fn is_overdue(&self) -> bool {
        if let Some(deadline) = self.deadline {
            self.status == TransferProcessStatus::InProgress && Utc::now() > deadline
        } else {
            false
        }
    }

    /// Get time remaining until deadline
    pub fn time_remaining(&self) -> Option<chrono::Duration> {
        if let Some(deadline) = self.deadline {
            let now = Utc::now();
            if deadline > now {
                Some(deadline - now)
            } else {
                Some(chrono::Duration::zero())
            }
        } else {
            None
        }
    }
}

/// Transfer requirement status
#[derive(Debug, Clone)]
pub struct TransferRequirementStatus {
    pub requirement_type: TransferRequirement,
    pub status: RequirementStatus,
    pub is_required: bool,
    pub assigned_to: Option<String>, // User ID
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Vec<String>,
}

impl TransferRequirementStatus {
    /// Create a new transfer requirement status
    pub fn new(requirement_type: TransferRequirement, is_required: bool) -> Self {
        Self {
            requirement_type,
            status: RequirementStatus::Pending,
            is_required,
            assigned_to: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
            notes: Vec::new(),
        }
    }

    /// Start working on the requirement
    pub fn start(&mut self) {
        self.status = RequirementStatus::InProgress;
        self.started_at = Some(Utc::now());
    }

    /// Complete the requirement
    pub fn complete(&mut self) {
        self.status = RequirementStatus::Completed;
        self.completed_at = Some(Utc::now());
    }

    /// Add a note
    pub fn add_note(&mut self, note: String) {
        self.notes.push(note);
    }

    /// Assign to a user
    pub fn assign_to(&mut self, user_id: String) {
        self.assigned_to = Some(user_id);
    }
}

/// Progress note for transfer process
#[derive(Debug, Clone)]
pub struct ProgressNote {
    pub id: Uuid,
    pub author: String, // User ID
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub visibility: NoteVisibility,
}

impl ProgressNote {
    /// Create a new progress note
    pub fn new(author: String, content: String, visibility: NoteVisibility) -> Self {
        Self {
            id: Uuid::new_v4(),
            author,
            content,
            timestamp: Utc::now(),
            visibility,
        }
    }
}

/// Transfer record for historical tracking
#[derive(Debug, Clone)]
pub struct TransferRecord {
    pub id: Uuid,
    pub component_id: Uuid,
    pub from_owner: String, // User ID or Role ID
    pub to_owner: String, // User ID or Role ID
    pub transfer_type: TransferType,
    pub timestamp: DateTime<Utc>,
    pub recorded_by: Option<String>, // User ID
    pub notes: Option<String>,
}

impl TransferRecord {
    /// Create a new transfer record
    pub fn new(
        component_id: Uuid,
        from_owner: String,
        to_owner: String,
        transfer_type: TransferType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            component_id,
            from_owner,
            to_owner,
            transfer_type,
            timestamp: Utc::now(),
            recorded_by: None,
            notes: None,
        }
    }

    /// Set who recorded the transfer
    pub fn set_recorded_by(&mut self, recorded_by: String) {
        self.recorded_by = Some(recorded_by);
    }

    /// Add notes
    pub fn add_notes(&mut self, notes: String) {
        self.notes = Some(notes);
    }
}

/// Transfer policy
#[derive(Debug, Clone)]
pub struct TransferPolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub requirements: Vec<TransferRequirement>,
    pub max_duration: chrono::Duration,
    pub created_at: DateTime<Utc>,
}

impl TransferPolicy {
    /// Create a new transfer policy
    pub fn new(
        id: String,
        name: String,
        requirements: Vec<TransferRequirement>,
        max_duration: chrono::Duration,
    ) -> Self {
        Self {
            id,
            name,
            description: String::new(),
            requirements,
            max_duration,
            created_at: Utc::now(),
        }
    }

    /// Set policy description
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

/// Community steward
#[derive(Debug, Clone)]
pub struct CommunitySteward {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub expertise_areas: Vec<StewardExpertise>,
    pub assigned_transfers: Vec<Uuid>, // Transfer process IDs
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

impl CommunitySteward {
    /// Create a new community steward
    pub fn new(user_id: String, name: String, expertise_areas: Vec<StewardExpertise>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            expertise_areas,
            assigned_transfers: Vec::new(),
            created_at: Utc::now(),
            is_active: true,
        }
    }

    /// Add assigned transfer
    pub fn add_assigned_transfer(&mut self, process_id: Uuid) {
        self.assigned_transfers.push(process_id);
    }

    /// Remove assigned transfer
    pub fn remove_assigned_transfer(&mut self, process_id: Uuid) {
        self.assigned_transfers.retain(|&id| id != process_id);
    }

    /// Deactivate steward
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Reactivate steward
    pub fn reactivate(&mut self) {
        self.is_active = true;
    }

    /// Check if steward has expertise in area
    pub fn has_expertise(&self, area: &StewardExpertise) -> bool {
        self.expertise_areas.contains(area)
    }
}

/// Transfer history entry
#[derive(Debug, Clone)]
pub struct TransferHistoryEntry {
    pub id: Uuid,
    pub description: String,
    pub component_id: Option<Uuid>,
    pub from_owner: Option<String>, // User ID or Role ID
    pub to_owner: Option<String>, // User ID or Role ID
    pub entry_type: TransferHistoryType,
    pub timestamp: DateTime<Utc>,
    pub recorded_by: Option<String>, // User ID
}

impl TransferHistoryEntry {
    /// Create a new transfer history entry
    pub fn new(
        description: String,
        component_id: Uuid,
        from_owner: String,
        to_owner: String,
        entry_type: TransferHistoryType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            description,
            component_id: Some(component_id),
            from_owner: Some(from_owner),
            to_owner: Some(to_owner),
            entry_type,
            timestamp: Utc::now(),
            recorded_by: None,
        }
    }

    /// Set who recorded the entry
    pub fn set_recorded_by(&mut self, recorded_by: String) {
        self.recorded_by = Some(recorded_by);
    }
}

/// Statistics about ownership transfers
#[derive(Debug, Clone)]
pub struct TransferStatistics {
    pub total_requests: usize,
    pub total_processes: usize,
    pub total_records: usize,
    pub total_stewards: usize,
    pub pending_requests: usize,
    pub approved_requests: usize,
    pub completed_processes: usize,
}

/// Transfer report
#[derive(Debug, Clone)]
pub struct TransferReport {
    pub generated_at: DateTime<Utc>,
    pub statistics: TransferStatistics,
    pub recent_transfers: usize,
    pub recent_history: usize,
}

/// Status of transfer requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferRequestStatus {
    Pending,
    Submitted,
    Approved,
    Rejected,
    Cancelled,
}

/// Status of transfer processes
#[derive(Debug, Clone, PartialEq)]
pub enum TransferProcessStatus {
    Created,
    InProgress,
    Completed,
    Cancelled,
    OnHold,
}

/// Types of transfer requirements
#[derive(Debug, Clone, PartialEq)]
pub enum TransferRequirement {
    DocumentationComplete,
    CommunityReview,
    KnowledgeTransfer,
    SupportAgreement,
    TransitionPeriod,
    CommunityConsent,
    TrainingComplete,
    AccessTransfer,
}

/// Status of requirements
#[derive(Debug, Clone, PartialEq)]
pub enum RequirementStatus {
    Pending,
    InProgress,
    Completed,
    Blocked,
}

/// Types of transfers
#[derive(Debug, Clone, PartialEq)]
pub enum TransferType {
    ComponentOwnership,
    FeatureOwnership,
    GovernanceResponsibility,
    DataStewardship,
    CodeOwnership,
    DocumentationOwnership,
}

/// Visibility of progress notes
#[derive(Debug, Clone, PartialEq)]
pub enum NoteVisibility {
    Public,
    Community,
    Stewards,
    Private,
}

/// Types of steward expertise
#[derive(Debug, Clone, PartialEq)]
pub enum StewardExpertise {
    Technical,
    Community,
    Governance,
    Documentation,
    Data,
    Security,
    UserExperience,
}

/// Types of transfer history entries
#[derive(Debug, Clone, PartialEq)]
pub enum TransferHistoryType {
    TransferRequested,
    TransferApproved,
    TransferStarted,
    TransferCompleted,
    TransferCancelled,
    RequirementCompleted,
    ProcessUpdated,
}

/// Priority levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Error types for transfer system
#[derive(Debug)]
pub enum TransferError {
    RequestNotFound(Uuid),
    ProcessNotFound(Uuid),
    RequirementNotFound(TransferRequirement),
    RequirementsNotMet,
    NoDeadlineSet,
    StewardNotFound(Uuid),
    UpdateError(String),
}

impl std::fmt::Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferError::RequestNotFound(id) => write!(f, "Transfer request not found: {}", id),
            TransferError::ProcessNotFound(id) => write!(f, "Transfer process not found: {}", id),
            TransferError::RequirementNotFound(req) => write!(f, "Transfer requirement not found: {:?}", req),
            TransferError::RequirementsNotMet => write!(f, "Not all requirements are met"),
            TransferError::NoDeadlineSet => write!(f, "No deadline set for process"),
            TransferError::StewardNotFound(id) => write!(f, "Community steward not found: {}", id),
            TransferError::UpdateError(msg) => write!(f, "Update error: {}", msg),
        }
    }
}

impl std::error::Error for TransferError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_transfer_system_initialization() {
        let system = OwnershipTransferSystem::new();
        assert!(system.transfer_requests.is_empty());
        assert!(system.transfer_processes.is_empty());
        assert!(!system.transfer_policies.is_empty());
        assert!(system.community_stewards.is_empty());
    }

    #[test]
    fn test_create_transfer_request() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let request = TransferRequest::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            "Transfer for community ownership".to_string(),
            "requester123".to_string(),
        );
        
        let request_id = system.create_transfer_request(request);
        assert!(!request_id.is_nil());
        assert_eq!(system.transfer_requests.len(), 1);
    }

    #[test]
    fn test_update_request_status() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let request = TransferRequest::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            "Transfer for community ownership".to_string(),
            "requester123".to_string(),
        );
        
        let request_id = system.create_transfer_request(request);
        let result = system.update_request_status(request_id, TransferRequestStatus::Submitted);
        assert!(result.is_ok());
        
        let updated_request = system.get_transfer_request(request_id).unwrap();
        assert_eq!(updated_request.status, TransferRequestStatus::Submitted);
    }

    #[test]
    fn test_start_transfer_process() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        let process_id = system.start_transfer_process(process);
        assert!(!process_id.is_nil());
        assert_eq!(system.transfer_processes.len(), 1);
    }

    #[test]
    fn test_add_requirement() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        let requirement = TransferRequirementStatus::new(TransferRequirement::TrainingComplete, true);
        let result = system.add_requirement(process_id, requirement);
        assert!(result.is_ok());
        
        let updated_process = system.get_transfer_process(process_id).unwrap();
        assert_eq!(updated_process.requirements.len(), 5); // 4 from policy + 1 added
    }

    #[test]
    fn test_complete_requirement() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        let result = system.complete_requirement(process_id, TransferRequirement::DocumentationComplete);
        assert!(result.is_ok());
        
        let updated_process = system.get_transfer_process(process_id).unwrap();
        let requirement = updated_process.requirements.iter()
            .find(|r| r.requirement_type == TransferRequirement::DocumentationComplete)
            .unwrap();
        assert_eq!(requirement.status, RequirementStatus::Completed);
    }

    #[test]
    fn test_check_requirements_met() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        // Complete all requirements
        for requirement in &[
            TransferRequirement::DocumentationComplete,
            TransferRequirement::CommunityReview,
            TransferRequirement::KnowledgeTransfer,
            TransferRequirement::SupportAgreement,
        ] {
            system.complete_requirement(process_id, *requirement).unwrap();
        }
        
        let all_met = system.check_requirements_met(process_id).unwrap();
        assert!(all_met);
    }

    #[test]
    fn test_complete_transfer_process() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        // Complete all requirements
        for requirement in &[
            TransferRequirement::DocumentationComplete,
            TransferRequirement::CommunityReview,
            TransferRequirement::KnowledgeTransfer,
            TransferRequirement::SupportAgreement,
        ] {
            system.complete_requirement(process_id, *requirement).unwrap();
        }
        
        let result = system.complete_transfer_process(process_id);
        assert!(result.is_ok());
        
        let updated_process = system.get_transfer_process(process_id).unwrap();
        assert_eq!(updated_process.status, TransferProcessStatus::Completed);
        assert_eq!(system.transfer_records.len(), 1);
    }

    #[test]
    fn test_add_community_steward() {
        let mut system = OwnershipTransferSystem::new();
        let steward = CommunitySteward::new(
            "user123".to_string(),
            "Alice Steward".to_string(),
            vec![StewardExpertise::Technical, StewardExpertise::Community],
        );
        
        system.add_community_steward(steward);
        assert_eq!(system.community_stewards.len(), 1);
    }

    #[test]
    fn test_get_statistics() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let request = TransferRequest::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            "Transfer for community ownership".to_string(),
            "requester123".to_string(),
        );
        
        system.create_transfer_request(request);
        
        let stats = system.get_statistics();
        assert_eq!(stats.total_requests, 1);
        assert_eq!(stats.total_processes, 0);
        assert_eq!(stats.total_records, 0);
    }

    #[test]
    fn test_search_requests() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let request = TransferRequest::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            "Transfer for community ownership of dashboard".to_string(),
            "requester123".to_string(),
        );
        
        system.create_transfer_request(request);
        
        let results = system.search_requests("dashboard");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_transfer_request_methods() {
        let mut request = TransferRequest::new(
            Uuid::new_v4(),
            "Test Component".to_string(),
            "from_owner".to_string(),
            "to_owner".to_string(),
            "Test reason".to_string(),
            "requester".to_string(),
        );
        
        // Test setting priority
        request.set_priority(PriorityLevel::High);
        assert_eq!(request.priority, PriorityLevel::High);
        
        // Test adding supporting documents
        let doc_id = Uuid::new_v4();
        request.add_supporting_documents(vec![doc_id]);
        assert!(request.supporting_documents.contains(&doc_id));
        
        // Test setting estimated impact
        request.set_estimated_impact("High impact".to_string());
        assert_eq!(request.estimated_impact, Some("High impact".to_string()));
        
        // Test status changes
        request.submit();
        assert_eq!(request.status, TransferRequestStatus::Submitted);
        
        request.approve();
        assert_eq!(request.status, TransferRequestStatus::Approved);
        
        request.reject();
        assert_eq!(request.status, TransferRequestStatus::Rejected);
    }

    #[test]
    fn test_transfer_process_progress() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Test Component".to_string(),
            "from_owner".to_string(),
            "to_owner".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        // Test progress calculation
        let progress = system.get_transfer_process(process_id).unwrap().get_progress();
        assert_eq!(progress, 0.0); // 0/4 requirements completed
        
        // Complete one requirement
        system.complete_requirement(process_id, TransferRequirement::DocumentationComplete).unwrap();
        
        let progress = system.get_transfer_process(process_id).unwrap().get_progress();
        assert_eq!(progress, 25.0); // 1/4 requirements completed
        
        // Test pending/completed requirements
        let pending = system.get_transfer_process(process_id).unwrap().get_pending_requirements();
        assert_eq!(pending.len(), 3); // 3 still pending
        
        let completed = system.get_transfer_process(process_id).unwrap().get_completed_requirements();
        assert_eq!(completed.len(), 1); // 1 completed
    }

    #[test]
    fn test_transfer_process_deadline() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Test Component".to_string(),
            "from_owner".to_string(),
            "to_owner".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        // Test deadline calculation
        let process_ref = system.get_transfer_process(process_id).unwrap();
        assert!(process_ref.deadline.is_some());
        
        // Test time remaining
        let time_remaining = process_ref.time_remaining();
        assert!(time_remaining.is_some());
        
        // Test is_overdue (should be false for new process)
        assert!(!process_ref.is_overdue());
    }

    #[test]
    fn test_transfer_requirement_status() {
        let mut requirement = TransferRequirementStatus::new(TransferRequirement::DocumentationComplete, true);
        
        // Test initial status
        assert_eq!(requirement.status, RequirementStatus::Pending);
        assert_eq!(requirement.is_required, true);
        assert_eq!(requirement.assigned_to, None);
        
        // Test starting requirement
        requirement.start();
        assert_eq!(requirement.status, RequirementStatus::InProgress);
        assert!(requirement.started_at.is_some());
        
        // Test completing requirement
        requirement.complete();
        assert_eq!(requirement.status, RequirementStatus::Completed);
        assert!(requirement.completed_at.is_some());
        
        // Test adding notes
        requirement.add_note("Test note".to_string());
        assert_eq!(requirement.notes.len(), 1);
        assert_eq!(requirement.notes[0], "Test note");
        
        // Test assigning to user
        requirement.assign_to("user123".to_string());
        assert_eq!(requirement.assigned_to, Some("user123".to_string()));
    }

    #[test]
    fn test_progress_note() {
        let note = ProgressNote::new(
            "user123".to_string(),
            "Test progress note".to_string(),
            NoteVisibility::Community,
        );
        
        assert_eq!(note.author, "user123");
        assert_eq!(note.content, "Test progress note");
        assert_eq!(note.visibility, NoteVisibility::Community);
        assert!(note.timestamp <= Utc::now());
    }

    #[test]
    fn test_transfer_record() {
        let component_id = Uuid::new_v4();
        let mut record = TransferRecord::new(
            component_id,
            "from_owner".to_string(),
            "to_owner".to_string(),
            TransferType::ComponentOwnership,
        );
        
        // Test initial values
        assert_eq!(record.component_id, component_id);
        assert_eq!(record.from_owner, "from_owner");
        assert_eq!(record.to_owner, "to_owner");
        assert_eq!(record.transfer_type, TransferType::ComponentOwnership);
        assert_eq!(record.recorded_by, None);
        assert_eq!(record.notes, None);
        
        // Test setting recorded by
        record.set_recorded_by("recorder123".to_string());
        assert_eq!(record.recorded_by, Some("recorder123".to_string()));
        
        // Test adding notes
        record.add_notes("Test notes".to_string());
        assert_eq!(record.notes, Some("Test notes".to_string()));
    }

    #[test]
    fn test_transfer_policy() {
        let mut policy = TransferPolicy::new(
            "test_policy".to_string(),
            "Test Policy".to_string(),
            vec![TransferRequirement::DocumentationComplete],
            Duration::days(7),
        );
        
        // Test initial values
        assert_eq!(policy.id, "test_policy");
        assert_eq!(policy.name, "Test Policy");
        assert_eq!(policy.requirements.len(), 1);
        assert_eq!(policy.max_duration, Duration::days(7));
        
        // Test setting description
        policy.set_description("Test policy description".to_string());
        assert_eq!(policy.description, "Test policy description");
    }

    #[test]
    fn test_community_steward() {
        let mut steward = CommunitySteward::new(
            "user123".to_string(),
            "Alice Steward".to_string(),
            vec![StewardExpertise::Technical, StewardExpertise::Community],
        );
        
        // Test initial values
        assert_eq!(steward.user_id, "user123");
        assert_eq!(steward.name, "Alice Steward");
        assert_eq!(steward.expertise_areas.len(), 2);
        assert_eq!(steward.assigned_transfers.len(), 0);
        assert_eq!(steward.is_active, true);
        
        // Test adding assigned transfer
        let process_id = Uuid::new_v4();
        steward.add_assigned_transfer(process_id);
        assert_eq!(steward.assigned_transfers.len(), 1);
        assert!(steward.assigned_transfers.contains(&process_id));
        
        // Test removing assigned transfer
        steward.remove_assigned_transfer(process_id);
        assert_eq!(steward.assigned_transfers.len(), 0);
        
        // Test deactivating/reactivating
        steward.deactivate();
        assert_eq!(steward.is_active, false);
        
        steward.reactivate();
        assert_eq!(steward.is_active, true);
        
        // Test expertise checking
        assert!(steward.has_expertise(&StewardExpertise::Technical));
        assert!(steward.has_expertise(&StewardExpertise::Community));
        assert!(!steward.has_expertise(&StewardExpertise::Governance));
    }

    #[test]
    fn test_transfer_history_entry() {
        let component_id = Uuid::new_v4();
        let mut entry = TransferHistoryEntry::new(
            "Test transfer history entry".to_string(),
            component_id,
            "from_owner".to_string(),
            "to_owner".to_string(),
            TransferHistoryType::TransferStarted,
        );
        
        // Test initial values
        assert_eq!(entry.description, "Test transfer history entry");
        assert_eq!(entry.component_id, Some(component_id));
        assert_eq!(entry.from_owner, Some("from_owner".to_string()));
        assert_eq!(entry.to_owner, Some("to_owner".to_string()));
        assert_eq!(entry.entry_type, TransferHistoryType::TransferStarted);
        assert_eq!(entry.recorded_by, None);
        
        // Test setting recorded by
        entry.set_recorded_by("recorder123".to_string());
        assert_eq!(entry.recorded_by, Some("recorder123".to_string()));
    }

    #[test]
    fn test_transfer_system_policies_and_stewards() {
        let system = OwnershipTransferSystem::new();
        
        // Test getting policies
        let policies = system.get_policies();
        assert!(!policies.is_empty());
        assert!(policies.iter().any(|p| p.id == "standard_transfer"));
        assert!(policies.iter().any(|p| p.id == "urgent_transfer"));
        assert!(policies.iter().any(|p| p.id == "governance_transfer"));
        
        // Test getting stewards (empty initially)
        let stewards = system.get_community_stewards();
        assert!(stewards.is_empty());
    }

    #[test]
    fn test_transfer_system_assign_steward() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        let process_id = system.start_transfer_process(process);
        
        // Add a steward
        let steward = CommunitySteward::new(
            "steward123".to_string(),
            "Bob Steward".to_string(),
            vec![StewardExpertise::Technical],
        );
        let steward_id = steward.id;
        system.add_community_steward(steward);
        
        // Assign steward to process
        let result = system.assign_steward(process_id, steward_id);
        assert!(result.is_ok());
        
        let updated_process = system.get_transfer_process(process_id).unwrap();
        assert_eq!(updated_process.assigned_steward, Some(steward_id));
    }

    #[test]
    fn test_transfer_system_overdue_processes() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[1]; // Urgent transfer policy (3 days)
        let mut process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        // Set deadline to past to make it overdue
        process.deadline = Some(Utc::now() - Duration::days(1));
        let process_id = system.start_transfer_process(process);
        
        // Test getting overdue processes
        let overdue_processes = system.get_overdue_processes();
        assert_eq!(overdue_processes.len(), 1);
        assert_eq!(overdue_processes[0].id, process_id);
    }

    #[test]
    fn test_transfer_system_extend_deadline() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        let process_id = system.start_transfer_process(process);
        let original_deadline = system.get_transfer_process(process_id).unwrap().deadline.unwrap();
        
        // Extend deadline by 7 days
        let result = system.extend_deadline(process_id, Duration::days(7));
        assert!(result.is_ok());
        
        let updated_deadline = system.get_transfer_process(process_id).unwrap().deadline.unwrap();
        assert_eq!(updated_deadline, original_deadline + Duration::days(7));
    }

    #[test]
    fn test_transfer_system_cancel_process() {
        let mut system = OwnershipTransferSystem::new();
        let component_id = Uuid::new_v4();
        let policy = &system.transfer_policies[0]; // Standard transfer policy
        let mut process = TransferProcess::new(
            component_id,
            "Dashboard Component".to_string(),
            "developer123".to_string(),
            "community123".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        process.start();
        let process_id = system.start_transfer_process(process);
        
        // Cancel the process
        let result = system.cancel_process(process_id);
        assert!(result.is_ok());
        
        let updated_process = system.get_transfer_process(process_id).unwrap();
        assert_eq!(updated_process.status, TransferProcessStatus::Cancelled);
        assert!(updated_process.completed_at.is_some());
        
        // Check that cancellation was recorded in history
        let history = system.get_transfer_history(Some(component_id));
        assert!(!history.is_empty());
        assert_eq!(history[0].entry_type, TransferHistoryType::TransferCancelled);
    }

    #[test]
    fn test_transfer_system_filtering() {
        let mut system = OwnershipTransferSystem::new();
        
        // Create requests with different statuses
        let component1_id = Uuid::new_v4();
        let mut request1 = TransferRequest::new(
            component1_id,
            "Component 1".to_string(),
            "from1".to_string(),
            "to1".to_string(),
            "Reason 1".to_string(),
            "requester1".to_string(),
        );
        request1.approve();
        
        let component2_id = Uuid::new_v4();
        let request2 = TransferRequest::new(
            component2_id,
            "Component 2".to_string(),
            "from2".to_string(),
            "to2".to_string(),
            "Reason 2".to_string(),
            "requester2".to_string(),
        );
        
        let request1_id = system.create_transfer_request(request1);
        let request2_id = system.create_transfer_request(request2);
        
        // Test filtering by status
        let approved_requests = system.get_requests_by_status(TransferRequestStatus::Approved);
        assert_eq!(approved_requests.len(), 1);
        assert_eq!(approved_requests[0].id, request1_id);
        
        let pending_requests = system.get_requests_by_status(TransferRequestStatus::Pending);
        assert_eq!(pending_requests.len(), 1);
        assert_eq!(pending_requests[0].id, request2_id);
        
        // Create processes with different statuses
        let policy = &system.transfer_policies[0];
        let mut process1 = TransferProcess::new(
            component1_id,
            "Component 1".to_string(),
            "from1".to_string(),
            "to1".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        process1.start();
        
        let mut process2 = TransferProcess::new(
            component2_id,
            "Component 2".to_string(),
            "from2".to_string(),
            "to2".to_string(),
            TransferType::ComponentOwnership,
            policy,
        );
        
        let process1_id = system.start_transfer_process(process1);
        
        // Complete process1
        for requirement in &[
            TransferRequirement::DocumentationComplete,
            TransferRequirement::CommunityReview,
            TransferRequirement::KnowledgeTransfer,
            TransferRequirement::SupportAgreement,
        ] {
            system.complete_requirement(process1_id, *requirement).unwrap();
        }
        system.complete_transfer_process(process1_id).unwrap();
        
        system.start_transfer_process(process2);
        
        // Test filtering processes by status
        let completed_processes = system.get_processes_by_status(TransferProcessStatus::Completed);
        assert_eq!(completed_processes.len(), 1);
        assert_eq!(completed_processes[0].id, process1_id);
        
        let in_progress_processes = system.get_processes_by_status(TransferProcessStatus::InProgress);
        assert_eq!(in_progress_processes.len(), 1);
        assert_eq!(in_progress_processes[0].component_id, component2_id);
    }
}