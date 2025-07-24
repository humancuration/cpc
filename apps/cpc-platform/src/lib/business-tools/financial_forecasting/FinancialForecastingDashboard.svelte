<script>
  import { onMount } from 'svelte';
  import KpiWidget from '../bi/KpiWidget.svelte';
  import DataTableWidget from '../bi/DataTableWidget.svelte';
  import { createForecast, scheduleTrainingSession, runSensitivityAnalysis } from '$lib/graphql/forecastMutations';
  import { getForecastScenarios } from '$lib/graphql/forecastQueries';
  import { getPermissions } from '$lib/auth/permissions';
  
  // Permission state
  let canRunSensitivity = false;
  
  import ForecastChart from './ForecastChart.svelte';
  import ScenarioEditor from './ScenarioEditor.svelte';
  import SensitivityPanel from './SensitivityPanel.svelte';
  import ComparisonView from './ComparisonView.svelte';
  
  let forecastScenarios = [];
  let loading = true;
  let error = null;
  let newForecastInput = {
    parameters: {
      start_date: new Date(),
      end_date: new Date(new Date().setMonth(new Date().getMonth() + 3)),
      interval: 'monthly',
      scenario_parameters: {
        "growth_rate": 1.1,
        "cost_increase": 1.05
      }
    }
  };
  let activeScenarioIndex = 0;
  let showScenarioEditor = false;
  let sensitivityResults = [];

  onMount(async () => {
    try {
      forecastScenarios = await getForecastScenarios();
      // Initialize permissions
      const permissions = getPermissions();
      canRunSensitivity = permissions.includes('RunSensitivityAnalysis');
      loading = false;
      
      const unsubscribe = forecastUpdates.subscribe(update => {
        if (update) {
          const index = forecastScenarios.findIndex(s => s.id === update.scenarioId);
          if (index !== -1) {
            forecastScenarios[index] = update;
            forecastScenarios = forecastScenarios;
          } else {
            forecastScenarios = [...forecastScenarios, update];
          }
        }
      });
      
      return () => unsubscribe();
    } catch (err) {
      error = err.message;
      loading = false;
    }
  });

  async function handleCreateForecast() {
    loading = true;
    try {
      const result = await createForecast(newForecastInput);
      forecastScenarios = [...forecastScenarios, result];
      loading = false;
    } catch (err) {
      error = err.message;
      loading = false;
    }
  }

  async function scheduleTraining() {
    if (!forecastScenarios[activeScenarioIndex]) return;
    
    const scenario = forecastScenarios[activeScenarioIndex];
    const result = await scheduleTrainingSession({
      scenario_id: scenario.id,
      title: `Training: ${scenario.name}`,
      description: scenario.description,
      collaborators: scenario.collaborators || []
    });
    
    if (result) {
      alert(`Training session scheduled! Event ID: ${result.event_id}`);
    }
  }

  async function handleRunSensitivityAnalysis(params) {
    if (!forecastScenarios[activeScenarioIndex]) return;
    
    loading = true;
    try {
      const scenario = forecastScenarios[activeScenarioIndex];
      const result = await runSensitivityAnalysis({
        forecastId: scenario.id,
        parameters: {
          revenueGrowth: params.revenueGrowth,
          expenseChange: params.expenseChange,
          interestRate: params.interestRate
        }
      });
      
      sensitivityResults = [...sensitivityResults, {
        id: sensitivityResults.length + 1,
        name: `Sensitivity ${sensitivityResults.length + 1}`,
        parameters: {
          ...scenario.parameters,
          scenario_parameters: {
            ...scenario.parameters.scenario_parameters,
            revenue_growth: params.revenueGrowth,
            cost_increase: params.expenseChange,
            interest_rate: params.interestRate
          }
        },
        projections: result.projections
      }];
    } catch (err) {
      error = err.message;
    }
    loading = false;
  }
</script>

<div class="financial-forecasting-dashboard">
  <h1>Financial Forecasting</h1>
  
  {#if loading}
    <p>Loading forecasts...</p>
  {:else if error}
    <p class="error">Error: {error}</p>
  {:else}
    <div class="controls">
      <h2>Create New Forecast</h2>
      <div>
        <label>Start Date: <input type="date" bind:value={newForecastInput.parameters.start_date} /></label>
        <label>End Date: <input type="date" bind:value={newForecastInput.parameters.end_date} /></label>
        <label>
          Interval:
          <select bind:value={newForecastInput.parameters.interval}>
            <option value="daily">Daily</option>
            <option value="weekly">Weekly</option>
            <option value="monthly" selected>Monthly</option>
            <option value="quarterly">Quarterly</option>
            <option value="yearly">Yearly</option>
          </select>
        </label>
        <button on:click={handleCreateForecast}>Create Forecast</button>
      </div>
    </div>

    <div class="scenario-tabs">
      {#each forecastScenarios as scenario, i}
        <button
          class:active={activeScenarioIndex === i}
          on:click={() => activeScenarioIndex = i}
        >
          {scenario.name}
        </button>
      {/each}
      <button on:click={() => showScenarioEditor = true}>+ Add Scenario</button>
    </div>
    
    {#if forecastScenarios.length > 0}
      <div class="scenario">
        <div class="scenario-header">
          <h3>{forecastScenarios[activeScenarioIndex].name}</h3>
          <button on:click={() => showScenarioEditor = true}>Edit Scenario</button>
          <button on:click={scheduleTraining} disabled={!forecastScenarios[activeScenarioIndex]}>
            Schedule Training Session
          </button>
        </div>
        
        <div class="projections">
          <ForecastChart projections={forecastScenarios[activeScenarioIndex].projections} />
          
          <SensitivityPanel on:run={handleRunSensitivityAnalysis} {canRunSensitivity} />
          
          {#if sensitivityResults.length > 0}
            <ComparisonView 
              baseScenario={forecastScenarios[activeScenarioIndex]} 
              sensitivityScenarios={sensitivityResults} 
            />
          {/if}
          
          <div class="projection-grid">
            <DataTableWidget
              title="Cash Flow Projections"
              headers={['Date', 'Inflow', 'Outflow', 'Net Cash Flow']}
              rows={forecastScenarios[activeScenarioIndex].projections.map(p => [
                p.date.toISOString().split('T')[0],
                `$${p.inflow.toFixed(2)}`,
                `$${p.outflow.toFixed(2)}`,
                `$${p.net_cash_flow.toFixed(2)}`
              ])}
            />
            
            <div class="kpi-summary">
              <KpiWidget
                title="Total Inflow"
                value={forecastScenarios[activeScenarioIndex].projections.reduce((sum, p) => sum + p.inflow, 0).toFixed(2)}
                format="currency"
              />
              <KpiWidget
                title="Total Outflow"
                value={forecastScenarios[activeScenarioIndex].projections.reduce((sum, p) => sum + p.outflow, 0).toFixed(2)}
                format="currency"
              />
              <KpiWidget
                title="Net Cash Flow"
                value={forecastScenarios[activeScenarioIndex].projections.reduce((sum, p) => sum + p.net_cash_flow, 0).toFixed(2)}
                format="currency"
              />
              <KpiWidget
                title="Avg Monthly Growth"
                value={(forecastScenarios[activeScenarioIndex].projections.length > 1 ?
                   (forecastScenarios[activeScenarioIndex].projections[forecastScenarios[activeScenarioIndex].projections.length - 1].inflow /
                    forecastScenarios[activeScenarioIndex].projections[0].inflow * 100 - 100).toFixed(2) : 0)}
                format="percent"
                trend="up"
              />
            </div>
          </div>
        </div>
        
        <div class="scenario-comparison">
          <h3>Scenario Comparison</h3>
          <DataTableWidget
            title="Key Metrics"
            headers={['Scenario', 'Total Inflow', 'Total Outflow', 'Net Cash Flow', 'Avg Growth']}
            rows={forecastScenarios.map(scenario => [
              scenario.name,
              `$${scenario.projections.reduce((sum, p) => sum + p.inflow, 0).toFixed(2)}`,
              `$${scenario.projections.reduce((sum, p) => sum + p.outflow, 0).toFixed(2)}`,
              `$${scenario.projections.reduce((sum, p) => sum + p.net_cash_flow, 0).toFixed(2)}`,
              `${(scenario.projections.length > 1 ?
                (scenario.projections[scenario.projections.length - 1].inflow / scenario.projections[0].inflow * 100 - 100).toFixed(2) : 0)}%`
            ])}
          />
        </div>
      </div>
    {/if}
    
    {#if showScenarioEditor}
      <ScenarioEditor
        bind:show={showScenarioEditor}
        scenarios={forecastScenarios}
        onSave={(updatedScenarios) => forecastScenarios = updatedScenarios}
      />
    {/if}
  {/if}
</div>

<style>
  .financial-forecasting-dashboard {
    padding: 20px;
  }
  
  .controls, .scenario {
    margin-bottom: 30px;
    padding: 15px;
    background: #f5f7fa;
    border-radius: 8px;
  }
  
  .scenario-tabs {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }
  
  .scenario-tabs button {
    padding: 8px 16px;
    background: #e2e8f0;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .scenario-tabs button.active {
    background: #3b82f6;
    color: white;
  }
  
  .scenario-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
  }
  
  .projection-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 20px;
    margin-top: 20px;
  }
  
  .projections {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
  
  .kpi-summary {
    display: flex;
    gap: 20px;
  }
  
  .financial-forecasting-dashboard label {
    margin-right: 15px;
  }
  
  .error {
    color: #e53e3e;
  }
  
  .sensitivity-results {
    margin-top: 30px;
    padding: 20px;
    background: #f8f9fa;
    border-radius: 8px;
  }
</style>