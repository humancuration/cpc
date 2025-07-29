use cpc_core::invoicing::model::{Invoice, Customer};
use cpc_core::accounting::money::Money;
use std::fs::File;
use std::io::{BufWriter, Write};
use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum PdfGenerationError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Generation error: {0}")]
    GenerationError(String),
}

pub struct InvoicePdfGenerator;

impl InvoicePdfGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_invoice_pdf(
        &self,
        invoice: &Invoice,
        output_path: &str,
    ) -> Result<(), PdfGenerationError> {
        // Simple HTML-based PDF generation for now
        // In production, this would use printpdf or similar
        
        let html_content = self.generate_html(invoice)?;
        
        // For now, write HTML to file as placeholder
        // Later integrate with wkhtmltopdf or similar
        let mut file = BufWriter::new(File::create(output_path)?);
        file.write_all(html_content.as_bytes())?;
        
        Ok(())
    }

    fn generate_html(&self, invoice: &Invoice) -> Result<String, PdfGenerationError> {
        let mut html = String::new();
        
        html.push_str(r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Invoice #"#);
        html.push_str(&invoice.number);
        html.push_str(r#"</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .header { display: flex; justify-content: space-between; margin-bottom: 40px; }
        .company-info { flex: 1; }
        .invoice-info { flex: 1; text-align: right; }
        .customer-info { margin-bottom: 40px; }
        .line-items { width: 100%; border-collapse: collapse; margin-bottom: 40px; }
        .line-items th, .line-items td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        .line-items th { background-color: #f2f2f2; }
        .totals { text-align: right; margin-bottom: 40px; }
        .notes { margin-top: 40px; }
    </style>
</head>
<body>
"#);

        // Header
        html.push_str(r#"
    <div class="header">
        <div class="company-info">
            <h1>Your Company Name</h1>
            <p>123 Business Street<br>
            City, State 12345<br>
            Phone: (555) 123-4567<br>
            Email: billing@company.com</p>
        </div>
        <div class="invoice-info">
            <h2>INVOICE</h2>
            <p><strong>Invoice #:</strong> "#);
        html.push_str(&invoice.number);
        html.push_str(r#"<br>
            <strong>Date:</strong> "#);
        html.push_str(&invoice.issue_date.format("%B %d, %Y").to_string());
        html.push_str(r#"<br>
            <strong>Due Date:</strong> "#);
        html.push_str(&invoice.due_date.format("%B %d, %Y").to_string());
        html.push_str(r#"</p>
        </div>
    </div>
"#);

        // Customer Info
        html.push_str(r#"
    <div class="customer-info">
        <h3>Bill To:</h3>
        <p><strong>"#);
        html.push_str(&invoice.customer.name);
        html.push_str(r#"</strong><br>"#);
        
        if let Some(email) = &invoice.customer.email {
            html.push_str(&format!("{}<br>", email));
        }
        
        if let Some(address) = &invoice.customer.address {
            html.push_str(&format!("{}<br>{}<br>", address.street, address.city));
            html.push_str(&format!("{}, {} {}", address.state, address.postal_code, address.country));
        }
        
        html.push_str(r#"</p>
    </div>
"#);

        // Line Items
        html.push_str(r#"
    <table class="line-items">
        <thead>
            <tr>
                <th>Description</th>
                <th>Quantity</th>
                <th>Unit Price</th>
                <th>Total</th>
            </tr>
        </thead>
        <tbody>
"#);

        for item in &invoice.line_items {
            html.push_str(&format!(
                r#"
            <tr>
                <td>{}</td>
                <td>{:.2}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>"#,
                item.description,
                item.quantity,
                item.unit_price,
                item.total
            ));
        }

        html.push_str(r#"
        </tbody>
    </table>
"#);

        // Totals
        html.push_str(r#"
    <div class="totals">
        <p><strong>Subtotal:</strong> "#);
        html.push_str(&invoice.subtotal.to_string());
        html.push_str(r#"<br>
        <strong>Tax:</strong> "#);
        html.push_str(&invoice.tax_amount.to_string());
        html.push_str(r#"<br>
        <strong><em>Total:</em></strong> "#);
        html.push_str(&invoice.total.to_string());
        html.push_str(r#"</p>
    </div>
"#);

        // Notes and Terms
        if invoice.notes.is_some() || invoice.terms.is_some() {
            html.push_str(r#"
    <div class="notes">
"#);
            
            if let Some(notes) = &invoice.notes {
                html.push_str(&format!("<h3>Notes:</h3><p>{}</p>", notes));
            }
            
            if let Some(terms) = &invoice.terms {
                html.push_str(&format!("<h3>Terms:</h3><p>{}</p>", terms));
            }
            
            html.push_str(r#"
    </div>
"#);
        }

        html.push_str(r#"
</body>
</html>
"#);

        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::invoicing::model::{Invoice, Customer};
    use cpc_core::accounting::money::Money;
    use uuid::Uuid;

    #[test]
    fn test_generate_html() {
        let customer = Customer::new("Test Customer".to_string());
        let mut invoice = Invoice::new(customer);
        invoice.add_line_item("Test Item".to_string(), 2.0, Money::from_decimal(100, 0), None);
        
        let generator = InvoicePdfGenerator::new();
        let html = generator.generate_html(&invoice).unwrap();
        
        assert!(html.contains("Test Customer"));
        assert!(html.contains("Test Item"));
        assert!(html.contains("200.00"));
    }
}