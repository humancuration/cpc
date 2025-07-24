use crate::db::DbPool;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Invoice {
    pub id: Uuid,
    pub issuer_id: Uuid,
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
    pub status: InvoiceStatus,
    pub notes: Option<String>,
    pub terms: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "invoice_status", rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInvoiceItemInput {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
}

pub struct InvoiceService {
    db: DbPool,
}

impl InvoiceService {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    pub async fn get_invoice(&self, invoice_id: Uuid) -> Result<Invoice, sqlx::Error> {
        let mut conn = self.db.get().await?;
        
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT id, issuer_id, client_name, client_email, client_address, 
                   invoice_number, issue_date, due_date, subtotal, tax_rate, 
                   tax_amount, total, status, notes, terms, created_at, updated_at
            FROM invoices
            WHERE id = $1
            "#
        )
        .bind(invoice_id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(invoice)
    }

    pub async fn get_user_invoices(
        &self,
        user_id: Uuid,
        status: Option<String>,
    ) -> Result<Vec<Invoice>, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let query = match status {
            Some(status) => {
                sqlx::query_as::<_, Invoice>(
                    r#"
                    SELECT id, issuer_id, client_name, client_email, client_address,
                           invoice_number, issue_date, due_date, subtotal, tax_rate,
                           tax_amount, total, status, notes, terms, created_at, updated_at
                    FROM invoices
                    WHERE issuer_id = $1 AND status = $2
                    ORDER BY created_at DESC
                    "#
                )
                .bind(user_id)
                .bind(status)
            }
            None => {
                sqlx::query_as::<_, Invoice>(
                    r#"
                    SELECT id, issuer_id, client_name, client_email, client_address,
                           invoice_number, issue_date, due_date, subtotal, tax_rate,
                           tax_amount, total, status, notes, terms, created_at, updated_at
                    FROM invoices
                    WHERE issuer_id = $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(user_id)
            }
        };

        let invoices = query.fetch_all(&mut *conn).await?;
        Ok(invoices)
    }

    pub async fn create_invoice(
        &self,
        user_id: Uuid,
        input: CreateInvoiceInput,
    ) -> Result<Invoice, sqlx::Error> {
        let mut conn = self.db.get().await?;
        let mut tx = conn.begin().await?;

        // Calculate totals
        let subtotal: f64 = input.items.iter().map(|item| item.quantity * item.unit_price).sum();
        let tax_amount = subtotal * (input.tax_rate / 100.0);
        let total = subtotal + tax_amount;

        // Create invoice
        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            INSERT INTO invoices (
                id, issuer_id, client_name, client_email, client_address,
                invoice_number, issue_date, due_date, subtotal, tax_rate,
                tax_amount, total, status, notes, terms, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 'draft', $13, $14, NOW(), NOW())
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(input.client_name)
        .bind(input.client_email)
        .bind(input.client_address)
        .bind(input.invoice_number)
        .bind(input.issue_date)
        .bind(input.due_date)
        .bind(subtotal)
        .bind(input.tax_rate)
        .bind(tax_amount)
        .bind(total)
        .bind(input.notes)
        .bind(input.terms)
        .fetch_one(&mut *tx)
        .await?;

        // Create invoice items
        for item in input.items {
            sqlx::query!(
                r#"
                INSERT INTO invoice_items (id, invoice_id, description, quantity, unit_price, total)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
                Uuid::new_v4(),
                invoice.id,
                item.description,
                item.quantity,
                item.unit_price,
                item.quantity * item.unit_price
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(invoice)
    }

    pub async fn send_invoice(&self, invoice_id: Uuid) -> Result<Invoice, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices
            SET status = 'sent', updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(invoice_id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(invoice)
    }

    pub async fn mark_as_paid(&self, invoice_id: Uuid) -> Result<Invoice, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices
            SET status = 'paid', updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#
        )
        .bind(invoice_id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(invoice)
    }

    pub async fn cancel_invoice(&self, invoice_id: Uuid) -> Result<Invoice, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let invoice = sqlx::query_as::<_, Invoice>(
            r#"
            UPDATE invoices
            SET status = 'cancelled', updated_at = NOW()
            WHERE id = $1 AND status = 'draft'
            RETURNING *
            "#
        )
        .bind(invoice_id)
        .fetch_one(&mut *conn)
        .await?;

        Ok(invoice)
    }

    pub async fn get_invoices_due_between(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<Invoice>, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let invoices = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT id, issuer_id, client_name, client_email, client_address,
                   invoice_number, issue_date, due_date, subtotal, tax_rate,
                   tax_amount, total, status, notes, terms, created_at, updated_at
            FROM invoices
            WHERE due_date >= $1 AND due_date < $2
            AND status NOT IN ('paid', 'cancelled')
            ORDER BY due_date ASC
            "#
        )
        .bind(start_date)
        .bind(end_date)
        .fetch_all(&mut *conn)
        .await?;

        Ok(invoices)
    }

    pub async fn get_overdue_invoices(&self) -> Result<Vec<Invoice>, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let invoices = sqlx::query_as::<_, Invoice>(
            r#"
            SELECT id, issuer_id, client_name, client_email, client_address,
                   invoice_number, issue_date, due_date, subtotal, tax_rate,
                   tax_amount, total, status, notes, terms, created_at, updated_at
            FROM invoices
            WHERE due_date < NOW()
            AND status NOT IN ('paid', 'cancelled')
            ORDER BY due_date ASC
            "#
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(invoices)
    }

    pub async fn get_invoice_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceItem>, sqlx::Error> {
        let mut conn = self.db.get().await?;

        let items = sqlx::query_as::<_, InvoiceItem>(
            r#"
            SELECT id, invoice_id, description, quantity, unit_price, total
            FROM invoice_items
            WHERE invoice_id = $1
            ORDER BY id
            "#
        )
        .bind(invoice_id)
        .fetch_all(&mut *conn)
        .await?;

        Ok(items)
    }
}