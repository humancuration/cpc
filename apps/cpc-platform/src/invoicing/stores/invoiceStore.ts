import { writable, derived } from 'svelte/store';
import type { Invoice, InvoiceSummary, InvoiceFilter } from '../types';

// Main store for all invoices
export const invoices = writable<Invoice[]>([]);

// Store for loading states
export const loading = writable({
  invoices: false,
  customers: false,
  pdf: false,
  sync: false
});

// Store for errors
export const errors = writable({
  invoices: null as string | null,
  customers: null as string | null,
  pdf: null as string | null,
  sync: null as string | null
});

// Store for filters
export const filters = writable<InvoiceFilter>({});

// Store for selected invoice
export const selectedInvoice = writable<Invoice | null>(null);

// Derived store for invoice summaries
export const invoiceSummaries = derived(
  [invoices, filters],
  ([$invoices, $filters]) => {
    let filtered = $invoices;

    // Apply filters
    if ($filters.customer_id) {
      filtered = filtered.filter(inv => inv.customer_id === $filters.customer_id);
    }
    if ($filters.status) {
      filtered = filtered.filter(inv => inv.status === $filters.status);
    }
    if ($filters.date_from) {
      filtered = filtered.filter(inv => inv.issue_date >= $filters.date_from!);
    }
    if ($filters.date_to) {
      filtered = filtered.filter(inv => inv.issue_date <= $filters.date_to!);
    }
    if ($filters.search_term) {
      const term = $filters.search_term.toLowerCase();
      filtered = filtered.filter(inv => 
        inv.number.toLowerCase().includes(term) ||
        inv.customer.name.toLowerCase().includes(term) ||
        inv.line_items.some(item => item.description.toLowerCase().includes(term))
      );
    }

    // Convert to summaries
    return filtered.map(invoice => ({
      id: invoice.id,
      number: invoice.number,
      customer_name: invoice.customer.name,
      total: invoice.total,
      status: invoice.status,
      issue_date: invoice.issue_date,
      due_date: invoice.due_date,
      balance_due: invoice.total, // TODO: Calculate based on payments
      is_overdue: invoice.status !== 'Paid' && new Date(invoice.due_date) < new Date()
    } as InvoiceSummary));
  }
);

// Store for statistics
export const invoiceStats = derived(
  invoices,
  ($invoices) => {
    const stats = {
      total: $invoices.length,
      draft: 0,
      sent: 0,
      viewed: 0,
      paid: 0,
      overdue: 0,
      cancelled: 0,
      total_amount: { amount: '0', currency: 'USD' },
      paid_amount: { amount: '0', currency: 'USD' },
      outstanding_amount: { amount: '0', currency: 'USD' }
    };

    // Count by status
    $invoices.forEach(invoice => {
      const status = invoice.status.toLowerCase();
      if (status === 'draft') stats.draft++;
      else if (status === 'sent') stats.sent++;
      else if (status === 'viewed') stats.viewed++;
      else if (status === 'paid') stats.paid++;
      else if (status === 'overdue') stats.overdue++;
      else if (status === 'cancelled') stats.cancelled++;
      
      // Sum amounts (simplified - would need actual calculation)
      if (invoice.status === 'Paid') {
        stats.paid_amount.amount = (parseFloat(stats.paid_amount.amount) + parseFloat(invoice.total.amount)).toString();
      } else {
        stats.outstanding_amount.amount = (parseFloat(stats.outstanding_amount.amount) + parseFloat(invoice.total.amount)).toString();
      }
      stats.total_amount.amount = (parseFloat(stats.total_amount.amount) + parseFloat(invoice.total.amount)).toString();
    });

    return stats;
  }
);

// Actions
export const invoiceActions = {
  setInvoices: (newInvoices: Invoice[]) => invoices.set(newInvoices),
  
  addInvoice: (invoice: Invoice) => {
    invoices.update($invoices => [...$invoices, invoice]);
  },
  
  updateInvoice: (updatedInvoice: Invoice) => {
    invoices.update($invoices => 
      $invoices.map(inv => inv.id === updatedInvoice.id ? updatedInvoice : inv)
    );
  },
  
  deleteInvoice: (id: string) => {
    invoices.update($invoices => $invoices.filter(inv => inv.id !== id));
  },
  
  selectInvoice: (invoice: Invoice | null) => selectedInvoice.set(invoice),
  
  setFilters: (newFilters: InvoiceFilter) => filters.set(newFilters),
  
  clearFilters: () => filters.set({}),
  
  setLoading: (key: keyof typeof loading, value: boolean) => {
    loading.update($loading => ({ ...$loading, [key]: value }));
  },
  
  setError: (key: keyof typeof errors, error: string | null) => {
    errors.update($errors => ({ ...$errors, [key]: error }));
  },
  
  clearAllErrors: () => errors.set({
    invoices: null,
    customers: null,
    pdf: null,
    sync: null
  })
};