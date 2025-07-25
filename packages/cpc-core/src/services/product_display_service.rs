use std::sync::Arc;
use async_graphql::*;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};
use tokio::sync::broadcast;

use crate::models::*;
use crate::error::CpcError;
use crate::product::{ProductExt, ProductValidate};

/// GraphQL query for retrieving a single product
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct ProductQuery {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub barcode: Option<String>,
    pub carbon_footprint: Option<f64>,
    pub packaging_type: Option<String>,
    pub nutritional_info: Option<String>,
    pub manufacturer: Option<String>,
    pub material_cost: Option<f64>,
    pub labor_cost: Option<f64>,
    pub supplier: Option<String>,
    pub current_stock: Option<u32>,
    pub reorder_level: Option<u32>,
    pub supply_chain: Option<SupplyChain>,
    pub cost: Option<Money>,
    pub location: Option<WarehouseLocation>,
}

/// Display-ready product data with calculated fields for Tauri commands
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDisplayData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub cost_breakdown: Vec<CostItem>,
    pub total_cost: f64,
    pub profit_margin: f64,
    pub validation_status: String,
    pub image_urls: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub category: String,
    pub amount: f64,
    pub percentage: f64,
}

/// Validation update information
#[derive(Debug, Clone, SimpleObject, Serialize, Deserialize)]
pub struct ValidationUpdate {
    pub product_id: String,
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    pub last_updated: String,
    pub confidence_score: f64,
}

/// Service for handling product display operations
pub struct ProductDisplayService {
    // Database connection or repository reference would go here
}

impl ProductDisplayService {
    /// Subscribe to validation updates for a product using broadcast channel
    pub fn get_validation_update_stream(&self) -> broadcast::Receiver<ValidationUpdate> {
        self.validation_update_sender.subscribe()
    }
    
    /// Update product validation and notify subscribers
    pub async fn update_and_notify(
        &self,
        product_id: String,
        is_valid: bool,
        validation_errors: Vec<String>,
        confidence_score: f64,
    ) -> Result<(), CpcError> {
        info!("Updating validation for product: {}", product_id);
        
        // TODO: Implement actual database update
        // For now, just broadcast the update
        
        let update = ValidationUpdate {
            product_id,
            is_valid,
            validation_errors,
            last_updated: chrono::Utc::now().to_rfc3339(),
            confidence_score,
        };
        
        // Send the update to all subscribers
        let _ = self.validation_update_sender.send(update);
        
        Ok(())
    }
    
    /// Subscribe to validation updates for a product (legacy method - use get_validation_update_stream instead)
    pub async fn subscribe_validation_updates(&self, product_id: String) -> Result<impl futures::Stream<Item = Result<ValidationUpdate, CpcError>>, CpcError> {
        use futures::stream::{self, StreamExt};
        use async_stream::stream;
        
        let mut rx = self.get_validation_update_stream();
        
        Ok(stream! {
            while let Ok(update) = rx.recv().await {
                if update.product_id == product_id {
                    yield Ok(update);
                }
            }
        })
    }

impl ProductDisplayService {
    /// Create a new product display service
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self {
            validation_update_sender: sender,
        }
    }
    
    /// Get a product by ID for GraphQL
    pub async fn get_product(&self, id: String) -> Result<ProductQuery, CpcError> {
        info!("Fetching product with ID: {}", id);
        
        // TODO: Implement actual database query
        // For now, return mock data
        let product = ProductQuery {
            id: id.clone(),
            name: format!("Product {}", id),
            brand: Some("EcoFriendly Co.".to_string()),
            description: Some("A sustainable product made with care for the environment".to_string()),
            barcode: Some(format!("1234567890{}", id)),
            carbon_footprint: Some(2.5),
            packaging_type: Some("Recyclable cardboard".to_string()),
            nutritional_info: Some("Energy: 200kcal, Protein: 10g, Fat: 5g".to_string()),
            manufacturer: Some("Sustainable Manufacturing Ltd.".to_string()),
            material_cost: Some(5.0),
            labor_cost: Some(3.0),
            supplier: Some("Green Supply Chain Co.".to_string()),
            current_stock: Some(150),
            reorder_level: Some(50),
            supply_chain: Some(SupplyChain {
                nodes: vec![
                    SupplyChainNode {
                        id: "1".to_string(),
                        node_type: NodeType::RawMaterial,
                        location: "Organic Farm, Oregon".to_string(),
                        company: "Green Farms Inc.".to_string(),
                        timestamp: "2024-01-15T08:00:00Z".to_string(),
                        coordinates: Coordinates { latitude: 44.0521, longitude: -123.0868 },
                    },
                    SupplyChainNode {
                        id: "2".to_string(),
                        node_type: NodeType::Manufacturer,
                        location: "Portland, OR".to_string(),
                        company: "Sustainable Manufacturing Ltd.".to_string(),
                        timestamp: "2024-01-16T10:00:00Z".to_string(),
                        coordinates: Coordinates { latitude: 45.5152, longitude: -122.6784 },
                    },
                    SupplyChainNode {
                        id: "3".to_string(),
                        node_type: NodeType::Distributor,
                        location: "Seattle, WA".to_string(),
                        company: "Green Supply Chain Co.".to_string(),
                        timestamp: "2024-01-17T14:00:00Z".to_string(),
                        coordinates: Coordinates { latitude: 47.6062, longitude: -122.3321 },
                    },
                ],
                segments: vec![
                    TransportationSegment {
                        from_node_id: "1".to_string(),
                        to_node_id: "2".to_string(),
                        method: TransportMethod::Truck,
                        duration_hours: 4,
                        carbon_footprint: 0.8,
                    },
                    TransportationSegment {
                        from_node_id: "2".to_string(),
                        to_node_id: "3".to_string(),
                        method: TransportMethod::Truck,
                        duration_hours: 3,
                        carbon_footprint: 0.6,
                    },
                ],
            }),
            cost: Some(Money { amount: 12.99, currency: "USD".to_string() }),
            location: Some(WarehouseLocation {
                id: "WH001".to_string(),
                name: "Main Warehouse".to_string(),
            }),
        };
        
        Ok(product)
    }

    /// Get a product by ID for Tauri display
    pub async fn get_product_display(&self, id: String) -> Result<ProductDisplayData, CpcError> {
        info!("Fetching product display data with ID: {}", id);
        
        // Get the base product data
        let product = self.get_product(id.clone()).await?;
        
        // Transform to display format
        let material_cost = product.material_cost.unwrap_or(0.0);
        let labor_cost = product.labor_cost.unwrap_or(0.0);
        let total_cost = material_cost + labor_cost;
        let selling_price = product.cost.as_ref().map(|c| c.amount).unwrap_or(0.0);
        let profit_margin = if selling_price > 0.0 {
            ((selling_price - total_cost) / selling_price) * 100.0
        } else {
            0.0
        };
        
        Ok(ProductDisplayData {
            id: product.id,
            name: product.name,
            description: product.description.unwrap_or_default(),
            cost_breakdown: vec![
                CostItem {
                    category: "Materials".to_string(),
                    amount: material_cost,
                    percentage: if total_cost > 0.0 { (material_cost / total_cost) * 100.0 } else { 0.0 },
                },
                CostItem {
                    category: "Labor".to_string(),
                    amount: labor_cost,
                    percentage: if total_cost > 0.0 { (labor_cost / total_cost) * 100.0 } else { 0.0 },
                },
            ],
            total_cost,
            profit_margin,
            validation_status: "valid".to_string(),
            image_urls: vec![],
        })
    }
    
    /// Get validation status for a product
    pub async fn get_validation_status(&self, product_id: String) -> Result<ValidationUpdate, CpcError> {
        info!("Getting validation status for product: {}", product_id);
        
        // TODO: Implement actual validation logic
        Ok(ValidationUpdate {
            product_id,
            is_valid: true,
            validation_errors: vec![],
            last_updated: chrono::Utc::now().to_rfc3339(),
            confidence_score: 0.95,
        })
    }
    
    /// Transform product data for display
    pub fn transform_product_data(&self, product: ProductQuery) -> DisplayProductData {
        DisplayProductData {
            id: product.id,
            name: product.name,
            brand: product.brand,
            description: product.description,
            barcode: product.barcode,
            carbon_footprint: product.carbon_footprint,
            packaging_type: product.packaging_type,
            nutritional_info: product.nutritional_info,
            manufacturer: product.manufacturer,
            material_cost: product.material_cost,
            labor_cost: product.labor_cost,
            supplier: product.supplier,
            current_stock: product.current_stock,
            reorder_level: product.reorder_level,
            supply_chain: product.supply_chain,
            cost: product.cost,
            location: product.location,
            calculated_fields: CalculatedFields {
                total_cost: self.calculate_total_cost(&product),
                profit_margin: self.calculate_profit_margin(&product),
                sustainability_score: self.calculate_sustainability_score(&product),
                stock_status: self.determine_stock_status(&product),
            },
        }
    }
    
    /// Calculate total cost based on material and labor costs
    fn calculate_total_cost(&self, product: &ProductQuery) -> f64 {
        product.material_cost.unwrap_or(0.0) + product.labor_cost.unwrap_or(0.0)
    }
    
    /// Calculate profit margin
    fn calculate_profit_margin(&self, product: &ProductQuery) -> f64 {
        let total_cost = self.calculate_total_cost(product);
        if total_cost > 0.0 && product.cost.is_some() {
            let selling_price = product.cost.as_ref().unwrap().amount;
            ((selling_price - total_cost) / selling_price) * 100.0
        } else {
            0.0
        }
    }
    
    /// Calculate sustainability score (0-100)
    fn calculate_sustainability_score(&self, product: &ProductQuery) -> f64 {
        let mut score = 50.0; // Base score
        
        // Bonus for low carbon footprint
        if let Some(carbon) = product.carbon_footprint {
            score += (5.0 - carbon).max(0.0) * 5.0;
        }
        
        // Bonus for sustainable packaging
        if let Some(packaging) = &product.packaging_type {
            if packaging.to_lowercase().contains("recyclable") {
                score += 20.0;
            }
        }
        
        score.min(100.0)
    }
    
    /// Determine stock status
    fn determine_stock_status(&self, product: &ProductQuery) -> String {
        match product.current_stock {
            Some(stock) => {
                let reorder = product.reorder_level.unwrap_or(0);
                if stock == 0 {
                    "Out of Stock".to_string()
                } else if stock <= reorder {
                    "Low Stock".to_string()
                } else {
                    "In Stock".to_string()
                }
            }
            None => "Unknown".to_string(),
        }
    }
}

/// Display-ready product data with calculated fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayProductData {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub description: Option<String>,
    pub barcode: Option<String>,
    pub carbon_footprint: Option<f64>,
    pub packaging_type: Option<String>,
    pub nutritional_info: Option<String>,
    pub manufacturer: Option<String>,
    pub material_cost: Option<f64>,
    pub labor_cost: Option<f64>,
    pub supplier: Option<String>,
    pub current_stock: Option<u32>,
    pub reorder_level: Option<u32>,
    pub supply_chain: Option<SupplyChain>,
    pub cost: Option<Money>,
    pub location: Option<WarehouseLocation>,
    pub calculated_fields: CalculatedFields,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculatedFields {
    pub total_cost: f64,
    pub profit_margin: f64,
    pub sustainability_score: f64,
    pub stock_status: String,
}

impl Default for ProductDisplayService {
    fn default() -> Self {
        Self::new()
    }
}