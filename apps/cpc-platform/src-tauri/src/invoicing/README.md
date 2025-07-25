# Invoicing System - Tauri Implementation

This directory contains the complete Tauri-based implementation of the CPC invoicing system, built according to ADR 0006.

## Architecture Overview

The invoicing system follows hexagonal architecture principles with the following layers:

- **Domain Layer** (`cpc-core`): Core business logic and models
- **Application Layer** (`invoicing/api.rs`): Tauri command handlers and business services
- **Infrastructure Layer** (`invoicing/repository.rs`, `invoicing/pdf_generator.rs`): Concrete implementations

## Components

### 1. Core Models (`cpc-core`)
- `Invoice`: Main invoice entity with line items
- `Customer`: Customer information
- `LineItem`: Individual line items on invoices
- `Money`: Type-safe money handling

### 2. Repositories (`invoicing/repository.rs`)
- `InMemoryInvoiceRepository`: Development/testing repository
- `InMemoryCustomerRepository`: Development/testing repository  
- `InMemorySyncRepository`: Development/testing repository

### 3. PDF Generator (`invoicing/pdf_generator.rs`)
- HTML-based PDF generation (placeholder for pdf integration)
- Generates invoices in PDF format with professional layout

### 4. Sync Service (`invoicing/sync.rs`)
- Conflict resolution for offline-first data
- Last-writer-wins strategy
- Manual conflict resolution support

### 5. API Service (`invoicing/api.rs`)
- Complete Tauri command handlers
- RESTful API for invoices, customers, and PDF generation
- Sync operations

## API Endpoints

### Invoices
- `create_invoice` - Create new invoice
- `get_invoice` - Get invoice by ID
- `update_invoice` - Update invoice
- `delete_invoice` - Delete invoice
- `list_invoices` - List invoices with filtering
- `search_invoices` - Search invoices

### Customers
- `create_customer` - Create new customer
- `get_customer` - Get customer by ID
- `update_customer` - Update customer
- `delete_customer` - Delete customer
- `list_customers` - List customers
- `search_customers` - Search customers

### PDF Operations
- `generate_pdf` - Generate PDF for invoice

### Sync Operations
- `sync_invoices` - Sync invoices with server
- `get_sync_state` - Get current sync state

## Usage Example (Yew Frontend)

```javascript
// Create a customer
const customer = await invoke('create_customer', {
  request: {
    name: 'John Doe',
    email: 'john@example.com',
    phone: '+1234567890'
  }
});

// Create an invoice
const invoice = await invoke('create_invoice', {
  request: {
    customer_id: customer.id,
    invoice_date: new Date().toISOString(),
    due_date: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    line_items: [{
      description: 'Software Development',
      quantity: 10,
      unit_price: { mantissa: 15000, exponent: 2 }, // $150.00
      tax_rate: 0.1
    }]
  }
});

// Generate PDF
const pdf = await invoke('generate_pdf', {
  request: { invoice_id: invoice.invoice.id }
});
```

## Development Setup

1. Ensure you have Rust and Tauri CLI installed
2. Run the development server:
   ```bash
   cargo tauri dev
   ```

## Testing

Run the test suite:
```bash
cargo test --package pds --test invoicing_test
```

## Future Enhancements

- SQLx database integration
- Actual PDF generation with pdf-rs
- Network sync implementation
- Advanced conflict resolution
- Invoice templates system
- Multi-currency support
- Tax calculation engine

## Security Considerations

- All data is currently stored in-memory (development mode)
- PDFs are generated to temporary directory
- No encryption implemented yet (development mode)
- Future: Implement local encryption and secure storage