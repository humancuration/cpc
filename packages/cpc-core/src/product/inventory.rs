use chrono::{DateTime, Utc};
use crate::product::model::Product;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryUpdate {
    pub product_id: String,
    pub quantity: i32,
    pub location: String,
    pub timestamp: DateTime<Utc>,
    pub notes: Option<String>,
}

pub fn update_inventory(product: &mut Product, quantity: i32, location: &str) -> Result<(), String> {
    if quantity < 0 && product.current_stock < (-quantity) as u32 {
        return Err("Insufficient stock".to_string());
    }
    
    product.current_stock = (product.current_stock as i32 + quantity) as u32;
    Ok(())
}

pub fn get_inventory_history(product_id: &str) -> Vec<InventoryUpdate> {
    // Placeholder implementation - would query database in real implementation
    vec![]
}