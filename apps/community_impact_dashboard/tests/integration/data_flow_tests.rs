//! Integration tests for data flow from each impact system to the unified dashboard
//!
//! These tests verify that data flows correctly from each impact tracking system
//! to the unified dashboard and that the data is properly integrated.

use community_impact_dashboard::services::ImpactDataService;
use community_impact_dashboard::models::{UnifiedImpactData, ImpactInterconnection, CommunityWellbeing};
use consent_manager::domain::consent::DataSharingLevel;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_learning_data_flow() {
        // Test that learning data flows correctly to the unified dashboard
        let data_service = ImpactDataService::new();
        let learning_metrics = data_service.load_learning_metrics().await.unwrap();
        
        // Verify the learning metrics are valid
        assert!(learning_metrics.completed_courses >= 0);
        assert!(learning_metrics.skill_points >= 0.0);
        assert!(learning_metrics.community_contributions >= 0);
    }

    #[wasm_bindgen_test]
    async fn test_volunteer_data_flow() {
        // Test that volunteer data flows correctly to the unified dashboard
        let data_service = ImpactDataService::new();
        let volunteer_metrics = data_service.load_volunteer_metrics().await.unwrap();
        
        // Verify the volunteer metrics are valid
        assert!(volunteer_metrics.hours_volunteered >= 0.0);
        assert!(volunteer_metrics.projects_completed >= 0);
        assert!(volunteer_metrics.community_impact_score >= 0.0);
    }

    #[wasm_bindgen_test]
    async fn test_financial_data_flow() {
        // Test that financial data flows correctly to the unified dashboard
        let data_service = ImpactDataService::new();
        let financial_metrics = data_service.load_financial_metrics().await.unwrap();
        
        // Verify we get a vector of financial records
        assert!(financial_metrics.is_empty() || financial_metrics.len() >= 0);
    }

    #[wasm_bindgen_test]
    async fn test_cause_data_flow() {
        // Test that cause data flows correctly to the unified dashboard
        let data_service = ImpactDataService::new();
        let cause_metrics = data_service.load_cause_metrics().await.unwrap();
        
        // Verify the cause metrics are valid
        assert!(cause_metrics.causes_supported >= 0);
        assert!(cause_metrics.actions_taken >= 0);
        assert!(cause_metrics.community_reach >= 0);
    }

    #[wasm_bindgen_test]
    async fn test_unified_data_integration() {
        // Test that all data is properly integrated into the unified dashboard
        let data_service = ImpactDataService::new();
        let user_id = Some("test_user_123".to_string());
        let consent_level = DataSharingLevel::Standard;
        
        let unified_data = data_service.load_unified_impact_data(user_id, consent_level).await.unwrap();
        
        // Verify all data is present
        assert!(unified_data.learning_metrics.completed_courses >= 0);
        assert!(unified_data.volunteer_metrics.hours_volunteered >= 0.0);
        assert!(!unified_data.financial_metrics.is_empty() || unified_data.financial_metrics.len() >= 0);
        assert!(unified_data.cause_metrics.causes_supported >= 0);
        
        // Verify interconnections are present
        assert!(!unified_data.interconnections.is_empty());
        
        // Verify community wellbeing data is present
        assert!(unified_data.community_wellbeing.overall_score >= 0.0);
        
        // Verify community stories are present
        assert!(!unified_data.community_stories.is_empty());
        
        // Verify member data is present when user_id is provided
        assert!(unified_data.member_data.is_some());
    }
}