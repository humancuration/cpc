<script>
  import { showNotification } from '$lib/notifications';
  
  export let scenario;
  export let canRunSensitivity;
  
  // Sensitivity parameters
  let sensitivityParams = {
    revenueGrowth: 0.0,
    expenseChange: 0.0,
    interestRate: 0.0
  };
  
  // Run sensitivity analysis
  async function runAnalysis() {
    if (!canRunSensitivity) {
      showNotification({
        type: 'error',
        message: 'You do not have permission to run sensitivity analysis'
      });
      return;
    }
    
    try {
      const response = await fetch(`/api/financial-forecasting/${scenario.name}/sensitivity`, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify(sensitivityParams)
      });
      
      if (!response.ok) throw new Error('Sensitivity analysis failed');
      
      const result = await response.json();
      // Update forecast with new projections
      forecast = result;
      showNotification({
        type: 'success',
        message: 'Sensitivity analysis completed successfully'
      });
    } catch (error) {
      showNotification({
        type: 'error',
        message: error.message || 'Failed to run sensitivity analysis'
      });
    }
  }
</script>

<div class="sensitivity-panel">
  <h2>Sensitivity Analysis</h2>
  
  {#if !canRunSensitivity}
    <div class="permission-warning">
      <p>⚠️ You don't have permission to run sensitivity analysis</p>
      <p>Contact your administrator to request the "RunSensitivityAnalysis" permission</p>
    </div>
  {/if}
  
  <div class="controls" class:disabled={!canRunSensitivity}>
    <label>
      Revenue Growth (%):
      <input 
        type="range" 
        min="-50" max="50" step="0.5"
        bind:value={sensitivityParams.revenueGrowth}
        disabled={!canRunSensitivity}
      />
      <span>{sensitivityParams.revenueGrowth}%</span>
    </label>
    
    <label>
      Expense Change (%):
      <input 
        type="range" 
        min="-30" max="30" step="0.5"
        bind:value={sensitivityParams.expenseChange}
        disabled={!canRunSensitivity}
      />
      <span>{sensitivityParams.expenseChange}%</span>
    </label>
    
    <label>
      Interest Rate (%):
      <input 
        type="range" 
        min="0" max="20" step="0.25"
        bind:value={sensitivityParams.interestRate}
        disabled={!canRunSensitivity}
      />
      <span>{sensitivityParams.interestRate}%</span>
    </label>
    
    <button on:click={runAnalysis} disabled={!canRunSensitivity}>
      Run Analysis
    </button>
  </div>
</div>

<style>
  .sensitivity-panel {
    padding: 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    background-color: #f9f9f9;
  }
  
  .permission-warning {
    padding: 1rem;
    background-color: #fff8e1;
    border-left: 4px solid #ffc107;
    margin-bottom: 1rem;
  }
  
  .disabled {
    opacity: 0.6;
    pointer-events: none;
  }
  
  label {
    display: block;
    margin-bottom: 1rem;
  }
  
  input[type="range"] {
    width: 100%;
    margin-top: 0.5rem;
  }
</style>