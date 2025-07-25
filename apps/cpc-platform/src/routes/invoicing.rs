// DEPRECATED: This file contains a legacy Axum router and is no longer in use.
// The invoicing frontend is now handled by Yew components and a Yew router.
// This file is preserved for historical reference and will be removed in the future.
/*
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::api::invoicing::{InvoicingService, InvoiceFilters};
use crate::context::{InvoicingContext, Invoice, InvoiceTemplate, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceRoutesState {
    pub invoicing_service: InvoicingService,
    pub context: InvoicingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub client_id: String,
    pub issue_date: String,
    pub due_date: String,
    pub line_items: Vec<CreateInvoiceLineItem>,
    pub notes: Option<String>,
    pub terms: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceLineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInvoiceRequest {
    pub status: Option<String>,
    pub notes: Option<String>,
    pub terms: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceListParams {
    pub status: Option<String>,
    pub client_id: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

pub fn create_invoicing_routes() -> Router<Arc<InvoiceRoutesState>> {
    Router::new()
        .route("/api/invoices", get(list_invoices))
        .route("/api/invoices", post(create_invoice))
        .route("/api/invoices/:id", get(get_invoice))
        .route("/api/invoices/:id", put(update_invoice))
        .route("/api/invoices/:id", delete(delete_invoice))
        .route("/api/invoices/:id/send", post(send_invoice))
        .route("/api/invoices/:id/duplicate", post(duplicate_invoice))
        .route("/api/templates", get(list_templates))
        .route("/api/templates", post(create_template))
        .route("/api/templates/:id", get(get_template))
        .route("/api/templates/:id", put(update_template))
        .route("/api/templates/:id", delete(delete_template))
        .route("/api/aging-report", get(get_aging_report))
        .route("/api/supplier-performance", get(get_supplier_performance))
        .route("/api/clients", get(list_clients))
        .route("/api/summary", get(get_summary))
}

async fn list_invoices(
    State(state): State<Arc<InvoiceRoutesState>>,
    Query(params): Query<InvoiceListParams>,
) -> Result<Json<PaginatedResponse<Invoice>>, Json<serde_json::Value>> {
    let filters = InvoiceFilters {
        status: params.status,
        client_id: params.client_id,
        date_from: params.date_from,
        date_to: params.date_to,
        min_amount: params.min_amount,
        max_amount: params.max_amount,
    };

    match state.invoicing_service.get_invoices(filters).await {
        Ok(invoices) => {
            let page = params.page.unwrap_or(1);
            let limit = params.limit.unwrap_or(20);
            let start = ((page - 1) * limit) as usize;
            let end = (start + limit as usize).min(invoices.len());
            
            let paginated_invoices = invoices[start..end].to_vec();
            
            Ok(Json(PaginatedResponse {
                data: paginated_invoices,
                total: invoices.len() as u32,
                page,
                limit,
            }))
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "error": "Failed to fetch invoices",
                "details": e.to_string()
            });
            Err(Json(error_response))
        }
    }
}

async fn create_invoice(
    State(state): State<Arc<InvoiceRoutesState>>,
    Json(payload): Json<CreateInvoiceRequest>,
) -> Result<Json<Invoice>, Json<serde_json::Value>> {
    // Implementation would create invoice via GraphQL
    let invoice = Invoice {
        id: uuid::Uuid::new_v4().to_string(),
        invoice_number: format!("INV-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")),
        client_id: payload.client_id.clone(),
        client_name: "Client Name".to_string(), // Would fetch from client service
        issue_date: chrono::DateTime::parse_from_rfc3339(&payload.issue_date)
            .unwrap_or_else(|_| chrono::Utc::now())
            .with_timezone(&chrono::Utc),
        due_date: chrono::DateTime::parse_from_rfc3339(&payload.due_date)
            .unwrap_or_else(|_| chrono::Utc::now())
            .with_timezone(&chrono::Utc),
        subtotal: payload.line_items.iter().map(|item| item.quantity * item.unit_price).sum(),
        tax_amount: 0.0, // Would calculate based on tax rules
        total: payload.line_items.iter().map(|item| item.quantity * item.unit_price).sum(),
        status: crate::context::InvoiceStatus::Draft,
        line_items: payload.line_items.into_iter().map(|item| {
            crate::context::InvoiceLineItem {
                id: uuid::Uuid::new_v4().to_string(),
                description: item.description,
                quantity: item.quantity,
                unit_price: item.unit_price,
                total: item.quantity * item.unit_price,
            }
        }).collect(),
        notes: payload.notes,
        terms: payload.terms,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // Update context
    state.context.add_invoice(invoice.clone()).await;
    
    Ok(Json(invoice))
}

async fn get_invoice(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, Json<serde_json::Value>> {
    let state_data = state.context.state.read().await;
    if let Some(invoice) = state_data.invoices.iter().find(|i| i.id == id) {
        Ok(Json(invoice.clone()))
    } else {
        let error_response = serde_json::json!({
            "error": "Invoice not found",
            "id": id
        });
        Err(Json(error_response))
    }
}

async fn update_invoice(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateInvoiceRequest>,
) -> Result<Json<Invoice>, Json<serde_json::Value>> {
    let mut state_data = state.context.state.write().await;
    if let Some(invoice) = state_data.invoices.iter_mut().find(|i| i.id == id) {
        if let Some(status) = payload.status {
            invoice.status = match status.as_str() {
                "paid" => crate::context::InvoiceStatus::Paid,
                "sent" => crate::context::InvoiceStatus::Sent,
                "overdue" => crate::context::InvoiceStatus::Overdue,
                "cancelled" => crate::context::InvoiceStatus::Cancelled,
                _ => invoice.status.clone(),
            };
        }
        
        if let Some(notes) = payload.notes {
            invoice.notes = Some(notes);
        }
        
        if let Some(terms) = payload.terms {
            invoice.terms = Some(terms);
        }
        
        invoice.updated_at = chrono::Utc::now();
        
        let updated_invoice = invoice.clone();
        drop(state_data);
        
        state.context.update_invoice(id, updated_invoice.clone()).await;
        Ok(Json(updated_invoice))
    } else {
        let error_response = serde_json::json!({
            "error": "Invoice not found",
            "id": id
        });
        Err(Json(error_response))
    }
}

async fn delete_invoice(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    state.context.remove_invoice(id.clone()).await;
    Ok(Json(serde_json::json!({
        "success": true,
        "id": id
    })))
}

async fn send_invoice(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let mut state_data = state.context.state.write().await;
    if let Some(invoice) = state_data.invoices.iter_mut().find(|i| i.id == id) {
        invoice.status = crate::context::InvoiceStatus::Sent;
        invoice.updated_at = chrono::Utc::now();
        
        let updated_invoice = invoice.clone();
        drop(state_data);
        
        state.context.update_invoice(id.clone(), updated_invoice).await;
        
        Ok(Json(serde_json::json!({
            "success": true,
            "message": "Invoice sent successfully"
        })))
    } else {
        let error_response = serde_json::json!({
            "error": "Invoice not found",
            "id": id
        });
        Err(Json(error_response))
    }
}

async fn duplicate_invoice(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
) -> Result<Json<Invoice>, Json<serde_json::Value>> {
    let state_data = state.context.state.read().await;
    if let Some(original) = state_data.invoices.iter().find(|i| i.id == id) {
        let new_invoice = Invoice {
            id: uuid::Uuid::new_v4().to_string(),
            invoice_number: format!("INV-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S")),
            client_id: original.client_id.clone(),
            client_name: original.client_name.clone(),
            issue_date: chrono::Utc::now(),
            due_date: chrono::Utc::now() + chrono::Duration::days(30),
            subtotal: original.subtotal,
            tax_amount: original.tax_amount,
            total: original.total,
            status: crate::context::InvoiceStatus::Draft,
            line_items: original.line_items.clone(),
            notes: original.notes.clone(),
            terms: original.terms.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        drop(state_data);
        state.context.add_invoice(new_invoice.clone()).await;
        Ok(Json(new_invoice))
    } else {
        let error_response = serde_json::json!({
            "error": "Invoice not found",
            "id": id
        });
        Err(Json(error_response))
    }
}

async fn list_templates(
    State(state): State<Arc<InvoiceRoutesState>>,
) -> Result<Json<Vec<InvoiceTemplate>>, Json<serde_json::Value>> {
    let state_data = state.context.state.read().await;
    Ok(Json(state_data.templates.clone()))
}

async fn create_template(
    State(state): State<Arc<InvoiceRoutesState>>,
    Json(template): Json<InvoiceTemplate>,
) -> Result<Json<InvoiceTemplate>, Json<serde_json::Value>> {
    let new_template = InvoiceTemplate {
        id: uuid::Uuid::new_v4().to_string(),
        ..template
    };
    
    state.context.state.write().await.templates.push(new_template.clone());
    Ok(Json(new_template))
}

async fn get_template(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
) -> Result<Json<InvoiceTemplate>, Json<serde_json::Value>> {
    let state_data = state.context.state.read().await;
    if let Some(template) = state_data.templates.iter().find(|t| t.id == id) {
        Ok(Json(template.clone()))
    } else {
        let error_response = serde_json::json!({
            "error": "Template not found",
            "id": id
        });
        Err(Json(error_response))
    }
}

async fn update_template(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
    Json(template): Json<InvoiceTemplate>,
) -> Result<Json<InvoiceTemplate>, Json<serde_json::Value>> {
    let mut state_data = state.context.state.write().await;
    if let Some(existing) = state_data.templates.iter_mut().find(|t| t.id == id) {
        *existing = template;
        Ok(Json(existing.clone()))
    } else {
        let error_response = serde_json::json!({
            "error": "Template not found",
            "id": id
        });
        Err(Json(error_response))
    }
}

async fn delete_template(
    State(state): State<Arc<InvoiceRoutesState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    state.context.state.write().await.templates.retain(|t| t.id != id);
    Ok(Json(serde_json::json!({
        "success": true,
        "id": id
    })))
}

async fn get_aging_report(
    State(state): State<Arc<InvoiceRoutesState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let days_past_due = params.get("days")
        .and_then(|d| d.parse::<i32>().ok())
        .unwrap_or(30);

    match state.invoicing_service.get_aging_report(days_past_due).await {
        Ok(report) => Ok(Json(serde_json!({ "data": report }))),
        Err(e) => {
            let error_response = serde_json::json!({
                "error": "Failed to fetch aging report",
                "details": e.to_string()
            });
            Err(Json(error_response))
        }
    }
}

async fn get_supplier_performance(
    State(state): State<Arc<InvoiceRoutesState>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    let supplier_id = params.get("supplier_id").cloned();

    match state.invoicing_service.get_supplier_performance(supplier_id).await {
        Ok(performance) => Ok(Json(serde_json!({ "data": performance }))),
        Err(e) => {
            let error_response = serde_json::json!({
                "error": "Failed to fetch supplier performance",
                "details": e.to_string()
            });
            Err(Json(error_response))
        }
    }
}

async fn list_clients(
    State(state): State<Arc<InvoiceRoutesState>>,
) -> Result<Json<Vec<Client>>, Json<serde_json::Value>> {
    let state_data = state.context.state.read().await;
    Ok(Json(state_data.clients.clone()))
}

async fn get_summary(
    State(state): State<Arc<InvoiceRoutesState>>,
) -> Result<Json<crate::context::InvoicingSummary>, Json<serde_json::Value>> {
    let summary = state.context.summary.read().await;
    Ok(Json(summary.clone()))
}
*/