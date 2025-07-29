use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub client_id: String,
    pub client_name: String,
    pub issue_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub subtotal: f64,
    pub tax_amount: f64,
    pub total: f64,
    pub status: InvoiceStatus,
    pub line_items: Vec<InvoiceLineItem>,
    pub notes: Option<String>,
    pub terms: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub id: String,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_default: bool,
    pub fields: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub name: String,
    pub email: String,
    pub address: String,
    pub phone: Option<String>,
    pub tax_id: Option<String>,
    pub payment_terms: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicingState {
    pub invoices: Vec<Invoice>,
    pub templates: Vec<InvoiceTemplate>,
    pub clients: Vec<Client>,
    pub selected_invoice: Option<Invoice>,
    pub selected_template: Option<InvoiceTemplate>,
    pub filters: InvoiceFilters,
    pub loading: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceFilters {
    pub status: Option<InvoiceStatus>,
    pub client_id: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoicingSummary {
    pub total_invoices: i64,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub pending_amount: f64,
    pub overdue_amount: f64,
    pub draft_count: i64,
    pub sent_count: i64,
    pub paid_count: i64,
    pub overdue_count: i64,
}

pub struct InvoicingContext {
    pub state: Arc<RwLock<InvoicingState>>,
    pub summary: Arc<RwLock<InvoicingSummary>>,
}

impl InvoicingContext {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(InvoicingState {
                invoices: Vec::new(),
                templates: Vec::new(),
                clients: Vec::new(),
                selected_invoice: None,
                selected_template: None,
                filters: InvoiceFilters::default(),
                loading: false,
                error: None,
            })),
            summary: Arc::new(RwLock::new(InvoicingSummary {
                total_invoices: 0,
                total_amount: 0.0,
                paid_amount: 0.0,
                pending_amount: 0.0,
                overdue_amount: 0.0,
                draft_count: 0,
                sent_count: 0,
                paid_count: 0,
                overdue_count: 0,
            })),
        }
    }

    pub async fn update_invoices(&self, invoices: Vec<Invoice>) {
        let mut state = self.state.write().await;
        state.invoices = invoices;
        self.recalculate_summary().await;
    }

    pub async fn add_invoice(&self, invoice: Invoice) {
        let mut state = self.state.write().await;
        state.invoices.push(invoice);
        self.recalculate_summary().await;
    }

    pub async fn update_invoice(&self, id: String, invoice: Invoice) {
        let mut state = self.state.write().await;
        if let Some(index) = state.invoices.iter().position(|i| i.id == id) {
            state.invoices[index] = invoice;
            self.recalculate_summary().await;
        }
    }

    pub async fn remove_invoice(&self, id: String) {
        let mut state = self.state.write().await;
        state.invoices.retain(|i| i.id != id);
        self.recalculate_summary().await;
    }

    pub async fn set_loading(&self, loading: bool) {
        let mut state = self.state.write().await;
        state.loading = loading;
    }

    pub async fn set_error(&self, error: Option<String>) {
        let mut state = self.state.write().await;
        state.error = error;
    }

    pub async fn set_filters(&self, filters: InvoiceFilters) {
        let mut state = self.state.write().await;
        state.filters = filters;
    }

    pub async fn set_selected_invoice(&self, invoice: Option<Invoice>) {
        let mut state = self.state.write().await;
        state.selected_invoice = invoice;
    }

    pub async fn set_selected_template(&self, template: Option<InvoiceTemplate>) {
        let mut state = self.state.write().await;
        state.selected_template = template;
    }

    pub async fn update_templates(&self, templates: Vec<InvoiceTemplate>) {
        let mut state = self.state.write().await;
        state.templates = templates;
    }

    pub async fn update_clients(&self, clients: Vec<Client>) {
        let mut state = self.state.write().await;
        state.clients = clients;
    }

    async fn recalculate_summary(&self) {
        let state = self.state.read().await;
        let mut summary = self.summary.write().await;

        summary.total_invoices = state.invoices.len() as i64;
        summary.total_amount = state.invoices.iter().map(|i| i.total).sum();
        summary.paid_amount = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Paid))
            .map(|i| i.total)
            .sum();
        summary.pending_amount = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Sent | InvoiceStatus::Viewed))
            .map(|i| i.total)
            .sum();
        summary.overdue_amount = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Overdue))
            .map(|i| i.total)
            .sum();

        summary.draft_count = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Draft))
            .count() as i64;
        summary.sent_count = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Sent))
            .count() as i64;
        summary.paid_count = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Paid))
            .count() as i64;
        summary.overdue_count = state.invoices.iter()
            .filter(|i| matches!(i.status, InvoiceStatus::Overdue))
            .count() as i64;
    }

    pub fn get_filtered_invoices(&self) -> Vec<Invoice> {
        // This would be called from the UI layer with async runtime
        // For now, return empty vec - actual filtering happens in UI layer
        Vec::new()
    }
}