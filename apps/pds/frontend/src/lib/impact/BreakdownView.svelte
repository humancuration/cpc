<script>
  export let breakdownItems = [];
</script>

<div class="breakdown-container">
  <h2 class="breakdown-title">Detailed Breakdown</h2>
  
  {#if breakdownItems.length > 0}
    <div class="breakdown-list">
      {#each breakdownItems as item}
        <div class="breakdown-card">
          <div class="breakdown-header">
            <span class="item-name">{item.itemName || item.item_name}</span>
            <span class="contribution">{Math.round(item.contribution || 0)}% contribution</span>
          </div>
          
          <div class="breakdown-details">
            <span class="label">Impact Score:</span>
            <span 
              class="score"
              style="color: {item.impactScore > 7.5 ? '#22c55e' : item.impactScore > 5.0 ? '#f97316' : '#ef4444'}"
            >
              {item.impactScore?.toFixed(1) || '0.0'}
            </span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>No breakdown data available</p>
    </div>
  {/if}
</div>

<style>
  .breakdown-container {
    padding: 1rem;
    max-width: 100%;
  }

  .breakdown-title {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1f2937;
  }

  .breakdown-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .breakdown-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }

  .breakdown-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .item-name {
    font-weight: 600;
    font-size: 1.125rem;
    color: #1f2937;
  }

  .contribution {
    font-size: 1rem;
    color: #6b7280;
  }

  .breakdown-details {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .label {
    color: #6b7280;
  }

  .score {
    font-weight: 600;
  }

  .empty-state {
    text-align: center;
    color: #6b7280;
    padding: 2rem;
  }
</style>