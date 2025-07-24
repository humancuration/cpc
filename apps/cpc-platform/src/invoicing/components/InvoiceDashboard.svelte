<script lang="ts">
  import { onMount } from 'svelte';
  import { InvoiceService } from '../services/invoiceService';
  import { invoices, loading, errors } from '../stores/invoiceStore';
  import InvoiceList from './InvoiceList.svelte';

  onMount(async () => {
    loading.update($loading => ({ ...$loading, invoices: true }));
    try {
      const result = await InvoiceService.listInvoices();
      if (result.success) {
        // Extract actual Invoice objects from InvoiceResponse
        const invoiceData = result.data.invoices.map(response => response.invoice);
        invoices.set(invoiceData);
        errors.update($errors => ({ ...$errors, invoices: null }));
      } else {
        errors.update($errors => ({ ...$errors, invoices: 'Failed to load invoices' }));
      }
    } catch (error) {
      errors.update($errors => ({ ...$errors, invoices: 'Failed to load invoices' }));
    } finally {
      loading.update($loading => ({ ...$loading, invoices: false }));
    }
  });
</script>

<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-gray-900">Invoices</h1>
    <p class="mt-2 text-gray-600">Manage your invoices and track payments</p>
  </div>

  <!-- Stats Cards -->
  <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4 mb-8">
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="p-5">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="text-2xl font-bold text-gray-900">0</div>
            <div class="text-sm font-medium text-gray-500">Draft</div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="p-5">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="text-2xl font-bold text-gray-900">0</div>
            <div class="text-sm font-medium text-gray-500">Sent</div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="p-5">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="text-2xl font-bold text-gray-900">0</div>
            <div class="text-sm font-medium text-gray-500">Paid</div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="bg-white overflow-hidden shadow rounded-lg">
      <div class="p-5">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="text-2xl font-bold text-gray-900">0</div>
            <div class="text-sm font-medium text-gray-500">Overdue</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Invoice List -->
  <InvoiceList />
</div>