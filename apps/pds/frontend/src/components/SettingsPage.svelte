<script>
  import { invoke } from '@tauri-apps/api';
  import { theme } from '../stores/theme';
  
  let encryptionKey = '';
  let networkProtocols = ['TCP', 'QUIC', 'WebSockets'];
  let selectedProtocols = ['TCP', 'WebSockets'];
  let bootstrapNodes = ['/ip4/10.0.0.1/tcp/4001/p2p/12D3KooWM8s3KQT7LKUpZb7hY4E3AbM4WZ1xWKQqQK3q4VZ7V5v2'];
  let newNode = '';
  let selectedTheme = '';

  $: selectedTheme = $theme;

  async function saveSettings() {
    try {
      // TODO: Implement protocol configuration commands
      console.log('Settings saved');
    } catch (err) {
      console.error('Failed to save settings:', err);
    }
  }

  function addBootstrapNode() {
    if (newNode.trim()) {
      bootstrapNodes = [...bootstrapNodes, newNode.trim()];
      newNode = '';
    }
  }

  function updateTheme(newTheme) {
    theme.set(newTheme);
  }
</script>

<div class="settings-page">
  <h2>Settings</h2>
  
  <div class="settings-section">
    <h3>Encryption</h3>
    <div class="form-group">
      <label>Current Encryption Key:</label>
      <input type="password" bind:value={encryptionKey} placeholder="Enter new key..." />
    </div>
  </div>

  <div class="settings-section">
    <h3>Network Protocols</h3>
    {#each networkProtocols as protocol}
      <label>
        <input type="checkbox" bind:group={selectedProtocols} value={protocol} />
        {protocol}
      </label>
    {/each}
  </div>

  <div class="settings-section">
    <h3>Visual Theme</h3>
    <select bind:value={selectedTheme} on:change={(e) => updateTheme(e.target.value)}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="blue">Blue</option>
      <option value="green">Green</option>
    </select>
  </div>

  <div class="settings-section">
    <h3>Bootstrap Nodes</h3>
    <div class="node-list">
      {#each bootstrapNodes as node}
        <div class="node-item">{node}</div>
      {/each}
    </div>
    <div class="add-node">
      <input type="text" bind:value={newNode} placeholder="Enter new node address..." />
      <button on:click={addBootstrapNode}>Add Node</button>
    </div>
  </div>

  <button on:click={saveSettings} class="save-button">Save Settings</button>
</div>

<style>
  .settings-page {
    padding: 20px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  .settings-section {
    margin: 20px 0;
    padding: 15px;
    border: 1px solid #eee;
    border-radius: 4px;
  }
  .form-group {
    margin: 10px 0;
  }
  .node-list {
    margin: 10px 0;
  }
  .node-item {
    padding: 5px;
    background: #f9f9f9;
    margin: 5px 0;
    border-radius: 4px;
  }
  .add-node {
    display: flex;
    gap: 10px;
    margin-top: 15px;
  }
  .save-button {
    margin-top: 20px;
    width: 100%;
  }
</style>