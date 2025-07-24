import { writable, derived } from 'svelte/store';
import type { Customer } from '../types';

// Main store for all customers
export const customers = writable<Customer[]>([]);

// Store for loading states
export const loading = writable({
  customers: false
});

// Store for errors
export const errors = writable({
  customers: null as string | null
});

// Store for selected customer
export const selectedCustomer = writable<Customer | null>(null);

// Derived store for customer options (for dropdowns)
export const customerOptions = derived(
  customers,
  ($customers) => $customers.map(customer => ({
    value: customer.id,
    label: customer.name,
    customer
  }))
);

// Derived store for customer map (for quick lookups)
export const customerMap = derived(
  customers,
  ($customers) => new Map($customers.map(c => [c.id, c]))
);

// Actions
export const customerActions = {
  setCustomers: (newCustomers: Customer[]) => customers.set(newCustomers),
  
  addCustomer: (customer: Customer) => {
    customers.update($customers => [...$customers, customer]);
  },
  
  updateCustomer: (updatedCustomer: Customer) => {
    customers.update($customers => 
      $customers.map(cust => cust.id === updatedCustomer.id ? updatedCustomer : cust)
    );
  },
  
  deleteCustomer: (id: string) => {
    customers.update($customers => $customers.filter(cust => cust.id !== id));
  },
  
  selectCustomer: (customer: Customer | null) => selectedCustomer.set(customer),
  
  setLoading: (value: boolean) => {
    loading.set({ customers: value });
  },
  
  setError: (error: string | null) => {
    errors.set({ customers: error });
  },
  
  clearError: () => errors.set({ customers: null }),
  
  // Helper to get customer by ID from store
  getCustomerById: (id: string) => {
    let found: Customer | null = null;
    customers.subscribe($customers => {
      found = $customers.find(c => c.id === id) || null;
    })();
    return found;
  }
};