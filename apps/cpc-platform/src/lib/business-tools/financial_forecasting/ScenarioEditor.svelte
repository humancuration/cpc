<script>
  import { saveScenario } from '$lib/graphql/forecastMutations';
  import { onMount } from 'svelte';
  import analytics from '$lib/analytics';
  
  export let show = false;
  export let scenarios = [];
  export let onSave = (scenarios) => {};
  
  let editedScenarios = [...scenarios];
  let newScenarioName = '';
  let newScenarioParams = {
    growthRate: 1.1,
    costIncrease: 1.05
  };
  let saving = false;
  let saveError = null;
  
  onMount(() => {
    editedScenarios = [...scenarios];
  });
  function addScenario() {
    if (newScenarioName.trim() === '') return;
    
    editedScenarios.push({
      name: newScenarioName,
      projections: [],
      parameters: {
        scenario_parameters: {...newScenarioParams}
      }
    });
    
    analytics.track('forecast_created', {
      scenario_name: newScenarioName,
      growth_rate: newScenarioParams.growth_rate,
      cost_increase: newScenarioParams.cost_increase
    });
    
    newScenarioName = '';
    newScenarioParams = {
      growth_rate: 1.1,
      cost_increase: 1.05
    };
  }
  }
  
  function removeScenario(index) {
    const removedScenario = editedScenarios[index];
    editedScenarios.splice(index, 1);
    analytics.track('scenario_compared', {
      action: 'removed',
      scenario_name: removedScenario.name
    });
  }
  
  async function handleSave() {
    saving = true;
    saveError = null;
    try {
      // Save each scenario to the backend
      const savedScenarios = [];
      for (const scenario of editedScenarios) {
        const saved = await saveScenario(scenario);
        savedScenarios.push(saved);
      }
      onSave(savedScenarios);
      analytics.track('scenario_compared', {
        action: 'saved',
        scenario_count: savedScenarios.length
      });
      show = false;
    } catch (err) {
      saveError = err.message;
    }
    saving = false;
  }
</script>

{#if show}
<div class="scenario-editor-overlay">
  <div class="scenario-editor">
    <h2>Manage Scenarios</h2>
    
    {#if saveError}
      <p class="error">{saveError}</p>
    {/if}
    
    <div class="scenario-list">
      {#each editedScenarios as scenario, i}
        <div class="scenario-item">
          <input type="text" bind:value={scenario.name} />
          <button on:click={() => removeScenario(i)}>Remove</button>
        </div>
      {/each}
    </div>
    
    <div class="add-scenario">
      <h3>Add New Scenario</h3>
      <div>
        <label>
          Scenario Name:
          <input type="text" bind:value={newScenarioName} />
        </label>
        <label>
          Growth Rate:
          <input type="number" bind:value={newScenarioParams.growthRate} step="0.05" />
        </label>
        <label>
          Cost Increase:
          <input type="number" bind:value={newScenarioParams.costIncrease} step="0.05" />
        </label>
        <button on:click={addScenario}>Add Scenario</button>
      </div>
    </div>
    
    <div class="actions">
      <button on:click={() => show = false}>Cancel</button>
      <button on:click={handleSave} disabled={saving}>
        {saving ? 'Saving...' : 'Save Changes'}
      </button>
    </div>
  </div>
</div>
{/if}

<style>
  .scenario-editor-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }
  
  .scenario-editor {
    background: white;
    padding: 20px;
    border-radius: 8px;
    width: 80%;
    max-width: 600px;
    max-height: 80vh;
    overflow-y: auto;
  }
  
  .scenario-item {
    display: flex;
    margin-bottom: 10px;
  }
  
  .scenario-item input {
    flex: 1;
    margin-right: 10px;
  }
  
  .add-scenario {
    margin-top: 20px;
    padding-top: 20px;
    border-top: 1px solid #eee;
  }
  
  .add-scenario label {
    display: block;
    margin-bottom: 10px;
  }
  
  .add-scenario input {
    margin-left: 10px;
  }
  
  .actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 20px;
  }
</style>