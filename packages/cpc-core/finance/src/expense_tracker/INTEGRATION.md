# Expense Tracker Module Integration Guide

This document explains how to integrate the Expense Tracker module with the existing CPC finance system.

## Overview

The Expense Tracker module is designed as a vertical slice that can be integrated into the existing finance system with minimal disruption. It follows the same architectural patterns as the existing modules (Budget, Savings, Wallet, Rewards).

## Integration Steps

### 1. Database Migration

Run the database migration to create the required tables:

```bash
# Run the migration file
psql -f packages/cpc-core/migrations/20250728000015_add_expense_tracker_tables.sql
```

This will create three new tables:
- `expenses`: Stores expense transactions
- `receipts`: Stores receipt images and OCR data
- `expense_sharing_preferences`: Stores user sharing preferences

### 2. Repository Integration

The module provides PostgreSQL implementations of all required repositories:

- `PostgresExpenseRepository`
- `PostgresReceiptRepository`
- `PostgresExpenseSharingPreferenceRepository`

These can be instantiated and registered in your dependency injection container.

### 3. Service Integration

Create an instance of `ExpenseServiceImpl` with the required dependencies:

```rust
use cpc_core::finance::expense_tracker::application::expense_service::ExpenseServiceImpl;
use cpc_core::finance::infrastructure::database::expense_tracker_repositories::{
    PostgresExpenseRepository,
    PostgresReceiptRepository,
    PostgresExpenseSharingPreferenceRepository,
};

// Assuming you have instances of wallet_service and budget_service
let expense_repo = Arc::new(PostgresExpenseRepository::new(db_pool.clone()));
let receipt_repo = Arc::new(PostgresReceiptRepository::new(db_pool.clone()));
let sharing_preference_repo = Arc::new(PostgresExpenseSharingPreferenceRepository::new(db_pool.clone()));

let expense_service = ExpenseServiceImpl::new(
    expense_repo,
    receipt_repo,
    sharing_preference_repo,
    wallet_service,
    budget_service,
);
```

### 4. Application Layer Integration

The module is already registered in the application layer through the `mod.rs` file. No additional steps are needed here.

### 5. Domain Layer Integration

The module is already registered in the domain layer through the `mod.rs` file. No additional steps are needed here.

## Dependency Integrations

### Wallet Service Integration

The Expense Tracker automatically integrates with the Wallet Service to deduct Dabloons when expenses are created:

1. When `create_expense` is called with Dabloons currency
2. The service calls `wallet_service.subtract_dabloons()`
3. The wallet balance is updated
4. A transaction record is created

### Budget Service Integration

The Expense Tracker integrates with the Budget Service to update spent amounts:

1. When an expense is linked to a budget category
2. The service calls `budget_service.update_spent_with_dabloons()`
3. The budget's spent amount is updated

### p2panda Integration

The module uses p2panda for secure data sharing:

1. `P2PExpenseSharing` handles encryption/decryption
2. Double Ratchet encryption ensures secure communication
3. User consent is verified before sharing

### Bevy Integration

The module provides Bevy components for receipt scanning:

1. `ReceiptScannerPlugin` provides camera access
2. `capture_receipt_image()` captures images from the camera
3. Integration with OCR service for text extraction

## API Usage Examples

### Creating an Expense

```rust
use cpc_core::finance::domain::primitives::{Money, Currency};
use cpc_core::finance::domain::expense_tracker::ExpenseCategory;

let expense = expense_service.create_expense(
    user_id,
    Money::new(dec!(25.50), Currency::Dabloons),
    ExpenseCategory::Food,
    Utc::now(),
    "Dinner at restaurant".to_string()
).await?;
```

### Processing a Receipt

```rust
use cpc_core::finance::domain::expense_tracker::ReceiptImageData;
use cpc_core::finance::expense_tracker::infrastructure::ocr::receipt_processor::OCRService;

// Save receipt
let receipt = expense_service.save_receipt(
    user_id,
    ReceiptImageData::Base64Data("base64_image_data".to_string())
).await?;

// Process receipt with OCR
let ocr_service = OCRService::new(receipt_repo);
ocr_service.process_receipt(receipt.id).await?;
```

### Sharing an Expense

```rust
use cpc_core::finance::expense_tracker::infrastructure::p2p::expense_sharing::{P2PExpenseSharing, UserKeys};

// Set up p2p sharing
let user_keys = UserKeys {
    private_key: vec![], // User's private key
    public_key: vec![],  // User's public key
};

let p2p_sharing = P2PExpenseSharing::new(p2p_manager, user_keys, expense_service);

// Share expense
p2p_sharing.share_expense(
    expense_id,
    vec!["recipient_node_id".to_string()],
    user_id
).await?;
```

## Configuration

The module requires no special configuration beyond the standard CPC finance module setup.

## Testing

### Unit Tests

Run unit tests with:

```bash
cargo test -p cpc-core --lib expense_tracker
```

### Integration Tests

Integration tests require a test database. Run with:

```bash
cargo test -p cpc-core --lib expense_tracker_integration
```

## Error Handling

The module uses the standard `FinanceError` enum for error handling, which is consistent with other finance modules.

## Security Considerations

1. All shared data is encrypted with p2panda's Double Ratchet
2. User consent is required for all data sharing
3. Anonymization options are available for privacy
4. Federation-wide opt-out registry is respected

## Performance Considerations

1. Database indexes are included in the migration for common queries
2. Receipt images are stored as binary data for efficient retrieval
3. Expense queries are optimized with date range filtering

## Monitoring and Logging

The module follows the standard CPC logging patterns using the `tracing` crate.

## Troubleshooting

### Common Issues

1. **Database Connection Errors**: Verify database connection parameters
2. **Wallet Integration Issues**: Check that wallet service is properly initialized
3. **p2p Sharing Failures**: Verify p2panda network connectivity
4. **OCR Processing Failures**: Check Tesseract installation (when implemented)

### Debugging Tips

1. Enable debug logging for the finance module
2. Check database query logs for performance issues
3. Verify repository implementations with unit tests
4. Use integration tests to validate end-to-end functionality