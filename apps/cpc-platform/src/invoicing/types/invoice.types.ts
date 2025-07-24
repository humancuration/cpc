// TypeScript interfaces matching Rust models from packages/cpc-core/src/invoicing/model.rs

export interface Money {
  amount: string;
  currency: string;
}

export enum InvoiceStatus {
  Draft = 'Draft',
  Sent = 'Sent',
  Viewed = 'Viewed',
  Paid = 'Paid',
  Overdue = 'Overdue',
  Cancelled = 'Cancelled',
}

export interface LineItem {
  id: string;
  description: string;
  quantity: number;
  unit_price: Money;
  total: Money;
  tax_rate?: number;
  metadata: Record<string, string>;
}

export interface Address {
  street: string;
  city: string;
  state: string;
  postal_code: string;
  country: string;
}

export interface Customer {
  id: string;
  name: string;
  email?: string;
  phone?: string;
  address?: Address;
  tax_id?: string;
  metadata: Record<string, string>;
  created_at: string;
  updated_at: string;
}

export interface Invoice {
  id: string;
  number: string;
  customer_id: string;
  customer: Customer;
  line_items: LineItem[];
  status: InvoiceStatus;
  issue_date: string;
  due_date: string;
  paid_date?: string;
  subtotal: Money;
  tax_amount: Money;
  total: Money;
  notes?: string;
  terms?: string;
  metadata: Record<string, string>;
  created_at: string;
  updated_at: string;
  sync_version: number;
}

export interface InvoiceFilter {
  customer_id?: string;
  status?: InvoiceStatus;
  date_from?: string;
  date_to?: string;
  search_term?: string;
}

// Helper types for UI state
export interface InvoiceSummary {
  id: string;
  number: string;
  customer_name: string;
  total: Money;
  status: InvoiceStatus;
  issue_date: string;
  due_date: string;
  balance_due: Money;
  is_overdue: boolean;
}

export interface InvoiceTotals {
  subtotal: Money;
  tax_amount: Money;
  total: Money;
}