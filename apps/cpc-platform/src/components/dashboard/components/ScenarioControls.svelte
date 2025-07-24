<script>
  import { createEventDispatcher } from 'svelte';
  
  export let parameters = {
    income_growth_rate: 0.03,
    expense_inflation: 0.025,
    investment_return: 0.08,
    inflation_rate: 0.035,
    retirement_age: 65,
    safety_factor: 1.2
  };
  
  export let activeScenario = 'base';
  
  const dispatch = createEventDispatcher();
  
  function updateParameter(key, value) {
    parameters[key] = parseFloat(value);
    dispatch('parameterChange', { key, value: parameters[key] });
  }
  
  function generateScenarios() {
    dispatch('generateScenarios', parameters);
  }
  
  function resetToDefaults() {
    parameters = {
      income_growth_rate: 0.03,
      expense_inflation: 0.025,
      investment_return: 0.08,
      inflation_rate: 0.035,
      retirement_age: 65,
      safety_factor: 1.2
    };
    dispatch('resetParameters', parameters);
  }
  
  const parameterLabels = {
    income_growth_rate: 'Income Growth Rate',
    expense_inflation: 'Expense Inflation',
    investment_return: 'Investment Return',
    inflation_rate: 'Inflation Rate',
    retirement_age: 'Retirement Age',
    safety_factor: 'Safety Factor'
  };
  
  const parameterRanges = {
    income_growth_rate: { min: -0.1, max: 0.15, step: 0.005 },
    expense_inflation: { min: 0, max: 0.1, step: 0.005 },
    investment_return: { min: 0.02, max: 0.15, step: 0.005 },
    inflation_rate: { min: 0.01, max: 0.08, step: 0.005 },
    retirement_age: { min: 50, max: 75, step: 1 },
    safety_factor: { min: 1.0, max: 2.0, step: 0.1 }
  };
</script>

<div class="scenario-controls">
  <div class="controls-header">
    <h3>Scenario Parameters</h3>
    <div class="scenario-tabs">
      <button 
        class="tab {activeScenario === 'base' ? 'active' : ''}"
        on:click={() => dispatch('scenarioSelect', 'base')}
      >
        Base Scenario
      </button>
      <button 
        class="tab {activeScenario === 'sensitivity' ? 'active' : ''}"
        on:click={() => dispatch('scenarioSelect', 'sensitivity')}
      >
        Sensitivity Analysis
      </button>
    </div>
  </div>
  
  {#if activeScenario === 'base'}
    <div class="parameters-grid">
      {#each Object.entries(parameters) as [key, value]}
        <div class="parameter-group">
          <label for={key}>{parameterLabels[key]}</label>
          <div class="input-group">
            <input
              id={key}
              type="range"
              min={parameterRanges[key].min}
              max={parameterRanges[key].max}
              step={parameterRanges[key].step}
              bind:value={parameters[key]}
              on:input={(e) => updateParameter(key, e.target.value)}
            />
            <span class="value-display">
              {key.includes('rate') ? (value * 100).toFixed(1) + '%' : value.toFixed(1)}
            </span>
          </div>
        </div>
      {/each}
    </div>
    
    <div class="actions">
      <button class="btn-primary" on:click={generateScenarios}>
        Generate Scenarios
      </button>
      <button class="btn-secondary" on:click={resetToDefaults}>
        Reset to Defaults
      </button>
    </div>
  {:else}
    <div class="sensitivity-info">
      <p>Click on any scenario in the matrix below to view its parameters.</p>
      <p>The sensitivity analysis shows how changes in key parameters affect your financial forecast.</p>
    </div>
  {/if}
</div>

<style>
  .scenario-controls {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .controls-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  .controls-header h3 {
    margin: 0;
    color: #2c3e50;
    font-size: 1.25rem;
  }
  
  .scenario-tabs {
    display: flex;
    gap: 0.5rem;
  }
  
  .tab {
    padding: 0.5rem 1rem;
    border: 1px solid #e0e0e0;
    background: white;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.2s;
  }
  
  .tab.active {
    background: #3498db;
    color: white;
    border-color: #3498db;
  }
  
  .parameters-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    margin-bottom: 1.5rem;
  }
  
  .parameter-group {
    display: flex;
    flex-direction: column;
  }
  
  label {
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #2c3e50;
  }
  
  .input-group {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  input[type="range"] {
    flex: 1;
    margin: 0;
  }
  
  .value-display {
    min-width: 50px;
    text-align: right;
    font-family: 'Courier New', monospace;
    font-weight: 600;
    color: #3498db;
  }
  
  .actions {
    display: flex;
    gap: 1rem;
    justify-content: flex-end;
  }
  
  .btn-primary, .btn-secondary {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 600;
    transition: background-color 0.2s;
  }
  
  .btn-primary {
    background: #3498db;
    color: white;
  }
  
  .btn-primary:hover {
    background: #2980b9;
  }
  
  .btn-secondary {
    background: #ecf0f1;
    color: #2c3e50;
  }
  
  .btn-secondary:hover {
    background: #bdc3c7;
  }
  
  .sensitivity-info {
    padding: 1rem;
    background: #f8f9fa;
    border-radius: 4px;
    color: #2c3e50;
  }
  
  .sensitivity-info p {
    margin: 0.5rem 0;
  }
  
  @media (max-width: 768px) {
    .controls-header {
      flex-direction: column;
      align-items: stretch;
      gap: 1rem;
    }
    
    .parameters-grid {
      grid-template-columns: 1fr;
    }
    
    .actions {
      flex-direction: column;
    }
  }
</style>