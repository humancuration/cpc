<script>
  import { onMount, onDestroy } from 'svelte';
  import { biStore, fetchImpactReport, subscribeToImpactReport, unsubscribeFromImpactReport } from './biStore.js';
  import WidgetGrid from './WidgetGrid.svelte';
  import ChartWidget from './ChartWidget.svelte';
  import KpiWidget from './KpiWidget.svelte';
  import DataTableWidget from './DataTableWidget.svelte';
  
  export let userId = null;
  
  let unsubscribe = null;
  
  $: if (userId) {
    loadImpactReport();
  }
  
  async function loadImpactReport() {
    if (!userId) return;
    
    try {
      await fetchImpactReport(userId);
    } catch (error) {
      console.error('Failed to load impact report:', error);
    }
  }
  
  onMount(() => {
    if (userId) {
      unsubscribe = subscribeToImpactReport(userId);
    }
  });
  
  onDestroy(() => {
    if (unsubscribe) {
      unsubscribe();
    }
    unsubscribeFromImpactReport();
  });
</script>

<div class="bi-dashboard">
  {#if $biStore.loading}
    <div class="loading-container">
      <div class="spinner"></div>
      <p>Loading impact report...</p>
    </div>
  {:else if $biStore.error}
    <div class="error-container">
      <h3>Error Loading Report</h3>
      <p>{$biStore.error}</p>
      <button on:click={loadImpactReport}>Retry</button>
    </div>
  {:else if $biStore.impactReport}
    <div class="dashboard-header">
      <h1>Impact Report</h1>
      <p class="generated-time">
        Generated: {new Date($biStore.impactReport.generatedAt).toLocaleString()}
      </p>
    </div>
    
    <WidgetGrid>
      <KpiWidget 
        title="Total Impact" 
        value={$biStore.impactReport.totalImpact} 
        format="currency"
        trend="up"
      />
      
      <ChartWidget 
        type="pie" 
        title="Impact Distribution" 
        data={$biStore.impactReport.distribution}
      />
      
      <ChartWidget 
        type="bar" 
        title="Impact Breakdown by Category" 
        data={$biStore.impactReport.breakdown}
      />
      
      <ChartWidget 
        type="line" 
        title="Impact Timeline" 
        data={$biStore.impactReport.timeline}
      />
      
      <DataTableWidget 
        title="Detailed Breakdown" 
        data={$biStore.impactReport.breakdown}
        columns={[
          { key: 'itemName', label: 'Item' },
          { key: 'category', label: 'Category' },
          { key: 'amount', label: 'Amount', format: 'currency' },
          { key: 'impactScore', label: 'Impact Score', format: 'number' }
        ]}
      />
    </WidgetGrid>
  {:else}
    <div class="empty-state">
      <h3>No Impact Report Available</h3>
      <p>Generate a new impact report to see your data.</p>
      <button on:click={() => fetchImpactReport(userId)}>Generate Report</button>
    </div>
  {/if}
</div>

<style>
  .bi-dashboard {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .dashboard-header {
    margin-bottom: 2rem;
  }

  .dashboard-header h1 {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: 0.5rem;
    color: #1a1a1a;
  }

  .generated-time {
    color: #666;
    font-size: 0.875rem;
  }

  .loading-container,
  .error-container,
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 4rem 2rem;
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: 1rem;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-container h3,
  .empty-state h3 {
    color: #e74c3c;
    margin-bottom: 0.5rem;
  }

  .empty-state h3 {
    color: #666;
  }

  button {
    background: #3498db;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
    margin-top: 1rem;
  }

  button:hover {
    background: #2980b9;
  }

  @media (max-width: 768px) {
    .bi-dashboard {
      padding: 1rem;
    }
  }
</style>