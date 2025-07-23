<script>
  export let timelinePoints = [];

  function formatDate(timestamp) {
    if (!timestamp) return '';
    
    const date = new Date(timestamp);
    return date.toLocaleDateString('en-US', { 
      month: 'short', 
      year: 'numeric' 
    });
  }

  function formatScore(score) {
    return (score || 0).toFixed(1);
  }
</script>

<div class="timeline-container">
  <h2 class="timeline-title">Impact Over Time</h2>
  
  {#if timelinePoints.length > 0}
    <div class="timeline-content">
      <!-- Placeholder for timeline chart -->
      <div class="chart-placeholder">
        <div class="chart-box">
          <span class="chart-text">Timeline Visualization</span>
        </div>
      </div>
      
      <div class="timeline-points">
        {#each timelinePoints as point}
          <div class="timeline-row">
            <span class="date">{formatDate(point.timestamp)}</span>
            <span class="score">{formatScore(point.score)}</span>
          </div>
        {/each}
      </div>
    </div>
  {:else}
    <div class="empty-state">
      <p>No timeline data available</p>
    </div>
  {/if}
</div>

<style>
  .timeline-container {
    padding: 1rem;
    max-width: 100%;
  }

  .timeline-title {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1f2937;
    text-align: center;
  }

  .timeline-content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .chart-placeholder {
    margin-bottom: 1.5rem;
    width: 100%;
  }

  .chart-box {
    width: 100%;
    height: 300px;
    background: #f3f4f6;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .chart-text {
    color: #6b7280;
    font-size: 0.875rem;
  }

  .timeline-points {
    width: 100%;
    max-width: 400px;
  }

  .timeline-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid #e5e7eb;
  }

  .timeline-row:last-child {
    border-bottom: none;
  }

  .date {
    font-size: 1rem;
    color: #1f2937;
  }

  .score {
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