//! Federation service for sharing visualization data

use reviews::Review;
use crate::data_generator::generators::products::Product;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum FederationError {
    NetworkError(String),
    SerializationError(String),
    PermissionError(String),
    P2pandaError(String), // New variant for p2panda errors
}

impl std::fmt::Display for FederationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FederationError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            FederationError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            FederationError::PermissionError(msg) => write!(f, "Permission error: {}", msg),
            FederationError::P2pandaError(msg) => write!(f, "P2panda error: {}", msg),
        }
    }
}

impl std::error::Error for FederationError {}

/// Data structure for shared visualization
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedVisualization {
    pub id: String,
    pub visualization_type: String,
    pub data: Vec<Review<Product>>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub creator: String,
}

/// Share visualization data to the federation
pub async fn share_visualization(
    data: Vec<Review<Product>>,
    visualization_type: &str,
) -> Result<String, FederationError> {
    // Create a shared visualization object
    let shared_vis = SharedVisualization {
        id: Uuid::new_v4().to_string(),
        visualization_type: visualization_type.to_string(),
        data,
        timestamp: Utc::now(),
        creator: "current_user".to_string(), // In a real app, this would be the actual user ID
    };
    
    // In a real implementation with p2panda, we would:
    // 1. Serialize the data to CBOR
    // 2. Create a p2panda document
    // 3. Sign and post to the network
    // 4. Return the document ID as share ID
    
    // For now, we'll simulate the p2panda document creation
    let share_id = shared_vis.id.clone();
    
    // Simulate p2panda document creation
    web_sys::console::log_1(
        &format!(
            "Creating p2panda document for {} visualization with {} reviews. Document ID: {}",
            visualization_type,
            shared_vis.data.len(),
            share_id
        )
        .into()
    );
    
    // In a real implementation, we would store this in Sled with TTL
    // For now, we'll just simulate the storage
    web_sys::console::log_1(
        &format!(
            "Storing shared visualization in Sled with TTL. ID: {}",
            share_id
        )
        .into()
    );
    
    Ok(share_id)
}

/// Generate embed code for a shared visualization
pub fn generate_embed_code(share_id: &str) -> String {
    format!(
        r#"<iframe src="https://federation.example.com/embed/{}" width="600" height="400" frameborder="0" allowfullscreen></iframe>"#,
        share_id
    )
}

/// Get shared visualization data from federation
pub async fn get_shared_visualization(
    share_id: &str,
) -> Result<SharedVisualization, FederationError> {
    // In a real implementation, we would fetch the data from the federation
    // For now, we'll simulate returning a shared visualization with mock data
    web_sys::console::log_1(&format!("Retrieving shared visualization with ID: {}", share_id).into());
    
    // Create some mock review data for demonstration
    let mock_reviews = vec![
        Review {
            id: "1".to_string(),
            product_id: "product_1".to_string(),
            user_id: "user_1".to_string(),
            timestamp: chrono::Utc::now() - chrono::Duration::days(5),
            content: "This product is amazing! I love the quality and design. Highly recommend to everyone.".to_string(),
            ratings: std::collections::HashMap::from([
                ("overall".to_string(), 5),
                ("quality".to_string(), 5),
                ("value".to_string(), 4),
            ]),
            metadata: std::collections::HashMap::new(),
        },
        Review {
            id: "2".to_string(),
            product_id: "product_1".to_string(),
            user_id: "user_2".to_string(),
            timestamp: chrono::Utc::now() - chrono::Duration::days(3),
            content: "Good product, but could be better. The delivery was fast though.".to_string(),
            ratings: std::collections::HashMap::from([
                ("overall".to_string(), 3),
                ("quality".to_string(), 3),
                ("value".to_string(), 3),
            ]),
            metadata: std::collections::HashMap::new(),
        },
        Review {
            id: "3".to_string(),
            product_id: "product_1".to_string(),
            user_id: "user_3".to_string(),
            timestamp: chrono::Utc::now() - chrono::Duration::days(1),
            content: "Not satisfied with the purchase. The product broke after a week of use.".to_string(),
            ratings: std::collections::HashMap::from([
                ("overall".to_string(), 2),
                ("quality".to_string(), 1),
                ("value".to_string(), 2),
            ]),
            metadata: std::collections::HashMap::new(),
        },
    ];
    
    // Simulate fetching data
    Ok(SharedVisualization {
        id: share_id.to_string(),
        visualization_type: "WordCloud".to_string(), // Default to word cloud for demo
        data: mock_reviews,
        timestamp: chrono::Utc::now(),
        creator: "demo_user".to_string(),
    })
}