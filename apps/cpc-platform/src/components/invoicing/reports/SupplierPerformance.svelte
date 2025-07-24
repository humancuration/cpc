<script>
  import { onMount } from 'svelte';
  import { BarChart, LineChart } from '$lib/bi';
  
  let performanceData = [];
  let selectedSupplier = null;
  let dateRange = {
    start: new Date(Date.now() - 365 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
    end: new Date().toISOString().split('T')[0]
  };
  let loading = true;
  let error = null;

  let chartData = {
    payment_timeline: [],
    volume_comparison: [],
    average_payment_times: []
  };

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      const params = new URLSearchParams({
        start_date: dateRange.start,
        end_date: dateRange.end
      });
      
      const [performanceRes, timelineRes] = await Promise.all([
        fetch(`/api/invoicing/reports/supplier-performance?${params}`),
        fetch(`/api/invoicing/reports/payment-timeline?${params}`)
      ]);
      
      if (!performanceRes.ok || !timelineRes.ok) {
        throw new Error('Failed to load supplier data');
      }
      
      performanceData = await performanceRes.json();
      const timelineData = await timelineRes.json();
      
      // Process data for charts
      chartData = {
        payment_timeline: timelineData.payment_timeline || [],
        volume_comparison: performanceData.map(s => ({
          name: s.supplier_name,
          value: s.total_invoices
        })),
        average_payment_times: performanceData.map(s => ({
          name: s.supplier_name,
          value: s.average_payment_days
        }))
      };
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

  function selectSupplier(supplier) {
    selectedSupplier = supplier;
  }

  function exportReport() {
    const csv = [
      ['Supplier', 'Total Invoices', 'Total Amount', 'Average Payment Days', 'On-Time Payment Rate', 'Last Payment Date'],
      ...performanceData.map(supplier => [
        supplier.supplier_name,
        supplier.total_invoices,
        supplier.total_amount,
        supplier.average_payment_days,
        `${supplier.on_time_rate}%`,
        supplier.last_payment_date
      ])
    ].map(row => row.join(',')).join('\n');

    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `supplier-performance-${new Date().toISOString().split('T')[0]}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  }
</script>

<div class="supplier-performance">
  <div class="header">
    <h2>Supplier Performance Report</h2>
    <button on:click={exportReport}>Export CSV</button>
  </div>

  <div class="filters">
    <label>
      Start Date
      <input type="date" bind:value={dateRange.start} on:change={loadData} />
    </label>
    <label>
      End Date
      <input type="date" bind:value={dateRange.end} on:change={loadData} />
    </label>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if loading}
    <div class="loading">Loading supplier data...</div>
  {:else}
    <div class="charts">
      <div class="chart-container">
        <h3>Invoice Volume by Supplier</h3>
        <BarChart 
          data={chartData.volume_comparison}
          xAxis="name"
          yAxis="value"
          width={400}
          height={300}
        />
      </div>

      <div class="chart-container">
        <h3>Average Payment Time by Supplier</h3>
        <BarChart 
          data={chartData.average_payment_times}
          xAxis="name"
          yAxis="value"
          width={400}
          height={300}
        />
      </div>

      <div class="chart-container">
        <h3>Payment Timeline</h3>
        <LineChart 
          data={chartData.payment_timeline}
          xAxis="date"
          yAxis="amount"
          width={800}
          height={300}
        />
      </div>
    </div>

    <table>
      <thead>
        <tr>
          <th>Supplier</th>
          <th>Total Invoices</th>
          <th>Total Amount</th>
          <th>Avg Payment Days</th>
          <th>On-Time Rate</th>
          <th>Last Payment</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each performanceData as supplier}
          <tr 
            class:selected={selectedSupplier?.id === supplier.id}
            on:click={() => selectSupplier(supplier)}
          >
            <td>{supplier.supplier_name}</td>
            <td>{supplier.total_invoices}</td>
            <td>{formatCurrency(supplier.total_amount)}</td>
            <td>{supplier.average_payment_days} days</td>
            <td>
              <span class="rate-badge" class:good={supplier.on_time_rate >= 80}>
                {supplier.on_time_rate}%
              </span>
            </td>
            <td>{formatDate(supplier.last_payment_date)}</td>
            <td>
              <button on:click={() => goto(`/invoicing/supplier/${supplier.id}`)}>
                View Details
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .supplier-performance {
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

  .charts {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-bottom: 2rem;
  }

  .chart-container {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 8px;
  }

  .chart-container h3 {
    margin-top: 0;
    text-align: center;
  }

  .chart-container:last-child {
    grid-column: 1 / -1;
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

  tr {
    cursor: pointer;
    transition: background-color 0.2s;
  }

  tr:hover {
    background-color: #f5f5f5;
  }

  tr.selected {
    background-color: #e3f2fd;
  }

  .rate-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 12px;
    font-size: 0.875rem;
    background: #f8d7da;
    color: #721c24;
  }

  .rate-badge.good {
    background: #d4edda;
    color: #155724;
  }

  button {
    background: none;
    border: 1px solid #007bff;
    color: #007bff;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  button:hover {
    background: #007bff;
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

  @media (max-width: 768px) {
    .charts {
      grid-template-columns: 1fr;
    }
  }
</style>