// DEPRECATED: This file will be removed on 2025-10-01
// Migrate to common_utils equivalents instead
//! Error compatibility shim for integrating common_utils
//!
//! This module provides compatibility between the CPay Core's PaymentError
//! and the common_utils::error::CommonError for backward compatibility
//! during the migration process.

use common_utils::error::CommonError;
use crate::models::PaymentError;

/// Implementation of From trait to convert PaymentError to CommonError
/// This allows existing code to work with the new error type while we
/// gradually migrate to using CommonError directly.
#[deprecated(since = "0.3.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<PaymentError> for CommonError {
    fn from(error: PaymentError) -> Self {
        error
    }
}

/// Implementation of From trait to convert CommonError to PaymentError
/// This is needed for backward compatibility in some cases where the
/// old error type is still expected.
#[deprecated(since = "0.3.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<CommonError> for PaymentError {
    fn from(error: CommonError) -> Self {
        error
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_payment_error_to_common_error() {
        let payment_error = PaymentError::generic("test error");
        let common_error: CommonError = payment_error.into();
        
        // Should be the same error since PaymentError is an alias to CommonError
        match common_error {
            CommonError::Generic(_) => assert!(true),
            _ => assert!(false, "Should be a Generic error"),
        }
    }
    
    #[test]
    fn test_common_error_to_payment_error() {
        let common_error = CommonError::generic("test error");
        let payment_error: PaymentError = common_error.into();
        
        // Should be the same error since PaymentError is an alias to CommonError
        match payment_error {
            PaymentError::Generic(_) => assert!(true),
            _ => assert!(false, "Should be a Generic error"),
        }
    }
}