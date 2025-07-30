//! Unit tests for the Android Rust user preferences Kotlin bindings

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use packages::domains::finance::domain::primitives::Currency;

    #[test]
    fn test_currency_from_code() {
        // Test valid currency codes
        assert_eq!(Currency::from_code("USD"), Some(Currency::USD));
        assert_eq!(Currency::from_code("EUR"), Some(Currency::EUR));
        assert_eq!(Currency::from_code("GBP"), Some(Currency::GBP));
        assert_eq!(Currency::from_code("JPY"), Some(Currency::JPY));
        assert_eq!(Currency::from_code("DABLOONS"), Some(Currency::Dabloons));
        
        // Test invalid currency code
        assert_eq!(Currency::from_code("INVALID"), None);
    }
    
    #[test]
    fn test_get_user_currency_from_local_storage() {
        let user_id = Uuid::new_v4();
        
        // This is a placeholder test for the mock function
        let result = get_user_currency_from_local_storage(user_id);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_set_user_currency_in_local_storage() {
        let user_id = Uuid::new_v4();
        let currency = Currency::EUR;
        
        // This is a placeholder test for the mock function
        let result = set_user_currency_in_local_storage(user_id, currency);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_currency_code_conversion() {
        // Test that all supported currencies can be converted to codes and back
        let currencies = vec![
            Currency::USD, Currency::EUR, Currency::GBP, Currency::JPY,
            Currency::CAD, Currency::AUD, Currency::CHF, Currency::CNY,
            Currency::SEK, Currency::NZD, Currency::MXN, Currency::SGD,
            Currency::HKD, Currency::NOK, Currency::KRW, Currency::TRY,
            Currency::RUB, Currency::INR, Currency::BRL, Currency::ZAR,
            Currency::Dabloons,
        ];
        
        for currency in currencies {
            let code = currency.code();
            let converted = Currency::from_code(code);
            assert_eq!(converted, Some(currency));
        }
    }
}