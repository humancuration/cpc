<script>
  import CameraView from '$lib/vision/CameraView.svelte';
  import RecognitionResults from '$lib/vision/RecognitionResults.svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  
  let recognitionResults = null;
  let loading = false;
  let error = null;
  
  async function handleRecognition(event) {
    loading = true;
    error = null;
    
    try {
      const { result } = event.detail;
      recognitionResults = result;
    } catch (err) {
      error = err.message || 'Recognition failed';
    } finally {
      loading = false;
    }
  }
  
  async function saveResults() {
    if (!recognitionResults) return;
    
    try {
      // This would integrate with GraphQL when available
      console.log('Saving results:', recognitionResults);
      
      // For now, just show success message
      alert('Results saved successfully!');
      
    } catch (err) {
      error = 'Failed to save results';
    }
  }
  
  function clearResults() {
    recognitionResults = null;
    error = null;
  }
</script>

<svelte:head>
  <title>Vision Recognition - CPC Desktop</title>
</svelte:head>

<div class="vision-container">
  <header class="page-header">
    <h1>Vision Recognition</h1>
    <p>Capture and analyze images using AI-powered recognition</p>
  </header>
  
  <main class="vision-main">
    <section class="camera-section">
      <h2>Camera</h2>
      <CameraView on:recognition={handleRecognition} />
    </section>
    
    {#if error}
      <div class="error-banner">
        <p>{error}</p>
        <button on:click={() => error = null}>Dismiss</button>
      </div>
    {/if}
    
    <section class="results-section">
      <div class="results-header">
        <h2>Recognition Results</h2>
        <div class="results-actions">
          {#if recognitionResults}
            <button on:click={saveResults} class="btn save">
              Save Results
            </button>
            <button on:click={clearResults} class="btn clear">
              Clear
            </button>
          {/if}
        </div>
      </div>
      
      <RecognitionResults 
        results={recognitionResults?.detections || []} 
        {loading} 
      />
    </section>
  </main>
</div>

<style>
  .vision-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  .page-header {
    text-align: center;
    margin-bottom: 2rem;
  }
  
  .page-header h1 {
    margin: 0;
    color: #333;
    font-size: 2.5rem;
  }
  
  .page-header p {
    margin: 0.5rem 0 0;
    color: #666;
    font-size: 1.2rem;
  }
  
  .vision-main {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    align-items: start;
  }
  
  .camera-section, .results-section {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }
  
  .camera-section h2, .results-section h2 {
    margin: 0 0 1rem;
    color: #333;
  }
  
  .results-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }
  
  .results-actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 4px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .btn.save {
    background: #28a745;
    color: white;
  }
  
  .btn.save:hover {
    background: #1e7e34;
  }
  
  .btn.clear {
    background: #dc3545;
    color: white;
  }
  
  .btn.clear:hover {
    background: #c82333;
  }
  
  .error-banner {
    grid-column: 1 / -1;
    background: #f8d7da;
    color: #721c24;
    padding: 1rem;
    border-radius: 4px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .error-banner button {
    background: none;
    border: none;
    color: #721c24;
    cursor: pointer;
    font-weight: bold;
  }
  
  @media (max-width: 768px) {
    .vision-main {
      grid-template-columns: 1fr;
    }
    
    .vision-container {
      padding: 1rem;
    }
  }
</style>