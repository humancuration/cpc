use std::fs;
use std::path::Path;
use base64::encode;
use tauri::api::file::read_binary;
use tauri::command;
use cpc_core::services::product_display_service::{ProductDisplayService, ProductDisplayData, CostItem, ValidationUpdate};
use futures::StreamExt;
use tokio::sync::mpsc;
use serde::{Deserialize, Serialize};

#[derive(serde::Serialize)]
pub struct FilePreview {
    content_type: String,
    data: String,
}

#[command]
pub async fn download_file(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err("File not found".to_string());
    }

    // In a real implementation, we would stream the file to the user
    // For this example, we'll just log the download
    println!("Downloading file: {:?}", path);
    Ok(())
}

#[command]
pub async fn get_file_preview(path: String) -> Result<FilePreview, String> {
    let path = Path::new(&path);
    if !path.exists() {
        return Err("File not found".to_string());
    }

    let content_type = mime_guess::from_path(path).first_or_octet_stream().to_string();

    // Only read and encode text and image files
    let data = if content_type.starts_with("text/") || content_type.starts_with("image/") {
        match read_binary(&path) {
            Ok(bytes) => encode(bytes),
            Err(e) => return Err(format!("Failed to read file: {}", e)),
        }
    } else {
        String::new()
    };

    Ok(FilePreview {
        content_type,
        data,
    })
}

#[tauri::command]
async fn watch_directory(
    path: String,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
async fn get_conflicts() -> Result<Vec<String>, String> {
    Ok(vec![])
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CostItem {
    pub category: String,
    pub amount: f64,
    pub percentage: f64,
}

#[command]
pub async fn get_product(product_id: String) -> Result<ProductDisplayData, String> {
    let service = ProductDisplayService::new();
    
    match service.get_product_display(product_id).await {
        Ok(product) => Ok(product),
        Err(e) => Err(e.to_string())
    }
}

#[command]
pub async fn subscribe_validation_updates(
    product_id: String,
    window: tauri::Window,
) -> Result<(), String> {
    let service = ProductDisplayService::new();
    
    // Create a channel to receive validation updates
    let (tx, mut rx) = mpsc::channel::<ValidationUpdate>(32);
    
    // Spawn a task to handle the subscription
    tauri::async_runtime::spawn(async move {
        match service.subscribe_validation_updates(product_id).await {
            Ok(mut stream) => {
                while let Some(update) = stream.next().await {
                    match update {
                        Ok(validation_update) => {
                            let _ = tx.send(validation_update).await;
                        }
                        Err(e) => {
                            log::error!("Error in validation stream: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to subscribe to validation updates: {}", e);
            }
        }
    });
    
    // Spawn another task to emit events to the frontend
    tauri::async_runtime::spawn(async move {
        while let Some(update) = rx.recv().await {
            let _ = window.emit("validation-update", update);
        }
    });
    
    Ok(())
}