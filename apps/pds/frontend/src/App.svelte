<script>
  import { onMount } from 'svelte';
  import { refreshNetworkStatus } from './stores/network';
  import { theme } from './stores/theme';
  
  import NetworkStatusDashboard from './components/NetworkStatusDashboard.svelte';
  import StorageConfigPanel from './components/StorageConfigPanel.svelte';
  import ContentBrowser from './components/ContentBrowser.svelte';
  import SettingsPage from './components/SettingsPage.svelte';
  import Timeline from './lib/social/Timeline.svelte';
  
  let currentView = 'network';
  
  // Initialize network status on app start
  onMount(() => {
    refreshNetworkStatus();
  });
</script>

<main class={$theme}>
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
      <button class:active={currentView === 'social'} on:click={() => currentView = 'social'}>
        Social
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
    {:else if currentView === 'social'}
      <Timeline />
    {/if}
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: Arial, sans-serif;
  }
  :global(.light) {
    --background: #f0f0f0;
    --content-bg: white;
    --text: black;
    --accent: #007bff;
  }
  :global(.dark) {
    --background: #333;
    --content-bg: #444;
    --text: white;
    --accent: #4da6ff;
  }
  :global(.blue) {
    --background: #e6f2ff;
    --content-bg: white;
    --text: #004080;
    --accent: #0066cc;
  }
  :global(.green) {
    --background: #e6ffe6;
    --content-bg: white;
    --text: #006600;
    --accent: #009900;
  }
  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background: var(--background);
    color: var(--text);
  }
  /* Update other styles to use variables, e.g. */
  .content {
    background: var(--content-bg);
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  button.active {
    background: var(--accent);
    color: var(--content-bg);
  }
</style>