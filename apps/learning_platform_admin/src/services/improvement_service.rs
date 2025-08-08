//! Improvement service for the learning platform admin dashboard
//!
//! This service provides access to improvement suggestions from the learning_impact_tracker crate.

use learning_impact_tracker::improvement::{ImprovementEngine, AutoImprovementSuggestion};
use learning_impact_tracker::analytics::DashboardSummary;
use learning_impact_tracker::feedback::FeedbackProcessingResult;
use wasm_bindgen_futures::spawn_local;
use yew::platform::pinned::oneshot;

/// Service for accessing improvement suggestions
pub struct ImprovementService;

impl ImprovementService {
    /// Get improvement suggestions based on analytics and feedback
    pub async fn get_improvement_suggestions(
        dashboard_summary: &DashboardSummary,
        feedback_results: &[FeedbackProcessingResult],
    ) -> Result<Vec<AutoImprovementSuggestion>, String> {
        // In a real implementation, this would fetch data from a backend API
        // For now, we'll create mock data
        
        let engine = ImprovementEngine::new();
        let suggestions = engine.generate_improvement_suggestions(dashboard_summary, feedback_results);
        
        Ok(suggestions)
    }
    
    /// Get A/B test results
    pub async fn get_ab_test_results(test_id: &str) -> Result<serde_json::Value, String> {
        // In a real implementation, this would fetch A/B test results
        // For now, we'll create mock data
        let mock_data = serde_json::json!({
            "status": "success",
            "data": {
                "test_id": test_id,
                "winning_variant": "variant_b",
                "metrics": {
                    "variant_a": {
                        "engagement_rate": 0.65,
                        "completion_rate": 0.72,
                        "helpfulness": 0.75
                    },
                    "variant_b": {
                        "engagement_rate": 0.78,
                        "completion_rate": 0.81,
                        "helpfulness": 0.82
                    }
                },
                "significance": 0.95,
                "confidence": 0.90
            }
        });
        
        Ok(mock_data)
    }
}