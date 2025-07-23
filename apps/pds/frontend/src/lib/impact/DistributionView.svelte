<script>
  export let distributionData = [];
</script>

<div class="distribution-container">
  <h2 class="distribution-title">Ethical Distribution</h2>
  
  {#if distributionData.length > 0}
    <div class="distribution-content">
      <!-- Placeholder for pie chart visualization -->
      <div class="chart-placeholder">
        <div class="chart-circle">
          <span class="chart-text">Pie Chart Visualization</span>
        </div>
      </div>
      
      <div class="distribution-stats">
        {#each distributionData as category}
          <div class="distribution-row">
            <span class="category-name">{category.category}</span>
            <span class="percentage">{Math.round((category.weight || 0) * 100)}%</span>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>No distribution data available</p>
    </div>
  {/if}
</div>

<style>
  .distribution-container {
    padding: 1rem;
    max-width: 100%;
  }

  .distribution-title {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1f2937;
    text-align: center;
  }

  .distribution-content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .chart-placeholder {
    margin-bottom: 1.5rem;
  }

  .chart-circle {
    width: 200px;
    height: 200px;
    background: #f3f4f6;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid #e5e7eb;
  }

  .chart-text {
    color: #6b7280;
    font-size: 0.875rem;
    text-align: center;
  }

  .distribution-stats {
    width: 100%;
    max-width: 400px;
  }

  .distribution-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e5e7eb;
  }

  .distribution-row:last-child {
    border-bottom: none;
  }

  .category-name {
    font-size: 1rem;
    color: #1f2937;
  }

  .percentage {
    font-weight: 600;
    font-size: 1rem;
    color: #1f2937;
  }

  .empty-state {
    text-align: center;
    color: #6b7280;
    padding: 2rem;
  }
</style>