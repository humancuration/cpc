<script>
  import { onMount } from 'svelte';
  import forecastStore, { cashFlowData, sensitivityScenarios, riskMetrics, baseScenario } from '../../stores/forecastStore.js';
  import CashFlowChart from './components/CashFlowChart.svelte';
  import SensitivityMatrix from './components/SensitivityMatrix.svelte';
  import ScenarioControls from './components/ScenarioControls.svelte';
  
  let loading = false;
  let error = null;
  
  // Subscribe to store
  const unsubscribe = forecastStore.subscribe(state => {
    loading = state.loading;
    error = state.error;
  });
  
  onMount(async () => {
    // Load user preferences when component mounts
    await forecastStore.loadPreferences();
    
    return () => {
      unsubscribe();
    };
  });
  
  async function handleGenerateScenarios(parameters) {
    try {
      await forecastStore.updateParameters(parameters);
      
      // Create new forecast job
      const jobId = await forecastStore.createForecast({
        parameters,
        scenarios: generateSensitivityScenarios(parameters)
      });
      
      console.log('Created forecast job:', jobId);
    } catch (err) {
      console.error('Failed to generate forecast:', err);
    }
  }
  
  function generateSensitivityScenarios(baseParams) {
    const scenarios = [];
    const variations = [
      { name: 'Optimistic', factor: 1.2 },
      { name: 'Pessimistic', factor: 0.8 },
      { name: 'High Inflation', adjustments: { inflation_rate: 0.05 } },
      { name: 'Low Returns', adjustments: { investment_return: 0.05 } }
    ];
    
    variations.forEach(variation => {
      const params = { ...baseParams };
      
      if (variation.factor) {
        params.investment_return *= variation.factor;
        params.income_growth_rate *= variation.factor;
      }
      
      if (variation.adjustments) {
        Object.assign(params, variation.adjustments);
      }
      
      scenarios.push({
        name: variation.name,
        parameters: params
      });
    });
    
    return scenarios;
  }
  
  function handleScenarioSelect(event) {
    forecastStore.setActiveScenario(event.detail);
  }
  
  function handleParameterChange(event) {
    forecastStore.updateParameters({ [event.detail.key]: event.detail.value });
  }
</script>

<div class="financial-dashboard">
  <header class="dashboard-header">
    <h1>Financial Forecasting Dashboard</h1>
    <div class="dashboard-actions">
      <button class="refresh-btn" on:click={() => forecastStore.loadForecast($forecastStore.currentJob)}>
        Refresh
      </button>
    </div>
  </header>
  
  {#if error}
    <div class="error-banner">
      <p>{error}</p>
      <button on:click={() => forecastStore.clearError()}>Dismiss</button>
    </div>
  {/if}
  
  {#if loading}
    <div class="loading-state">
      <div class="spinner"></div>
      <p>Calculating financial forecast...</p>
    </div>
  {/if}
  
  <div class="dashboard-content">
    <div class="controls-section">
      <ScenarioControls
        parameters={$forecastStore.parameters}
        activeScenario={$forecastStore.activeScenario}
        on:parameterChange={handleParameterChange}
        on:generateScenarios={handleGenerateScenarios}
        on:scenarioSelect={handleScenarioSelect}
      />
    </div>
    
    {#if $cashFlowData.length > 0}
      <div class="charts-section">
        <div class="chart-container">
          <h2>Cash Flow Projections</h2>
          <CashFlowChart projections={$cashFlowData} />
        </div>
        
        {#if $sensitivityScenarios.length > 0}
          <div class="matrix-container">
            <h2>Sensitivity Analysis</h2>
            <SensitivityMatrix
              scenarios={$sensitivityScenarios}
              baseScenario={$baseScenario}
              on:scenarioSelect={handleScenarioSelect}
            />
          </div>
        {/if}
        
        {#if $riskMetrics}
          <div class="metrics-container">
            <h2>Risk Metrics</h2>
            <div class="metrics-grid">
              <div class="metric-card">
                <h3>Success Probability</h3>
                <span class="metric-value">{$riskMetrics.probabilitySuccess}%</span>
              </div>
              <div class="metric-card">
                <h3>Worst Case</h3>
                <span class="metric-value">${$riskMetrics.worstCase.toLocaleString()}</span>
              </div>
              <div class="metric-card">
                <h3>Best Case</h3>
                <span class="metric-value">${$riskMetrics.bestCase.toLocaleString()}</span>
              </div>
              <div class="metric-card">
                <h3>Average Outcome</h3>
                <span class="metric-value">${$riskMetrics.averageOutcome.toLocaleString()}</span>
              </div>
            </div>
          </div>
        {/if}
      </div>
    {:else if !$forecastStore.currentJob}
      <div class="empty-state">
        <h2>Get Started</h2>
        <p>Configure your parameters and generate your first financial forecast</p>
      </div>
    {/if}
  </div>
</div>

<style>
  .financial-dashboard {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }
  
  .dashboard-header h1 {
    margin: 0;
    color: #2c3e50;
  }
  
  .refresh-btn {
    padding: 0.5rem 1rem;
    background: #3498db;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .refresh-btn:hover {
    background: #2980b9;
  }
  
  .error-banner {
    background: #fee;
    border: 1px solid #fcc;
    color: #c33;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .error-banner button {
    background: none;
    border: none;
    color: #c33;
    cursor: pointer;
    font-size: 1.2rem;
  }
  
  .loading-state {
    text-align: center;
    padding: 2rem;
  }
  
  .spinner {
    border: 4px solid #f3f3f3;
    border-top: 4px solid #3498db;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    animation: spin 2s linear infinite;
    margin: 0 auto 1rem;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  .dashboard-content {
    display: grid;
    gap: 2rem;
  }
  
  .controls-section {
    background: white;
    border-radius: 8px;
    overflow: hidden;
  }
  
  .charts-section {
    display: grid;
    gap: 2rem;
  }
  
  .chart-container,
  .matrix-container,
  .metrics-container {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .chart-container h2,
  .matrix-container h2,
  .metrics-container h2 {
    margin-top: 0;
    color: #2c3e50;
  }
  
  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
  }
  
  .metric-card {
    background: #f8f9fa;
    padding: 1rem;
    border-radius: 4px;
    text-align: center;
  }
  
  .metric-card h3 {
    margin: 0 0 0.5rem 0;
    font-size: 0.9rem;
    color: #7f8c8d;
  }
  
  .metric-value {
    font-size: 1.5rem;
    font-weight: 600;
    color: #2c3e50;
  }
  
  .empty-state {
    text-align: center;
    padding: 3rem;
    color: #7f8c8d;
  }
  
  @media (max-width: 768px) {
    .financial-dashboard {
      padding: 1rem;
    }
    
    .dashboard-header {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }
    
    .metrics-grid {
      grid-template-columns: 1fr;
    }
  }
</style>