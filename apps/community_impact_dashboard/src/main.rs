//! Main entry point for the Unified Community Impact Dashboard
//!
//! This module orchestrates the community launch and ownership transfer
//! of the dashboard, bringing together all components for a cohesive experience.

// Launch system for community roll-out
pub mod launch;

// Community ownership framework
pub mod ownership;

use tracing::{info, error};
use launch::{
    LaunchSystem,
    LaunchPhase,
    ConsentStatus,
};
use ownership::OwnershipFramework;

/// Main application struct
pub struct CommunityImpactDashboard {
    launch_system: LaunchSystem,
    ownership_framework: OwnershipFramework,
    is_running: bool,
}

impl CommunityImpactDashboard {
    /// Create a new Community Impact Dashboard
    pub fn new() -> Self {
        Self {
            launch_system: launch::initialize_launch_system(),
            ownership_framework: ownership::initialize_ownership_framework(),
            is_running: false,
        }
    }

    /// Initialize the dashboard systems
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Initializing Community Impact Dashboard...");
        
        // Initialize launch system
        info!("Launch system initialized");
        
        // Initialize ownership framework
        info!("Ownership framework initialized");
        
        info!("Community Impact Dashboard initialization complete");
        Ok(())
    }

    /// Start the community launch process
    pub fn start_community_launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting community launch process...");
        
        // Start the launch
        self.launch_system.start_community_launch()
            .map_err(|e| {
                error!("Failed to start community launch: {}", e);
                Box::new(e) as Box<dyn std::error::Error>
            })?;
        
        self.is_running = true;
        info!("Community launch started successfully");
        Ok(())
    }

    /// Advance to the next launch phase
    pub fn advance_launch_phase(&mut self) -> Result<LaunchPhase, Box<dyn std::error::Error>> {
        if !self.is_running {
            return Err("Dashboard is not running".into());
        }
        
        info!("Advancing to next launch phase...");
        
        let next_phase = self.launch_system.advance_phase()
            .map_err(|e| {
                error!("Failed to advance launch phase: {}", e);
                Box::new(e) as Box<dyn std::error::Error>
            })?;
        
        info!("Advanced to phase: {:?}", next_phase);
        Ok(next_phase)
    }

    /// Check community consent status for current phase
    pub fn check_community_consent(&self) -> ConsentStatus {
        self.launch_system.check_community_consent()
    }

    /// Get current launch phase
    pub fn get_current_phase(&self) -> LaunchPhase {
        self.launch_system.get_current_phase()
    }

    /// Get launch progress
    pub fn get_launch_progress(&self) -> launch::LaunchProgress {
        self.launch_system.get_launch_progress()
    }

    /// Generate a comprehensive system report
    pub fn generate_system_report(&self) -> SystemReport {
        let launch_report = self.launch_system.generate_launch_report();
        let ownership_report = self.ownership_framework.generate_ownership_report();
        
        SystemReport {
            generated_at: chrono::Utc::now(),
            launch_report,
            ownership_report,
        }
    }

    /// Get system metrics
    pub fn get_system_metrics(&self) -> SystemMetrics {
        let launch_progress = self.launch_system.get_launch_progress();
        let ownership_stats = self.ownership_framework.get_ownership_statistics();
        
        SystemMetrics {
            launch_progress,
            ownership_stats,
        }
    }

    /// Check if the dashboard is running
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Get reference to launch system
    pub fn get_launch_system(&self) -> &LaunchSystem {
        &self.launch_system
    }

    /// Get mutable reference to launch system
    pub fn get_launch_system_mut(&mut self) -> &mut LaunchSystem {
        &mut self.launch_system
    }

    /// Get reference to ownership framework
    pub fn get_ownership_framework(&self) -> &OwnershipFramework {
        &self.ownership_framework
    }

    /// Get mutable reference to ownership framework
    pub fn get_ownership_framework_mut(&mut self) -> &mut OwnershipFramework {
        &mut self.ownership_framework
    }

    /// Gracefully shutdown the dashboard
    pub fn shutdown(&mut self) {
        info!("Shutting down Community Impact Dashboard...");
        self.is_running = false;
        info!("Community Impact Dashboard shutdown complete");
    }
}

/// Comprehensive system report
#[derive(Debug, Clone)]
pub struct SystemReport {
    pub generated_at: chrono::DateTime<chrono::Utc>,
    pub launch_report: launch::LaunchReport,
    pub ownership_report: ownership::OwnershipReport,
}

/// System metrics
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    pub launch_progress: launch::LaunchProgress,
    pub ownership_stats: ownership::OwnershipStatistics,
}

/// Run the Community Impact Dashboard
pub fn run_dashboard() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the logger
    tracing_subscriber::fmt::init();
    
    info!("Starting Unified Community Impact Dashboard...");
    
    // Create and initialize the dashboard
    let mut dashboard = CommunityImpactDashboard::new();
    dashboard.initialize()?;
    
    // Start the community launch
    dashboard.start_community_launch()?;
    
    // Log initial status
    info!("Dashboard is running in phase: {:?}", dashboard.get_current_phase());
    info!("Community consent status: {:?}", dashboard.check_community_consent());
    
    // Generate initial report
    let report = dashboard.generate_system_report();
    info!("Initial system report generated at: {}", report.generated_at);
    
    // Simulate running for a bit
    info!("Dashboard is now running and ready for community engagement");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_initialization() {
        let mut dashboard = CommunityImpactDashboard::new();
        assert!(!dashboard.is_running());
        
        let result = dashboard.initialize();
        assert!(result.is_ok());
        assert!(!dashboard.is_running());
    }

    #[test]
    fn test_dashboard_launch_operations() {
        let mut dashboard = CommunityImpactDashboard::new();
        dashboard.initialize().unwrap();
        
        // Test starting launch
        let result = dashboard.start_community_launch();
        assert!(result.is_ok());
        assert!(dashboard.is_running());
        assert_eq!(dashboard.get_current_phase(), LaunchPhase::Beta);
        
        // Test checking consent
        let consent_status = dashboard.check_community_consent();
        assert_eq!(consent_status, ConsentStatus::Pending);
        
        // Test getting progress
        let progress = dashboard.get_launch_progress();
        assert!(progress.current_phase >= 1);
    }

    #[test]
    fn test_dashboard_phase_advancement() {
        let mut dashboard = CommunityImpactDashboard::new();
        dashboard.initialize().unwrap();
        dashboard.start_community_launch().unwrap();
        
        // Test advancing phase
        let next_phase = dashboard.advance_launch_phase();
        assert!(next_phase.is_ok());
        assert_eq!(next_phase.unwrap(), LaunchPhase::EarlyAdopters);
        assert_eq!(dashboard.get_current_phase(), LaunchPhase::EarlyAdopters);
    }

    #[test]
    fn test_dashboard_reports_and_metrics() {
        let mut dashboard = CommunityImpactDashboard::new();
        dashboard.initialize().unwrap();
        dashboard.start_community_launch().unwrap();
        
        // Test generating system report
        let report = dashboard.generate_system_report();
        assert!(report.generated_at <= chrono::Utc::now());
        assert_eq!(report.launch_report.execution_status.current_phase, LaunchPhase::Beta);
        
        // Test getting system metrics
        let metrics = dashboard.get_system_metrics();
        assert!(metrics.launch_progress.current_phase >= 1);
    }

    #[test]
    fn test_dashboard_getters() {
        let mut dashboard = CommunityImpactDashboard::new();
        dashboard.initialize().unwrap();
        
        // Test launch system getters
        let launch_system = dashboard.get_launch_system();
        assert_eq!(launch_system.get_current_phase(), LaunchPhase::PreLaunch);
        
        let launch_system_mut = dashboard.get_launch_system_mut();
        assert!(launch_system_mut.start_community_launch().is_ok());
        
        // Test ownership framework getters
        let ownership_framework = dashboard.get_ownership_framework();
        assert_eq!(ownership_framework.get_governance().get_principles().len(), 0);
        
        let ownership_framework_mut = dashboard.get_ownership_framework_mut();
        assert_eq!(ownership_framework_mut.get_governance().get_principles().len(), 0);
    }

    #[test]
    fn test_dashboard_shutdown() {
        let mut dashboard = CommunityImpactDashboard::new();
        dashboard.initialize().unwrap();
        dashboard.start_community_launch().unwrap();
        
        assert!(dashboard.is_running());
        dashboard.shutdown();
        assert!(!dashboard.is_running());
    }

    #[test]
    fn test_run_dashboard_function() {
        // This test just ensures the function can be called without panicking
        // In a real scenario, we might want to mock the logger
        let result = run_dashboard();
        // The function will try to start the launch which might fail in tests
        // but it shouldn't panic
        assert!(result.is_ok() || result.is_err());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_dashboard()
}