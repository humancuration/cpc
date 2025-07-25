use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub author_id: String,
    pub likes: u32,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub author_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub votes_for: u32,
    pub votes_against: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum FeedItem {
    Post {
        id: String,
        content: String,
        author_id: String,
        likes: u32,
        comments: u32,
    },
    Proposal {
        id: String,
        title: String,
        description: String,
        votes_for: u32,
        votes_against: u32,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyChain {
    pub nodes: Vec<SupplyChainNode>,
    pub segments: Vec<TransportationSegment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplyChainNode {
    pub id: String,
    pub node_type: NodeType,
    pub location: String,
    pub company: String,
    pub timestamp: DateTime<Utc>,
    pub coordinates: Coordinates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportationSegment {
    pub from_node_id: String,
    pub to_node_id: String,
    pub method: TransportMethod,
    pub duration_hours: i32,
    pub carbon_footprint: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NodeType {
    RawMaterial,
    Manufacturer,
    Distributor,
    Retailer,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransportMethod {
    Ship,
    Truck,
    Plane,
    Train,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub brand: String,
    pub description: String,
    pub barcode: String,
    pub carbon_footprint: f64,
    pub packaging_type: String,
    pub nutritional_info: String,
    pub manufacturer: String,
    pub material_cost: f64,
    pub labor_cost: f64,
    pub supplier: String,
    pub current_stock: i32,
    pub reorder_level: i32,
    pub supply_chain: Option<SupplyChain>,
}