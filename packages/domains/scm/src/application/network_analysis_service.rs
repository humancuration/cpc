//! Network analysis service for supply chain optimization

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    supply_chain_network::{SupplyChainNetwork, NetworkNode, NetworkConnection, NodeType},
    shipment::Shipment,
    inventory_item::InventoryItem,
    primitives::{NodeId, DomainError, Result as DomainResult},
    consent::NetworkConsentSettings,
};
use crate::infrastructure::database::repositories::{
    SupplyChainNetworkRepository, ShipmentRepository, InventoryItemRepository,
};

/// Service error types
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("Repository error: {0}")]
    Repository(String),
    
    #[error("Permission denied: {message}")]
    PermissionDenied { message: String },
    
    #[error("Network not found")]
    NetworkNotFound,
    
    #[error("No path found between nodes")]
    NoPathFound,
}

/// Result type for service operations
pub type Result<T> = std::result::Result<T, ServiceError>;

/// Privacy consent service trait
#[async_trait]
pub trait PrivacyConsentService: Send + Sync {
    async fn verify_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType,
    ) -> Result<()>;
}

/// Consent types for network analysis operations
#[derive(Debug, Clone)]
pub enum ConsentType {
    NetworkView,
    NetworkAnalysis,
    NetworkOptimization,
}

/// Result of network analysis
#[derive(Debug, Clone)]
pub struct NetworkAnalysisResult {
    pub network_id: Uuid,
    pub total_lead_time: i32,
    pub total_cost: Option<f64>,
    pub bottleneck_nodes: Vec<NodeId>,
    pub underutilized_nodes: Vec<NodeId>,
    pub recommended_improvements: Vec<String>,
}

/// Shipment performance metrics
#[derive(Debug, Clone)]
pub struct ShipmentPerformance {
    pub on_time_delivery_rate: f64,
    pub average_transit_time: f64,
    pub delay_frequency: f64,
    pub carrier_performance: std::collections::HashMap<String, f64>,
}

/// Network analysis service for supply chain optimization
pub struct NetworkAnalysisService {
    network_repo: Arc<dyn SupplyChainNetworkRepository>,
    shipment_repo: Arc<dyn ShipmentRepository>,
    inventory_repo: Arc<dyn InventoryItemRepository>,
    privacy_service: Arc<dyn PrivacyConsentService>,
}

impl NetworkAnalysisService {
    pub fn new(
        network_repo: Arc<dyn SupplyChainNetworkRepository>,
        shipment_repo: Arc<dyn ShipmentRepository>,
        inventory_repo: Arc<dyn InventoryItemRepository>,
        privacy_service: Arc<dyn PrivacyConsentService>,
    ) -> Self {
        Self {
            network_repo,
            shipment_repo,
            inventory_repo,
            privacy_service,
        }
    }

    /// Analyze a supply chain network
    pub async fn analyze_network(
        &self,
        user_id: Uuid,
        network_id: Uuid,
    ) -> Result<NetworkAnalysisResult> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::NetworkAnalysis)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to analyze network: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find network
        let network = self.network_repo
            .find_by_id(network_id)
            .await
            .map_err(|_| ServiceError::NetworkNotFound)?;

        // Calculate total lead time (simplified - in a real implementation, 
        // this would be more complex and consider multiple paths)
        let total_lead_time = network.connections.iter().map(|c| c.lead_time_days).sum();

        // Calculate total cost if available
        let total_cost = network.connections.iter().map(|c| c.cost_per_unit.unwrap_or(0.0)).sum::<f64>();
        let total_cost = if total_cost > 0.0 { Some(total_cost) } else { None };

        // Identify bottleneck nodes (nodes with many connections)
        let mut node_connection_count = std::collections::HashMap::new();
        for connection in &network.connections {
            *node_connection_count.entry(connection.start_node_id).or_insert(0) += 1;
            *node_connection_count.entry(connection.end_node_id).or_insert(0) += 1;
        }

        let bottleneck_threshold = (node_connection_count.len() as f64 * 0.8) as usize;
        let bottleneck_nodes: Vec<NodeId> = node_connection_count
            .iter()
            .filter(|(_, &count)| count >= bottleneck_threshold)
            .map(|(&node_id, _)| node_id)
            .collect();

        // Identify underutilized nodes (nodes with few connections)
        let underutilized_threshold = (node_connection_count.len() as f64 * 0.2) as usize;
        let underutilized_nodes: Vec<NodeId> = node_connection_count
            .iter()
            .filter(|(_, &count)| count <= underutilized_threshold)
            .map(|(&node_id, _)| node_id)
            .collect();

        // Generate recommended improvements (simplified)
        let mut recommended_improvements = Vec::new();
        
        if !bottleneck_nodes.is_empty() {
            recommended_improvements.push(
                "Consider adding redundant paths to reduce bottleneck nodes".to_string()
            );
        }
        
        if !underutilized_nodes.is_empty() {
            recommended_improvements.push(
                "Consider optimizing or removing underutilized nodes".to_string()
            );
        }
        
        if total_cost.is_some() && total_cost.unwrap() > 10000.0 {
            recommended_improvements.push(
                "Consider negotiating better rates with carriers to reduce transportation costs".to_string()
            );
        }

        Ok(NetworkAnalysisResult {
            network_id,
            total_lead_time,
            total_cost,
            bottleneck_nodes,
            underutilized_nodes,
            recommended_improvements,
        })
    }

    /// Calculate lead time between two nodes
    pub async fn calculate_lead_time(
        &self,
        user_id: Uuid,
        network_id: Uuid,
        start_node: NodeId,
        end_node: NodeId,
    ) -> Result<i32> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::NetworkView)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to calculate lead time: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find network
        let network = self.network_repo
            .find_by_id(network_id)
            .await
            .map_err(|_| ServiceError::NetworkNotFound)?;

        // Calculate lead time using the domain method
        network.calculate_lead_time(start_node, end_node)
            .map_err(|e| match e {
                DomainError::NetworkValidationError { message } if message.contains("No path exists") => {
                    ServiceError::NoPathFound
                }
                _ => ServiceError::Domain(e),
            })
    }

    /// Get shipment performance metrics
    pub async fn get_shipment_performance(
        &self,
        user_id: Uuid,
        network_id: Uuid,
    ) -> Result<ShipmentPerformance> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::NetworkAnalysis)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to analyze shipment performance: {}", message),
                    }
                }
                _ => e,
            })?;

        // Get all shipments for this network (simplified - would need to link shipments to networks)
        let shipments = self.shipment_repo
            .find_all()
            .await
            .map_err(|e| ServiceError::Repository(e.to_string()))?;

        if shipments.is_empty() {
            return Ok(ShipmentPerformance {
                on_time_delivery_rate: 0.0,
                average_transit_time: 0.0,
                delay_frequency: 0.0,
                carrier_performance: std::collections::HashMap::new(),
            });
        }

        // Calculate on-time delivery rate
        let delivered_shipments: Vec<&Shipment> = shipments.iter()
            .filter(|s| s.status == crate::domain::shipment::ShipmentStatus::Delivered)
            .collect();
        
        let on_time_deliveries = delivered_shipments.iter()
            .filter(|s| !s.is_delayed())
            .count();
        
        let on_time_delivery_rate = if delivered_shipments.is_empty() {
            0.0
        } else {
            on_time_deliveries as f64 / delivered_shipments.len() as f64
        };

        // Calculate average transit time
        let total_transit_days: i32 = delivered_shipments.iter()
            .map(|s| s.current_transit_days())
            .sum();
        
        let average_transit_time = if delivered_shipments.is_empty() {
            0.0
        } else {
            total_transit_days as f64 / delivered_shipments.len() as f64
        };

        // Calculate delay frequency
        let delayed_shipments = delivered_shipments.iter()
            .filter(|s| s.is_delayed())
            .count();
        
        let delay_frequency = if delivered_shipments.is_empty() {
            0.0
        } else {
            delayed_shipments as f64 / delivered_shipments.len() as f64
        };

        // Calculate carrier performance
        let mut carrier_performance = std::collections::HashMap::new();
        let mut carrier_shipments = std::collections::HashMap::new();
        let mut carrier_on_time = std::collections::HashMap::new();

        for shipment in &shipments {
            if shipment.status == crate::domain::shipment::ShipmentStatus::Delivered {
                let count = carrier_shipments.entry(shipment.carrier.clone()).or_insert(0);
                *count += 1;
                
                if !shipment.is_delayed() {
                    let on_time_count = carrier_on_time.entry(shipment.carrier.clone()).or_insert(0);
                    *on_time_count += 1;
                }
            }
        }

        for (carrier, total_count) in carrier_shipments {
            let on_time_count = *carrier_on_time.get(&carrier).unwrap_or(&0);
            let performance = if total_count == 0 {
                0.0
            } else {
                on_time_count as f64 / total_count as f64
            };
            carrier_performance.insert(carrier, performance);
        }

        Ok(ShipmentPerformance {
            on_time_delivery_rate,
            average_transit_time,
            delay_frequency,
            carrier_performance,
        })
    }

    /// Identify critical suppliers in the network
    pub async fn identify_critical_suppliers(
        &self,
        user_id: Uuid,
        network_id: Uuid,
    ) -> Result<Vec<NodeId>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::NetworkAnalysis)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to identify critical suppliers: {}", message),
                    }
                }
                _ => e,
            })?;

        // Find network
        let network = self.network_repo
            .find_by_id(network_id)
            .await
            .map_err(|_| ServiceError::NetworkNotFound)?;

        // Identify supplier nodes that are critical (have many connections)
        let mut supplier_connections = std::collections::HashMap::new();
        
        for node in &network.nodes {
            if matches!(node.node_type, NodeType::Supplier) {
                supplier_connections.insert(node.id, 0);
            }
        }
        
        for connection in &network.connections {
            if supplier_connections.contains_key(&connection.start_node_id) {
                *supplier_connections.get_mut(&connection.start_node_id).unwrap() += 1;
            }
            if supplier_connections.contains_key(&connection.end_node_id) {
                *supplier_connections.get_mut(&connection.end_node_id).unwrap() += 1;
            }
        }
        
        // Suppliers with more than average connections are considered critical
        let average_connections = if supplier_connections.is_empty() {
            0.0
        } else {
            supplier_connections.values().sum::<usize>() as f64 / supplier_connections.len() as f64
        };
        
        let critical_suppliers: Vec<NodeId> = supplier_connections
            .iter()
            .filter(|(_, &connections)| connections as f64 > average_connections)
            .map(|(&node_id, _)| node_id)
            .collect();

        Ok(critical_suppliers)
    }

    /// Optimize network by suggesting improvements
    pub async fn optimize_network(
        &self,
        user_id: Uuid,
        network_id: Uuid,
    ) -> Result<Vec<String>> {
        // Validate user has necessary permissions
        self.privacy_service
            .verify_consent(user_id, ConsentType::NetworkOptimization)
            .await
            .map_err(|e| match e {
                ServiceError::PermissionDenied { message } => {
                    ServiceError::PermissionDenied {
                        message: format!("Insufficient permissions to optimize network: {}", message),
                    }
                }
                _ => e,
            })?;

        // Analyze network
        let analysis = self.analyze_network(user_id, network_id).await?;
        
        // Return recommendations
        Ok(analysis.recommended_improvements)
    }
}