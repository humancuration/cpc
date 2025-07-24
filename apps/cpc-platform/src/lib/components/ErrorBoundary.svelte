<script>
  import { onMount } from 'svelte';
  
  export let fallback = null;
  export let onError = null;
  
  let error = null;
  let info = null;
  
  onMount(() => {
    const handleError = (event) => {
      error = event.error;
      info = event.filename + ':' + event.lineno;
      
      if (onError) {
        onError(error, info);
      }
      
      console.error('Error caught by ErrorBoundary:', error, info);
    };
    
    window.addEventListener('error', handleError);
    
    return () => {
      window.removeEventListener('error', handleError);
    };
  });
  
  function handleRetry() {
    error = null;
    info = null;
    window.location.reload();
  }
</script>

{#if error}
  <div class="error-boundary" role="alert" aria-live="polite">
    {#if fallback}
      <svelte:component this={fallback} {error} {info} onRetry={handleRetry} />
    {:else}
      <div class="error-content">
        <div class="error-icon" aria-hidden="true">⚠️</div>
        <h2>Something went wrong</h2>
        <p>We're sorry, but something unexpected happened.</p>
        {#if error}
          <details>
            <summary>Error details</summary>
            <pre>{error.message}</pre>
            <small>{info}</small>
          </details>
        {/if}
        <button 
          class="retry-btn" 
          on:click={handleRetry}
          aria-label="Reload the page"
        >
          Reload Page
        </button>
      </div>
    {/if}
  </div>
{:else}
  <slot />
{/if}

<style>
  .error-boundary {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
    padding: 40px;
    text-align: center;
  }
  
  .error-content {
    max-width: 400px;
  }
  
  .error-icon {
    font-size: 48px;
    margin-bottom: 16px;
  }
  
  .error-content h2 {
    margin: 0 0 8px 0;
    color: #333;
  }
  
  .error-content p {
    margin: 0 0 16px 0;
    color: #666;
  }
  
  .retry-btn {
    background: #007bff;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 24px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .retry-btn:hover {
    background: #0056b3;
  }
  
  details {
    margin: 16px 0;
    text-align: left;
  }
  
  summary {
    cursor: pointer;
    font-weight: 500;
  }
  
  pre {
    background: #f5f5f5;
    padding: 8px;
    border-radius: 4px;
    font-size: 12px;
    overflow-x: auto;
  }
</style>