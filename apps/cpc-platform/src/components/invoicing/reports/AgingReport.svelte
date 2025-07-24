<script>
  import { onMount } from 'svelte';
  
  let agingData = [];
  let dateRange = {
    start: new Date(Date.now() - 90 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
    end: new Date().toISOString().split('T')[0]
  };
  let loading = true;
  let error = null;
  let summary = {
    current: 0,
    '1-30': 0,
    '31-60': 0,
    '61-90': 0,
    '90+': 0,
    total: 0
  };

  onMount(async () => {
    await loadAgingReport();
  });

  async function loadAgingReport() {
    try {
      const params = new URLSearchParams({
        start_date: dateRange.start,
        end_date: dateRange.end
      });
      
      const response = await fetch(`/api/invoicing/reports/aging?${params}`);
      if (!response.ok) throw new Error('Failed to load aging report');
      
      const data = await response.json();
      agingData = data.invoices;
      summary = data.summary;
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
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

  function getDaysOverdue(invoice) {
    const today = new Date();
    const dueDate = new Date(invoice.due_date);
    const diffTime = today - dueDate;
    return Math.max(0, Math.floor(diffTime / (1000 * 60 * 60 * 24)));
  }

  function getAgingBucket(invoice) {
    const daysOverdue = getDaysOverdue(invoice);
    
    if (daysOverdue === 0) return 'current';
    if (daysOverdue <= 30) return '1-30';
    if (daysOverdue <= 60) return '31-60';
    if (daysOverdue <= 90) return '61-90';
    return '90+';
  }

  function exportReport() {
    const csv = [
      ['Customer', 'Invoice #', 'Issue Date', 'Due Date', 'Days Overdue', 'Amount', 'Status'],
      ...agingData.map(invoice => [
        invoice.customer_name,
        invoice.invoice_number,
        formatDate(invoice.issue_date),
        formatDate(invoice.due_date),
        getDaysOverdue(invoice),
        invoice.amount,
        invoice.status
      ])
    ].map(row => row.join(',')).join('\n');

    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `aging-report-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="aging-report">
  <div class="header">
    <h2>Aging Report</h2>
    <button on:click={exportReport}>Export CSV</button>
  </div>

  <div class="filters">
    <label>
      Start Date
      <input type="date" bind:value={dateRange.start} on:change={loadAgingReport} />
    </label>
    <label>
      End Date
      <input type="date" bind:value={dateRange.end} on:change={loadAgingReport} />
    </label>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  <div class="summary">
    <h3>Summary</h3>
    <div class="summary-grid">
      <div class="summary-card">
        <h4>Current</h4>
        <p>{formatCurrency(summary.current)}</p>
      </div>
      <div class="summary-card overdue-1-30">
        <h4>1-30 Days</h4>
        <p>{formatCurrency(summary['1-30'])}</p>
      </div>
      <div class="summary-card overdue-31-60">
        <h4>31-60 Days</h4>
        <p>{formatCurrency(summary['31-60'])}</p>
      </div>
      <div class="summary-card overdue-61-90">
        <h4>61-90 Days</h4>
        <p>{formatCurrency(summary['61-90'])}</p>
      </div>
      <div class="summary-card overdue-90-plus">
        <h4>90+ Days</h4>
        <p>{formatCurrency(summary['90+'])}</p>
      </div>
      <div class="summary-card total">
        <h4>Total Outstanding</h4>
        <p>{formatCurrency(summary.total)}</p>
      </div>
    </div>
  </div>

  {#if loading}
    <div class="loading">Loading aging report...</div>
  {:else}
    <table>
      <thead>
        <tr>
          <th>Customer</th>
          <th>Invoice #</th>
          <th>Issue Date</th>
          <th>Due Date</th>
          <th>Days Overdue</th>
          <th>Amount</th>
          <th>Status</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each agingData as invoice}
          <tr class={getAgingBucket(invoice)}>
            <td>{invoice.customer_name}</td>
            <td>{invoice.invoice_number}</td>
            <td>{formatDate(invoice.issue_date)}</td>
            <td>{formatDate(invoice.due_date)}</td>
            <td>{getDaysOverdue(invoice)}</td>
            <td>{formatCurrency(invoice.amount)}</td>
            <td>
              <span class="status-badge {invoice.status}">{invoice.status}</span>
            </td>
            <td>
              <button on:click={() => goto(`/invoicing/invoice/${invoice.id}`)}>View</button>
              {#if invoice.status === 'sent'}
                <button on:click={() => goto(`/invoicing/invoice/${invoice.id}/remind`)}>Send Reminder</button>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .aging-report {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .header h2 {
    margin: 0;
  }

  .header button {
    background: #007bff;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .filters {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .filters label {
    display: flex;
    flex-direction: column;
  }

  .filters input {
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  .summary {
    margin-bottom: 2rem;
  }

  .summary h3 {
    margin-bottom: 1rem;
  }

  .summary-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 1rem;
  }

  .summary-card {
    padding: 1rem;
    border-radius: 8px;
    text-align: center;
    border: 1px solid #e0e0e0;
  }

  .summary-card h4 {
    margin: 0 0 0.5rem 0;
    font-size: 0.875rem;
    color: #666;
  }

  .summary-card p {
    margin: 0;
    font-size: 1.25rem;
    font-weight: bold;
  }

  .summary-card.total {
    background: #007bff;
    color: white;
  }

  .overdue-1-30 {
    background: #fff3cd;
  }

  .overdue-31-60 {
    background: #f8d7da;
  }

  .overdue-61-90 {
    background: #f5c6cb;
  }

  .overdue-90-plus {
    background: #f1aeb5;
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
    font-size: 0.875rem;
    text-transform: capitalize;
  }

  .status-badge.sent {
    background: #cce5ff;
    color: #004085;
  }

  .status-badge.paid {
    background: #d4edda;
    color: #155724;
  }

  .status-badge.overdue {
    background: #f8d7da;
    color: #721c24;
  }

  tr.current {
    background: #f8f9fa;
  }

  tr.overdue-1-30 {
    background: #fff3cd;
  }

  tr.overdue-31-60 {
    background: #f8d7da;
  }

  tr.overdue-61-90 {
    background: #f5c6cb;
  }

  tr.overdue-90-plus {
    background: #f1aeb5;
  }

  button {
    background: none;
    border: 1px solid #007bff;
    color: #007bff;
    padding: 0.25rem 0.5rem;
    margin-right: 0.25rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  button:hover {
    background-color: #007bff;
    color: white;
  }

  .loading {
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