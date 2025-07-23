<script>
  export let results = [];
  export let loading = false;
  
  $: hasResults = results && results.length > 0;
  $: detectionCount = results?.length || 0;
</script>

<div class="results-container">
  {#if loading}
    <div class="loading">
      <div class="spinner"></div>
      <p>Analyzing image...</p>
    </div>
  {:else if hasResults}
    <div class="results-header">
      <h3>Recognition Results ({detectionCount} detected)</h3>
    </div>
    
    <div class="results-grid">
      {#each results as detection}
        <div class="result-card">
          <div class="result-header">
            <span class="label">{detection.label}</span>
            <span class="confidence">
              {(detection.confidence * 100).toFixed(1)}%
            </span>
          </div>
          
          {#if detection.bbox}
            <div class="bbox-info">
              <small>
                Position: ({detection.bbox.x.toFixed(0)}, {detection.bbox.y.toFixed(0)})
                <br />
                Size: {detection.bbox.width.toFixed(0)} × {detection.bbox.height.toFixed(0)}
              </small>
            </div>
          {/if}
          
          <div class="confidence-bar">
            <div 
              class="confidence-fill" 
              style="width: {(detection.confidence * 100)}%"
            ></div>
          </div>
        </div>
      {/each}
    </div>
    
    <div class="results-summary">
      <p>Processing time: {results.processing_time_ms || 'N/A'}ms</p>
    </div>
  {:else}
    <div class="no-results">
      <p>No objects detected</p>
      <small>Try adjusting the camera angle or lighting</small>
    </div>
  {/if}
</div>

<style>
  .results-container {
    padding: 1rem;
    max-width: 800px;
    margin: 0 auto;
  }
  
  .results-header {
    margin-bottom: 1rem;
  }
  
  .results-header h3 {
    margin: 0;
    color: #333;
  }
  
  .results-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
  }
  
  .result-card {
    background: #f8f9fa;
    border: 1px solid #dee2e6;
    border-radius: 8px;
    padding: 1rem;
    transition: transform 0.2s, box-shadow 0.2s;
  }
  
  .result-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
  
  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  
  .label {
    font-weight: bold;
    color: #007bff;
  }
  
  .confidence {
    font-size: 0.9rem;
    color: #28a745;
    font-weight: 600;
  }
  
  .bbox-info {
    margin: 0.5rem 0;
    color: #6c757d;
  }
  
  .confidence-bar {
    width: 100%;
    height: 4px;
    background: #e9ecef;
    border-radius: 2px;
    overflow: hidden;
  }
  
  .confidence-fill {
    height: 100%;
    background: linear-gradient(to right, #28a745, #17a2b8);
    transition: width 0.3s ease;
  }
  
  .loading {
    text-align: center;
    padding: 2rem;
  }
  
  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e9ecef;
    border-top: 4px solid #007bff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 1rem;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  .no-results {
    text-align: center;
    padding: 2rem;
    color: #6c757d;
  }
  
  .results-summary {
    margin-top: 1rem;
    text-align: center;
    color: #6c757d;
    font-size: 0.9rem;
  }
</style>