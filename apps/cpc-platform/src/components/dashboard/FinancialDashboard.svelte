<script>
  import { onMount } from 'svelte';
  import { forecastStore } from '../stores/forecastStore.js';
  import CashFlowChart from './components/CashFlowChart.svelte';
  import SensitivityMatrix from './components/SensitivityMatrix.svelte';
  import ScenarioControls from './components/ScenarioControls.svelte';
  
  export let jobId = null;
  
  let loading = false;
  let error = null;
  
  onMount(async () => {
    if (jobId) {
      loading = true;
      try {
        await forecastStore.loadForecast(jobId);
      } catch (err) {
        error = err.message;
      } finally {
        loading = false;
      }
    }
  });
  
  $: if (jobId && jobId !== $forecastStore.currentJobId) {
    loading = true;
    forecastStore.loadForecast(jobId).catch(err => {
      error = err.message;
    }).finally(() => {
      loading = false;
    });
  }
</script>

<div class="financial-dashboard">
  <div class="dashboard-header">
    <h1>Financial Forecast Dashboard</h1>
    {#if jobId}
      <span class="job-id">Job ID: {jobId}</span>
    {/if}
  </div>
  
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Loading forecast data...</p>
    </div>
  {:else if error}
    <div class="error">
      <p>Error: {error}</p>
      <button on:click={() => forecastStore.loadForecast(jobId)}>Retry</button>
    </div>
  {:else if $forecastStore.data}
    <div class="dashboard-content">
      <div class="controls-section">
        <ScenarioControls 
          scenarios={$forecastStore.data.sensitivity_scenarios}
          baseScenario={$forecastStore.data.base_scenario}
          on:scenarioSelect={(e) => forecastStore.selectScenario(e.detail)}
        />
      </div>
      
      <div class="charts-section">
        <div class="chart-container">
          <h2>Cash Flow Projections</h2>
          <CashFlowChart 
            projections={$forecastStore.data.projections}
            selectedScenario={$forecastStore.selectedScenario}
          />
        </div>
        
        <div class="matrix-container">
          <h2>Sensitivity Analysis</h2>
          <SensitivityMatrix 
            scenarios={$forecastStore.data.sensitivity_scenarios}
            baseScenario={$forecastStore.data.base_scenario}
          />
        </div>
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>No forecast data available. Create a new forecast to get started.</p>
    </div>
  {/if}
</div>

<style>
  .financial-dashboard {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #e0e0e0;
  }
  
  .dashboard-header h1 {
    margin: 0;
    color: #2c3e50;
  }
  
  .job-id {
    font-family: monospace;
    color: #666;
    font-size: 0.9rem;
  }
  
  .loading, .error, .empty-state {
    text-align: center;
    padding: 3rem;
  }
  
  .spinner {
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  .error {
    color: #e74c3c;
  }
  
  .error button {
    background: #e74c3c;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    margin-top: 1rem;
  }
  
  .dashboard-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .controls-section {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .charts-section {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 2rem;
  }
  
  .chart-container, .matrix-container {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .chart-container h2, .matrix-container h2 {
    margin: 0 0 1rem 0;
    color: #2c3e50;
    font-size: 1.2rem;
  }
  
  @media (max-width: 768px) {
    .charts-section {
      grid-template-columns: 1fr;
    }
    
    .financial-dashboard {
      padding: 1rem;
    }
  }
</style>