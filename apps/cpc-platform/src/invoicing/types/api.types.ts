// TypeScript interfaces matching request/response DTOs from apps/cpc-platform/src-tauri/src/invoicing/api.rs

import type { Invoice, Customer, InvoiceStatus } from './invoice.types';

// Request DTOs
export interface CreateLineItemRequest {
  description: string;
  quantity: number;
  unit_price: {
    amount: string;
    currency: string;
  };
  tax_rate?: number;
}

export interface CreateInvoiceRequest {
  customer_id: string;
  invoice_date: string;
  due_date: string;
  line_items: CreateLineItemRequest[];
  notes?: string;
  terms?: string;
}

export interface UpdateInvoiceRequest {
  id: string;
  status?: InvoiceStatus;
  notes?: string;
  terms?: string;
}

export interface CreateCustomerRequest {
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  state?: string;
  postal_code?: string;
  country?: string;
  tax_id?: string;
}

export interface UpdateCustomerRequest {
  id: string;
  name?: string;
  email?: string;
  phone?: string;
  address?: string;
  city?: string;
  state?: string;
  postal_code?: string;
  country?: string;
  tax_id?: string;
}

export interface GeneratePdfRequest {
  invoice_id: string;
}

// Response DTOs
export interface InvoiceResponse {
  invoice: Invoice;
  customer: Customer;
}

export interface InvoiceListResponse {
  invoices: InvoiceResponse[];
  total_count: number;
}

export interface CustomerListResponse {
  customers: Customer[];
  total_count: number;
}

export interface GeneratePdfResponse {
  pdf_url: string;
  file_path: string;
}

// Sync types
export interface SyncResult {
  synced_invoices: number;
  synced_customers: number;
  conflicts_resolved: number;
  errors: string[];
}

export interface SyncState {
  last_sync: string | null;
  pending_changes: number;
  is_syncing: boolean;
  sync_error: string | null;
}

// Error handling
export interface ApiError {
  message: string;
  code?: string;
  details?: any;
}

export type ApiResult<T> = 
  | { success: true; data: T }
  | { success: false; error: ApiError };