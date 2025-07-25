use super::graphql::{
    fetch_invoice_dashboard_data as fetch_dashboard_data,
    fetch_invoice_details,
    generate_invoice_pdf_mutation,
    create_invoice_mutation,
};
use serde::{Deserialize, Serialize};
use crate::invoicing::graphql::create_invoice;
use tauri::api::dialog::FileDialogBuilder;
use tauri::Window;
use base64::{engine::general_purpose, Engine as _};
use std::io::Write;

#[tauri::command]
pub async fn fetch_invoice_dashboard_data(
    organization_id: String,
) -> Result<String, String> {
    match fetch_dashboard_data(organization_id).await {
        Ok(data) => Ok(serde_json::to_string(&data).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_invoice_details(invoice_id: String) -> Result<String, String> {
    match fetch_invoice_details(invoice_id).await {
        Ok(data) => Ok(serde_json::to_string(&data.invoice).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn generate_invoice_pdf(window: Window, invoice_id: String, invoice_number: String) -> Result<(), String> {
    let base64_pdf = generate_invoice_pdf_mutation(invoice_id)
        .await
        .map_err(|e| e.to_string())?;

    let pdf_bytes = general_purpose::STANDARD.decode(&base64_pdf)
        .map_err(|e| e.to_string())?;

    let default_filename = format!("Invoice-{}.pdf", invoice_number);
    
    let dialog = FileDialogBuilder::new()
        .set_title("Save Invoice PDF")
        .set_file_name(&default_filename)
        .add_filter("PDF", &["pdf"]);

    let window_clone = window.clone();
    dialog.save_file(move |file_path| {
        if let Some(path) = file_path {
            if let Err(e) = std::fs::write(&path, &pdf_bytes) {
                 // It's good practice to inform the user of the error.
                // Here we just log it, but a dialog could be shown.
                eprintln!("Failed to save file: {}", e);
            }
        }
    });

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoicePayload {
    client_name: String,
    client_email: String,
    client_address: String,
    issue_date: String,
    due_date: String,
    line_items: Vec<LineItemPayload>,
    notes: Option<String>,
    tax_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineItemPayload {
    description: String,
    quantity: f64,
    unit_price: f64,
}

#[tauri::command]
pub async fn create_invoice(
    payload: CreateInvoicePayload,
) -> Result<String, String> {
    
    let items = payload.line_items.into_iter().map(|item| {
        create_invoice::CreateInvoiceItemInput {
            description: item.description,
            quantity: item.quantity,
            unit_price: item.unit_price,
        }
    }).collect();

    let input = create_invoice::CreateInvoiceInput {
        client_name: payload.client_name,
        client_email: Some(payload.client_email),
        client_address: Some(payload.client_address),
        invoice_number: "INV-".to_owned() + &chrono::Utc::now().timestamp_millis().to_string(), // Simplified invoice number generation
        issue_date: payload.issue_date.parse().unwrap(),
        due_date: payload.due_date.parse().unwrap(),
        items,
        tax_rate: payload.tax_rate,
        notes: payload.notes,
        terms: None,
    };

    match create_invoice_mutation(input).await {
        Ok(data) => Ok(serde_json::to_string(&data).unwrap()),
        Err(e) => Err(e.to_string()),
    }
}