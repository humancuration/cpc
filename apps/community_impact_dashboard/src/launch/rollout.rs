//! Gradual rollout mechanism
//!
//! This module provides functionality for gradually rolling out the dashboard
//! to community members, starting with beta users and expanding to full launch.

use tracing::info;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Rollout manager for gradual launch
pub struct RolloutManager {
    phases: Vec<RolloutPhase>,
    current_phase: usize,
    participants: HashMap<String, RolloutParticipant>,
}

impl RolloutManager {
    /// Create a new rollout manager
    pub fn new() -> Self {
        Self {
            phases: vec![
                RolloutPhase::new("beta".to_string(), 0.1, "Beta testing phase with core community members"),
                RolloutPhase::new("early_adopter".to_string(), 0.3, "Early adopter phase with active community members"),
                RolloutPhase::new("majority".to_string(), 0.6, "Majority community rollout"),
                RolloutPhase::new("full_launch".to_string(), 1.0, "Full community launch"),
            ],
            current_phase: 0,
            participants: HashMap::new(),
        }
    }
    
    /// Get the current rollout phase
    pub fn get_current_phase(&self) -> &RolloutPhase {
        &self.phases[self.current_phase]
    }
    
    /// Advance to the next rollout phase
    pub fn advance_phase(&mut self) -> Result<(), RolloutError> {
        if self.current_phase < self.phases.len() - 1 {
            self.current_phase += 1;
            info!("Advanced to rollout phase: {}", self.phases[self.current_phase].name);
            Ok(())
        } else {
            Err(RolloutError::NoMorePhases)
        }
    }
    
    /// Add a participant to the rollout
    pub fn add_participant(&mut self, user_id: String, role: ParticipantRole) {
        let participant = RolloutParticipant::new(user_id.clone(), role);
        self.participants.insert(user_id, participant);
        info!("Added participant to rollout: {:?}", role);
    }
    
    /// Check if a user should have access based on current phase
    pub fn has_access(&self, user_id: &str) -> bool {
        if let Some(participant) = self.participants.get(user_id) {
            // Admins and facilitators always have access
            match participant.role {
                ParticipantRole::Admin | ParticipantRole::Facilitator => true,
                ParticipantRole::BetaTester => self.current_phase >= 0,
                ParticipantRole::EarlyAdopter => self.current_phase >= 1,
                ParticipantRole::CommunityMember => {
                    let progress = self.get_rollout_progress();
                    progress >= self.phases[self.current_phase].target_percentage
                }
            }
        } else {
            false
        }
    }
    
    /// Get the current rollout progress (0.0 to 1.0)
    pub fn get_rollout_progress(&self) -> f64 {
        self.phases[self.current_phase].target_percentage
    }
    
    /// Get participants by role
    pub fn get_participants_by_role(&self, role: ParticipantRole) -> Vec<&RolloutParticipant> {
        self.participants.values()
            .filter(|p| p.role == role)
            .collect()
    }
    
    /// Get rollout statistics
    pub fn get_rollout_stats(&self) -> RolloutStats {
        let total_participants = self.participants.len();
        let active_participants = self.participants.values()
            .filter(|p| self.has_access(&p.user_id))
            .count();
        
        let participants_by_role: HashMap<ParticipantRole, usize> = 
            self.participants.values()
                .fold(HashMap::new(), |mut acc, p| {
                    *acc.entry(p.role).or_insert(0) += 1;
                    acc
                });
        
        RolloutStats {
            current_phase: self.current_phase,
            total_phases: self.phases.len(),
            progress: self.get_rollout_progress(),
            total_participants,
            active_participants,
            participants_by_role,
        }
    }
}

impl Default for RolloutManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Rollout phase definition
#[derive(Debug, Clone)]
pub struct RolloutPhase {
    /// Name of the phase
    pub name: String,
    
    /// Target percentage of community (0.0 to 1.0)
    pub target_percentage: f64,
    
    /// Description of the phase
    pub description: String,
    
    /// When the phase started
    pub started_at: Option<DateTime<Utc>>,
    
    /// When the phase completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl RolloutPhase {
    /// Create a new rollout phase
    pub fn new(name: String, target_percentage: f64, description: &str) -> Self {
        Self {
            name,
            target_percentage,
            description: description.to_string(),
            started_at: None,
            completed_at: None,
        }
    }
}

/// Participant roles in the rollout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParticipantRole {
    /// System administrators
    Admin,
    
    /// Community facilitators
    Facilitator,
    
    /// Beta testers
    BetaTester,
    
    /// Early adopters
    EarlyAdopter,
    
    /// Regular community members
    CommunityMember,
}

/// Rollout participant
#[derive(Debug, Clone)]
pub struct RolloutParticipant {
    /// User identifier
    pub user_id: String,
    
    /// Participant role
    pub role: ParticipantRole,
    
    /// When the participant was added
    pub added_at: DateTime<Utc>,
    
    /// Whether the participant has completed onboarding
    pub onboarding_completed: bool,
}

impl RolloutParticipant {
    /// Create a new rollout participant
    pub fn new(user_id: String, role: ParticipantRole) -> Self {
        Self {
            user_id,
            role,
            added_at: Utc::now(),
            onboarding_completed: false,
        }
    }
    
    /// Mark onboarding as completed
    pub fn complete_onboarding(&mut self) {
        self.onboarding_completed = true;
    }
}

/// Rollout statistics
#[derive(Debug, Clone)]
pub struct RolloutStats {
    /// Current phase index
    pub current_phase: usize,
    
    /// Total number of phases
    pub total_phases: usize,
    
    /// Current rollout progress (0.0 to 1.0)
    pub progress: f64,
    
    /// Total number of participants
    pub total_participants: usize,
    
    /// Number of active participants (with access)
    pub active_participants: usize,
    
    /// Participants by role
    pub participants_by_role: HashMap<ParticipantRole, usize>,
}

/// Error type for rollout operations
#[derive(Debug)]
pub enum RolloutError {
    /// No more phases to advance to
    NoMorePhases,
    
    /// Invalid phase transition
    InvalidTransition,
}

impl std::fmt::Display for RolloutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RolloutError::NoMorePhases => write!(f, "No more rollout phases available"),
            RolloutError::InvalidTransition => write!(f, "Invalid rollout phase transition"),
        }
    }
}

impl std::error::Error for RolloutError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rollout_initialization() {
        let rollout = RolloutManager::new();
        assert_eq!(rollout.phases.len(), 4);
        assert_eq!(rollout.current_phase, 0);
        assert_eq!(rollout.get_current_phase().name, "beta");
    }
    
    #[test]
    fn test_add_participant() {
        let mut rollout = RolloutManager::new();
        rollout.add_participant("user123".to_string(), ParticipantRole::BetaTester);
        assert_eq!(rollout.participants.len(), 1);
    }
    
    #[test]
    fn test_access_control() {
        let mut rollout = RolloutManager::new();
        
        // Add participants with different roles
        rollout.add_participant("admin1".to_string(), ParticipantRole::Admin);
        rollout.add_participant("facilitator1".to_string(), ParticipantRole::Facilitator);
        rollout.add_participant("beta1".to_string(), ParticipantRole::BetaTester);
        rollout.add_participant("member1".to_string(), ParticipantRole::CommunityMember);
        
        // In beta phase, only admins, facilitators, and beta testers have access
        assert!(rollout.has_access("admin1"));
        assert!(rollout.has_access("facilitator1"));
        assert!(rollout.has_access("beta1"));
        assert!(!rollout.has_access("member1"));
        
        // Advance to early adopter phase
        rollout.advance_phase().unwrap();
        assert!(rollout.has_access("member1")); // Early adopters now have access
    }
    
    #[test]
    fn test_advance_phase() {
        let mut rollout = RolloutManager::new();
        let initial_phase = rollout.current_phase;
        
        rollout.advance_phase().unwrap();
        assert_eq!(rollout.current_phase, initial_phase + 1);
        
        // Advance through all phases
        for _ in 0..2 {
            rollout.advance_phase().unwrap();
        }
        
        // Should fail when trying to advance past the last phase
        assert!(rollout.advance_phase().is_err());
    }
}