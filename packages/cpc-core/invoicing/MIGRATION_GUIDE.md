# Invoicing Module Migration Guide

This guide provides instructions for migrating to the new Invoicing & Quoting module.

## Overview

The Invoicing & Quoting module has been implemented as a vertical slice within `packages/cpc-core/invoicing/` following the hexagonal architecture pattern.

## Migration Steps

### 1. Update Cargo.toml Dependencies

Add the new invoicing module as a dependency:

```toml
[dependencies]
cpc-invoicing = { path = "packages/cpc-core/invoicing" }
```

### 2. Update Feature Flags

If you were using the old invoicing implementation, update your feature flags:

```toml
[features]
invoicing = ["cpc-invoicing"]
```

### 3. Update Import Statements

Replace old import statements:

```rust
// Old
use cpc_core::invoicing::model::Invoice;
use cpc_core::invoicing::service::InvoiceService;

// New
use cpc_invoicing::domain::Invoice;
use cpc_invoicing::application::InvoiceService;
```

### 4. Database Migration

The new module uses a different database schema. Run the following SQL migrations:

```sql
-- Create invoices table
CREATE TABLE invoices (
    id UUID PRIMARY KEY,
    client_id UUID NOT NULL,
    client_name TEXT NOT NULL,
    client_email TEXT NOT NULL,
    items JSONB NOT NULL,
    total_amount DECIMAL NOT NULL,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Create quotes table
CREATE TABLE quotes (
    id UUID PRIMARY KEY,
    client_id UUID NOT NULL,
    client_name TEXT NOT NULL,
    client_email TEXT NOT NULL,
    items JSONB NOT NULL,
    total_amount DECIMAL NOT NULL,
    validity_period_days INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
```

### 5. Update Service Initialization

Update your service initialization code to use the new module structure:

```rust
// Old
let invoice_service = cpc_core::invoicing::service::InvoiceService::new();

// New
use cpc_invoicing::application::{InvoiceService, PgInvoiceRepository};
let repo = Arc::new(PgInvoiceRepository::new(db_pool));
let p2p_manager = Arc::new(P2PManager::new());
let invoice_service = InvoiceService::new(repo, p2p_manager);
```

## Breaking Changes

1. **Module Location**: The invoicing module has moved from `cpc_core::invoicing` to `cpc_invoicing`
2. **API Changes**: Many method signatures have changed to follow the new architecture
3. **Data Models**: The internal data models have been refactored for better consistency
4. **Feature Flags**: New feature flags are required for optional components

## New Features

1. **Quotes Support**: Full quote management functionality
2. **P2P Integration**: Secure data sharing using p2panda
3. **3D Visualizations**: Bevy-based invoice status visualization
4. **Web Components**: Yew-based web interface components
5. **Improved Architecture**: Clean separation of concerns with hexagonal architecture

## Support

For migration assistance, please contact the development team or refer to the documentation in `ARCHITECTURE.md`.