<script>
  export let baseScenario;
  export let sensitivityScenarios = [];
  
  // Function to calculate variance
  const calculateVariance = (baseValue, newValue) => {
    if (baseValue === 0) return 0;
    return ((newValue - baseValue) / baseValue) * 100;
  };
</script>

<div class="comparison-view">
  <h3>Scenario Comparison</h3>
  
  {#if sensitivityScenarios.length === 0}
    <p>No sensitivity scenarios available. Run an analysis first.</p>
  {:else}
    <div class="scenarios">
      {#each sensitivityScenarios as scenario (scenario.id)}
        <div class="scenario-card">
          <h4>{scenario.name}</h4>
          
          <div class="metrics">
            <div class="metric">
              <span>Projected Revenue:</span>
              <span class:positive={calculateVariance(baseScenario.totalRevenue, scenario.totalRevenue) > 0}
                     class:negative={calculateVariance(baseScenario.totalRevenue, scenario.totalRevenue) < 0}>
                ${scenario.totalRevenue.toLocaleString()} 
                ({calculateVariance(baseScenario.totalRevenue, scenario.totalRevenue).toFixed(2)}%)
              </span>
            </div>
            
            <div class="metric">
              <span>Operating Costs:</span>
              <span class:positive={calculateVariance(baseScenario.operatingCosts, scenario.operatingCosts) < 0}
                     class:negative={calculateVariance(baseScenario.operatingCosts, scenario.operatingCosts) > 0}>
                ${scenario.operatingCosts.toLocaleString()} 
                ({calculateVariance(baseScenario.operatingCosts, scenario.operatingCosts).toFixed(2)}%)
              </span>
            </div>
            
            <div class="metric">
              <span>Net Profit:</span>
              <span class:positive={calculateVariance(baseScenario.netProfit, scenario.netProfit) > 0}
                     class:negative={calculateVariance(baseScenario.netProfit, scenario.netProfit) < 0}>
                ${scenario.netProfit.toLocaleString()} 
                ({calculateVariance(baseScenario.netProfit, scenario.netProfit).toFixed(2)}%)
              </span>
            </div>
          </div>
          
          <div class="summary">
            <strong>Key Changes:</strong>
            <ul>
              {#each scenario.changes as change (change.id)}
                <li>{change.description}</li>
              {/each}
            </ul>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .comparison-view {
    margin-top: 2rem;
  }
  
  .scenarios {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 1rem;
  }
  
  .scenario-card {
    padding: 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    background-color: #f9f9f9;
  }
  
  .metric {
    margin-bottom: 0.5rem;
  }
  
  .positive {
    color: green;
    font-weight: bold;
  }
  
  .negative {
    color: red;
    font-weight: bold;
  }
  
  .summary {
    margin-top: 1rem;
    padding-top: 0.5rem;
    border-top: 1px dashed #ccc;
  }
</style>