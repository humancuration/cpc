import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

// Initialize the store with default values
export const storageMetrics = writable({
  data: {
    used: 0,
    limit: 1 * 1024 * 1024 * 1024, // Default 1GB in bytes
    breakdown: {}
  },
  loading: false,
  error: null,
  lastUpdated: null
});

let intervalId = null;

function setupRefresh() {
  if (intervalId) {
    clearInterval(intervalId);
  }
  intervalId = setInterval(refreshStorageMetrics, 10000); // Refresh every 10 seconds
}

function handleVisibilityChange() {
  if (document.visibilityState === 'visible') {
    setupRefresh();
    refreshStorageMetrics(); // Refresh immediately when tab becomes visible
  } else {
    clearInterval(intervalId);
    intervalId = null;
  }
}

document.addEventListener('visibilitychange', handleVisibilityChange);

export async function refreshStorageMetrics() {
  storageMetrics.update(metrics => ({ ...metrics, loading: true, error: null }));
  
  try {
    // Fetch storage usage
    const usage = await invoke('get_storage_usage');
    
    // Fetch storage breakdown
    const breakdown = await invoke('get_storage_breakdown');
    
    storageMetrics.set({
      data: {
        used: usage.used,
        limit: usage.limit,
        breakdown
      },
      loading: false,
      error: null,
      lastUpdated: new Date()
    });
  } catch (err) {
    storageMetrics.update(metrics => ({
      ...metrics,
      loading: false,
      error: err.message,
      lastUpdated: new Date()
    }));
  }
}

export async function setStorageLimit(limitGB) {
  const limitBytes = limitGB * 1024 * 1024 * 1024;
  try {
    await invoke('set_storage_limit', { limit: limitBytes });
    await refreshStorageMetrics();
  } catch (err) {
    console.error('Failed to set storage limit:', err);
    throw err;
  }
}

// Start the periodic refresh initially
setupRefresh();