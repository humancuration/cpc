use serde::{Serialize, Deserialize};
use crate::types::SupplyChain;
use crate::models::{Money, WarehouseLocation};
use validator::Validate;
use super::extensions::ProductExt;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Product {
    #[validate(length(min = 1, message = "ID is required"))]
    pub id: String,
    
    #[validate(length(min = 1, max = 100, message = "Name must be between 1-100 characters"))]
    pub name: String,
    
    #[validate(length(max = 100, message = "Brand must be less than 100 characters"))]
    pub brand: Option<String>,
    
    #[validate(length(max = 1000, message = "Description must be less than 1000 characters"))]
    pub description: Option<String>,
    
    pub barcode: Option<String>,
    
    #[validate(range(min = 0, message = "Carbon footprint must be non-negative"))]
    pub carbon_footprint: Option<f32>,
    
    pub packaging_type: Option<String>,
    pub nutritional_info: Option<String>,
    pub manufacturer: Option<String>,
    
    #[validate(range(min = 0, message = "Material cost must be non-negative"))]
    pub material_cost: Option<f32>,
    
    #[validate(range(min = 0, message = "Labor cost must be non-negative"))]
    pub labor_cost: Option<f32>,
    
    pub supplier: Option<String>,
    
    #[validate(range(min = 0, message = "Current stock must be non-negative"))]
    pub current_stock: Option<u32>,
    
    #[validate(range(min = 0, message = "Reorder level must be non-negative"))]
    pub reorder_level: Option<u32>,
    
    pub supply_chain: Option<SupplyChain>,
    
    #[validate]
    pub cost: Option<Money>,
    
    #[validate]
    pub location: Option<WarehouseLocation>,
}