use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Customer {
    pub name: String,
    pub email: String,
    pub address: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: String,
    pub customer: Customer,
    pub line_items: Vec<LineItem>,
    pub total: f64,
    pub status: String,
    pub issue_date: String,
    pub due_date: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct InvoiceSummary {
    pub total_revenue: f64,
    pub outstanding_invoices: f64,
    pub overdue_invoices: f64,
    pub recent_invoices: Vec<InvoiceNode>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvoiceNode {
    pub id: String,
    #[serde(rename = "invoiceNumber")]
    pub invoice_number: String,
    #[serde(rename = "customerName")]
    pub customer_name: String,
    pub total: f64,
    pub status: String,
    #[serde(rename = "dueDate")]
    pub due_date: String,
}