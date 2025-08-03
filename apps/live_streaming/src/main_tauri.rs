//! Main entry point for the Tauri desktop application

use tauri::{AppHandle, Manager, RunEvent, WindowEvent};
use cpc_live_streaming::web::module::initialize;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    tracing::info!("Starting Live Streaming Tauri application");
    
    // Initialize database connection
    // In a real application, you would use actual database credentials
    let pool = PgPool::connect("postgresql://localhost/live_streaming").await?;
    
    // Initialize the live streaming module
    let _module = initialize(pool);
    
    // Create the Tauri application
    tauri::Builder::default()
        .setup(|_app| {
            tracing::info!("Tauri application setup complete");
            Ok(())
        })
        .on_page_load(|window, _payload| {
            tracing::info!("Page loaded in window: {}", window.label());
        })
        .build(tauri::generate_context!())
        .expect("Error while building Tauri application")
        .run(|_app_handle, event| match event {
            RunEvent::WindowEvent { label, event: WindowEvent::CloseRequested { .. }, .. } => {
                tracing::info!("Window {} closed", label);
            }
            RunEvent::ExitRequested { .. } => {
                tracing::info!("Exit requested");
            }
            _ => {}
        });
    
    Ok(())
}