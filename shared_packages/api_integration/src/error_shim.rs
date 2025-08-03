// DEPRECATED: This file will be removed on 2025-10-01
// Migrate to common_utils equivalents instead
//! Error compatibility shim for integrating common_utils
//!
//! This module provides compatibility between the API Integration's custom error types
//! and the common_utils::error::CommonError for backward compatibility
//! during the migration process.

use common_utils::error::CommonError;
use crate::application::request_routing::RequestRoutingError;
use crate::application::api_management::ApiManagementError;

/// Implementation of From trait to convert RequestRoutingError to CommonError
/// This allows existing code to work with the new error type while we
/// gradually migrate to using CommonError directly.
#[deprecated(since = "0.2.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<RequestRoutingError> for CommonError {
    fn from(error: RequestRoutingError) -> Self {
        CommonError::Generic(error.to_string())
    }
}

/// Implementation of From trait to convert ApiManagementError to CommonError
/// This allows existing code to work with the new error type while we
/// gradually migrate to using CommonError directly.
#[deprecated(since = "0.2.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<ApiManagementError> for CommonError {
    fn from(error: ApiManagementError) -> Self {
        CommonError::Generic(error.to_string())
    }
}

/// Implementation of From trait to convert CommonError to RequestRoutingError
/// This is needed for backward compatibility in some cases where the
/// old error type is still expected.
#[deprecated(since = "0.2.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<CommonError> for RequestRoutingError {
    fn from(error: CommonError) -> Self {
        match error {
            CommonError::Generic(msg) => RequestRoutingError::InternalError(msg),
            CommonError::Crypto(msg) => RequestRoutingError::AdapterError(msg),
            CommonError::InvalidInput(msg) => RequestRoutingError::TransformationError(msg),
            _ => RequestRoutingError::InternalError("Unknown error".to_string()),
        }
    }
}

/// Implementation of From trait to convert CommonError to ApiManagementError
/// This is needed for backward compatibility in some cases where the
/// old error type is still expected.
#[deprecated(since = "0.2.0", note = "Will be removed in 0.4.0 - use common_utils::error::CommonError directly")]
impl From<CommonError> for ApiManagementError {
    fn from(error: CommonError) -> Self {
        match error {
            CommonError::Generic(msg) => ApiManagementError::StorageError(msg),
            CommonError::Crypto(msg) => ApiManagementError::AdapterConfigError(
                crate::domain::adapter_config::AdapterConfigError::InvalidConfiguration(msg)
            ),
            CommonError::InvalidInput(msg) => ApiManagementError::ValidationError(msg),
            _ => ApiManagementError::StorageError("Unknown error".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::request_routing::RequestRoutingError;
    use crate::application::api_management::ApiManagementError;
    
    #[test]
    fn test_request_routing_error_to_common_error() {
        let routing_error = RequestRoutingError::EndpointNotFound("test".to_string());
        let common_error: CommonError = routing_error.into();
        
        // Should be converted to a generic error with the string representation
        match common_error {
            CommonError::Generic(_) => assert!(true),
            _ => assert!(false, "Should be converted to Generic error"),
        }
    }
    
    #[test]
    fn test_api_management_error_to_common_error() {
        let api_error = ApiManagementError::ValidationError("test".to_string());
        let common_error: CommonError = api_error.into();
        
        // Should be converted to a generic error with the string representation
        match common_error {
            CommonError::Generic(_) => assert!(true),
            _ => assert!(false, "Should be converted to Generic error"),
        }
    }
    
    #[test]
    fn test_common_error_to_request_routing_error() {
        let common_error = CommonError::generic("test error");
        let routing_error: RequestRoutingError = common_error.into();
        
        // Should be converted to an appropriate RequestRoutingError variant
        match routing_error {
            RequestRoutingError::InternalError(_) => assert!(true),
            _ => assert!(false, "Should be converted to InternalError"),
        }
    }
    
    #[test]
    fn test_common_error_to_api_management_error() {
        let common_error = CommonError::invalid_input("test error");
        let api_error: ApiManagementError = common_error.into();
        
        // Should be converted to an appropriate ApiManagementError variant
        match api_error {
            ApiManagementError::ValidationError(_) => assert!(true),
            _ => assert!(false, "Should be converted to ValidationError"),
        }
    }
}