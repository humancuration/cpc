# Developer Guide: Testing

*Last updated: 2025-07-23*

This guide provides an overview of the testing strategies and methodologies used in the CPC platform.

## Testing Philosophy

We believe in a comprehensive testing approach that combines unit, integration, and end-to-end (E2E) tests to ensure the quality and reliability of our software.

*   **Unit Tests**: These tests focus on individual components or functions in isolation. They are fast to run and help us verify that the basic building blocks of our system work as expected.
*   **Integration Tests**: These tests verify that different parts of the system work together correctly. For example, we have integration tests that check the interaction between our backend services and the database.
*   **End-to-End (E2E) Tests**: These tests simulate real user scenarios and verify that the entire system works as expected, from the UI to the backend and back.

## Testing Media Features

The media-post integration is a critical feature, and we have a dedicated set of tests to ensure its functionality.

### Backend Testing

The backend tests for the media feature can be found in `apps/backend/tests/`.

*   **`media_post_integration.rs`**: This file contains integration tests that verify the core logic of associating media with posts. These tests focus on the service and repository layers, ensuring that the database interactions are correct.
*   **`e2e_media_post_workflow.rs`**: This file contains end-to-end tests that simulate the entire media post workflow from a backend perspective. These tests make GraphQL requests to a running instance of the backend and verify that the entire process works as expected, including media processing notifications.

### Frontend (E2E) Testing

The frontend E2E tests are located in `apps/cpc-platform/cypress/e2e/`.

*   **`media_post_workflow.cy.js`**: This Cypress test file simulates a user attaching media to a post in the browser. It covers:
    *   Uploading a file.
    *   Waiting for the processing to complete by listening for UI changes.
    *   Submitting the post.
    *   Verifying that the post appears in the feed with the correct media attached.

These tests are crucial for ensuring a seamless user experience and are run as part of our continuous integration (CI) pipeline.
## Error Handling Examples

### Handling Camera Permission Errors

```rust
use crate::types::product::{BarcodeError, BarcodeErrorCode};
use crate::services::camera::CameraService;

async fn handle_permission() {
    match CameraService::ensure_permission().await {
        Ok(_) => {
            // Proceed with camera access
        }
        Err(e) => {
            match e.code {
                BarcodeErrorCode::CameraPermissionDenied => {
                    // Show user instructions to enable camera access
                }
                BarcodeErrorCode::CameraNotAvailable => {
                    // Show alternative input methods
                }
                _ => {
                    // Handle other errors
                }
            }
        }
    }
}
```

### Handling Barcode Scanning Errors

```rust
use crate::services::barcode::BarcodeService;
use crate::types::product::{BarcodeError, BarcodeErrorCode};

async fn scan_product(barcode: &str) {
    match BarcodeService::scan_barcode(barcode).await {
        Ok(product) => {
            // Process product data
        }
        Err(e) => {
            match e.code {
                BarcodeErrorCode::InvalidBarcodeFormat => {
                    // Show format error to user
                }
                BarcodeErrorCode::ScanTimeout => {
                    // Suggest retry or manual entry
                }
                BarcodeErrorCode::NotFound => {
                    // Offer to add new product
                }
                _ => {
                    // Handle other errors
                }
            }
        }
    }
}
```

### Error Recovery Strategies

1. **Automatic retry**: For transient errors like network issues
2. **User guidance**: Clear instructions for permission errors
3. **Alternative flows**: Manual entry when scanning fails
4. **Error-specific UI**: Tailored messages for each error type