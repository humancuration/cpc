use chrono::{DateTime, Utc, Duration};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct InvoiceCalculator;

impl InvoiceCalculator {
    pub fn calculate_subtotal(line_items: &[InvoiceLineItem]) -> Decimal {
        line_items.iter()
            .map(|item| Decimal::from_f64(item.quantity).unwrap_or_default() * Decimal::from_f64(item.unit_price).unwrap_or_default())
            .sum()
    }

    pub fn calculate_tax_amount(subtotal: Decimal, tax_rate: Decimal) -> Decimal {
        subtotal * tax_rate
    }

    pub fn calculate_total(subtotal: Decimal, tax_amount: Decimal, discount: Option<Decimal>) -> Decimal {
        let total = subtotal + tax_amount;
        if let Some(discount_amount) = discount {
            total - discount_amount
        } else {
            total
        }
    }

    pub fn calculate_due_date(issue_date: DateTime<Utc>, payment_terms: &str) -> DateTime<Utc> {
        match payment_terms {
            "net_15" => issue_date + Duration::days(15),
            "net_30" => issue_date + Duration::days(30),
            "net_45" => issue_date + Duration::days(45),
            "net_60" => issue_date + Duration::days(60),
            "due_on_receipt" => issue_date,
            _ => issue_date + Duration::days(30),
        }
    }

    pub fn generate_invoice_number(prefix: &str, sequence: u64) -> String {
        format!("{}{:06}", prefix, sequence)
    }

    pub fn is_overdue(due_date: DateTime<Utc>, current_date: DateTime<Utc>) -> bool {
        current_date > due_date
    }

    pub fn calculate_overdue_days(due_date: DateTime<Utc>, current_date: DateTime<Utc>) -> i64 {
        if current_date > due_date {
            (current_date - due_date).num_days()
        } else {
            0
        }
    }

    pub fn calculate_late_fee(
        invoice_amount: Decimal,
        overdue_days: i64,
        late_fee_rate: Decimal,
        max_late_fee: Option<Decimal>,
    ) -> Decimal {
        let fee = invoice_amount * late_fee_rate * Decimal::from(overdue_days) / dec!(100);
        if let Some(max) = max_late_fee {
            fee.min(max)
        } else {
            fee
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvoiceFormatter;

impl InvoiceFormatter {
    pub fn format_currency(amount: Decimal, currency: &str) -> String {
        match currency {
            "USD" => format!("${:.2}", amount),
            "EUR" => format!("€{:.2}", amount),
            "GBP" => format!("£{:.2}", amount),
            "JPY" => format!("¥{:.0}", amount),
            _ => format!("{:.2} {}", amount, currency),
        }
    }

    pub fn format_date(date: DateTime<Utc>) -> String {
        date.format("%B %d, %Y").to_string()
    }

    pub fn format_short_date(date: DateTime<Utc>) -> String {
        date.format("%m/%d/%Y").to_string()
    }

    pub fn format_percent(value: Decimal) -> String {
        format!("{:.2}%", value * dec!(100))
    }

    pub fn format_address(address: &Address) -> String {
        vec![
            address.street.clone(),
            format!("{}, {} {}", address.city, address.state, address.zip),
            address.country.clone(),
        ]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct InvoiceValidator;

impl InvoiceValidator {
    pub fn validate_invoice(invoice: &Invoice) -> Result<(), ValidationError> {
        if invoice.client_id.is_empty() {
            return Err(ValidationError::InvalidClient);
        }

        if invoice.line_items.is_empty() {
            return Err(ValidationError::NoLineItems);
        }

        for item in &invoice.line_items {
            if item.description.trim().is_empty() {
                return Err(ValidationError::InvalidLineItem("Description cannot be empty".to_string()));
            }

            if item.quantity <= 0.0 {
                return Err(ValidationError::InvalidLineItem("Quantity must be positive".to_string()));
            }

            if item.unit_price < 0.0 {
                return Err(ValidationError::InvalidLineItem("Unit price cannot be negative".to_string()));
            }
        }

        if invoice.due_date <= invoice.issue_date {
            return Err(ValidationError::InvalidDates);
        }

        Ok(())
    }

    pub fn validate_email(email: &str) -> Result<(), ValidationError> {
        if !email.contains('@') || !email.contains('.') {
            return Err(ValidationError::InvalidEmail(email.to_string()));
        }
        Ok(())
    }

    pub fn validate_tax_id(tax_id: &str, country: &str) -> Result<(), ValidationError> {
        match country {
            "US" => {
                if tax_id.len() != 9 || !tax_id.chars().all(|c| c.is_ascii_digit()) {
                    return Err(ValidationError::InvalidTaxId("Invalid US tax ID format".to_string()));
                }
            }
            "CA" => {
                if !tax_id.chars().all(|c| c.is_ascii_digit() || c.is_ascii_uppercase()) {
                    return Err(ValidationError::InvalidTaxId("Invalid Canadian tax ID format".to_string()));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct InvoiceExporter;

impl InvoiceExporter {
    pub fn to_pdf(invoice: &Invoice) -> Result<Vec<u8>, ExportError> {
        // Placeholder for PDF generation
        // Would use a library like printpdf or wkhtmltopdf
        Ok(Vec::new())
    }

    pub fn to_csv(invoices: &[Invoice]) -> Result<String, ExportError> {
        let mut csv = String::from("Invoice Number,Client,Issue Date,Due Date,Total,Status\n");
        
        for invoice in invoices {
            csv.push_str(&format!(
                "{},{},{},{},{},{}\n",
                invoice.invoice_number,
                invoice.client_name,
                invoice.issue_date.format("%Y-%m-%d"),
                invoice.due_date.format("%Y-%m-%d"),
                invoice.total,
                format!("{:?}", invoice.status)
            ));
        }
        
        Ok(csv)
    }

    pub fn to_json(invoices: &[Invoice]) -> Result<String, ExportError> {
        serde_json::to_string_pretty(invoices)
            .map_err(|e| ExportError::SerializationError(e.to_string()))
    }

    pub fn to_xml(invoices: &[Invoice]) -> Result<String, ExportError> {
        // Placeholder for XML generation
        Ok(String::new())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    InvalidClient,
    NoLineItems,
    InvalidLineItem(String),
    InvalidDates,
    InvalidEmail(String),
    InvalidTaxId(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidClient => write!(f, "Invalid client"),
            ValidationError::NoLineItems => write!(f, "Invoice must have at least one line item"),
            ValidationError::InvalidLineItem(msg) => write!(f, "Invalid line item: {}", msg),
            ValidationError::InvalidDates => write!(f, "Due date must be after issue date"),
            ValidationError::InvalidEmail(email) => write!(f, "Invalid email format: {}", email),
            ValidationError::InvalidTaxId(msg) => write!(f, "Invalid tax ID: {}", msg),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Debug, Clone)]
pub enum ExportError {
    GenerationError(String),
    SerializationError(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::GenerationError(msg) => write!(f, "Generation error: {}", msg),
            ExportError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for ExportError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    pub id: String,
    pub description: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total: f64,
}

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
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

#[derive(Debug, Clone)]
pub struct InvoiceNumberGenerator {
    prefix: String,
    sequence: Arc<tokio::sync::Mutex<u64>>,
}

impl InvoiceNumberGenerator {
    pub fn new(prefix: String, start_sequence: u64) -> Self {
        Self {
            prefix,
            sequence: Arc::new(tokio::sync::Mutex::new(start_sequence)),
        }
    }

    pub async fn next(&self) -> String {
        let mut seq = self.sequence.lock().await;
        *seq += 1;
        format!("{}{:06}", self.prefix, *seq)
    }

    pub async fn reset(&self, new_sequence: u64) {
        let mut seq = self.sequence.lock().await;
        *seq = new_sequence;
    }
}

#[derive(Debug, Clone)]
pub struct TaxCalculator {
    pub tax_rates: HashMap<String, Decimal>,
}

impl TaxCalculator {
    pub fn new() -> Self {
        let mut tax_rates = HashMap::new();
        tax_rates.insert("US-CA".to_string(), dec!(0.0875));
        tax_rates.insert("US-NY".to_string(), dec!(0.08875));
        tax_rates.insert("US-TX".to_string(), dec!(0.0625));
        tax_rates.insert("CA-ON".to_string(), dec!(0.13));
        tax_rates.insert("GB".to_string(), dec!(0.20));
        
        Self { tax_rates }
    }

    pub fn calculate_tax(&self, amount: Decimal, jurisdiction: &str) -> Decimal {
        self.tax_rates
            .get(jurisdiction)
            .copied()
            .unwrap_or_default()
            * amount
    }

    pub fn add_jurisdiction(&mut self, jurisdiction: String, rate: Decimal) {
        self.tax_rates.insert(jurisdiction, rate);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_subtotal() {
        let items = vec![
            InvoiceLineItem {
                id: Uuid::new_v4().to_string(),
                description: "Test service".to_string(),
                quantity: 2.0,
                unit_price: 100.0,
                total: 200.0,
            },
            InvoiceLineItem {
                id: Uuid::new_v4().to_string(),
                description: "Another service".to_string(),
                quantity: 1.0,
                unit_price: 50.0,
                total: 50.0,
            },
        ];

        let subtotal = InvoiceCalculator::calculate_subtotal(&items);
        assert_eq!(subtotal, dec!(250.0));
    }

    #[test]
    fn test_calculate_tax() {
        let subtotal = dec!(1000.0);
        let tax_rate = dec!(0.0875);
        let tax = InvoiceCalculator::calculate_tax_amount(subtotal, tax_rate);
        assert_eq!(tax, dec!(87.5));
    }

    #[test]
    fn test_format_currency() {
        let amount = dec!(1234.56);
        assert_eq!(InvoiceFormatter::format_currency(amount, "USD"), "$1234.56");
        assert_eq!(InvoiceFormatter::format_currency(amount, "EUR"), "€1234.56");
    }

    #[test]
    fn test_validate_email() {
        assert!(InvoiceValidator::validate_email("test@example.com").is_ok());
        assert!(InvoiceValidator::validate_email("invalid-email").is_err());
    }
}