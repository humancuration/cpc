use async_graphql::{dataloader::DataLoader, async_trait::async_trait, *};
use chrono::NaiveDate;
use sqlx::types::BigDecimal;
use std::collections::HashMap;
use std::sync::Arc;
use futures_util::stream::{Stream, self};

use crate::db::DbPool;
use super::models::{Customer, Invoice, InvoiceLineItem, InvoiceStatus, Payment};
use super::repositories::{InvoiceRepository, PgInvoiceRepository};

// --- Input Objects ---

#[derive(InputObject)]
struct CreateCustomerInput {
    organization_id: ID,
    name: String,
    email: Option<String>,
    address: Option<String>,
}

#[derive(InputObject, Clone)]
struct CreateInvoiceLineItemInput {
    description: String,
    quantity: f64,
    unit_price: f64,
    tax_rate: Option<f64>,
}

#[derive(InputObject)]
struct CreateInvoiceInput {
    organization_id: ID,
    customer_id: ID,
    status: Option<InvoiceStatus>,
    currency: String,
    issue_date: String,
    due_date: String,
    notes: Option<String>,
    line_items: Vec<CreateInvoiceLineItemInput>,
}

// --- Dataloaders for related entities ---

pub struct CustomerLoader {
    pool: DbPool,
}

#[async_trait]
impl async_graphql::dataloader::Loader<i64> for CustomerLoader {
    type Value = Customer;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let customers = sqlx::query_as!(
            Customer,
            "SELECT id, organization_id, name, email, address, created_at, updated_at FROM customers WHERE id = ANY($1)",
            keys
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Arc::new)?;

        Ok(customers.into_iter().map(|c| (c.id, c)).collect())
    }
}

// --- GraphQL Query Root ---

#[derive(Default)]
pub struct InvoicingQuery;

#[Object]
impl InvoicingQuery {
    async fn customer(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Customer>> {
        let repo = PgInvoiceRepository::new(ctx.data_unchecked::<DbPool>().clone());
        let customer_id = id.parse::<i64>()?;
        Ok(repo.get_customer_by_id(customer_id).await?)
    }

    async fn invoice(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Invoice>> {
        let repo = PgInvoiceRepository::new(ctx.data_unchecked::<DbPool>().clone());
        let invoice_id = id.parse::<i64>()?;
        Ok(repo.get_invoice_by_id(invoice_id).await?)
    }
}

// --- GraphQL Mutation Root ---

#[derive(Default)]
pub struct InvoicingMutation;

#[Object]
impl InvoicingMutation {
    async fn create_customer(&self, ctx: &Context<'_>, input: CreateCustomerInput) -> Result<Customer> {
        let repo = PgInvoiceRepository::new(ctx.data_unchecked::<DbPool>().clone());
        let org_id = input.organization_id.parse::<i64>()?;
        
        repo.create_customer(
            org_id,
            &input.name,
            input.email.as_deref(),
            input.address.as_deref(),
        ).await.map_err(|e| e.into())
    }

    async fn create_invoice(&self, ctx: &Context<'_>, input: CreateInvoiceInput) -> Result<Invoice> {
        let repo = PgInvoiceRepository::new(ctx.data_unchecked::<DbPool>().clone());
        let org_id = input.organization_id.parse::<i64>()?;
        let customer_id = input.customer_id.parse::<i64>()?;
        let issue_date = NaiveDate::parse_from_str(&input.issue_date, "%Y-%m-%d")?;
        let due_date = NaiveDate::parse_from_str(&input.due_date, "%Y-%m-%d")?;
        
        let line_items = input.line_items.into_iter().map(|li| {
            // This is a dummy conversion. In a real scenario, you would create
            // a proper InvoiceLineItem entity. This is a simplification.
            let quantity = BigDecimal::from((li.quantity * 100.0).round() as i64) / 100;
            let unit_price = BigDecimal::from((li.unit_price * 100.0).round() as i64) / 100;
            let total = &quantity * &unit_price;
            super::models::InvoiceLineItem {
                id: 0, // Not yet created
                invoice_id: 0,
                description: li.description,
                quantity,
                unit_price,
                tax_rate: li.tax_rate.map(BigDecimal::from),
                total,
            }
        }).collect();

        repo.create_invoice(
            org_id,
            customer_id,
            input.status.unwrap_or(InvoiceStatus::Draft),
            &input.currency,
            issue_date,
            due_date,
            input.notes.as_deref(),
            line_items,
        )
        .await.map_err(|e| e.into())
    }
}

// --- GraphQL Subscription Root ---

#[derive(Default)]
pub struct InvoicingSubscription;

#[Subscription]
impl InvoicingSubscription {
    async fn invoice_updated(&self, _id: ID) -> impl Stream<Item = Invoice> {
        // This is a placeholder. A real implementation would use a pub/sub system
        // like Redis or a simple broadcast channel.
        stream::empty()
    }
}