use sqlx::{PgPool, Result};
use super::models::{Customer, Invoice, InvoiceLineItem, Payment, InvoiceStatus};
use crate::db::DbPool;
use async_trait::async_trait;
use chrono::{NaiveDate};
use sqlx::types::BigDecimal;

#[async_trait]
pub trait InvoiceRepository {
    async fn get_customer_by_id(&self, customer_id: i64) -> Result<Option<Customer>>;
    async fn create_customer(&self, org_id: i64, name: &str, email: Option<&str>, address: Option<&str>) -> Result<Customer>;
    async fn get_invoice_by_id(&self, invoice_id: i64) -> Result<Option<Invoice>>;
    // A more complex create_invoice might take a struct, but this is fine for now
    async fn create_invoice(&self, org_id: i64, customer_id: i64, status: InvoiceStatus, currency: &str, issue_date: NaiveDate, due_date: NaiveDate, notes: Option<&str>, line_items: Vec<InvoiceLineItem>) -> Result<Invoice>;
}

pub struct PgInvoiceRepository {
    pool: DbPool,
}

impl PgInvoiceRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvoiceRepository for PgInvoiceRepository {
    async fn get_customer_by_id(&self, customer_id: i64) -> Result<Option<Customer>> {
        sqlx::query_as!(
            Customer,
            "SELECT id, organization_id, name, email, address, created_at, updated_at FROM customers WHERE id = $1",
            customer_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_customer(&self, org_id: i64, name: &str, email: Option<&str>, address: Option<&str>) -> Result<Customer> {
        sqlx::query_as!(
            Customer,
            "INSERT INTO customers (organization_id, name, email, address) VALUES ($1, $2, $3, $4) RETURNING id, organization_id, name, email, address, created_at, updated_at",
            org_id,
            name,
            email,
            address
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn get_invoice_by_id(&self, invoice_id: i64) -> Result<Option<Invoice>> {
        sqlx::query_as!(
            Invoice,
            r#"SELECT id, organization_id, customer_id, invoice_number, status AS "status: _", currency, issue_date, due_date, notes, subtotal, tax_total, total, created_at, updated_at FROM invoices WHERE id = $1"#,
            invoice_id
        )
        .fetch_optional(&self.pool)
        .await
    }
    
    // This is a simplified example. A real implementation would handle transactions
    // for creating the invoice and its line items atomically.
    async fn create_invoice(
        &self,
        org_id: i64,
        customer_id: i64,
        status: InvoiceStatus,
        currency: &str,
        issue_date: NaiveDate,
        due_date: NaiveDate,
        notes: Option<&str>,
        line_items: Vec<InvoiceLineItem>
    ) -> Result<Invoice> {
        let mut tx = self.pool.begin().await?;

        // Simplified subtotal/total calculation
        let subtotal: BigDecimal = line_items.iter().map(|li| &li.total).sum();
        let tax_total = BigDecimal::from(0); // placeholder
        let total = &subtotal + &tax_total;
        
        // This is a placeholder for a real invoice number generator
        let invoice_number = format!("INV-{}", chrono::Utc::now().timestamp_millis());

        let invoice = sqlx::query_as!(
            Invoice,
            r#"
            INSERT INTO invoices (organization_id, customer_id, invoice_number, status, currency, issue_date, due_date, notes, subtotal, tax_total, total)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, organization_id, customer_id, invoice_number, status AS "status: _", currency, issue_date, due_date, notes, subtotal, tax_total, total, created_at, updated_at
            "#,
            org_id,
            customer_id,
            invoice_number,
            status as _, // Cast enum to string
            currency,
            issue_date,
            due_date,
            notes,
            subtotal,
            tax_total,
            total
        )
        .fetch_one(&mut *tx)
        .await?;

        for item in line_items {
             sqlx::query!(
                "INSERT INTO invoice_line_items (invoice_id, description, quantity, unit_price, total) VALUES ($1, $2, $3, $4, $5)",
                invoice.id,
                item.description,
                item.quantity,
                item.unit_price,
                item.total
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(invoice)
    }
}