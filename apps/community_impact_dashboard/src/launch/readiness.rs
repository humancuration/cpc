//! Launch readiness checklist automation
//!
//! This module provides automated checking of launch readiness criteria
//! to ensure a smooth community launch.

use tracing::info;
use std::collections::HashMap;

/// Launch readiness checklist automation
pub struct LaunchReadinessChecklist {
    checks: HashMap<String, bool>,
}

impl LaunchReadinessChecklist {
    /// Create a new launch readiness checklist
    pub fn new() -> Self {
        let mut checklist = Self {
            checks: HashMap::new(),
        };
        
        // Initialize all required checks as false
        checklist.initialize_checks();
        checklist
    }
    
    /// Initialize all required readiness checks
    fn initialize_checks(&mut self) {
        self.checks.insert("dashboard_functionality_verified".to_string(), false);
        self.checks.insert("community_validation_workflows_tested".to_string(), false);
        self.checks.insert("onboarding_experience_validated".to_string(), false);
        self.checks.insert("data_integration_confirmed".to_string(), false);
        self.checks.insert("performance_benchmarks_met".to_string(), false);
        self.checks.insert("accessibility_features_verified".to_string(), false);
        self.checks.insert("security_audits_completed".to_string(), false);
        self.checks.insert("documentation_complete".to_string(), false);
        self.checks.insert("facilitator_training_materials_ready".to_string(), false);
        self.checks.insert("community_notification_system_configured".to_string(), false);
        self.checks.insert("feedback_collection_mechanisms_ready".to_string(), false);
        self.checks.insert("celebration_framework_prepared".to_string(), false);
    }
    
    /// Mark a specific check as completed
    pub fn mark_completed(&mut self, check_name: &str) -> bool {
        if self.checks.contains_key(check_name) {
            self.checks.insert(check_name.to_string(), true);
            info!("Marked '{}' as completed", check_name);
            true
        } else {
            false
        }
    }
    
    /// Check if all required items are completed
    pub fn is_ready(&self) -> bool {
        self.checks.values().all(|&completed| completed)
    }
    
    /// Get the overall readiness status
    pub fn get_status(&self) -> ReadinessStatus {
        let completed_count = self.checks.values().filter(|&&completed| completed).count();
        let total_count = self.checks.len();
        
        ReadinessStatus {
            completed: completed_count,
            total: total_count,
            ready: self.is_ready(),
        }
    }
    
    /// Get detailed status of all checks
    pub fn get_detailed_status(&self) -> HashMap<String, bool> {
        self.checks.clone()
    }
}

impl Default for LaunchReadinessChecklist {
    fn default() -> Self {
        Self::new()
    }
}

/// Readiness status information
#[derive(Debug, Clone)]
pub struct ReadinessStatus {
    /// Number of completed checks
    pub completed: usize,
    
    /// Total number of checks
    pub total: usize,
    
    /// Whether all checks are completed
    pub ready: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_checklist_initialization() {
        let checklist = LaunchReadinessChecklist::new();
        assert_eq!(checklist.checks.len(), 12);
        assert!(!checklist.is_ready());
    }
    
    #[test]
    fn test_mark_completed() {
        let mut checklist = LaunchReadinessChecklist::new();
        assert!(checklist.mark_completed("dashboard_functionality_verified"));
        assert!(!checklist.is_ready());
    }
    
    #[test]
    fn test_all_completed() {
        let mut checklist = LaunchReadinessChecklist::new();
        
        for check_name in checklist.checks.keys() {
            checklist.mark_completed(check_name);
        }
        
        assert!(checklist.is_ready());
        let status = checklist.get_status();
        assert_eq!(status.completed, status.total);
        assert!(status.ready);
    }
    
    #[test]
    fn test_invalid_check_name() {
        let mut checklist = LaunchReadinessChecklist::new();
        assert!(!checklist.mark_completed("invalid_check_name"));
    }
}