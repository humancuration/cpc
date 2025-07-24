<script lang="ts">
  import { invoices, loading, errors, invoiceSummaries, invoiceActions } from '../stores/invoiceStore';
  import type { InvoiceSummary } from '../types';

  function selectInvoice(summary: InvoiceSummary) {
    // TODO: Load full invoice details
    console.log('Selected invoice:', summary.id);
  }

  function formatCurrency(money: { amount: string; currency: string }) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: money.currency || 'USD'
    }).format(parseFloat(money.amount));
  }

  function formatDate(date: string) {
    return new Date(date).toLocaleDateString();
  }

  function getStatusColor(status: string) {
    const colors = {
      Draft: 'bg-gray-100 text-gray-800',
      Sent: 'bg-blue-100 text-blue-800',
      Viewed: 'bg-purple-100 text-purple-800',
      Paid: 'bg-green-100 text-green-800',
      Overdue: 'bg-red-100 text-red-800',
      Cancelled: 'bg-gray-100 text-gray-800'
    };
    return colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800';
  }
</script>

<div class="bg-white shadow-sm rounded-lg">
  <div class="px-4 py-5 sm:p-6">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-medium text-gray-900">Invoices</h3>
      <button
        class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
        on:click={() => console.log('Create new invoice')}
      >
        New Invoice
      </button>
    </div>

    {#if $loading.invoices}
      <div class="text-center py-8">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
        <p class="mt-2 text-gray-500">Loading invoices...</p>
      </div>
    {:else if $errors.invoices}
      <div class="text-center py-8">
        <p class="text-red-600">{$errors.invoices}</p>
      </div>
    {:else if $invoiceSummaries.length === 0}
      <div class="text-center py-8">
        <p class="text-gray-500">No invoices found</p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead>
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Invoice
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Customer
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Date
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Due Date
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Total
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
              <th class="relative px-6 py-3">
                <span class="sr-only">Actions</span>
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each $invoiceSummaries as summary}
              <tr class="hover:bg-gray-50 cursor-pointer" on:click={() => selectInvoice(summary)}>
                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                  {summary.number}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {summary.customer_name}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {formatDate(summary.issue_date)}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {formatDate(summary.due_date)}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {formatCurrency(summary.total)}
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(summary.status)}">
                    {summary.status}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <button
                    class="text-blue-600 hover:text-blue-900"
                    on:click|stopPropagation={() => console.log('Edit invoice:', summary.id)}
                  >
                    Edit
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>