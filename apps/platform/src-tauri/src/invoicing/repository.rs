use cpc_core::invoicing::model::{Invoice, Customer, InvoiceStatus};
use cpc_core::invoicing::repository::{
    InvoiceRepository, CustomerRepository, InvoiceSyncRepository,
    RepositoryError, SyncChanges, SyncConflict, InvoiceChange, CustomerChange
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// In-memory repositories for development/testing
// These will be replaced with SeaORM implementations

#[derive(Debug, Clone)]
pub struct InMemoryInvoiceRepository {
    invoices: Arc<RwLock<HashMap<Uuid, Invoice>>>,
}

impl InMemoryInvoiceRepository {
    pub fn new() -> Self {
        Self {
            invoices: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl InvoiceRepository for InMemoryInvoiceRepository {
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError> {
        let mut invoices = self.invoices.write().await;
        
        if invoices.contains_key(&invoice.id) {
            return Err(RepositoryError::DuplicateKey(invoice.id.to_string()));
        }
        
        invoices.insert(invoice.id, invoice.clone());
        Ok(invoice)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        Ok(invoices.get(&id).cloned())
    }

    async fn update(&self, invoice: Invoice) -> Result<Invoice, RepositoryError> {
        let mut invoices = self.invoices.write().await;
        
        if !invoices.contains_key(&invoice.id) {
            return Err(RepositoryError::NotFound(invoice.id.to_string()));
        }
        
        invoices.insert(invoice.id, invoice.clone());
        Ok(invoice)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let mut invoices = self.invoices.write().await;
        
        if invoices.remove(&id).is_none() {
            return Err(RepositoryError::NotFound(id.to_string()));
        }
        
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        Ok(invoices.values().cloned().collect())
    }

    async fn list_by_customer(&self, customer_id: Uuid) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        Ok(invoices
            .values()
            .filter(|invoice| invoice.customer_id == customer_id)
            .cloned()
            .collect())
    }

    async fn list_by_status(&self, status: InvoiceStatus) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        Ok(invoices
            .values()
            .filter(|invoice| invoice.status == status)
            .cloned()
            .collect())
    }

    async fn list_by_date_range(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        Ok(invoices
            .values()
            .filter(|invoice| {
                invoice.invoice_date >= start_date && invoice.invoice_date <= end_date
            })
            .cloned()
            .collect())
    }

    async fn get_pending_sync(
        &self,
        since: DateTime<Utc>,
    ) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        Ok(invoices
            .values()
            .filter(|invoice| invoice.updated_at > since)
            .cloned()
            .collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().await;
        let query = query.to_lowercase();
        
        Ok(invoices
            .values()
            .filter(|invoice| {
                invoice.number.to_lowercase().contains(&query)
                    || invoice
                        .line_items
                        .iter()
                        .any(|item| item.description.to_lowercase().contains(&query))
            })
            .cloned()
            .collect())
    }
}

#[derive(Debug, Clone)]
pub struct InMemoryCustomerRepository {
    customers: Arc<RwLock<HashMap<Uuid, Customer>>>,
}

impl InMemoryCustomerRepository {
    pub fn new() -> Self {
        Self {
            customers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl CustomerRepository for InMemoryCustomerRepository {
    async fn create(&self, customer: Customer) -> Result<Customer, RepositoryError> {
        let mut customers = self.customers.write().await;
        
        if customers.contains_key(&customer.id) {
            return Err(RepositoryError::DuplicateKey(customer.id.to_string()));
        }
        
        customers.insert(customer.id, customer.clone());
        Ok(customer)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Customer>, RepositoryError> {
        let customers = self.customers.read().await;
        Ok(customers.get(&id).cloned())
    }

    async fn update(&self, customer: Customer) -> Result<Customer, RepositoryError> {
        let mut customers = self.customers.write().await;
        
        if !customers.contains_key(&customer.id) {
            return Err(RepositoryError::NotFound(customer.id.to_string()));
        }
        
        customers.insert(customer.id, customer.clone());
        Ok(customer)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let mut customers = self.customers.write().await;
        
        if customers.remove(&id).is_none() {
            return Err(RepositoryError::NotFound(id.to_string()));
        }
        
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Customer>, RepositoryError> {
        let customers = self.customers.read().await;
        Ok(customers.values().cloned().collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Customer>, RepositoryError> {
        let customers = self.customers.read().await;
        let query = query.to_lowercase();
        
        Ok(customers
            .values()
            .filter(|customer| {
                customer.name.to_lowercase().contains(&query)
                    || customer.email.as_ref().map_or(false, |email| email.to_lowercase().contains(&query))
                    || customer.phone.as_ref().map_or(false, |phone| phone.to_lowercase().contains(&query))
            })
            .cloned()
            .collect())
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<Customer>, RepositoryError> {
        let customers = self.customers.read().await;
        Ok(customers
            .values()
            .find(|customer| customer.email.as_ref() == Some(&email.to_string()))
            .cloned())
    }
}

#[derive(Debug, Clone)]
pub struct InMemorySyncRepository {
    sync_time: Arc<RwLock<DateTime<Utc>>>,
    changes: Arc<RwLock<Vec<InvoiceChange>>>,
}

impl InMemorySyncRepository {
    pub fn new() -> Self {
        Self {
            sync_time: Arc::new(RwLock::new(Utc::now() - chrono::Duration::days(30))),
            changes: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl InvoiceSyncRepository for InMemorySyncRepository {
    async fn get_changes_since(&self, since: DateTime<Utc>) -> Result<SyncChanges, RepositoryError> {
        let changes = self.changes.read().await;
        
        let invoice_changes: Vec<InvoiceChange> = changes
            .iter()
            .filter(|change| change.timestamp > since)
            .cloned()
            .collect();
        
        Ok(SyncChanges {
            invoices: invoice_changes,
            customers: Vec::new(), // TODO: Implement customer sync
            sync_timestamp: Utc::now(),
        })
    }

    async fn apply_changes(&self, changes: SyncChanges) -> Result<(), RepositoryError> {
        let mut current_changes = self.changes.write().await;
        current_changes.extend(changes.invoices);
        Ok(())
    }

    async fn resolve_conflicts(&self, conflicts: Vec<SyncConflict>) -> Result<(), RepositoryError> {
        // In-memory implementation just logs conflicts
        tracing::info!("Resolving {} conflicts", conflicts.len());
        Ok(())
    }

    async fn get_last_sync_time(&self) -> Result<DateTime<Utc>, RepositoryError> {
        let sync_time = self.sync_time.read().await;
        Ok(*sync_time)
    }

    async fn update_sync_time(&self, sync_time: DateTime<Utc>) -> Result<(), RepositoryError> {
        let mut current_time = self.sync_time.write().await;
        *current_time = sync_time;
        Ok(())
    }

    async fn record_change(
        &self,
        invoice_id: Uuid,
        change_type: cpc_core::invoicing::repository::ChangeType,
        invoice_data: &Invoice,
    ) -> Result<(), RepositoryError> {
        let mut changes = self.changes.write().await;
        
        changes.push(InvoiceChange {
            invoice_id,
            change_type,
            invoice: invoice_data.clone(),
            timestamp: Utc::now(),
            previous_version: Some(invoice_data.sync_version),
        });
        
        Ok(())
    }
}

// Repository factory for dependency injection
#[derive(Debug, Clone)]
pub struct RepositoryFactory {
    invoice_repo: InMemoryInvoiceRepository,
    customer_repo: InMemoryCustomerRepository,
    sync_repo: InMemorySyncRepository,
}

impl RepositoryFactory {
    pub fn new() -> Self {
        Self {
            invoice_repo: InMemoryInvoiceRepository::new(),
            customer_repo: InMemoryCustomerRepository::new(),
            sync_repo: InMemorySyncRepository::new(),
        }
    }

    pub fn invoice_repository(&self) -> &InMemoryInvoiceRepository {
        &self.invoice_repo
    }

    pub fn customer_repository(&self) -> &InMemoryCustomerRepository {
        &self.customer_repo
    }

    pub fn sync_repository(&self) -> &InMemorySyncRepository {
        &self.sync_repo
    }
}

// Helper functions for testing
pub mod test_helpers {
    use super::*;
    use cpc_core::invoicing::model::{Invoice, Customer, InvoiceStatus, LineItem};
    use cpc_core::invoicing::money::Money;
    use uuid::Uuid;

    pub fn create_test_customer() -> Customer {
        Customer {
            id: Uuid::new_v4(),
            name: "Test Customer".to_string(),
            email: Some("test@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            address: Some("123 Test St".to_string()),
            city: Some("Test City".to_string()),
            state: Some("TS".to_string()),
            postal_code: Some("12345".to_string()),
            country: Some("US".to_string()),
            tax_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn create_test_invoice(customer_id: Uuid) -> Invoice {
        Invoice {
            id: Uuid::new_v4(),
            number: "INV-001".to_string(),
            customer_id,
            invoice_date: Utc::now(),
            due_date: Utc::now() + chrono::Duration::days(30),
            status: InvoiceStatus::Draft,
            line_items: vec![
                LineItem {
                    id: Uuid::new_v4(),
                    description: "Test Service".to_string(),
                    quantity: 1.0,
                    unit_price: Money::from_decimal(100, 0),
                    tax_rate: Some(0.1),
                }
            ],
            subtotal: Money::from_decimal(100, 0),
            tax_amount: Money::from_decimal(10, 0),
            total: Money::from_decimal(110, 0),
            notes: None,
            terms: None,
            paid_amount: Money::from_decimal(0, 0),
            sync_version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}