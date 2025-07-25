// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use cpc_core::services::product_display_service::ProductDisplayService;
use tracing::{info, error};

mod product_commands;

#[tokio::main]
async fn main() {
    tracing_subscriber::init();
    
    info!("Starting CPC Platform Tauri application");

    let product_service = Arc::new(ProductDisplayService::new());
    let supply_chain_service = Arc::new(SupplyChainService::new());

    tauri::Builder::default()
        .manage(product_service)
        .manage(supply_chain_service)
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            product_commands::get_product_details,
            product_commands::subscribe_to_product_validation,
            product_commands::get_validation_status,
            product_commands::update_product_validation,
            supply_chain_commands::get_supply_chain,
            supply_chain_commands::subscribe_to_supply_chain_updates,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}