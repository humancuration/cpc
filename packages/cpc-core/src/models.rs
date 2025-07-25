use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Money {
    #[validate(range(min = 0.0, message = "Amount must be non-negative"))]
    pub amount: f64,
    
    #[validate(length(min = 1, max = 10, message = "Currency must be 1-10 characters"))]
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct WarehouseLocation {
    #[validate(length(min = 1, max = 50, message = "ID must be 1-50 characters"))]
    pub id: String,
    
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Proposal {
    #[validate(length(min = 1, message = "ID is required"))]
    pub id: String,
    
    #[validate(length(min = 1, max = 100, message = "Title must be between 1-100 characters"))]
    pub title: String,
    
    #[validate(length(min = 1, max = 1000, message = "Description must be between 1-1000 characters"))]
    pub description: String,
    
    #[validate(range(min = 0, message = "Votes must be non-negative"))]
    pub votes_for: u32,
    
    #[validate(range(min = 0, message = "Votes must be non-negative"))]
    pub votes_against: u32,
    
    #[validate(length(min = 1, message = "Author ID is required"))]
    pub author_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedItem {
    Post {
        id: String,
        content: String,
        author_id: String,
        likes: u32,
        comments: u32,
    },
    // SupplyChain variant removed from FeedItem

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SupplyChain {
    pub nodes: Vec<SupplyChainNode>,
    pub segments: Vec<TransportationSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SupplyChainNode {
    #[validate(length(min = 1, message = "ID is required"))]
    pub id: String,
    
    pub node_type: NodeType,
    
    #[validate(length(min = 1, message = "Location is required"))]
    pub location: String,
    
    #[validate(length(min = 1, message = "Company is required"))]
    pub company: String,
    
    pub timestamp: String,
    
    pub coordinates: Coordinates,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct TransportationSegment {
    #[validate(length(min = 1, message = "From node ID is required"))]
    pub from_node_id: String,
    
    #[validate(length(min = 1, message = "To node ID is required"))]
    pub to_node_id: String,
    
    pub method: TransportMethod,
    
    #[validate(range(min = 0, message = "Duration must be non-negative"))]
    pub duration_hours: u32,
    
    #[validate(range(min = 0.0, message = "Carbon footprint must be non-negative"))]
    pub carbon_footprint: f64,  // Changed to f64 to match protobuf
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    RawMaterial,
    Manufacturer,
    Distributor,
    Retailer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportMethod {
    Ship,
    Truck,
    Plane,
    Train,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub total_members: u32,
    pub active_members: u32,
    pub total_products: u32,
    pub total_sales: f64,
    pub total_profit: f64,
    pub total_carbon_saved: f32,
    pub avg_profit_per_member: f64,
    pub member_engagement: f32,
}