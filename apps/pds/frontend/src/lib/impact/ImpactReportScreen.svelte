<script>
  import { onMount } from 'svelte';
  import { 
    impactReport, 
    isLoading, 
    error, 
    totalImpact,
    breakdownItems,
    distributionData,
    timelinePoints,
    fetchImpactReport 
  } from '../../stores/impact.js';
  
  import BreakdownView from './BreakdownView.svelte';
  import DistributionView from './DistributionView.svelte';
  import TimelineView from './TimelineView.svelte';
  
  export let userId = null;
  
  onMount(() => {
    fetchImpactReport(userId);
  });
  
  $: reportData = $impactReport;
  $: loading = $isLoading;
  $: errorMessage = $error;
</script>

<div class="impact-report-container">
  <header class="report-header">
    <h1>Impact Report</h1>
    {#if reportData}
      <div class="total-impact">
        <span class="label">Total Impact Score:</span>
        <span class="score">{$totalImpact}</span>
      </div>
    {/if}
  </header>
  
  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Loading impact report...</p>
    </div>
  {:else if errorMessage}
    <div class="error-state">
      <h3>Error</h3>
      <p>{errorMessage}</p>
      <button class="retry-button" on:click={() => fetchImpactReport(userId)}>
        Retry
      </button>
    </div>
  {:else if reportData}
    <div class="report-content">
      <div class="report-grid">
        <section class="report-section">
          <BreakdownView {breakdownItems} />
        </section>
        
        <section class="report-section">
          <DistributionView {distributionData} />
        </section>
        
        <section class="report-section full-width">
          <TimelineView {timelinePoints} />
        </section>
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <h3>No Impact Report Available</h3>
      <p>Generate a new impact report to see your ethical impact analysis.</p>
      <button class="generate-button" on:click={() => fetchImpactReport(userId)}>
        Generate Report
      </button>
    </div>
  {/if}
</div>

<style>
  .impact-report-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1rem;
  }

  .report-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #e5e7eb;
  }

  .report-header h1 {
    font-size: 2rem;
    font-weight: 700;
    color: #1f2937;
    margin: 0;
  }

  .total-impact {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .total-impact .label {
    font-size: 1rem;
    color: #6b7280;
  }

  .total-impact .score {
    font-size: 1.5rem;
    font-weight: 700;
    color: #22c55e;
  }

  .loading-state, .error-state, .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 3rem;
    text-align: center;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e5e7eb;
    border-top: 4px solid #3b82f6;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }

  .error-state h3, .empty-state h3 {
    font-size: 1.25rem;
    font-weight: 600;
    color: #1f2937;
    margin-bottom: 0.5rem;
  }

  .error-state p, .empty-state p {
    color: #6b7280;
    margin-bottom: 1rem;
  }

  .retry-button, .generate-button {
    background: #3b82f6;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .retry-button:hover, .generate-button:hover {
    background: #2563eb;
  }

  .report-content {
    margin-top: 1rem;
  }

  .report-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }

  .report-section {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    overflow: hidden;
  }

  .full-width {
    grid-column: 1 / -1;
  }

  @media (max-width: 768px) {
    .report-grid {
      grid-template-columns: 1fr;
    }

    .report-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
  }
</style>