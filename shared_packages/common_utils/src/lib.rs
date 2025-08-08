//! # Common Utilities for CPC Platform
//!
//! This crate provides a collection of reusable utility functions and components
//! that can be shared across all CPC applications and services.
//!
//! ## Modules
//!
//! - `error`: Common error types and result aliases
//! - `logging`: Wrapper around tracing for consistent logging
//! - `serialization`: JSON and MessagePack serialization utilities
//! - `async_utils`: Common async utilities and patterns
//! - `crypto`: Cryptographic functions for hashing and encryption
//! - `datetime`: DateTime parsing and formatting utilities
//! - `data_structures`: Common data structures like LRU cache and ring buffer
//! - `financial`: High-precision financial calculations using fixed-point arithmetic

pub mod error;
pub mod logging;
pub mod serialization;
pub mod async_utils;
pub mod crypto;
pub mod datetime;
pub mod data_structures;
pub mod financial;

// Re-export the most commonly used items for convenience
pub use error::{CommonError, Result};
pub use logging::{trace, debug, info, warn, error};
pub use serialization::{to_json, from_json, to_msgpack, from_msgpack};
pub use async_utils::{retry, with_timeout};
pub use crypto::{hash_sha256, encrypt, decrypt};
pub use datetime::{now_utc, format_datetime};
pub use data_structures::{LruCache, RingBuffer};
pub use financial::{MonetaryValue, FinancialError, FinancialResult, RoundingStrategy};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reexports() {
        // Test that we can use the re-exported items
        let _error = CommonError::generic("test");
        let _now = now_utc();
        
        // Test that modules are accessible
        let _json_result = serialization::to_json(&"test");
        let _hash = crypto::hash_sha256("test");
        
        // Test financial module
        let _amount = financial::MonetaryValue::new(fixed::types::I64F64::from_num(100.50), "USD");
    }
}