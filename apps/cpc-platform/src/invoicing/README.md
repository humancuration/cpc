# Invoicing Module

A complete invoicing system for the CPC platform built with Svelte/TypeScript.

## Architecture

The invoicing module follows a clean architecture pattern with:
- **Services**: Business logic for API interactions
- **Stores**: Svelte stores for state management
- **Components**: Reusable UI components
- **Types**: Strongly typed interfaces matching backend models

## File Structure

```
invoicing/
├── services/
│   ├── invoiceService.ts     # Invoice CRUD operations
│   ├── customerService.ts    # Customer management
│   └── pdfService.ts         # PDF generation
├── stores/
│   ├── invoiceStore.ts       # Invoice state management
│   ├── customerStore.ts      # Customer state management
│   └── index.ts             # Store exports
├── components/
│   ├── InvoiceDashboard.svelte # Main dashboard
│   ├── InvoiceList.svelte     # Invoice listing
│   └── index.ts              # Component exports
├── types/
│   ├── invoice.types.ts      # Core invoice interfaces
│   ├── api.types.ts          # API request/response types
│   └── index.ts              # Type exports
└── README.md                 # This file
```

## Usage

### Basic Setup

```typescript
// Import stores and services
import { InvoiceService, CustomerService } from './invoicing/services';
import { invoices, invoiceActions } from './invoicing/stores/invoiceStore';
import InvoiceDashboard from './invoicing/components/InvoiceDashboard.svelte';
```

### Creating an Invoice

```typescript
import { InvoiceService } from './invoicing/services';

const request = {
  customer_id: 'customer-123',
  invoice_date: new Date().toISOString(),
  due_date: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
  line_items: [{
    description: 'Professional services',
    quantity: 5,
    unit_price: { amount: '150.00', currency: 'USD' }
  }]
};

const result = await InvoiceService.createInvoice(request);
if (result.success) {
  // Handle success
}
```

### Using Stores

```typescript
// Access invoice data
import { invoices, invoiceActions } from './invoicing/stores/invoiceStore';

// Subscribe to invoices
$: invoiceList = $invoices;

// Add new invoice
invoiceActions.addInvoice(newInvoice);

// Update existing invoice
invoiceActions.updateInvoice(updatedInvoice);

// Set loading state
invoiceActions.setLoading('invoices', true);
```

## Services

### InvoiceService

- `createInvoice(request: CreateInvoiceRequest): Promise<ApiResult<Invoice>>`
- `getInvoice(id: string): Promise<ApiResult<Invoice | null>>`
- `updateInvoice(request: UpdateInvoiceRequest): Promise<ApiResult<Invoice>>`
- `deleteInvoice(id: string): Promise<ApiResult<void>>`
- `listInvoices(status?, customerId?, limit?, offset?): Promise<ApiResult<InvoiceListResponse>>`
- `searchInvoices(query: string): Promise<ApiResult<InvoiceListResponse>>`

### CustomerService

- `createCustomer(request: CreateCustomerRequest): Promise<ApiResult<Customer>>`
- `getCustomer(id: string): Promise<ApiResult<Customer | null>>`
- `updateCustomer(request: UpdateCustomerRequest): Promise<ApiResult<Customer>>`
- `deleteCustomer(id: string): Promise<ApiResult<void>>`
- `listCustomers(limit?, offset?): Promise<ApiResult<{ customers: Customer[]; total_count: number }>>`

## Stores

### Invoice Store

- `invoices` - Writable store of all invoices
- `invoiceSummaries` - Derived store for summary views
- `invoiceStats` - Derived store for statistics
- `loading` - Loading states
- `errors` - Error states
- `selectedInvoice` - Currently selected invoice

### Customer Store

- `customers` - Writable store of all customers
- `customerOptions` - Derived store for dropdown options
- `customerMap` - Derived store for quick lookups
- `selectedCustomer` - Currently selected customer

## Components

### InvoiceDashboard

Main dashboard component showing:
- Invoice statistics
- Invoice list
- Quick actions

### InvoiceList

Table component displaying:
- Invoice details
- Status badges
- Action buttons
- Responsive design

## Type System

The module uses TypeScript interfaces that match the backend Rust models:

- `Invoice` - Complete invoice model
- `Customer` - Customer information
- `LineItem` - Individual line items
- `Money` - Currency amounts
- `Address` - Customer address

## Mock Implementation

All services currently use mock data for development:
- Realistic delay simulation (400-1000ms)
- UUID generation for IDs
- Proper error handling
- Type-safe responses

## Integration with Tauri

When Tauri integration is ready:
1. Replace mock implementations with Tauri commands
2. Update service methods to use Tauri invoke
3. Maintain the same API for seamless migration

## Next Steps

1. **Real Tauri Integration**: Connect to actual backend
2. **Invoice Form**: Create/edit invoice forms
3. **PDF Generation**: Integrate PDF service
4. **Customer Management**: Customer CRUD UI
5. **Payment Tracking**: Add payment processing
6. **Advanced Filtering**: Date ranges, status filters
7. **Export Features**: CSV, PDF exports
8. **Real-time Updates**: WebSocket notifications

## Development Notes

- All services are fully typed
- Store actions provide clear state management
- Components use Tailwind CSS for styling
- Mock data provides realistic development experience
- Error handling is implemented throughout