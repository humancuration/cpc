<script>
  import { onMount } from 'svelte';
  import { storageMetrics, refreshStorageMetrics, setStorageLimit } from '../stores/storage';
  import { SecureStorage } from '../stores/secureStorage';
  import StorageUsageChart from './StorageUsageChart.svelte';
  
  let newStorageLimit = 1; // Temporary value for slider
  let useSecureStorage = false;
  let secureStorageSize = 0;
  
  onMount(async () => {
    await refreshStorageMetrics();
    await loadSecureStorageSettings();
  });
  
  function formatDate(date) {
    if (!date) return 'Never';
    return new Date(date).toLocaleTimeString();
  }
  
  function bytesToGB(bytes) {
    return bytes / (1024 * 1024 * 1024);
  }
  
  async function handleSetStorageLimit() {
    try {
      await setStorageLimit(newStorageLimit);
    } catch (err) {
      console.error('Failed to set storage limit:', err);
    }
  }
  
  async function loadSecureStorageSettings() {
    try {
      const secureSetting = await SecureStorage.retrieve('useSecureStorage');
      useSecureStorage = secureSetting === 'true';
      
      secureStorageSize = await SecureStorage.size();
    } catch (err) {
      console.error('Failed to load secure storage settings:', err);
    }
  }
  
  async function toggleSecureStorage() {
    try {
      useSecureStorage = !useSecureStorage;
      await SecureStorage.store('useSecureStorage', useSecureStorage.toString());
      
      // Update secure storage size after change
      secureStorageSize = await SecureStorage.size();
      
      // Refresh main storage metrics to include secure storage
      await refreshStorageMetrics();
    } catch (err) {
      console.error('Failed to toggle secure storage:', err);
      useSecureStorage = !useSecureStorage; // Revert on error
    }
  }
</script>

<div class="storage-panel">
  <h2>Storage Configuration</h2>
  
  {#if $storageMetrics.error}
    <div class="error-state">
      <div class="error-icon">⚠️</div>
      <p>Failed to load storage data: {$storageMetrics.error}</p>
      <button on:click={refreshStorageMetrics}>Retry</button>
    </div>
  {:else}
    <div class="config-item">
      <label>Storage Limit (GB):</label>
      <input type="range" min="1" max="2048" bind:value={newStorageLimit}
             on:change={handleSetStorageLimit}
             disabled={$storageMetrics.loading}>
      <span title="Set storage limit in gigabytes">{newStorageLimit} GB</span>
    </div>

    <div class="usage-meter">
      <div class="meter-bar" style={`width: ${Math.min(100, (bytesToGB($storageMetrics.data.used) / newStorageLimit) * 100)}%`} />
      <div class="meter-labels">
        <span>0 GB</span>
        <span title="Current storage usage">{bytesToGB($storageMetrics.data.used).toFixed(2)} GB used</span>
        <span title="Storage limit">{newStorageLimit.toFixed(1)} GB</span>
      </div>
    </div>
    
    <div class="chart-section">
      <h3>Storage Breakdown</h3>
      <StorageUsageChart breakdown={$storageMetrics.data.breakdown} />
    </div>
    
    <div class="security-section">
      <h3>Security Settings</h3>
      <label>
        <input type="checkbox" bind:checked={useSecureStorage} on:change={toggleSecureStorage}>
        Use Secure Storage for sensitive data
      </label>
      <p class="hint">Encrypts API keys and credentials using AES-256-GCM</p>
      <p class="hint">Current secure storage: {(secureStorageSize / 1024 / 1024).toFixed(2)} MB</p>
    </div>
    
    <div class="footer">
      <div class="timestamp">
        Last updated: {formatDate($storageMetrics.lastUpdated)}
      </div>
      <button on:click={refreshStorageMetrics} disabled={$storageMetrics.loading}>
        {#if $storageMetrics.loading}
          <span class="spinner">⏳</span> Refreshing...
        {:else}
          Refresh
        {/if}
      </button>
    </div>
  {/if}
</div>

<style>
  .storage-panel {
    padding: 20px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  .config-item {
    margin: 15px 0;
    display: flex;
    align-items: center;
    gap: 10px;
  }
  input[type="range"] {
    flex: 1;
  }
  .usage-meter {
    margin-top: 20px;
    height: 20px;
    background: #eee;
    border-radius: 10px;
    position: relative;
  }
  .meter-bar {
    height: 100%;
    background: #4CAF50;
    border-radius: 10px;
    transition: width 0.3s ease;
  }
  .meter-labels {
    display: flex;
    justify-content: space-between;
    margin-top: 5px;
    font-size: 0.8em;
    color: #666;
  }
  .meter-labels span:hover {
    text-decoration: underline;
    cursor: help;
  }
  .chart-section {
    margin-top: 20px;
  }
  .footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 15px;
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