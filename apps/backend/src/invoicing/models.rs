use async_graphql::{ComplexObject, Context, dataloader::DataLoader, Enum, Result, SimpleObject};
use chrono::{DateTime, NaiveDate, Utc};
use crate::invoicing::graphql::CustomerLoader;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::BigDecimal;

#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Customer {
    pub id: i64,
    pub organization_id: i64,
    pub name: String,
    pub email: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Partial,
    Void,
}

#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Invoice {
    pub id: i64,
    pub organization_id: i64,
    pub customer_id: i64,
    pub invoice_number: String,
    pub status: InvoiceStatus,
    pub currency: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub notes: Option<String>,
    pub subtotal: BigDecimal,
    pub tax_total: BigDecimal,
    pub total: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct InvoiceLineItem {
    pub id: i64,
    pub invoice_id: i64,
    pub description: String,
    pub quantity: BigDecimal,
    pub unit_price: BigDecimal,
    pub tax_rate: Option<BigDecimal>,
    pub total: BigDecimal,
}

#[derive(Debug, Clone, FromRow, SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct Payment {
    pub id: i64,
    pub invoice_id: i64,
    pub payment_date: NaiveDate,
    pub amount: BigDecimal,
    pub payment_method: Option<String>,
    pub transaction_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Placeholder for the full Customer object resolver
#[ComplexObject]
impl Customer {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn organization_id(&self) -> String {
        self.organization_id.to_string()
    }
}

// Placeholder for the full Invoice object resolver
#[ComplexObject]
impl Invoice {
     async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn organization_id(&self) -> String {
        self.organization_id.to_string()
    async fn customer(&self, ctx: &Context<'_>) -> Result<Customer> {
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        let customer = loader.load_one(self.customer_id).await?;
        customer.ok_or_else(|| "Customer not found".into())
    }
    }
    async fn line_items(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<InvoiceLineItem>> {
        let pool = ctx.data_unchecked::<sqlx::PgPool>();
        let items = sqlx::query_as!(
            InvoiceLineItem,
            "SELECT * FROM invoice_line_items WHERE invoice_id = $1",
            self.id
        )
        .fetch_all(pool)
        .await?;
        Ok(items)
    }
    async fn payments(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Payment>> {
        unimplemented!()
    }
}

#[ComplexObject]
impl InvoiceLineItem {
    async fn id(&self) -> String {
        self.id.to_string()
    }
}

#[ComplexObject]
impl Payment {
    async fn id(&self) -> String {
        self.id.to_string()
    }
    async fn invoice(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Invoice> {
        unimplemented!()
    }
}