// DEPRECATED: This file will be removed on 2025-10-01
// Migrate to common_utils equivalents instead
//! Error compatibility shim for integrating common_utils
//!
//! This module provides compatibility between the wallet's FinancialError
//! and the common_utils::error::CommonError for backward compatibility
//! during the migration process.

use common_utils::error::CommonError;
use crate::domain::primitives::FinancialError;

/// Implementation of From trait to convert FinancialError to CommonError
/// This allows existing code to work with the new error type while we
/// gradually migrate to using CommonError directly.
#[deprecated(since = "0.2.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<FinancialError> for CommonError {
    fn from(error: FinancialError) -> Self {
        CommonError::Generic(error.to_string())
    }
}

/// Implementation of From trait to convert CommonError to FinancialError
/// This is needed for backward compatibility in some cases where the
/// old error type is still expected.
#[deprecated(since = "0.2.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<CommonError> for FinancialError {
    fn from(error: CommonError) -> Self {
        match error {
            CommonError::Generic(msg) => FinancialError::InvalidCurrency,
            CommonError::Crypto(msg) => FinancialError::InvalidAmount,
            CommonError::InvalidInput(msg) => FinancialError::InvalidAmount,
            _ => FinancialError::InvalidCurrency,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_financial_error_to_common_error() {
        let financial_error = FinancialError::InvalidAmount;
        let common_error: CommonError = financial_error.into();
        
        // Should be converted to a generic error with the string representation
        match common_error {
            CommonError::Generic(_) => assert!(true),
            _ => assert!(false, "Should be converted to Generic error"),
        }
    }
    
    #[test]
    fn test_common_error_to_financial_error() {
        let common_error = CommonError::generic("test error");
        let financial_error: FinancialError = common_error.into();
        
        // Should be converted to an appropriate FinancialError variant
        match financial_error {
            FinancialError::InvalidCurrency => assert!(true),
            _ => assert!(false, "Should be converted to InvalidCurrency"),
        }
    }
}