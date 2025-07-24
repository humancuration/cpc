#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::invoicing::model::{Invoice, Customer, InvoiceStatus};
    use cpc_core::invoicing::money::Money;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_invoice_creation() {
        // This is a basic integration test to verify our invoicing system works
        // In a real scenario, we'd use the actual API service
        
        // Create a customer
        let customer = Customer::new(
            "Test Customer".to_string(),
            Some("test@example.com".to_string()),
            Some("+1234567890".to_string()),
            Some("123 Test St".to_string()),
            Some("Test City".to_string()),
            Some("TS".to_string()),
            Some("12345".to_string()),
            Some("US".to_string()),
            None,
        );

        // Create an invoice
        let mut invoice = Invoice::new(
            customer.id,
            chrono::Utc::now(),
            chrono::Utc::now() + chrono::Duration::days(30),
        );

        // Add line items
        invoice.add_line_item(
            "Test Service".to_string(),
            2.0,
            Money::from_decimal(50, 0),
            Some(0.1),
        );

        // Validate totals
        assert_eq!(invoice.subtotal, Money::from_decimal(100, 0));
        assert_eq!(invoice.tax_amount, Money::from_decimal(10, 0));
        assert_eq!(invoice.total, Money::from_decimal(110, 0));

        // Test status changes
        invoice.status = InvoiceStatus::Sent;
        assert_eq!(invoice.status, InvoiceStatus::Sent);

        println!("✅ Invoice system basic test passed");
    }

    #[tokio::test]
    async fn test_money_calculations() {
        let money = Money::from_decimal(12345, 2); // $123.45
        assert_eq!(money.amount_mantissa(), 12345);
        assert_eq!(money.amount_exponent(), 2);
        
        let money2 = Money::from_decimal(54321, 2); // $543.21
        let sum = money + money2;
        assert_eq!(sum.amount_mantissa(), 66666);
        
        println!("✅ Money calculations test passed");
    }
}