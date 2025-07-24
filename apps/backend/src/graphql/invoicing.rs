use async_graphql::{Context, Object, Result, ID, SimpleObject, InputObject};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::{
    services::invoicing::{InvoiceService, InvoiceStatus},
    notifications::{NotificationService, NotificationType},
};

/// GraphQL representation of an Invoice
#[derive(SimpleObject)]
pub struct Invoice {
    pub id: ID,
    pub issuer_id: ID,
    pub client_name: String,
    pub client_email: Option<String>,
    pub client_address: Option<String>,
    pub invoice_number: String,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub subtotal: f64,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub total: f64,
    pub status: InvoiceStatusType,
    pub notes: Option<String>,
    pub terms: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// GraphQL enum for invoice status
#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum InvoiceStatusType {
    Draft,
    Sent,
    Paid,
    Overdue,
    Cancelled,
}

impl From<InvoiceStatus> for InvoiceStatusType {
    fn from(status: InvoiceStatus) -> Self {
        match status {
            InvoiceStatus::Draft => InvoiceStatusType::Draft,
            InvoiceStatus::Sent => InvoiceStatusType::Sent,
            InvoiceStatus::Paid => InvoiceStatusType::Paid,
            InvoiceStatus::Overdue => InvoiceStatusType::Overdue,
            InvoiceStatus::Cancelled => InvoiceStatusType::Cancelled,
        }
    }
}

/// GraphQL representation of an Invoice Item
#[derive(SimpleObject)]
pub struct InvoiceItem {
    pub id: ID,
    pub invoice_id: ID,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

/// Input for creating an invoice
#[derive(InputObject)]
pub struct CreateInvoiceInput {
    pub client_name: String,
    pub client_email: Option<String>,
    pub client_address: Option<String>,
    pub invoice_number: String,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub items: Vec<CreateInvoiceItemInput>,
    pub tax_rate: f64,
    pub notes: Option<String>,
    pub terms: Option<String>,
}

/// Input for creating an invoice item
#[derive(InputObject)]
pub struct CreateInvoiceItemInput {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
}

/// Input for updating invoice status
#[derive(InputObject)]
pub struct UpdateInvoiceStatusInput {
    pub invoice_id: ID,
    pub status: InvoiceStatusType,
}

impl From<crate::services::invoicing::Invoice> for Invoice {
    fn from(invoice: crate::services::invoicing::Invoice) -> Self {
        Self {
            id: invoice.id.into(),
            issuer_id: invoice.issuer_id.into(),
            client_name: invoice.client_name,
            client_email: invoice.client_email,
            client_address: invoice.client_address,
            invoice_number: invoice.invoice_number,
            issue_date: invoice.issue_date,
            due_date: invoice.due_date,
            subtotal: invoice.subtotal,
            tax_rate: invoice.tax_rate,
            tax_amount: invoice.tax_amount,
            total: invoice.total,
            status: invoice.status.into(),
            notes: invoice.notes,
            terms: invoice.terms,
            created_at: invoice.created_at,
            updated_at: invoice.updated_at,
        }
    }
}

#[derive(Default)]
pub struct InvoicingQuery;

#[Object]
impl InvoicingQuery {
    async fn invoice(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Invoice>> {
        let invoice_service = ctx.data::<InvoiceService>()?;
        let invoice_id = Uuid::parse_str(&id)?;
        
        let invoice = invoice_service
            .get_invoice(invoice_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get invoice: {}", e)))?;
        
        Ok(Some(invoice.into()))
    }
    
    async fn user_invoices(
        &self,
        ctx: &Context<'_>,
        status: Option<String>,
    ) -> Result<Vec<Invoice>> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        let invoice_service = ctx.data::<InvoiceService>()?;
        
        let invoices = invoice_service
            .get_user_invoices(auth.user_id, status)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get user invoices: {}", e)))?;
        
        Ok(invoices.into_iter().map(Into::into).collect())
    }
}

pub struct InvoicingMutation;

#[Object]
impl InvoicingMutation {
    async fn create_invoice(
        &self,
        ctx: &Context<'_>,
        input: CreateInvoiceInput,
    ) -> Result<Invoice> {
        let auth = ctx.data::<crate::auth::AuthContext>()?;
        let invoice_service = ctx.data::<InvoiceService>()?;
        
        let invoice_input = crate::services::invoicing::CreateInvoiceInput {
            client_name: input.client_name,
            client_email: input.client_email,
            client_address: input.client_address,
            invoice_number: input.invoice_number,
            issue_date: input.issue_date,
            due_date: input.due_date,
            items: input.items.into_iter().map(|item| {
                crate::services::invoicing::CreateInvoiceItemInput {
                    description: item.description,
                    quantity: item.quantity,
                    unit_price: item.unit_price,
                }
            }).collect(),
            tax_rate: input.tax_rate,
            notes: input.notes,
            terms: input.terms,
        };
        
        let invoice = invoice_service
            .create_invoice(auth.user_id, invoice_input)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create invoice: {}", e)))?;
        
        Ok(invoice.into())
    }
    
    async fn send_invoice(&self, ctx: &Context<'_>, id: ID) -> Result<Invoice> {
        let invoice_service = ctx.data::<InvoiceService>()?;
        let notification_service = ctx.data::<NotificationService>()?;
        let invoice_id = Uuid::parse_str(&id)?;
        
        let invoice = invoice_service
            .send_invoice(invoice_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to send invoice: {}", e)))?;
        
        // Trigger notification
        notification_service.send_notification(
            invoice.issuer_id,
            NotificationType::InvoiceIssued {
                invoice_id: invoice.id,
                due_date: invoice.due_date,
            },
            serde_json::json!({
                "invoice_id": invoice.id,
                "invoice_number": invoice.invoice_number,
                "client_name": invoice.client_name,
                "total": invoice.total,
                "due_date": invoice.due_date,
            }),
        ).await?;
        
        Ok(invoice.into())
    }
    
    async fn mark_invoice_paid(&self, ctx: &Context<'_>, id: ID) -> Result<Invoice> {
        let invoice_service = ctx.data::<InvoiceService>()?;
        let invoice_id = Uuid::parse_str(&id)?;
        
        let invoice = invoice_service
            .mark_as_paid(invoice_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to mark invoice as paid: {}", e)))?;
        
        Ok(invoice.into())
    }
    
    async fn cancel_invoice(&self, ctx: &Context<'_>, id: ID) -> Result<Invoice> {
        let invoice_service = ctx.data::<InvoiceService>()?;
        let invoice_id = Uuid::parse_str(&id)?;
        
        let invoice = invoice_service
            .cancel_invoice(invoice_id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to cancel invoice: {}", e)))?;
        
        Ok(invoice.into())
    }
    
    async fn add_invoice_item(
        &self,
        ctx: &Context<'_>,
        invoice_id: ID,
        item: CreateInvoiceItemInput,
    ) -> Result<InvoiceItem> {
        let invoice_service = ctx.data::<InvoiceService>()?;
        let invoice_uuid = Uuid::parse_str(&invoice_id)?;
        
        // TODO: Implement add invoice item functionality
        // For now, returning placeholder
        Err(async_graphql::Error::new("Adding invoice items after creation not yet implemented"))
    }
}