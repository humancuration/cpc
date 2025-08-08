//! Feedback service for the volunteer coordination admin dashboard
//!
//! This service provides access to feedback data from the volunteer_impact_tracker crate.

use volunteer_impact_tracker::feedback::{FeedbackCollector, FeedbackProcessingResult};
use wasm_bindgen_futures::spawn_local;
use yew::platform::pinned::oneshot;

/// Service for accessing feedback data
pub struct FeedbackService;

impl FeedbackService {
    /// Get feedback processing results for a specific visualization
    pub async fn get_feedback_results(viz_id: &str) -> Result<FeedbackProcessingResult, String> {
        // In a real implementation, this would fetch data from a backend API
        // For now, we'll create mock data
        
        let collector = FeedbackCollector::new();
        let result = collector.process_feedback_for_viz(viz_id);
        
        result.map_err(|e| e.to_string())
    }
    
    /// Get all feedback for a specific visualization
    pub async fn get_all_feedback(viz_id: &str) -> Result<serde_json::Value, String> {
        // In a real implementation, this would fetch all feedback data
        // For now, we'll create mock data
        let mock_data = serde_json::json!({
            "status": "success",
            "data": {
                "visualization_id": viz_id,
                "feedback_count": 42,
                "average_rating": 4.3,
                "helpful_percentage": 85,
                "feedback_items": [
                    {
                        "user_id": "volunteer_123",
                        "rating": 5,
                        "comment": "This visualization really helped me understand my impact!",
                        "helpful": true,
                        "timestamp": "2025-08-07T10:30:00Z"
                    },
                    {
                        "user_id": "volunteer_456",
                        "rating": 4,
                        "comment": "Good overview but could use more details",
                        "helpful": true,
                        "timestamp": "2025-08-07T09:15:00Z"
                    }
                ]
            }
        });
        
        Ok(mock_data)
    }
}