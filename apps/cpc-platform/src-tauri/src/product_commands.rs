use tauri::State;
use cpc_core::services::product_display_service::{ProductDisplayService, ProductQuery, ValidationUpdate};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use std::sync::Arc;
use tracing::{info, error};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductDetailsResponse {
    pub product: ProductQuery,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ValidationSubscription {
    pub product_id: String,
    pub channel: String,
}

/// Get detailed product information by ID
#[tauri::command]
pub async fn get_product_details(
    product_id: String,
    service: State<'_, Arc<ProductDisplayService>>,
) -> Result<ProductDetailsResponse, String> {
    info!("Tauri command: get_product_details for product: {}", product_id);
    
    match service.get_product(product_id.clone()).await {
        Ok(product) => Ok(ProductDetailsResponse {
            product,
            status: "success".to_string(),
        }),
        Err(e) => {
            error!("Failed to get product details: {:?}", e);
            Err(format!("Failed to get product: {}", e))
        }
    }
}

/// Subscribe to product validation updates
#[tauri::command]
pub async fn subscribe_to_product_validation(
    product_id: String,
    service: State<'_, Arc<ProductDisplayService>>,
) -> Result<String, String> {
    info!("Tauri command: subscribe_to_product_validation for product: {}", product_id);
    
    // Create a new subscription channel
    let mut rx = service.get_validation_update_stream();
    
    // Spawn a task to listen for updates and emit Tauri events
    let app_handle = tauri::async_runtime::spawn(async move {
        while let Ok(update) = rx.recv().await {
            if update.product_id == product_id {
                // Emit Tauri event to the frontend
                let _ = tauri::Manager::emit_all(
                    &tauri::AppHandle::new(),
                    format!("validation-update-{}", product_id),
                    update,
                );
            }
        }
    });
    
    Ok(format!("subscribed-{}", product_id))
}

/// Get current validation status for a product
#[tauri::command]
pub async fn get_validation_status(
    product_id: String,
    service: State<'_, Arc<ProductDisplayService>>,
) -> Result<ValidationUpdate, String> {
    info!("Tauri command: get_validation_status for product: {}", product_id);
    
    service.get_validation_status(product_id)
        .await
        .map_err(|e| format!("Failed to get validation status: {}", e))
}

/// Update product validation and notify subscribers
#[tauri::command]
pub async fn update_product_validation(
    product_id: String,
    is_valid: bool,
    validation_errors: Vec<String>,
    confidence_score: f64,
    service: State<'_, Arc<ProductDisplayService>>,
) -> Result<bool, String> {
    info!("Tauri command: update_product_validation for product: {}", product_id);
    
    service.update_and_notify(
        product_id,
        is_valid,
        validation_errors,
        confidence_score,
    )
    .await
    .map_err(|e| format!("Failed to update validation: {}", e))
}