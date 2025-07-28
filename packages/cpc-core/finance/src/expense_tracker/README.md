# Expense Tracker Module

The Expense Tracker module provides comprehensive functionality for tracking personal expenses with dual-currency support (traditional currency + Dabloons), receipt scanning, automatic categorization, and secure p2p sharing with granular consent controls.

## Features

- **Dual-Currency Support**: Track expenses in both traditional currencies and Dabloons
- **Receipt Scanning**: Capture and process receipts using OCR technology
- **Automatic Categorization**: Automatically categorize expenses based on merchant and item data
- **Secure p2p Sharing**: Share expense data securely with other users using p2panda Double Ratchet encryption
- **Granular Consent Controls**: Fine-grained control over what expense data is shared and with whom
- **Time-Limited Sharing**: Control when expense data can be shared
- **Recipient-Specific Rules**: Set different sharing rules for different recipients
- **Privacy-Preserving**: Anonymization options for shared data

## Architecture

The module follows the Hexagonal Architecture pattern with a vertical slice implementation:

```
expense_tracker/
├── domain/                 # Core business logic and models
├── application/            # Application services and interfaces
├── infrastructure/         # Concrete implementations
│   ├── database/           # Database repositories
│   ├── p2p/                # p2p sharing implementation
│   ├── ocr/                # OCR receipt processing
│   └── bevy/               # Bevy integration for UI
└── presentation/           # UI components and presentation logic
```

## Key Components

### Domain Layer

- `Expense`: Represents a single expense transaction
- `Receipt`: Stores receipt image data and OCR results
- `ExpenseCategory`: Enum of expense categories
- `ExpenseSharingPreferences`: User preferences for data sharing, including time limits and recipient-specific rules

### Application Layer

- `ExpenseService`: Main interface for expense operations
- `ExpenseRepository`: Interface for expense persistence
- `ReceiptRepository`: Interface for receipt persistence

### Infrastructure Layer

- `PostgresExpenseRepository`: PostgreSQL implementation of ExpenseRepository
- `PostgresReceiptRepository`: PostgreSQL implementation of ReceiptRepository
- `P2PExpenseSharing`: Secure p2p sharing using p2panda
- `OCRService`: Receipt text extraction and parsing
- `ReceiptScannerPlugin`: Bevy integration for camera access

## Database Schema

The module uses three main tables:

1. `expenses`: Stores expense transactions
2. `receipts`: Stores receipt images and OCR data
3. `expense_sharing_preferences`: Stores user sharing preferences

## Usage Examples

### Creating an Expense

```rust
use cpc_core::finance::expense_tracker::application::expense_service::ExpenseService;
use cpc_core::finance::domain::primitives::{Money, Currency};
use cpc_core::finance::domain::expense_tracker::ExpenseCategory;

let expense = expense_service.create_expense(
    user_id,
    Money::new(dec!(15.75), Currency::USD),
    ExpenseCategory::Food,
    Utc::now(),
    "Lunch at cafe".to_string()
).await?;
```

### Processing a Receipt

```rust
use cpc_core::finance::expense_tracker::infrastructure::ocr::receipt_processor::OCRService;

ocr_service.process_receipt(receipt_id).await?;
```

### Sharing an Expense

```rust
use cpc_core::finance::expense_tracker::infrastructure::p2p::expense_sharing::P2PExpenseSharing;

p2p_sharing.share_expense(expense_id, vec![recipient_node_id], user_id).await?;
```

## Privacy and Security

The module implements several privacy and security features:

- **End-to-End Encryption**: All shared data is encrypted using p2panda's Double Ratchet
- **Granular Consent**: Users can control exactly what data is shared and with whom
- **Time-Limited Sharing**: Users can set time limits on when data can be shared
- **Recipient-Specific Rules**: Users can set different sharing rules for different recipients
- **Anonymization**: Option to share data without personally identifiable information
- **Federation-Wide Opt-Out**: Respects federation-wide data sharing preferences

## Integration Points

The module integrates with several other components:

- **Wallet Service**: Deducts Dabloons when expenses are created
- **Budget Service**: Updates budget spent amounts when expenses are linked to budgets
- **Bevy**: Provides camera access for receipt scanning
- **p2panda**: Enables secure p2p data sharing