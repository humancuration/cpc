use cpc_core::invoicing::model::{Invoice, Customer, InvoiceStatus};
use cpc_core::invoicing::repository::{InvoiceRepository, CustomerRepository};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::invoicing::repository::RepositoryFactory;
use crate::invoicing::pdf_generator::PdfGenerator;
use crate::invoicing::sync::{InvoiceSyncService, LastWriterWinsResolver, SyncResult, SyncState};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] cpc_core::invoicing::repository::RepositoryError),
    
    #[error("PDF generation error: {0}")]
    PdfGenerationError(#[from] crate::invoicing::pdf_generator::PdfGenerationError),
    
    #[error("Sync error: {0}")]
    SyncError(#[from] crate::invoicing::sync::SyncError),
    
    #[error("Invalid input: {0}")]
    ValidationError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl From<ApiError> for tauri::Error {
    fn from(error: ApiError) -> Self {
        tauri::Error::FailedToInvokeApi(error.to_string())
    }
}

// Request/Response DTOs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub customer_id: Uuid,
    pub invoice_date: chrono::DateTime<chrono::Utc>,
    pub due_date: chrono::DateTime<chrono::Utc>,
    pub line_items: Vec<CreateLineItemRequest>,
    pub notes: Option<String>,
    pub terms: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLineItemRequest {
    pub description: String,
    pub quantity: f64,
    pub unit_price: cpc_core::invoicing::money::Money,
    pub tax_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInvoiceRequest {
    pub id: Uuid,
    pub status: Option<InvoiceStatus>,
    pub notes: Option<String>,
    pub terms: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceResponse {
    pub invoice: Invoice,
    pub customer: Customer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceListResponse {
    pub invoices: Vec<InvoiceResponse>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerListResponse {
    pub customers: Vec<Customer>,
    pub total_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePdfRequest {
    pub invoice_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratePdfResponse {
    pub pdf_url: String,
    pub file_path: String,
}

// API service
pub struct InvoiceApiService {
    repository_factory: RepositoryFactory,
    pdf_generator: PdfGenerator,
    sync_service: InvoiceSyncService,
}

impl InvoiceApiService {
    pub fn new(repository_factory: RepositoryFactory) -> Self {
        let pdf_generator = PdfGenerator::new();
        let invoice_repo = repository_factory.invoice_repository().clone();
        let customer_repo = repository_factory.customer_repository().clone();
        let sync_repo = repository_factory.sync_repository().clone();
        let conflict_resolver = LastWriterWinsResolver;
        
        let sync_service = InvoiceSyncService::new(
            invoice_repo.clone(),
            customer_repo.clone(),
            sync_repo.clone(),
            conflict_resolver,
        );
        
        Self {
            repository_factory,
            pdf_generator,
            sync_service,
        }
    }

    // Invoice operations
    pub async fn create_invoice(&self, request: CreateInvoiceRequest) -> Result<InvoiceResponse, ApiError> {
        let customer = self.repository_factory
            .customer_repository()
            .get_by_id(request.customer_id)
            .await?
            .ok_or_else(|| ApiError::ValidationError("Customer not found".to_string()))?;

        let mut invoice = Invoice::new(
            request.customer_id,
            request.invoice_date,
            request.due_date,
        );

        // Add line items
        for item in request.line_items {
            invoice.add_line_item(
                item.description,
                item.quantity,
                item.unit_price,
                item.tax_rate,
            );
        }

        invoice.notes = request.notes;
        invoice.terms = request.terms;

        let invoice = self.repository_factory
            .invoice_repository()
            .create(invoice)
            .await?;

        Ok(InvoiceResponse { invoice, customer })
    }

    pub async fn get_invoice(&self, id: Uuid) -> Result<Option<InvoiceResponse>, ApiError> {
        let invoice = self.repository_factory
            .invoice_repository()
            .get_by_id(id)
            .await?;

        match invoice {
            Some(invoice) => {
                let customer = self.repository_factory
                    .customer_repository()
                    .get_by_id(invoice.customer_id)
                    .await?
                    .ok_or_else(|| {
                        ApiError::ValidationError(format!("Customer {} not found", invoice.customer_id))
                    })?;

                Ok(Some(InvoiceResponse { invoice, customer }))
            }
            None => Ok(None),
        }
    }

    pub async fn update_invoice(&self, request: UpdateInvoiceRequest) -> Result<InvoiceResponse, ApiError> {
        let mut invoice = self.repository_factory
            .invoice_repository()
            .get_by_id(request.id)
            .await?
            .ok_or_else(|| ApiError::ValidationError("Invoice not found".to_string()))?;

        if let Some(status) = request.status {
            invoice.status = status;
        }
        
        if let Some(notes) = request.notes {
            invoice.notes = Some(notes);
        }
        
        if let Some(terms) = request.terms {
            invoice.terms = Some(terms);
        }

        invoice.updated_at = chrono::Utc::now();
        invoice.increment_version();

        let invoice = self.repository_factory
            .invoice_repository()
            .update(invoice)
            .await?;

        let customer = self.repository_factory
            .customer_repository()
            .get_by_id(invoice.customer_id)
            .await?
            .ok_or_else(|| {
                ApiError::ValidationError(format!("Customer {} not found", invoice.customer_id))
            })?;

        Ok(InvoiceResponse { invoice, customer })
    }

    pub async fn delete_invoice(&self, id: Uuid) -> Result<(), ApiError> {
        self.repository_factory
            .invoice_repository()
            .delete(id)
            .await?;
        Ok(())
    }

    pub async fn list_invoices(
        &self,
        status: Option<InvoiceStatus>,
        customer_id: Option<Uuid>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<InvoiceListResponse, ApiError> {
        let invoices = match (status, customer_id) {
            (Some(status), Some(customer_id)) => {
                let mut invoices = self.repository_factory
                    .invoice_repository()
                    .list_by_customer(customer_id)
                    .await?;
                invoices.retain(|i| i.status == status);
                invoices
            }
            (Some(status), None) => {
                self.repository_factory
                    .invoice_repository()
                    .list_by_status(status)
                    .await?
            }
            (None, Some(customer_id)) => {
                self.repository_factory
                    .invoice_repository()
                    .list_by_customer(customer_id)
                    .await?
            }
            (None, None) => {
                self.repository_factory
                    .invoice_repository()
                    .list()
                    .await?
            }
        };

        let total_count = invoices.len();
        
        // Apply pagination
        let start = offset.unwrap_or(0);
        let end = start + limit.unwrap_or(usize::MAX);
        let invoices = invoices.into_iter().skip(start).take(end - start).collect::<Vec<_>>();

        // Get customers for each invoice
        let mut invoice_responses = Vec::new();
        for invoice in invoices {
            let customer = self.repository_factory
                .customer_repository()
                .get_by_id(invoice.customer_id)
                .await?
                .ok_or_else(|| {
                    ApiError::ValidationError(format!("Customer {} not found", invoice.customer_id))
                })?;

            invoice_responses.push(InvoiceResponse { invoice, customer });
        }

        Ok(InvoiceListResponse {
            invoices: invoice_responses,
            total_count,
        })
    }

    pub async fn search_invoices(&self, query: &str) -> Result<InvoiceListResponse, ApiError> {
        let invoices = self.repository_factory
            .invoice_repository()
            .search(query)
            .await?;

        let total_count = invoices.len();
        let mut invoice_responses = Vec::new();
        
        for invoice in invoices {
            let customer = self.repository_factory
                .customer_repository()
                .get_by_id(invoice.customer_id)
                .await?
                .ok_or_else(|| {
                    ApiError::ValidationError(format!("Customer {} not found", invoice.customer_id))
                })?;

            invoice_responses.push(InvoiceResponse { invoice, customer });
        }

        Ok(InvoiceListResponse {
            invoices: invoice_responses,
            total_count,
        })
    }

    // Customer operations
    pub async fn create_customer(&self, request: CreateCustomerRequest) -> Result<Customer, ApiError> {
        let customer = Customer::new(
            request.name,
            request.email,
            request.phone,
            request.address,
            request.city,
            request.state,
            request.postal_code,
            request.country,
            request.tax_id,
        );

        self.repository_factory
            .customer_repository()
            .create(customer)
            .await
            .map_err(Into::into)
    }

    pub async fn get_customer(&self, id: Uuid) -> Result<Option<Customer>, ApiError> {
        self.repository_factory
            .customer_repository()
            .get_by_id(id)
            .await
            .map_err(Into::into)
    }

    pub async fn update_customer(&self, request: UpdateCustomerRequest) -> Result<Customer, ApiError> {
        let mut customer = self.repository_factory
            .customer_repository()
            .get_by_id(request.id)
            .await?
            .ok_or_else(|| ApiError::ValidationError("Customer not found".to_string()))?;

        if let Some(name) = request.name {
            customer.name = name;
        }
        if let Some(email) = request.email {
            customer.email = Some(email);
        }
        if let Some(phone) = request.phone {
            customer.phone = Some(phone);
        }
        if let Some(address) = request.address {
            customer.address = Some(address);
        }
        if let Some(city) = request.city {
            customer.city = Some(city);
        }
        if let Some(state) = request.state {
            customer.state = Some(state);
        }
        if let Some(postal_code) = request.postal_code {
            customer.postal_code = Some(postal_code);
        }
        if let Some(country) = request.country {
            customer.country = Some(country);
        }
        if let Some(tax_id) = request.tax_id {
            customer.tax_id = Some(tax_id);
        }

        customer.updated_at = chrono::Utc::now();

        self.repository_factory
            .customer_repository()
            .update(customer)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_customer(&self, id: Uuid) -> Result<(), ApiError> {
        // Check if customer has invoices
        let invoices = self.repository_factory
            .invoice_repository()
            .list_by_customer(id)
            .await?;
        
        if !invoices.is_empty() {
            return Err(ApiError::ValidationError(
                "Cannot delete customer with existing invoices".to_string()
            ));
        }

        self.repository_factory
            .customer_repository()
            .delete(id)
            .await
            .map_err(Into::into)
    }

    pub async fn list_customers(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<CustomerListResponse, ApiError> {
        let customers = self.repository_factory
            .customer_repository()
            .list()
            .await?;

        let total_count = customers.len();
        
        // Apply pagination
        let start = offset.unwrap_or(0);
        let end = start + limit.unwrap_or(usize::MAX);
        let customers = customers.into_iter().skip(start).take(end - start).collect::<Vec<_>>();

        Ok(CustomerListResponse {
            customers,
            total_count,
        })
    }

    pub async fn search_customers(&self, query: &str) -> Result<CustomerListResponse, ApiError> {
        let customers = self.repository_factory
            .customer_repository()
            .search(query)
            .await?;

        Ok(CustomerListResponse {
            customers,
            total_count: customers.len(),
        })
    }

    // PDF operations
    pub async fn generate_pdf(&self, invoice_id: Uuid) -> Result<GeneratePdfResponse, ApiError> {
        let invoice = self.repository_factory
            .invoice_repository()
            .get_by_id(invoice_id)
            .await?
            .ok_or_else(|| ApiError::ValidationError("Invoice not found".to_string()))?;

        let pdf_path = self.pdf_generator.generate(&invoice).await?;
        
        Ok(GeneratePdfResponse {
            pdf_url: format!("file://{}", pdf_path),
            file_path: pdf_path,
        })
    }

    // Sync operations
    pub async fn sync_invoices(&self) -> Result<SyncResult, ApiError> {
        self.sync_service.sync().await.map_err(Into::into)
    }

    pub async fn get_sync_state(&self) -> Result<SyncState, ApiError> {
        self.sync_service.get_sync_state().await.map_err(Into::into)
    }
}

// Tauri command handlers
#[tauri::command]
pub async fn create_invoice(
    request: CreateInvoiceRequest,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<InvoiceResponse, String> {
    state.create_invoice(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_invoice(
    id: Uuid,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<Option<InvoiceResponse>, String> {
    state.get_invoice(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_invoice(
    request: UpdateInvoiceRequest,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<InvoiceResponse, String> {
    state.update_invoice(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_invoice(
    id: Uuid,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<(), String> {
    state.delete_invoice(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_invoices(
    status: Option<InvoiceStatus>,
    customer_id: Option<Uuid>,
    limit: Option<usize>,
    offset: Option<usize>,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<InvoiceListResponse, String> {
    state.list_invoices(status, customer_id, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_invoices(
    query: String,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<InvoiceListResponse, String> {
    state.search_invoices(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn create_customer(
    request: CreateCustomerRequest,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<Customer, String> {
    state.create_customer(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_customer(
    id: Uuid,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<Option<Customer>, String> {
    state.get_customer(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_customer(
    request: UpdateCustomerRequest,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<Customer, String> {
    state.update_customer(request).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_customer(
    id: Uuid,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<(), String> {
    state.delete_customer(id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_customers(
    limit: Option<usize>,
    offset: Option<usize>,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<CustomerListResponse, String> {
    state.list_customers(limit, offset).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_customers(
    query: String,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<CustomerListResponse, String> {
    state.search_customers(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn generate_pdf(
    request: GeneratePdfRequest,
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<GeneratePdfResponse, String> {
    state.generate_pdf(request.invoice_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sync_invoices(
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<SyncResult, String> {
    state.sync_invoices().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_sync_state(
    state: tauri::State<'_, InvoiceApiService>,
) -> Result<SyncState, String> {
    state.get_sync_state().await.map_err(|e| e.to_string())
}