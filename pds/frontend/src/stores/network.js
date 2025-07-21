import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

// Initialize the store with default values including state flags
export const networkStatus = writable({
  data: {
    peers: 0,
    isOnline: false,
    bandwidthUp: 0,
    bandwidthDown: 0
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
  intervalId = setInterval(refreshNetworkStatus, 5000);
}

function handleVisibilityChange() {
  if (document.visibilityState === 'visible') {
    setupRefresh();
    refreshNetworkStatus(); // Refresh immediately when tab becomes visible
  } else {
    clearInterval(intervalId);
    intervalId = null;
  }
}

document.addEventListener('visibilitychange', handleVisibilityChange);

export async function refreshNetworkStatus() {
  networkStatus.update(status => ({ ...status, loading: true, error: null }));
  try {
    const data = await invoke('get_network_status');
    networkStatus.set({
      data,
      loading: false,
      error: null,
      lastUpdated: new Date()
    });
  } catch (err) {
    networkStatus.update(status => ({
      ...status,
      loading: false,
      error: err.message,
      lastUpdated: new Date()
    }));
  }
}

// Start the periodic refresh initially
setupRefresh();