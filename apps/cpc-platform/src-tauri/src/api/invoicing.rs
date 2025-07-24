use serde::{Deserialize, Serialize};
use crate::types::invoice::{Invoice, InvoiceTemplate, Contact};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub recipient_id: String,
    pub items: Vec<InvoiceItemRequest>,
    pub due_date: String,
    pub template_id: Option<String>,
    pub notes: Option<String>,
    pub tax_rate: f64,
    pub discount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItemRequest {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceResponse {
    pub success: bool,
    pub invoice_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTemplatesResponse {
    pub templates: Vec<InvoiceTemplate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetContactsResponse {
    pub contacts: Vec<Contact>,
}

#[tauri::command]
pub async fn get_invoice_templates() -> Result<GetTemplatesResponse, String> {
    // TODO: Implement actual database query
    let templates = vec![
        InvoiceTemplate {
            id: "default".to_string(),
            name: "Standard Template".to_string(),
            description: Some("Default invoice template".to_string()),
            default_layout: "standard".to_string(),
            custom_fields: None,
        },
        InvoiceTemplate {
            id: "professional".to_string(),
            name: "Professional Template".to_string(),
            description: Some("Professional layout with company branding".to_string()),
            default_layout: "professional".to_string(),
            custom_fields: None,
        },
    ];
    
    Ok(GetTemplatesResponse { templates })
}

#[tauri::command]
pub async fn get_contacts() -> Result<GetContactsResponse, String> {
    // TODO: Implement actual database query
    let contacts = vec![
        Contact {
            id: "1".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone: Some("+1-555-0123".to_string()),
            address: Some("123 Main St, Anytown, USA".to_string()),
        },
        Contact {
            id: "2".to_string(),
            name: "Jane Smith".to_string(),
            email: "jane@example.com".to_string(),
            phone: Some("+1-555-0124".to_string()),
            address: Some("456 Oak Ave, Somewhere, USA".to_string()),
        },
    ];
    
    Ok(GetContactsResponse { contacts })
}

#[tauri::command]
pub async fn create_invoice(request: CreateInvoiceRequest) -> Result<CreateInvoiceResponse, String> {
    // TODO: Implement actual invoice creation with database storage
    
    // Validate request
    if request.recipient_id.is_empty() {
        return Err("Recipient ID is required".to_string());
    }
    
    if request.items.is_empty() {
        return Err("At least one item is required".to_string());
    }
    
    for item in &request.items {
        if item.description.is_empty() {
            return Err("Item description is required".to_string());
        }
        if item.quantity <= 0.0 {
            return Err("Item quantity must be greater than 0".to_string());
        }
        if item.unit_price < 0.0 {
            return Err("Item price must be non-negative".to_string());
        }
    }
    
    // Here you would typically:
    // 1. Save to database
    // 2. Generate invoice PDF
    // 3. Send email notification
    // 4. Return the created invoice ID
    
    let invoice_id = Some(uuid::Uuid::new_v4().to_string());
    
    Ok(CreateInvoiceResponse {
        success: true,
        invoice_id,
        error: None,
    })
}