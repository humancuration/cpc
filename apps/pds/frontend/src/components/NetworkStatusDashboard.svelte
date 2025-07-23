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
    padding: 25px;
    background: linear-gradient(to bottom right, #f8f9fa, #e9ecef);
    border-radius: 12px;
    position: relative;
    box-shadow: 0 4px 12px rgba(0,0,0,0.08);
  }
  
  .dashboard h2 {
    margin-top: 0;
    margin-bottom: 20px;
    color: #343a40;
    font-size: 1.5rem;
    font-weight: 600;
  }
  
  .status-indicator {
    display: flex;
    align-items: center;
    margin-bottom: 20px;
    padding: 12px;
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  }
  
  .led {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    margin-right: 12px;
    transition: background 0.3s ease;
  }
  
  .online { 
    background: #2ecc71; 
    box-shadow: 0 0 8px rgba(46, 204, 113, 0.5);
  }
  
  .offline { 
    background: #e74c3c; 
    box-shadow: 0 0 8px rgba(231, 76, 60, 0.5);
  }
  
  .metrics {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 15px;
    margin-bottom: 20px;
  }
  
  .metric {
    flex: 1;
    text-align: center;
    position: relative;
    cursor: help;
    background: white;
    padding: 15px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    transition: transform 0.2s ease, box-shadow 0.2s ease;
  }
  
  .metric:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.1);
  }
  
  .metric h3 {
    margin-top: 0;
    margin-bottom: 10px;
    font-size: 0.9rem;
    color: #6c757d;
  }
  
  .metric p {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #343a40;
  }
  
  .metric:hover::after {
    content: attr(title);
    position: absolute;
    bottom: 110%;
    left: 50%;
    transform: translateX(-50%);
    background: #343a40;
    color: white;
    padding: 8px 12px;
    border-radius: 6px;
    white-space: nowrap;
    z-index: 100;
    font-size: 0.85rem;
    box-shadow: 0 4px 6px rgba(0,0,0,0.1);
  }
  
  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 15px;
    border-top: 1px solid #dee2e6;
  }
  
  .timestamp {
    font-size: 0.85em;
    color: #6c757d;
  }
  
  .footer button {
    background: #007bff;
    color: white;
    border: none;
    border-radius: 6px;
    padding: 8px 16px;
    cursor: pointer;
    transition: background 0.2s ease;
    font-weight: 500;
  }
  
  .footer button:hover {
    background: #0069d9;
  }
  
  .footer button:disabled {
    background: #a5d1ff;
    cursor: not-allowed;
  }
  
  .error-state {
    background: #fff5f5;
    border: 1px solid #ffe3e3;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 20px;
    display: flex;
    align-items: center;
    gap: 15px;
  }
  
  .error-icon {
    font-size: 1.8em;
    color: #e74c3c;
  }
  
  .spinner {
    margin-right: 8px;
    animation: spin 1.5s linear infinite;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>