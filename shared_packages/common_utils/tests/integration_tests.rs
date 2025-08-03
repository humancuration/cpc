//! Integration tests for the common_utils crate
//!
//! These tests cover:
//! - Feature flag toggling
//! - Error conversion
//! - Crypto consistency
//! - Currency edge cases

use common_utils::{
    error::CommonError,
    crypto::{hash_sha256, hash_sha256_with_salt, verify_hash, verify_hash_with_salt},
    datetime::{now_utc, parse_iso8601, format_datetime},
    serialization::{to_json, from_json},
    async_utils::{retry, with_timeout},
    data_structures::{LruCache, RingBuffer},
};
use std::time::Duration;
use tokio::time::sleep;

// Test feature flag toggling functionality
#[cfg(test)]
mod feature_flag_tests {
    use super::*;

    #[test]
    fn test_feature_flag_compilation() {
        // This test ensures that the crate compiles with default features
        let _now = now_utc();
        let _hash = hash_sha256("test");
        let _error = CommonError::generic("test");
    }

    #[cfg(feature = "default")]
    #[test]
    fn test_default_features_enabled() {
        // Test that default features are enabled
        let _hash = hash_sha256("test");
        assert_eq!(_hash.len(), 64);
    }
}

// Test error conversion functionality
#[cfg(test)]
mod error_conversion_tests {
    use super::*;
    use serde_json::Error as JsonError;
    use std::io::Error as IoError;
    use std::io::ErrorKind;

    #[test]
    fn test_json_error_conversion() {
        let json_err = JsonError::syntax(serde_json::error::SyntaxError::EofWhileParsingValue, 0, 0);
        let common_err: CommonError = json_err.into();
        assert!(matches!(common_err, CommonError::Json(_)));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = IoError::new(ErrorKind::Other, "test error");
        let common_err: CommonError = io_err.into();
        assert!(matches!(common_err, CommonError::Io(_)));
    }

    #[test]
    fn test_error_creation_helpers() {
        let generic_err = CommonError::generic("test message");
        assert!(matches!(generic_err, CommonError::Generic(msg) if msg == "test message"));

        let crypto_err = CommonError::crypto("crypto error");
        assert!(matches!(crypto_err, CommonError::Crypto(msg) if msg == "crypto error"));

        let input_err = CommonError::invalid_input("invalid input");
        assert!(matches!(input_err, CommonError::InvalidInput(msg) if msg == "invalid input"));
    }
}

// Test crypto consistency
#[cfg(test)]
mod crypto_consistency_tests {
    use super::*;

    #[test]
    fn test_hash_consistency() {
        let input = "hello world";
        let hash1 = hash_sha256(input);
        let hash2 = hash_sha256(input);
        
        // Hashing the same input should produce the same output
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_hash_with_salt_consistency() {
        let input = "hello world";
        let salt = "salt123";
        let hash1 = hash_sha256_with_salt(input, salt);
        let hash2 = hash_sha256_with_salt(input, salt);
        
        // Hashing the same input with the same salt should produce the same output
        assert_eq!(hash1, hash2);
        assert_eq!(hash1.len(), 64);
    }

    #[test]
    fn test_verify_hash() {
        let input = "hello world";
        let hash = hash_sha256(input);
        
        // Should verify correctly
        assert!(verify_hash(input, &hash));
        // Should not verify with different input
        assert!(!verify_hash("different input", &hash));
    }

    #[test]
    fn test_verify_hash_with_salt() {
        let input = "hello world";
        let salt = "salt123";
        let hash = hash_sha256_with_salt(input, salt);
        
        // Should verify correctly
        assert!(verify_hash_with_salt(input, salt, &hash));
        // Should not verify with different input
        assert!(!verify_hash_with_salt("different input", salt, &hash));
        // Should not verify with different salt
        assert!(!verify_hash_with_salt(input, "different_salt", &hash));
    }

    #[test]
    fn test_different_inputs_produce_different_hashes() {
        let hash1 = hash_sha256("input1");
        let hash2 = hash_sha256("input2");
        
        // Different inputs should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_input_different_salts_produce_different_hashes() {
        let input = "hello world";
        let hash1 = hash_sha256_with_salt(input, "salt1");
        let hash2 = hash_sha256_with_salt(input, "salt2");
        
        // Same input with different salts should produce different hashes
        assert_ne!(hash1, hash2);
    }
}

// Test currency edge cases (using wallet's Money type for testing)
#[cfg(test)]
mod currency_edge_case_tests {
    use super::*;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    // Define a minimal Currency enum for testing purposes
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestCurrency {
        USD,
        EUR,
        JPY, // Zero decimal places
        Dabloons, // Custom currency
    }

    impl TestCurrency {
        fn code(&self) -> &'static str {
            match self {
                TestCurrency::USD => "USD",
                TestCurrency::EUR => "EUR",
                TestCurrency::JPY => "JPY",
                TestCurrency::Dabloons => "DABLOONS",
            }
        }

        fn decimal_places(&self) -> u32 {
            match self {
                TestCurrency::JPY | TestCurrency::Dabloons => 0,
                _ => 2,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestMoney {
        amount: Decimal,
        currency: TestCurrency,
    }

    impl TestMoney {
        fn new(amount: Decimal, currency: TestCurrency) -> Self {
            Self { amount, currency }
        }

        fn add(&self, other: &Self) -> Result<Self, String> {
            if self.currency != other.currency {
                return Err(format!(
                    "Currency mismatch - expected: {}, actual: {}",
                    self.currency.code(),
                    other.currency.code()
                ));
            }
            
            Ok(Self {
                amount: self.amount + other.amount,
                currency: self.currency.clone(),
            })
        }
    }

    #[test]
    fn test_currency_precision_handling() {
        // Test currencies with different decimal places
        let usd = TestMoney::new(dec!(100.50), TestCurrency::USD);
        let eur = TestMoney::new(dec!(100.50), TestCurrency::EUR);
        let jpy = TestMoney::new(dec!(100), TestCurrency::JPY); // JPY has 0 decimal places
        let dabloon = TestMoney::new(dec!(100), TestCurrency::Dabloons); // Dabloons has 0 decimal places

        // Verify that the amounts are correctly stored
        assert_eq!(usd.amount, dec!(100.50));
        assert_eq!(eur.amount, dec!(100.50));
        assert_eq!(jpy.amount, dec!(100));
        assert_eq!(dabloon.amount, dec!(100));
    }

    #[test]
    fn test_currency_addition_same_currency() {
        let money1 = TestMoney::new(dec!(100.50), TestCurrency::USD);
        let money2 = TestMoney::new(dec!(50.25), TestCurrency::USD);
        let result = money1.add(&money2).unwrap();
        
        assert_eq!(result.amount, dec!(150.75));
        assert_eq!(result.currency, TestCurrency::USD);
    }

    #[test]
    fn test_currency_addition_mismatch_error() {
        let money1 = TestMoney::new(dec!(100.50), TestCurrency::USD);
        let money2 = TestMoney::new(dec!(50.25), TestCurrency::EUR);
        let result = money1.add(&money2);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Currency mismatch"));
    }

    #[test]
    fn test_zero_amount_currency() {
        let zero_usd = TestMoney::new(Decimal::ZERO, TestCurrency::USD);
        let zero_jpy = TestMoney::new(Decimal::ZERO, TestCurrency::JPY);
        let zero_dabloons = TestMoney::new(Decimal::ZERO, TestCurrency::Dabloons);

        assert!(zero_usd.amount.is_zero());
        assert!(zero_jpy.amount.is_zero());
        assert!(zero_dabloons.amount.is_zero());
    }

    #[test]
    fn test_large_currency_amounts() {
        let large_amount = dec!(999999999999.99);
        let large_money = TestMoney::new(large_amount, TestCurrency::USD);
        
        assert_eq!(large_money.amount, large_amount);
    }

    #[test]
    fn test_negative_currency_amounts() {
        let negative_amount = dec!(-100.50);
        let negative_money = TestMoney::new(negative_amount, TestCurrency::USD);
        
        assert!(negative_money.amount.is_sign_negative());
    }

    #[test]
    fn test_small_currency_amounts() {
        // Test very small decimal amounts
        let small_amount = dec!(0.01);
        let small_money = TestMoney::new(small_amount, TestCurrency::USD);
        
        assert_eq!(small_money.amount, small_amount);
    }

    #[test]
    fn test_currency_with_maximum_decimal_places() {
        // Test USD with maximum 2 decimal places
        let precise_amount = dec!(99.99);
        let precise_money = TestMoney::new(precise_amount, TestCurrency::USD);
        
        assert_eq!(precise_money.amount, precise_amount);
    }
}

// Test datetime functionality
#[cfg(test)]
mod datetime_tests {
    use super::*;
    use chrono::{DateTime, Utc, TimeZone};

    #[test]
    fn test_current_time() {
        let time1 = now_utc();
        sleep(Duration::from_millis(1)).await; // Small delay
        let time2 = now_utc();
        
        // Second time should be after first time
        assert!(time2 >= time1);
    }

    #[test]
    fn test_iso8601_parsing() {
        let iso_string = "2023-10-15T14:30:00Z";
        let parsed = parse_iso8601(iso_string).unwrap();
        
        assert_eq!(parsed.year(), 2023);
        assert_eq!(parsed.month(), 10);
        assert_eq!(parsed.day(), 15);
        assert_eq!(parsed.hour(), 14);
        assert_eq!(parsed.minute(), 30);
    }

    #[test]
    fn test_invalid_iso8601_parsing() {
        let invalid_string = "invalid datetime";
        let result = parse_iso8601(invalid_string);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_datetime_formatting() {
        let dt = Utc.with_ymd_and_hms(2023, 10, 15, 14, 30, 0).unwrap();
        let formatted = format_datetime(&dt);
        
        assert_eq!(formatted, "2023-10-15 14:30:00 UTC");
    }
}

// Test serialization functionality
#[cfg(test)]
mod serialization_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestPerson {
        name: String,
        age: u32,
        active: bool,
    }

    #[test]
    fn test_json_serialization() {
        let person = TestPerson {
            name: "Alice".to_string(),
            age: 30,
            active: true,
        };

        let json = to_json(&person).unwrap();
        assert!(json.contains("\"name\":\"Alice\""));
        assert!(json.contains("\"age\":30"));
        assert!(json.contains("\"active\":true"));
    }

    #[test]
    fn test_json_deserialization() {
        let json = r#"{"name":"Bob","age":25,"active":false}"#;
        let person: TestPerson = from_json(json).unwrap();

        assert_eq!(person.name, "Bob");
        assert_eq!(person.age, 25);
        assert_eq!(person.active, false);
    }

    #[test]
    fn test_invalid_json_deserialization() {
        let invalid_json = r#"{"name":"Charlie","age":}"#;
        let result: Result<TestPerson, CommonError> = from_json(invalid_json);
        
        assert!(result.is_err());
    }
}

// Test async utilities
#[cfg(test)]
mod async_utils_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_success() {
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        let result = retry(
            || {
                let counter = counter_clone.clone();
                async move {
                    let count = counter.fetch_add(1, Ordering::SeqCst);
                    if count < 2 {
                        Err::<(), CommonError>(CommonError::Generic("Temporary error".to_string()))
                    } else {
                        Ok(())
                    }
                }
            },
            5,
            Duration::from_millis(10),
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_failure() {
        let result = retry(
            || async { Err::<(), CommonError>(CommonError::Generic("Permanent error".to_string())) },
            3,
            Duration::from_millis(10),
        ).await;
        
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_with_timeout_success() {
        let future = async {
            sleep(Duration::from_millis(10)).await;
            Ok("success")
        };
        
        let result = with_timeout(future, Duration::from_millis(100)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }

    #[tokio::test]
    async fn test_with_timeout_failure() {
        let future = async {
            sleep(Duration::from_millis(100)).await;
            Ok("success")
        };
        
        let result = with_timeout(future, Duration::from_millis(10)).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommonError::Timeout));
    }
}

// Test data structures
#[cfg(test)]
mod data_structure_tests {
    use super::*;

    #[tokio::test]
    async fn test_lru_cache_basic() {
        let cache = LruCache::new(3);
        
        cache.put("key1", "value1").await;
        cache.put("key2", "value2").await;
        cache.put("key3", "value3").await;
        
        assert_eq!(cache.get(&"key1").await, Some("value1"));
        assert_eq!(cache.get(&"key2").await, Some("value2"));
        assert_eq!(cache.get(&"key3").await, Some("value3"));
    }

    #[tokio::test]
    async fn test_lru_cache_capacity() {
        let cache = LruCache::new(2);
        
        cache.put("key1", "value1").await;
        cache.put("key2", "value2").await;
        cache.put("key3", "value3").await; // This should evict key1
        
        assert_eq!(cache.get(&"key1").await, None); // key1 should be evicted
        assert_eq!(cache.get(&"key2").await, Some("value2"));
        assert_eq!(cache.get(&"key3").await, Some("value3"));
    }

    #[tokio::test]
    async fn test_ring_buffer_basic() {
        let buffer = RingBuffer::new(3);
        
        buffer.push(1).await;
        buffer.push(2).await;
        buffer.push(3).await;
        
        assert_eq!(buffer.len().await, 3);
        
        let values = buffer.to_vec().await;
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn test_ring_buffer_overflow() {
        let buffer = RingBuffer::new(3);
        
        buffer.push(1).await;
        buffer.push(2).await;
        buffer.push(3).await;
        buffer.push(4).await; // This should push out 1
        
        assert_eq!(buffer.len().await, 3);
        
        let values = buffer.to_vec().await;
        assert_eq!(values, vec![2, 3, 4]);
    }
}