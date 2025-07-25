use chrono::{Utc, Duration};
use crate::types::{SupplyChain, SupplyChainNode, TransportationSegment, NodeType, TransportMethod, Coordinates};

pub fn mock_supply_chain() -> SupplyChain {
    SupplyChain {
        nodes: vec![
            SupplyChainNode {
                id: "node1".into(),
                node_type: NodeType::RawMaterial,
                location: "Coffee Farm, Colombia".into(),
                company: "Green Beans Co.".into(),
                timestamp: Utc::now() - Duration::days(90),
                coordinates: Coordinates { latitude: 4.5709, longitude: -74.2973 },
            },
            SupplyChainNode {
                id: "node2".into(),
                node_type: NodeType::Manufacturer,
                location: "Processing Plant, Brazil".into(),
                company: "Bean Processors Inc.".into(),
                timestamp: Utc::now() - Duration::days(60),
                coordinates: Coordinates { latitude: -15.7801, longitude: -47.9292 },
            },
            SupplyChainNode {
                id: "node3".into(),
                node_type: NodeType::Distributor,
                location: "Distribution Center, USA".into(),
                company: "Global Distributors LLC".into(),
                timestamp: Utc::now() - Duration::days(30),
                coordinates: Coordinates { latitude: 39.7392, longitude: -104.9903 },
            },
            SupplyChainNode {
                id: "node4".into(),
                node_type: NodeType::Retailer,
                location: "Local Store, Seattle".into(),
                company: "Neighborhood Market".into(),
                timestamp: Utc::now() - Duration::days(7),
                coordinates: Coordinates { latitude: 47.6062, longitude: -122.3321 },
            },
        ],
        segments: vec![
            TransportationSegment {
                from_node_id: "node1".into(),
                to_node_id: "node2".into(),
                method: TransportMethod::Ship,
                duration_hours: 240,
                carbon_footprint: 150.0,
            },
            TransportationSegment {
                from_node_id: "node2".into(),
                to_node_id: "node3".into(),
                method: TransportMethod::Plane,
                duration_hours: 12,
                carbon_footprint: 500.0,
            },
            TransportationSegment {
                from_node_id: "node3".into(),
                to_node_id: "node4".into(),
                method: TransportMethod::Truck,
                duration_hours: 48,
                carbon_footprint: 120.0,
            },
        ],
    }
}