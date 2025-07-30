//! Tests for the currency formatter

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use crate::{
        presentation::localization::formatter::CurrencyFormatter,
        domain::currency::{CurrencyCode, Currency},
    };

    #[test]
    fn test_comprehensive_locale_formatting() {
        let formatter = CurrencyFormatter::new();
        
        // Test various locales with USD
        let usd_currency: Currency = CurrencyCode::new("USD").into();
        
        // US English
        let result = formatter.format_currency(dec!(1234.56), &usd_currency, &"en-US".into());
        assert_eq!(result, "$1,234.56");
        
        // British English
        let result = formatter.format_currency(dec!(1234.56), &usd_currency, &"en-GB".into());
        assert_eq!(result, "$1,234.56");
        
        // German
        let result = formatter.format_currency(dec!(1234.56), &usd_currency, &"de-DE".into());
        assert_eq!(result, "1.234,56 $");
        
        // French
        let result = formatter.format_currency(dec!(1234.56), &usd_currency, &"fr-FR".into());
        assert_eq!(result, "1 234,56 $");
        
        // Spanish
        let result = formatter.format_currency(dec!(1234.56), &usd_currency, &"es-ES".into());
        assert_eq!(result, "1.234,56 $");
    }

    #[test]
    fn test_zero_decimal_currencies() {
        let formatter = CurrencyFormatter::new();
        
        // Japanese Yen (0 decimal places)
        let jpy_currency: Currency = CurrencyCode::new("JPY").into();
        
        let result = formatter.format_currency(dec!(1234), &jpy_currency, &"en-US".into());
        assert_eq!(result, "¥1,234");
        
        let result = formatter.format_currency(dec!(1234.56), &jpy_currency, &"ja-JP".into());
        assert_eq!(result, "¥1,234");
    }

    #[test]
    fn test_three_decimal_currencies() {
        let formatter = CurrencyFormatter::new();
        
        // Bahraini Dinar (3 decimal places)
        let bhd_currency: Currency = CurrencyCode::new("BHD").into();
        
        let result = formatter.format_currency(dec!(12.345), &bhd_currency, &"en-US".into());
        assert_eq!(result, "BD12.345");
    }

    #[test]
    fn test_large_numbers() {
        let formatter = CurrencyFormatter::new();
        let usd_currency: Currency = CurrencyCode::new("USD").into();
        
        // Test large number formatting
        let result = formatter.format_currency(dec!(1234567890.12), &usd_currency, &"en-US".into());
        assert_eq!(result, "$1,234,567,890.12");
    }

    #[test]
    fn test_negative_numbers() {
        let formatter = CurrencyFormatter::new();
        let usd_currency: Currency = CurrencyCode::new("USD").into();
        
        // Test negative number formatting
        let result = formatter.format_currency(dec!(-1234.56), &usd_currency, &"en-US".into());
        assert_eq!(result, "-$1,234.56");
    }

    #[test]
    fn test_format_with_code() {
        let formatter = CurrencyFormatter::new();
        let usd_currency: Currency = CurrencyCode::new("USD").into();
        
        // Test formatting with currency code
        let result = formatter.format_currency_with_code(dec!(1234.56), &usd_currency, &"en-US".into());
        assert_eq!(result, "$1,234.56 (USD)");
    }
}