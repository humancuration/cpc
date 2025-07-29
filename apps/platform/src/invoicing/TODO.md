This needs refactoring to Yew

# Invoice Management Frontend Implementation TODO

## 1. File Structure Creation
- [ ] Create `apps/cpc-platform/src/invoicing/types/` directory
- [ ] Create TypeScript interfaces matching Rust models
- [ ] Create `apps/cpc-platform/src/invoicing/services/` directory
- [ ] Create `apps/cpc-platform/src/invoicing/stores/` directory
- [ ] Create `apps/cpc-platform/src/invoicing/components/` directory

## 2. TypeScript Interfaces
- [ ] Create `types/invoice.types.ts` - Invoice, LineItem, InvoiceStatus
- [ ] Create `types/customer.types.ts` - Customer, Address
- [ ] Create `types/api.types.ts` - Request/Response DTOs matching api.rs

## 3. Service Layer
- [ ] Create `services/invoiceService.ts` - Tauri command wrappers
- [ ] Create `services/customerService.ts` - Customer CRUD operations
- [ ] Create `services/pdfService.ts` - PDF generation
- [ ] Create `services/syncService.ts` - Sync status and operations

## 4. State Management
- [ ] Create `stores/invoiceStore.ts` - Invoice state management
- [ ] Create `stores/customerStore.ts` - Customer state management
- [ ] Create `stores/syncStore.ts` - Sync state management

## 5. Components
- [ ] Create `components/InvoiceForm.svelte` - Invoice creation/editing
- [ ] Create `components/InvoiceList.svelte` - Paginated list with filters
- [ ] Create `components/CustomerManager.svelte` - Customer CRUD
- [ ] Create `components/PDFPreview.svelte` - PDF rendering
- [ ] Create `components/SyncStatus.svelte` - Sync indicators

## 6. UI Features
- [ ] Responsive layout with Tailwind CSS
- [ ] Form validation feedback
- [ ] Real-time updates via stores
- [ ] Loading states for async operations
- [ ] Error handling and user feedback

## 7. Integration
- [ ] Replace existing GraphQL calls with Tauri commands
- [ ] Update routes to use new components
- [ ] Test sync functionality
- [ ] Validate PDF generation