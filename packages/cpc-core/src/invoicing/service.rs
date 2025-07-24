use crate::accounting::money::Money;
use crate::invoicing::model::{Invoice, InvoiceFilter, InvoiceStatus, Customer};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum InvoiceServiceError {
    #[error("Invoice not found: {0}")]
    InvoiceNotFound(Uuid),
    
    #[error("Invalid invoice state: {0}")]
    InvalidState(String),
    
    #[error("Customer not found: {0}")]
    CustomerNotFound(Uuid),
    
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

pub trait InvoiceCalculator {
    fn calculate_subtotal(&self, invoice: &Invoice) -> Money;
    fn calculate_tax(&self, invoice: &Invoice) -> Money;
    fn calculate_total(&self, invoice: &Invoice) -> Money;
    fn calculate_balance_due(&self, invoice: &Invoice) -> Money;
    fn calculate_overdue_amount(&self, invoice: &Invoice, as_of: DateTime<Utc>) -> Money;
}

pub struct StandardInvoiceCalculator;

impl InvoiceCalculator for StandardInvoiceCalculator {
    fn calculate_subtotal(&self, invoice: &Invoice) -> Money {
        invoice.line_items.iter()
            .map(|item| item.total)
            .fold(Money::zero(), |acc, total| acc + total)
    }

    fn calculate_tax(&self, invoice: &Invoice) -> Money {
        invoice.line_items.iter()
            .filter_map(|item| {
                item.tax_rate.map(|rate| item.total * rate / 100.0)
            })
            .fold(Money::zero(), |acc, tax| acc + tax)
    }

    fn calculate_total(&self, invoice: &Invoice) -> Money {
        self.calculate_subtotal(invoice) + self.calculate_tax(invoice)
    }

    fn calculate_balance_due(&self, invoice: &Invoice) -> Money {
        match invoice.status {
            InvoiceStatus::Paid => Money::zero(),
            _ => invoice.total,
        }
    }

    fn calculate_overdue_amount(&self, invoice: &Invoice, as_of: DateTime<Utc>) -> Money {
        if invoice.status == InvoiceStatus::Paid {
            return Money::zero();
        }

        if as_of > invoice.due_date {
            invoice.total
        } else {
            Money::zero()
        }
    }
}

pub struct InvoiceReportService {
    calculator: Box<dyn InvoiceCalculator>,
}

impl InvoiceReportService {
    pub fn new(calculator: Box<dyn InvoiceCalculator>) -> Self {
        Self { calculator }
    }

    pub fn generate_summary_report(
        &self,
        invoices: &[Invoice],
        date_from: DateTime<Utc>,
        date_to: DateTime<Utc>,
    ) -> InvoiceSummaryReport {
        let filtered_invoices: Vec<&Invoice> = invoices.iter()
            .filter(|inv| inv.issue_date >= date_from && inv.issue_date <= date_to)
            .collect();

        let total_invoiced = filtered_invoices.iter()
            .map(|inv| inv.total)
            .fold(Money::zero(), |acc, total| acc + total);

        let total_paid = filtered_invoices.iter()
            .filter(|inv| inv.status == InvoiceStatus::Paid)
            .map(|inv| inv.total)
            .fold(Money::zero(), |acc, total| acc + total);

        let total_outstanding = filtered_invoices.iter()
            .filter(|inv| inv.status != InvoiceStatus::Paid)
            .map(|inv| self.calculator.calculate_balance_due(inv))
            .fold(Money::zero(), |acc, balance| acc + balance);

        let overdue_invoices: Vec<&Invoice> = filtered_invoices.iter()
            .filter(|inv| {
                inv.status != InvoiceStatus::Paid && 
                Utc::now() > inv.due_date
            })
            .copied()
            .collect();

        let total_overdue = overdue_invoices.iter()
            .map(|inv| inv.total)
            .fold(Money::zero(), |acc, total| acc + total);

        InvoiceSummaryReport {
            total_invoiced,
            total_paid,
            total_outstanding,
            total_overdue,
            overdue_invoices: overdue_invoices.into_iter().cloned().collect(),
            date_range: (date_from, date_to),
        }
    }

    pub fn generate_customer_report(
        &self,
        invoices: &[Invoice],
        customer_id: Uuid,
    ) -> CustomerReport {
        let customer_invoices: Vec<&Invoice> = invoices.iter()
            .filter(|inv| inv.customer_id == customer_id)
            .collect();

        let total_invoiced = customer_invoices.iter()
            .map(|inv| inv.total)
            .fold(Money::zero(), |acc, total| acc + total);

        let total_paid = customer_invoices.iter()
            .filter(|inv| inv.status == InvoiceStatus::Paid)
            .map(|inv| inv.total)
            .fold(Money::zero(), |acc, total| acc + total);

        let outstanding_balance = customer_invoices.iter()
            .filter(|inv| inv.status != InvoiceStatus::Paid)
            .map(|inv| self.calculator.calculate_balance_due(inv))
            .fold(Money::zero(), |acc, balance| acc + balance);

        CustomerReport {
            customer_id,
            total_invoiced,
            total_paid,
            outstanding_balance,
            invoice_count: customer_invoices.len(),
        }
    }

    pub fn generate_aging_report(
        &self,
        invoices: &[Invoice],
        as_of: DateTime<Utc>,
    ) -> AgingReport {
        let mut buckets = HashMap::new();
        let now = as_of;

        for invoice in invoices {
            if invoice.status == InvoiceStatus::Paid {
                continue;
            }

            let days_overdue = (now - invoice.due_date).num_days();
            let bucket = match days_overdue {
                d if d <= 0 => "Current",
                d if d <= 30 => "1-30",
                d if d <= 60 => "31-60",
                d if d <= 90 => "61-90",
                _ => "90+",
            };

            let entry = buckets.entry(bucket.to_string()).or_insert_with(|| AgingBucket {
                name: bucket.to_string(),
                total_amount: Money::zero(),
                invoice_count: 0,
            });

            entry.total_amount = entry.total_amount + invoice.total;
            entry.invoice_count += 1;
        }

        AgingReport {
            as_of_date: as_of,
            buckets: buckets.into_iter().map(|(_, bucket)| bucket).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceSummaryReport {
    pub total_invoiced: Money,
    pub total_paid: Money,
    pub total_outstanding: Money,
    pub total_overdue: Money,
    pub overdue_invoices: Vec<Invoice>,
    pub date_range: (DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerReport {
    pub customer_id: Uuid,
    pub total_invoiced: Money,
    pub total_paid: Money,
    pub outstanding_balance: Money,
    pub invoice_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgingReport {
    pub as_of_date: DateTime<Utc>,
    pub buckets: Vec<AgingBucket>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgingBucket {
    pub name: String,
    pub total_amount: Money,
    pub invoice_count: usize,
}

pub struct InvoiceValidationService;

impl InvoiceValidationService {
    pub fn validate_invoice(invoice: &Invoice) -> Result<(), InvoiceServiceError> {
        // Validate basic invoice structure
        if invoice.number.is_empty() {
            return Err(InvoiceServiceError::InvalidState("Invoice number cannot be empty".to_string()));
        }

        if invoice.customer.name.is_empty() {
            return Err(InvoiceServiceError::InvalidState("Customer name cannot be empty".to_string()));
        }

        if invoice.line_items.is_empty() {
            return Err(InvoiceServiceError::InvalidState("Invoice must have at least one line item".to_string()));
        }

        // Validate line items
        for item in &invoice.line_items {
            if item.description.is_empty() {
                return Err(InvoiceServiceError::InvalidState("Line item description cannot be empty".to_string()));
            }
            
            if item.quantity <= 0.0 {
                return Err(InvoiceServiceError::InvalidState("Line item quantity must be positive".to_string()));
            }
            
            if item.unit_price < Money::zero() {
                return Err(InvoiceServiceError::InvalidState("Line item price cannot be negative".to_string()));
            }
        }

        // Validate dates
        if invoice.due_date < invoice.issue_date {
            return Err(InvoiceServiceError::InvalidState("Due date must be after issue date".to_string()));
        }

        Ok(())
    }

    pub fn validate_customer(customer: &Customer) -> Result<(), InvoiceServiceError> {
        if customer.name.is_empty() {
            return Err(InvoiceServiceError::InvalidState("Customer name cannot be empty".to_string()));
        }

        if let Some(email) = &customer.email {
            if !email.contains('@') {
                return Err(InvoiceServiceError::InvalidState("Invalid email format".to_string()));
            }
        }

        Ok(())
    }
}