//! Database repositories for invoicing and quoting

use crate::domain::{Invoice, Quote, PaymentStatus, QuoteStatus, InvoiceItem, QuoteItem};
use crate::application::{InvoiceRepository, QuoteRepository, RepositoryError, QuoteServiceError};
use crate::infrastructure::database::models::{InvoiceRecord, QuoteRecord};
use async_trait::async_trait;
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use serde_json;

pub struct PgInvoiceRepository {
    pool: PgPool,
}

impl PgInvoiceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    fn payment_status_to_string(status: &PaymentStatus) -> String {
        match status {
            PaymentStatus::Draft => "draft",
            PaymentStatus::Sent => "sent",
            PaymentStatus::Viewed => "viewed",
            PaymentStatus::Paid => "paid",
            PaymentStatus::Overdue => "overdue",
            PaymentStatus::Partial => "partial",
        }
    }
    
    fn string_to_payment_status(status: &str) -> PaymentStatus {
        match status {
            "draft" => PaymentStatus::Draft,
            "sent" => PaymentStatus::Sent,
            "viewed" => PaymentStatus::Viewed,
            "paid" => PaymentStatus::Paid,
            "overdue" => PaymentStatus::Overdue,
            "partial" => PaymentStatus::Partial,
            _ => PaymentStatus::Draft,
        }
    }
}

#[async_trait]
impl InvoiceRepository for PgInvoiceRepository {
    async fn create(&self, invoice: Invoice) -> Result<Invoice, RepositoryError> {
        let items_json = serde_json::to_value(&invoice.items)
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        let record = sqlx::query_as!(
            InvoiceRecord,
            r#"
            INSERT INTO invoices (id, client_id, client_name, client_email, items, total_amount, due_date, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, client_id, client_name, client_email, items, total_amount, due_date, status, created_at, updated_at
            "#,
            invoice.id,
            invoice.client_id,
            invoice.client_name,
            invoice.client_email,
            items_json,
            invoice.total_amount,
            invoice.due_date,
            Self::payment_status_to_string(&invoice.status),
            invoice.created_at,
            invoice.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(Invoice {
            id: record.id,
            client_id: record.client_id,
            client_name: record.client_name,
            client_email: record.client_email,
            items: serde_json::from_value(record.items)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?,
            total_amount: record.total_amount,
            due_date: record.due_date,
            status: Self::string_to_payment_status(&record.status),
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
    
    async fn update_status(&self, id: Uuid, status: PaymentStatus) -> Result<Invoice, RepositoryError> {
        let updated_at = Utc::now();
        let status_str = Self::payment_status_to_string(&status);
        
        let record = sqlx::query_as!(
            InvoiceRecord,
            r#"
            UPDATE invoices 
            SET status = $1, updated_at = $2
            WHERE id = $3
            RETURNING id, client_id, client_name, client_email, items, total_amount, due_date, status, created_at, updated_at
            "#,
            status_str,
            updated_at,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        
        Ok(Invoice {
            id: record.id,
            client_id: record.client_id,
            client_name: record.client_name,
            client_email: record.client_email,
            items: serde_json::from_value(record.items)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?,
            total_amount: record.total_amount,
            due_date: record.due_date,
            status: Self::string_to_payment_status(&record.status),
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Invoice, RepositoryError> {
        let record = sqlx::query_as!(
            InvoiceRecord,
            r#"
            SELECT id, client_id, client_name, client_email, items, total_amount, due_date, status, created_at, updated_at
            FROM invoices
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| RepositoryError::NotFound(id))?;
        
        Ok(Invoice {
            id: record.id,
            client_id: record.client_id,
            client_name: record.client_name,
            client_email: record.client_email,
            items: serde_json::from_value(record.items)
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?,
            total_amount: record.total_amount,
            due_date: record.due_date,
            status: Self::string_to_payment_status(&record.status),
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
}

pub struct PgQuoteRepository {
    pool: PgPool,
}

impl PgQuoteRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    fn quote_status_to_string(status: &QuoteStatus) -> String {
        match status {
            QuoteStatus::Draft => "draft",
            QuoteStatus::Sent => "sent",
            QuoteStatus::Accepted => "accepted",
            QuoteStatus::Rejected => "rejected",
            QuoteStatus::Expired => "expired",
        }
    }
    
    fn string_to_quote_status(status: &str) -> QuoteStatus {
        match status {
            "draft" => QuoteStatus::Draft,
            "sent" => QuoteStatus::Sent,
            "accepted" => QuoteStatus::Accepted,
            "rejected" => QuoteStatus::Rejected,
            "expired" => QuoteStatus::Expired,
            _ => QuoteStatus::Draft,
        }
    }
}

#[async_trait]
impl QuoteRepository for PgQuoteRepository {
    async fn create(&self, quote: Quote) -> Result<Quote, QuoteServiceError> {
        let items_json = serde_json::to_value(&quote.items)
            .map_err(|e| QuoteServiceError::RepositoryError(e.to_string()))?;
        
        let validity_period_days = quote.validity_period.num_days();
        
        let record = sqlx::query_as!(
            QuoteRecord,
            r#"
            INSERT INTO quotes (id, client_id, client_name, client_email, items, total_amount, validity_period_days, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, client_id, client_name, client_email, items, total_amount, validity_period_days, status, created_at, updated_at
            "#,
            quote.id,
            quote.client_id,
            quote.client_name,
            quote.client_email,
            items_json,
            quote.total_amount,
            validity_period_days,
            Self::quote_status_to_string(&quote.status),
            quote.created_at,
            quote.updated_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| QuoteServiceError::RepositoryError(e.to_string()))?;
        
        Ok(Quote {
            id: record.id,
            client_id: record.client_id,
            client_name: record.client_name,
            client_email: record.client_email,
            items: serde_json::from_value(record.items)
                .map_err(|e| QuoteServiceError::RepositoryError(e.to_string()))?,
            total_amount: record.total_amount,
            validity_period: chrono::Duration::days(record.validity_period_days),
            status: Self::string_to_quote_status(&record.status),
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Quote, QuoteServiceError> {
        let record = sqlx::query_as!(
            QuoteRecord,
            r#"
            SELECT id, client_id, client_name, client_email, items, total_amount, validity_period_days, status, created_at, updated_at
            FROM quotes
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| QuoteServiceError::RepositoryError(e.to_string()))?;
        
        Ok(Quote {
            id: record.id,
            client_id: record.client_id,
            client_name: record.client_name,
            client_email: record.client_email,
            items: serde_json::from_value(record.items)
                .map_err(|e| QuoteServiceError::RepositoryError(e.to_string()))?,
            total_amount: record.total_amount,
            validity_period: chrono::Duration::days(record.validity_period_days),
            status: Self::string_to_quote_status(&record.status),
            created_at: record.created_at,
            updated_at: record.updated_at,
        })
    }
    
    async fn accept(&self, id: Uuid) -> Result<Invoice, QuoteServiceError> {
        // This would typically involve updating the quote status and creating an invoice
        // For now, we'll just return an error as this should be handled at the service level
        Err(QuoteServiceError::RepositoryError("Accept operation should be handled at service level".to_string()))
    }
}