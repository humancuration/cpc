use cpc_core::invoicing::model::{Invoice, Customer, InvoiceStatus, LineItem, Address};
use cpc_core::invoicing::repository::{InvoiceRepository, CustomerRepository, RepositoryError};
use cpc_core::accounting::money::{Money, Currency};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, QueryOrder, Condition};
use sea_orm::ActiveValue::{Set, NotSet};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::str::FromStr;

// SeaORM entities would be defined here
// For now, we'll use a simplified in-memory implementation for demonstration

pub struct InMemoryInvoiceRepository {
    invoices: std::sync::RwLock<std::collections::HashMap<Uuid, Invoice>>,
}

impl InMemoryInvoiceRepository {
    pub fn new() -> Self {
        Self {
            invoices: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl InvoiceRepository for InMemoryInvoiceRepository {
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError> {
        let mut invoices = self.invoices.write().unwrap();
        invoices.insert(invoice.id, invoice.clone());
        Ok(invoice)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().unwrap();
        Ok(invoices.get(&id).cloned())
    }

    async fn get_by_number(&self, number: &str) -> Result<Option<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().unwrap();
        Ok(invoices.values()
            .find(|inv| inv.number == number)
            .cloned())
    }

    async fn update(&self, invoice: Invoice) -> Result<Invoice, RepositoryError> {
        let mut invoices = self.invoices.write().unwrap();
        invoices.insert(invoice.id, invoice.clone());
        Ok(invoice)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let mut invoices = self.invoices.write().unwrap();
        invoices.remove(&id);
        Ok(())
    }

    async fn list(&self, filter: Option<cpc_core::invoicing::model::InvoiceFilter>) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().unwrap();
        let mut result: Vec<Invoice> = invoices.values().cloned().collect();
        
        if let Some(filter) = filter {
            result.retain(|invoice| {
                if let Some(customer_id) = filter.customer_id {
                    if invoice.customer_id != customer_id {
                        return false;
                    }
                }
                
                if let Some(status) = &filter.status {
                    if &invoice.status != status {
                        return false;
                    }
                }
                
                if let Some(date_from) = filter.date_from {
                    if invoice.issue_date < date_from {
                        return false;
                    }
                }
                
                if let Some(date_to) = filter.date_to {
                    if invoice.issue_date > date_to {
                        return false;
                    }
                }
                
                if let Some(search_term) = &filter.search_term {
                    if !invoice.number.contains(search_term) && 
                       !invoice.notes.as_ref().unwrap_or(&String::new()).contains(search_term) {
                        return false;
                    }
                }
                
                true
            });
        }
        
        Ok(result)
    }

    async fn get_by_customer(&self, customer_id: Uuid) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().unwrap();
        Ok(invoices.values()
            .filter(|inv| inv.customer_id == customer_id)
            .cloned()
            .collect())
    }

    async fn get_pending_sync(&self, last_sync_time: DateTime<Utc>) -> Result<Vec<Invoice>, RepositoryError> {
        let invoices = self.invoices.read().unwrap();
        Ok(invoices.values()
            .filter(|inv| inv.updated_at > last_sync_time)
            .cloned()
            .collect())
    }

    async fn update_sync_version(&self, id: Uuid, version: u64) -> Result<(), RepositoryError> {
        let mut invoices = self.invoices.write().unwrap();
        if let Some(invoice) = invoices.get_mut(&id) {
            invoice.sync_version = version;
        }
        Ok(())
    }
}

pub struct InMemoryCustomerRepository {
    customers: std::sync::RwLock<std::collections::HashMap<Uuid, Customer>>,
}

impl InMemoryCustomerRepository {
    pub fn new() -> Self {
        Self {
            customers: std::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl CustomerRepository for InMemoryCustomerRepository {
    async fn create(&self, customer: Customer) -> Result<Customer, RepositoryError> {
        let mut customers = self.customers.write().unwrap();
        customers.insert(customer.id, customer.clone());
        Ok(customer)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<Customer>, RepositoryError> {
        let customers = self.customers.read().unwrap();
        Ok(customers.get(&id).cloned())
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<Customer>, RepositoryError> {
        let customers = self.customers.read().unwrap();
        Ok(customers.values()
            .find(|cust| cust.email.as_ref() == Some(&email.to_string()))
            .cloned())
    }

    async fn update(&self, customer: Customer) -> Result<Customer, RepositoryError> {
        let mut customers = self.customers.write().unwrap();
        customers.insert(customer.id, customer.clone());
        Ok(customer)
    }

    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError> {
        let mut customers = self.customers.write().unwrap();
        customers.remove(&id);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Customer>, RepositoryError> {
        let customers = self.customers.read().unwrap();
        Ok(customers.values().cloned().collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Customer>, RepositoryError> {
        let customers = self.customers.read().unwrap();
        Ok(customers.values()
            .filter(|cust| {
                cust.name.contains(query) || 
                cust.email.as_ref().map_or(false, |email| email.contains(query))
            })
            .cloned()
            .collect())
    }

    async fn get_statistics(&self, customer_id: Uuid) -> Result<cpc_core::invoicing::repository::CustomerStats, RepositoryError> {
        // This would be implemented with a proper database query
        // For now, return empty stats
        Ok(cpc_core::invoicing::repository::CustomerStats {
            customer_id,
            total_invoices: 0,
            total_invoiced_amount: cpc_core::accounting::money::Money::zero(),
            total_paid_amount: cpc_core::accounting::money::Money::zero(),
            outstanding_balance: cpc_core::accounting::money::Money::zero(),
            last_invoice_date: None,
        })
    }
}

// This function will be used to migrate to SeaORM when ready
pub async fn create_seaorm_repositories(_db: DatabaseConnection) -> (
    impl InvoiceRepository,
    impl CustomerRepository,
) {
    // Return in-memory implementations for now
    (
        InMemoryInvoiceRepository::new(),
        InMemoryCustomerRepository::new(),
    )
}