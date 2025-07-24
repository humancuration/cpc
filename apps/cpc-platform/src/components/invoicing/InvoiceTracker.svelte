<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  
  let invoices = [];
  let filteredInvoices = [];
  let searchTerm = '';
  let statusFilter = 'all';
  let dateRange = { start: '', end: '' };
  let loading = true;
  let error = null;

  const statusOptions = [
    { value: 'all', label: 'All' },
    { value: 'draft', label: 'Draft' },
    { value: 'sent', label: 'Sent' },
    { value: 'paid', label: 'Paid' },
    { value: 'overdue', label: 'Overdue' }
  ];

  onMount(async () => {
    await loadInvoices();
  });

  async function loadInvoices() {
    try {
      const response = await fetch('/api/invoicing/invoices');
      if (!response.ok) throw new Error('Failed to load invoices');
      
      invoices = await response.json();
      filteredInvoices = invoices;
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  function filterInvoices() {
    filteredInvoices = invoices.filter(invoice => {
      const matchesSearch = !searchTerm || 
        invoice.recipient_name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        invoice.invoice_number.toLowerCase().includes(searchTerm.toLowerCase());
      
      const matchesStatus = statusFilter === 'all' || invoice.status === statusFilter;
      
      const matchesDateRange = (!dateRange.start || new Date(invoice.issue_date) >= new Date(dateRange.start)) &&
        (!dateRange.end || new Date(invoice.issue_date) <= new Date(dateRange.end));
      
      return matchesSearch && matchesStatus && matchesDateRange;
    });
  }

  function viewInvoice(id) {
    goto(`/invoicing/invoice/${id}`);
  }

  function editInvoice(id) {
    goto(`/invoicing/edit/${id}`);
  }

  async function sendInvoice(id) {
    try {
      const response = await fetch(`/api/invoicing/invoices/${id}/send`, {
        method: 'POST'
      });
      
      if (response.ok) {
        await loadInvoices();
      }
    } catch (err) {
      error = 'Failed to send invoice';
    }
  }

  async function markAsPaid(id) {
    try {
      const response = await fetch(`/api/invoicing/invoices/${id}/pay`, {
        method: 'POST'
      });
      
      if (response.ok) {
        await loadInvoices();
      }
    } catch (err) {
      error = 'Failed to mark as paid';
    }
  }

  function getStatusColor(status) {
    const colors = {
      draft: '#6c757d',
      sent: '#007bff',
      paid: '#28a745',
      overdue: '#dc3545'
    };
    return colors[status] || '#6c757d';
  }

  function formatCurrency(amount) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD'
    }).format(amount);
  }

  function formatDate(date) {
    return new Date(date).toLocaleDateString();
  }

  $: if (invoices.length > 0) {
    filterInvoices();
  }
</script>

<div class="invoice-tracker">
  <h2>Invoice Tracker</h2>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="filters">
    <input 
      type="text" 
      placeholder="Search invoices..." 
      bind:value={searchTerm}
      on:input={filterInvoices}
    />
    
    <select bind:value={statusFilter} on:change={filterInvoices}>
      {#each statusOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
    
    <input 
      type="date" 
      bind:value={dateRange.start}
      on:change={filterInvoices}
    />
    
    <input 
      type="date" 
      bind:value={dateRange.end}
      on:change={filterInvoices}
    />
  </div>

  {#if loading}
    <div class="loading">Loading invoices...</div>
  {:else if filteredInvoices.length === 0}
    <div class="empty">No invoices found</div>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Invoice #</th>
          <th>Recipient</th>
          <th>Issue Date</th>
          <th>Due Date</th>
          <th>Amount</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each filteredInvoices as invoice}
          <tr>
            <td>{invoice.invoice_number}</td>
            <td>{invoice.recipient_name}</td>
            <td>{formatDate(invoice.issue_date)}</td>
            <td>{formatDate(invoice.due_date)}</td>
            <td>{formatCurrency(invoice.total_amount)}</td>
            <td>
              <span 
                class="status-badge" 
                style="background-color: {getStatusColor(invoice.status)}"
              >
                {invoice.status}
              </span>
            </td>
            <td>
              <button on:click={() => viewInvoice(invoice.id)}>View</button>
              {#if invoice.status === 'draft'}
                <button on:click={() => editInvoice(invoice.id)}>Edit</button>
                <button on:click={() => sendInvoice(invoice.id)}>Send</button>
              {:else if invoice.status === 'sent'}
                <button on:click={() => markAsPaid(invoice.id)}>Mark Paid</button>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .invoice-tracker {
    padding: 2rem;
  }

  .filters {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    flex-wrap: wrap;
  }

  .filters input, .filters select {
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #ddd;
  }

  th {
    background-color: #f8f9fa;
    font-weight: bold;
  }

  .status-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    color: white;
    font-size: 0.875rem;
    text-transform: capitalize;
  }

  button {
    background: none;
    border: 1px solid #007bff;
    color: #007bff;
    padding: 0.25rem 0.5rem;
    margin-right: 0.25rem;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background-color: #007bff;
    color: white;
  }

  .loading, .empty {
    text-align: center;
    padding: 2rem;
    color: #6c757d;
  }

  .error {
    background: #f8d7da;
    color: #721c24;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }
</style>