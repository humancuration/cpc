<script>
  import { onMount } from 'svelte';
  import { refreshNetworkStatus } from './stores/network';
  
  import NetworkStatusDashboard from './components/NetworkStatusDashboard.svelte';
  import StorageConfigPanel from './components/StorageConfigPanel.svelte';
  import ContentBrowser from './components/ContentBrowser.svelte';
  import SettingsPage from './components/SettingsPage.svelte';
  
  let currentView = 'network';
  
  // Initialize network status on app start
  onMount(() => {
    refreshNetworkStatus();
  });
</script>

<main>
  <header>
    <h1>Cooperative Peer Cloud</h1>
    <nav>
      <button class:active={currentView === 'network'} on:click={() => currentView = 'network'}>
        Network
      </button>
      <button class:active={currentView === 'storage'} on:click={() => currentView = 'storage'}>
        Storage
      </button>
      <button class:active={currentView === 'content'} on:click={() => currentView = 'content'}>
        Content
      </button>
      <button class:active={currentView === 'settings'} on:click={() => currentView = 'settings'}>
        Settings
      </button>
    </nav>
  </header>

  <div class="content">
    {#if currentView === 'network'}
      <NetworkStatusDashboard />
    {:else if currentView === 'storage'}
      <StorageConfigPanel />
    {:else if currentView === 'content'}
      <ContentBrowser />
    {:else if currentView === 'settings'}
      <SettingsPage />
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: Arial, sans-serif;
    background: #f0f0f0;
  }
  
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
  
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px 0;
    border-bottom: 1px solid #ddd;
    margin-bottom: 20px;
  }
  
  nav {
    display: flex;
    gap: 10px;
  }
  
  button {
    padding: 8px 15px;
    background: #eee;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  button.active {
    background: #007bff;
    color: white;
  }
  
  .content {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
</style>