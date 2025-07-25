use crate::types::Product;
use crate::mock::mock_supply_chain;
use tauri::State;
use sqlx::PgPool;

#[tauri::command]
pub async fn get_product_by_id(
    id: String,
    pool: State<'_, PgPool>
) -> Result<Product, String> {
    // In development, return mock product data
    #[cfg(debug_assertions)]
    if id == "mock" {
        let mut mock_product = Product {
            id: "mock".to_string(),
            name: "Organic Coffee".to_string(),
            brand: "Ethical Beans Co.".to_string(),
            description: "Premium organic coffee beans".to_string(),
            barcode: "1234567890".to_string(),
            carbon_footprint: 2.5,
            packaging_type: "Compostable bag".to_string(),
            nutritional_info: "".to_string(),
            manufacturer: "Green Coffee Processors".to_string(),
            material_cost: 5.0,
            labor_cost: 3.0,
            supplier: "Global Distributors".to_string(),
            current_stock: 100,
            reorder_level: 50,
            // supply_chain: Paused feature development
            // supply_chain: Some(mock_supply_chain()),
        };
        return Ok(mock_product);
    }

    // TODO: Implement actual database lookup in production
    // TODO: Implement product database lookup
    // TODO: Implement supply chain when feature resumes
    Err("Product not found".to_string())
}