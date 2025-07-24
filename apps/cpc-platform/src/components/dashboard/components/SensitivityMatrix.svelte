<script>
  import { createEventDispatcher } from 'svelte';
  
  export let scenarios = [];
  export let baseScenario = null;
  
  const dispatch = createEventDispatcher();
  
  function formatCurrency(value) {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(value);
  }
  
  function getChangeColor(base, current) {
    const change = ((current - base) / base) * 100;
    if (change > 0) return 'positive';
    if (change < 0) return 'negative';
    return 'neutral';
  }
  
  function getChangeText(base, current) {
    const change = ((current - base) / base) * 100;
    return `${change >= 0 ? '+' : ''}${change.toFixed(1)}%`;
  }
</script>

<div class="sensitivity-matrix">
  {#if scenarios.length > 0}
    <table>
      <thead>
        <tr>
          <th>Scenario</th>
          <th>Final Net Worth</th>
          <th>Change</th>
          <th>Parameters</th>
        </tr>
      </thead>
      <tbody>
        <tr class="base-scenario">
          <td>
            <strong>{baseScenario?.name || 'Base Scenario'}</strong>
            <span class="badge">Base</span>
          </td>
          <td class="value">{formatCurrency(baseScenario?.final_net_worth || 0)}</td>
          <td>-</td>
          <td class="parameters">
            {#each Object.entries(baseScenario?.parameters || {}) as [key, value]}
              <span class="parameter-tag">{key}: {value}</span>
            {/each}
          </td>
        </tr>
        
        {#each scenarios as scenario}
          <tr 
            class="scenario-row"
            on:click={() => dispatch('scenarioSelect', scenario)}
            on:keypress={() => dispatch('scenarioSelect', scenario)}
            tabindex="0"
            role="button"
          >
            <td>{scenario.name}</td>
            <td class="value">{formatCurrency(scenario.final_net_worth)}</td>
            <td class="change {getChangeColor(baseScenario?.final_net_worth || 0, scenario.final_net_worth)}">
              {getChangeText(baseScenario?.final_net_worth || 0, scenario.final_net_worth)}
            </td>
            <td class="parameters">
              {#each Object.entries(scenario.parameters || {}) as [key, value]}
                <span class="parameter-tag">{key}: {value}</span>
              {/each}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <div class="empty-state">
      <p>No sensitivity scenarios available</p>
    </div>
  {/if}
</div>

<style>
  .sensitivity-matrix {
    overflow-x: auto;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.9rem;
  }
  
  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #e0e0e0;
  }
  
  th {
    background-color: #f8f9fa;
    font-weight: 600;
    color: #2c3e50;
  }
  
  .base-scenario {
    background-color: #f0f8ff;
  }
  
  .scenario-row {
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .scenario-row:hover {
    background-color: #f8f9fa;
  }
  
  .scenario-row:focus {
    outline: 2px solid #3498db;
    outline-offset: -2px;
  }
  
  .badge {
    display: inline-block;
    background: #3498db;
    color: white;
    padding: 0.125rem 0.5rem;
    border-radius: 12px;
    font-size: 0.75rem;
    margin-left: 0.5rem;
  }
  
  .value {
    font-weight: 600;
    font-family: 'Courier New', monospace;
  }
  
  .change {
    font-weight: 600;
    font-family: 'Courier New', monospace;
  }
  
  .change.positive {
    color: #27ae60;
  }
  
  .change.negative {
    color: #e74c3c;
  }
  
  .change.neutral {
    color: #7f8c8d;
  }
  
  .parameters {
    font-size: 0.8rem;
  }
  
  .parameter-tag {
    display: inline-block;
    background: #ecf0f1;
    color: #2c3e50;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    margin: 0.125rem;
  }
  
  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #7f8c8d;
  }
  
  @media (max-width: 768px) {
    table {
      font-size: 0.8rem;
    }
    
    th, td {
      padding: 0.5rem;
    }
  }
</style>