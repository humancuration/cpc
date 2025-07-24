use crate::invoicing::model::{Invoice, Customer, InvoiceFilter};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),
    
    #[error("Foreign key constraint violation: {0}")]
    ForeignKeyViolation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Version conflict: expected version {expected}, found {found}")]
    VersionConflict {
        expected: u64,
        found: u64,
    },
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

#[async_trait]
pub trait InvoiceRepository: Send + Sync {
    /// Create a new invoice
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError>;
    
    /// Get an invoice by ID
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Invoice>, RepositoryError>;
    
    /// Get an invoice by number
    async fn get_by_number(&self, number: &str) -> Result<Option<Invoice>, RepositoryError>;
    
    /// Update an existing invoice
    async fn update(&self, invoice: Invoice) -> Result<Invoice, RepositoryError>;
    
    /// Delete an invoice
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    
    /// List invoices with optional filtering
    async fn list(&self, filter: Option<InvoiceFilter>) -> Result<Vec<Invoice>, RepositoryError>;
    
    /// Get invoices by customer ID
    async fn get_by_customer(&self, customer_id: Uuid) -> Result<Vec<Invoice>, RepositoryError>;
    
    /// Get invoices with pending sync
    async fn get_pending_sync(&self, last_sync_time: DateTime<Utc>) -> Result<Vec<Invoice>, RepositoryError>;
    
    /// Update sync version
    async fn update_sync_version(&self, id: Uuid, version: u64) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    /// Create a new customer
    async fn create(&self, customer: Customer) -> Result<Customer, RepositoryError>;
    
    /// Get a customer by ID
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Customer>, RepositoryError>;
    
    /// Get a customer by email
    async fn get_by_email(&self, email: &str) -> Result<Option<Customer>, RepositoryError>;
    
    /// Update an existing customer
    async fn update(&self, customer: Customer) -> Result<Customer, RepositoryError>;
    
    /// Delete a customer
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    
    /// List all customers
    async fn list(&self) -> Result<Vec<Customer>, RepositoryError>;
    
    /// Search customers by name or email
    async fn search(&self, query: &str) -> Result<Vec<Customer>, RepositoryError>;
    
    /// Get customer statistics
    async fn get_statistics(&self, customer_id: Uuid) -> Result<CustomerStats, RepositoryError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerStats {
    pub customer_id: Uuid,
    pub total_invoices: u64,
    pub total_invoiced_amount: crate::accounting::money::Money,
    pub total_paid_amount: crate::accounting::money::Money,
    pub outstanding_balance: crate::accounting::money::Money,
    pub last_invoice_date: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait InvoiceSyncRepository: Send + Sync {
    /// Get changes since last sync
    async fn get_changes_since(
        &self,
        last_sync_time: DateTime<Utc>,
    ) -> Result<SyncChanges, RepositoryError>;
    
    /// Apply sync changes
    async fn apply_changes(
        &self,
        changes: SyncChanges,
    ) -> Result<(), RepositoryError>;
    
    /// Resolve conflicts
    async fn resolve_conflicts(
        &self,
        conflicts: Vec<SyncConflict>,
    ) -> Result<(), RepositoryError>;
    
    /// Get current sync timestamp
    async fn get_last_sync_time(&self) -> Result<DateTime<Utc>, RepositoryError>;
    
    /// Update sync timestamp
    async fn update_sync_time(&self, sync_time: DateTime<Utc>) -> Result<(), RepositoryError>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncChanges {
    pub invoices: Vec<InvoiceChange>,
    pub customers: Vec<CustomerChange>,
    pub sync_timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceChange {
    pub invoice_id: Uuid,
    pub change_type: ChangeType,
    pub invoice: Invoice,
    pub previous_version: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerChange {
    pub customer_id: Uuid,
    pub change_type: ChangeType,
    pub customer: Customer,
    pub previous_version: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncConflict {
    pub entity_id: Uuid,
    pub entity_type: EntityType,
    pub local_version: u64,
    pub remote_version: u64,
    pub local_data: Vec<u8>,
    pub remote_data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EntityType {
    Invoice,
    Customer,
}

#[derive(Debug, Clone)]
pub struct RepositoryOptions {
    pub encryption_key: Option<Vec<u8>>,
    pub compression_enabled: bool,
    pub max_sync_batch_size: usize,
}

impl Default for RepositoryOptions {
    fn default() -> Self {
        Self {
            encryption_key: None,
            compression_enabled: true,
            max_sync_batch_size: 100,
        }
    }
}

pub trait RepositoryFactory {
    fn create_invoice_repository(&self, options: RepositoryOptions) -> Box<dyn InvoiceRepository>;
    fn create_customer_repository(&self, options: RepositoryOptions) -> Box<dyn CustomerRepository>;
    fn create_sync_repository(&self, options: RepositoryOptions) -> Box<dyn InvoiceSyncRepository>;
}