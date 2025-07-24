pub mod api;
pub mod pdf_generator;
pub mod repository;
pub mod sync;

// Re-exports for convenience
pub use api::*;
pub use pdf_generator::*;
pub use repository::*;
pub use sync::*;

use crate::invoicing::repository::RepositoryFactory;
use crate::invoicing::api::InvoiceApiService;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared state for the invoicing system
#[derive(Debug)]
pub struct InvoiceAppState {
    pub api_service: Arc<RwLock<InvoiceApiService>>,
}

impl InvoiceAppState {
    pub fn new() -> Self {
        let repository_factory = RepositoryFactory::new();
        let api_service = InvoiceApiService::new(repository_factory);
        
        Self {
            api_service: Arc::new(RwLock::new(api_service)),
        }
    }
}

/// Initialize the invoicing system
pub fn init_invoicing(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Create shared state
    let state = InvoiceAppState::new();
    
    // Manage state
    app.manage(state.api_service.clone());
    
    Ok(())
}

/// Register all Tauri commands
pub fn register_commands<R: tauri::Runtime>(invoke_handler: tauri::Invoke<R>) -> tauri::Invoke<R> {
    invoke_handler
        .invoke_handler(tauri::generate_handler![
            // Invoice commands
            api::create_invoice,
            api::get_invoice,
            api::update_invoice,
            api::delete_invoice,
            api::list_invoices,
            api::search_invoices,
            
            // Customer commands
            api::create_customer,
            api::get_customer,
            api::update_customer,
            api::delete_customer,
            api::list_customers,
            api::search_customers,
            
            // PDF commands
            api::generate_pdf,
            
            // Sync commands
            api::sync_invoices,
            api::get_sync_state,
        ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::invoicing::model::{Invoice, Customer};
    use cpc_core::invoicing::money::Money;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_invoicing_system_initialization() {
        let state = InvoiceAppState::new();
        let api_service = state.api_service.read().await;
        
        // Test that we can create a customer
        let customer = api::CreateCustomerRequest {
            name: "Test Customer".to_string(),
            email: Some("test@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            address: Some("123 Test St".to_string()),
            city: Some("Test City".to_string()),
            state: Some("TS".to_string()),
            postal_code: Some("12345".to_string()),
            country: Some("US".to_string()),
            tax_id: None,
        };
        
        let result = api_service.create_customer(customer).await;
        assert!(result.is_ok());
        
        let customer = result.unwrap();
        assert_eq!(customer.name, "Test Customer");
    }

    #[tokio::test]
    async fn test_invoice_lifecycle() {
        let state = InvoiceAppState::new();
        let api_service = state.api_service.write().await;
        
        // Create a customer
        let customer = api::CreateCustomerRequest {
            name: "Test Customer".to_string(),
            email: Some("test@example.com".to_string()),
            ..Default::default()
        };
        
        let customer = api_service.create_customer(customer).await.unwrap();
        
        // Create an invoice
        let invoice = api::CreateInvoiceRequest {
            customer_id: customer.id,
            invoice_date: chrono::Utc::now(),
            due_date: chrono::Utc::now() + chrono::Duration::days(30),
            line_items: vec![
                api::CreateLineItemRequest {
                    description: "Test Service".to_string(),
                    quantity: 1.0,
                    unit_price: Money::from_decimal(100, 0),
                    tax_rate: Some(0.1),
                }
            ],
            notes: None,
            terms: None,
        };
        
        let result = api_service.create_invoice(invoice).await;
        assert!(result.is_ok());
        
        let invoice_response = result.unwrap();
        assert_eq!(invoice_response.invoice.total, Money::from_decimal(110, 0));
    }
}

// Default implementations for request structs
impl Default for api::CreateCustomerRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: None,
            phone: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
            tax_id: None,
        }
    }
}

impl Default for api::CreateInvoiceRequest {
    fn default() -> Self {
        Self {
            customer_id: Uuid::nil(),
            invoice_date: chrono::Utc::now(),
            due_date: chrono::Utc::now() + chrono::Duration::days(30),
            line_items: Vec::new(),
            notes: None,
            terms: None,
        }
    }
}