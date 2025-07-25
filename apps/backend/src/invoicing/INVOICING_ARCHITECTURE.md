# Invoicing System Architecture

This document outlines the end-to-end architecture for the invoicing system within the CPC platform.

## 1. Architectural Principles

The invoicing system will adhere to the overarching architectural principles of the CPC platform:

- **Hexagonal Architecture:** The core business logic for invoicing will be isolated from external concerns like databases, APIs, and UI. This will be achieved by defining ports (interfaces) for data persistence and external services, and adapters that implement these ports.
- **Screaming Architecture:** The project structure will clearly communicate its purpose. The `invoicing` module will contain all the necessary components for this feature.
- **Vertical Slices:** Each feature or use case (e.g., "Create Invoice," "Process Payment") will be implemented as a vertical slice, touching all layers of the architecture from the UI to the database.

## 2. Core Components

The invoicing system will be built within the `apps/backend` and `packages/cpc-core` crates.

### 2.1. `cpc-core`: Domain Logic

The `packages/cpc-core/src/invoicing` module will contain the pure business logic and domain models.

- **`models.rs`**:
    - `Invoice`: Represents an invoice with its header, line items, totals, and status.
    - `InvoiceLineItem`: A single line item on an invoice.
    - `Customer`: Represents a customer to whom an invoice is issued.
    - `Payment`: Represents a payment made against an invoice.
    - `TaxRate`: Represents a tax rate that can be applied to line items.
- **`services.rs`**:
    - `InvoiceService`: Contains the core business logic for creating, updating, and managing invoices.
    - `PaymentService`: Handles payment processing and allocation.
    - `ReportingService`: Generates reports and analytics.
- **`ports.rs`**:
    - `InvoiceRepository`: Defines the interface for persisting and retrieving invoice data.
    - `CustomerRepository`: Defines the interface for customer data.
    - `PaymentRepository`: Defines the interface for payment data.

### 2.2. `apps/backend`: Application & Infrastructure

The `apps/backend/src/invoicing` module will implement the application layer and infrastructure concerns.

- **`graphql.rs`**: Defines the GraphQL schema, queries, mutations, and subscriptions for invoicing.
- **`db.rs`**: Implements the `InvoiceRepository`, `CustomerRepository`, and `PaymentRepository` ports using `SQLx` to interact with the database.
- **`pdf.rs`**: Implements the PDF generation logic using `pdf-rs`.
- **`jobs.rs`**: Handles long-running tasks like sending recurring invoices or processing late fees.

## 3. Database Schema (SQLx)

The database schema will be designed to support the required features.

```sql
-- Organizations Table
CREATE TABLE organizations (
    id BIGSERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Users & Permissions
-- (Leveraging existing user/auth system)
-- A join table will map users to organizations with specific roles.
CREATE TABLE organization_users (
    user_id BIGINT REFERENCES users(id),
    organization_id BIGINT REFERENCES organizations(id),
    role TEXT NOT NULL, -- e.g., 'creator', 'approver', 'payer', 'admin'
    PRIMARY KEY (user_id, organization_id)
);

-- Customers Table
CREATE TABLE customers (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    name TEXT NOT NULL,
    email TEXT,
    address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Invoices Table
CREATE TABLE invoices (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    invoice_number TEXT NOT NULL,
    status TEXT NOT NULL, -- 'draft', 'sent', 'paid', 'partial', 'void'
    currency VARCHAR(3) NOT NULL,
    issue_date DATE NOT NULL,
    due_date DATE NOT NULL,
    notes TEXT,
    subtotal DECIMAL(10, 2) NOT NULL,
    tax_total DECIMAL(10, 2) NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (organization_id, invoice_number)
);

-- Invoice Line Items Table
CREATE TABLE invoice_line_items (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    quantity DECIMAL(10, 2) NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    tax_rate DECIMAL(5, 2), -- e.g., 8.25 for 8.25%
    total DECIMAL(10, 2) NOT NULL
);

-- Payments Table
CREATE TABLE payments (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    payment_date DATE NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    payment_method TEXT, -- 'credit_card', 'bank_transfer', etc.
    transaction_id TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Recurring Invoices Table
CREATE TABLE recurring_invoices (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    customer_id BIGINT NOT NULL REFERENCES customers(id),
    frequency TEXT NOT NULL, -- 'daily', 'weekly', 'monthly', 'yearly'
    start_date DATE NOT NULL,
    end_date DATE,
    last_sent_date DATE,
    template JSONB NOT NULL, -- A template for the invoice to be created
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Audit Trail
CREATE TABLE invoice_audit_trail (
    id BIGSERIAL PRIMARY KEY,
    invoice_id BIGINT NOT NULL REFERENCES invoices(id),
    user_id BIGINT REFERENCES users(id),
    action TEXT NOT NULL, -- 'created', 'sent', 'payment_received', 'updated'
    details JSONB,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 3.1. Database Migration Plan

Migrations will be managed using `sqlx-cli`.

1.  `001_create_initial_invoicing_tables.sql`: Create all the tables defined above.
2.  Future migrations will handle schema alterations as new requirements arise.

## 4. GraphQL API (`schema.invoicing.graphql`)

The GraphQL API will provide the interface for the UI.

- **Types:** `Invoice`, `Customer`, `Payment`, `InvoiceLineItem`, etc.
- **Mutations:**
    - `createInvoice(input: CreateInvoiceInput!): Invoice`
    - `updateInvoice(id: ID!, input: UpdateInvoiceInput!): Invoice`
    - `sendInvoice(id: ID!): Invoice`
    - `recordPayment(input: RecordPaymentInput!): Payment`
- **Queries:**
    - `invoice(id: ID!): Invoice`
    - `invoices(filter: InvoiceFilterInput): [Invoice!]!`
    - `customers: [Customer!]!`
- **Subscriptions:**
    - `invoiceUpdated(id: ID!): Invoice`

## 5. PDF Generation

- A `generate_invoice_pdf` function will be created in `apps/backend/src/invoicing/pdf.rs`.
- It will take an `Invoice` object as input.
- It will use the `pdf-rs` library to create a professional-looking PDF document.
- The PDF will be returned as a byte stream or a temporary URL.

## 6. Permissions and Roles

- **Invoice Creator:** Can create and manage their own draft invoices.
- **Approver:** Can approve invoices, allowing them to be sent.
- **Payer:** Can view invoices and make payments.
- **Admin:** Full access to all invoicing features for their organization.

Permissions will be checked in the GraphQL resolver layer, based on the user's role within their organization.

## 7. State Management (Yew UI)

- **Stores:** Yewdux or a similar state management library will be used.
    - `InvoiceStore`: Caches invoices, handles loading states.
    - `CustomerStore`: Manages customer data.
- **Components:**
    - `InvoiceForm`: For creating and editing invoices.
    - `InvoiceList`: Displays a list of invoices with filtering and sorting.
    - `InvoiceDetails`: Shows a single invoice and its payment history.
    - `PaymentForm`: For recording payments.

## 8. Integration with Accounting Systems

- A generic `AccountingExport` service will be designed.
- It will initially support exporting data to CSV format.
- Adapters can be created for specific accounting systems (e.g., QuickBooks, Xero) in the future.
- Webhooks can be used to notify external systems of invoice events.