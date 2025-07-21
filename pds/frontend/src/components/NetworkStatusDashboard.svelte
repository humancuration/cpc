<script>
  import { networkStatus, refreshNetworkStatus } from '../stores/network';
  import { onMount } from 'svelte';
  
  onMount(() => {
    refreshNetworkStatus();
  });
  
  function formatDate(date) {
    if (!date) return 'Never';
    return new Date(date).toLocaleTimeString();
  }
</script>

<div class="dashboard">
  <h2>Network Status</h2>
  
  {#if $networkStatus.error}
    <div class="error-state">
      <div class="error-icon">⚠️</div>
      <p>Failed to load network data: {$networkStatus.error}</p>
      <button on:click={refreshNetworkStatus}>Retry</button>
    </div>
  {:else}
    <div class="status-indicator">
      <div class="led {$networkStatus.data.isOnline ? 'online' : 'offline'}"></div>
      <span>{$networkStatus.data.isOnline ? 'Online' : 'Offline'}</span>
    </div>
    
    <div class="metrics">
      <div class="metric" title="Number of peers currently connected">
        <h3>Connected Peers</h3>
        <p>{$networkStatus.data.peers}</p>
      </div>
      <div class="metric" title="Current upload speed">
        <h3>Upload</h3>
        <p>{$networkStatus.data.bandwidthUp.toFixed(2)} KB/s</p>
      </div>
      <div class="metric" title="Current download speed">
        <h3>Download</h3>
        <p>{$networkStatus.data.bandwidthDown.toFixed(2)} KB/s</p>
      </div>
    </div>
    
    <div class="footer">
      <div class="timestamp">
        Last updated: {formatDate($networkStatus.lastUpdated)}
      </div>
      <button on:click={refreshNetworkStatus} disabled={$networkStatus.loading}>
        {#if $networkStatus.loading}
          <span class="spinner">⏳</span> Refreshing...
        {:else}
          Refresh
        {/if}
      </button>
    </div>
  {/if}
</div>

<style>
  .dashboard {
    padding: 20px;
    background: #f5f5f5;
    border-radius: 8px;
    position: relative;
  }
  .status-indicator {
    display: flex;
    align-items: center;
    margin-bottom: 15px;
  }
  .led {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    margin-right: 8px;
  }
  .online { background: green; }
  .offline { background: red; }
  .metrics {
    display: flex;
    gap: 20px;
    margin-bottom: 15px;
  }
  .metric {
    flex: 1;
    text-align: center;
    position: relative;
    cursor: help;
  }
  .metric:hover::after {
    content: attr(title);
    position: absolute;
    bottom: 100%;
    left: 50%;
    transform: translateX(-50%);
    background: #333;
    color: white;
    padding: 5px;
    border-radius: 4px;
    white-space: nowrap;
    z-index: 100;
  }
  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .timestamp {
    font-size: 0.8em;
    color: #666;
  }
  .error-state {
    background: #ffebee;
    border: 1px solid #ffcdd2;
    border-radius: 4px;
    padding: 10px;
    margin-bottom: 15px;
    display: flex;
    align-items: center;
    gap: 10px;
  }
  .error-icon {
    font-size: 1.5em;
  }
  .spinner {
    margin-right: 5px;
  }
</style>