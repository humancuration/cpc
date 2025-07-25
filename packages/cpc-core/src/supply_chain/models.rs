use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplyChain {
    pub product_id: String,
    pub nodes: Vec<SupplyChainNode>,
    pub segments: Vec<TransportationSegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplyChainNode {
    pub id: String,
    pub node_type: NodeType,
    pub name: String, // e.g., "FairTrade Coffee Cooperative"
    pub location: String, // e.g., "Antioquia, Colombia"
    pub coordinates: (f64, f64), // (latitude, longitude)
    pub certifications: Vec<EthicalCertification>,
    pub cooperative_metrics: Option<CooperativeMetrics>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType { RawMaterial, Manufacturer, Distributor, Retailer }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EthicalCertification {
    pub name: String, // e.g., "Fair Trade Certified"
    pub authority: String, // e.g., "Fairtrade International"
    pub validation_date: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CooperativeMetrics {
    pub fair_wage_verified: bool,
    pub profit_sharing_percentage: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportationSegment {
    pub from_node_id: String,
    pub to_node_id: String,
    pub method: TransportMethod,
    pub duration_hours: u32,
    pub environmental_impact: EnvironmentalImpact,
    pub cost: TransportCost,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransportMethod { Ship, Truck, Plane, Train }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentalImpact {
    pub carbon_footprint_kg_co2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransportCost {
    pub amount: f64,
    pub currency: String,
}