<script>
  import { onMount, afterUpdate } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { currentPath, assets } from '../stores/assetStore';
  import { toast } from '@zerodevx/svelte-toast'; // Using a toast library for notifications
  import { createEventDispatcher } from 'svelte';
  
  const dispatch = createEventDispatcher();
  
  // Local state
  let searchTerm = '';
  let isDragging = false;
  let filteredAssets = [];
  
  // Filter assets based on search term
  $: {
    if ($assets) {
      filteredAssets = $assets.filter(asset => 
        asset.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        asset.tags?.some(tag => tag.toLowerCase().includes(searchTerm.toLowerCase()))
      );
    }
  }
  
  // Fetch assets when path changes
  $: {
    if ($currentPath !== undefined) {
      fetchAssets();
    }
  }
  
  // Fetch assets from backend
  async function fetchAssets() {
    try {
      const result = await invoke('get_assets_in_path', { 
        path: $currentPath 
      });
      assets.set(result);
    } catch (error) {
      toast.push(`Error loading assets: ${error}`, { theme: { '--toastBackground': '#f44336' } });
    }
  }
  
  // Handle file import via drag-and-drop
  async function handleDrop(event) {
    event.preventDefault();
    isDragging = false;
    
    const files = Array.from(event.dataTransfer.files);
    if (files.length === 0) return;
    
    try {
      // Process each file
      for (const file of files) {
        const result = await invoke('import_asset', {
          filePath: file.path,
          targetPath: $currentPath
        });
        toast.push(`Imported: ${file.name}`);
      }
      
      // Refresh asset list
      fetchAssets();
    } catch (error) {
      toast.push(`Import failed: ${error}`, { theme: { 'toastBackground': '#f44336' } });
    }
  }
  
  // Navigate to a folder
  function navigateToFolder(path) {
    currentPath.set(path);
  }
  
  // Go up one level
  function goUp() {
    if ($currentPath === '') return;
    const pathParts = $currentPath.split('/');
    pathParts.pop();
    currentPath.set(pathParts.join('/'));
  }
  
  // Initialize component
  onMount(() => {
    if ($currentPath === undefined) {
      currentPath.set('');
    }
  });
</script>

<div class="asset-browser"
     on:dragover={() => isDragging = true}
     on:dragleave={() => isDragging = false}
     on:drop={handleDrop}
     class:active-drag={isDragging}>
  
  <!-- Breadcrumb navigation -->
  <div class="breadcrumbs">
    <button on:click={goUp} class="up-button" disabled={$currentPath === ''}>
      ↑
    </button>
    <span class="path-display">/{$currentPath}</span>
  </div>
  
  <!-- Search and filter -->
  <div class="search-bar">
    <input type="text" bind:value={searchTerm} placeholder="Search assets..." />
    <button on:click={() => searchTerm = ''}>Clear</button>
  </div>
  
  <!-- Asset grid -->
  <div class="asset-grid">
    {#each filteredAssets as asset (asset.id)}
      <div class="asset-card" on:click={() => handleSelect(asset)}>
        {#if asset.thumbnail_url}
          <img src={asset.thumbnail_url} alt={asset.name} class="thumbnail" />
        {:else}
          <div class="thumbnail placeholder">
            {asset.type === 'folder' ? '📁' : '📄'}
          </div>
        {/if}
        <div class="asset-name">{asset.name}</div>
        {#if asset.lock_info}
          <div class="lock-indicator" title="Locked by {asset.lock_info.user_name}">🔒</div>
        {/if}
      </div>
    {:else}
      <div class="empty-state">
        {searchTerm ? 'No assets match your search' : 'No assets in this folder'}
      </div>
    {/each}
  </div>
  
  <!-- Drag overlay -->
  {#if isDragging}
    <div class="drag-overlay">
      <div class="drag-message">Drop files to import</div>
    </div>
  {/if}
</div>

<style>
  .asset-browser {
    display: flex;
    flex-direction: column;
    height: 100%;
    position: relative;
  }
  
  .breadcrumbs {
    display: flex;
    align-items: center;
    padding: 8px;
    background-color: #f5f5f5;
    border-bottom: 1px solid #ddd;
  }
  
  .up-button {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 1.2rem;
    padding: 4px 8px;
  }
  
  .up-button:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }
  
  .path-display {
    margin-left: 8px;
    font-size: 0.9rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  
  .search-bar {
    padding: 8px;
    display: flex;
    gap: 8px;
  }
  
  .search-bar input {
    flex-grow: 1;
    padding: 6px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
  
  .asset-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 16px;
    padding: 16px;
    overflow-y: auto;
    flex-grow: 1;
  }
  
  .asset-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    cursor: pointer;
    border-radius: 4px;
    overflow: hidden;
    position: relative;
    transition: transform 0.2s;
  }
  
  .asset-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.1);
  }
  
  .thumbnail {
    width: 100%;
    height: 100px;
    object-fit: cover;
    background-color: #f0f0f0;
  }
  
  .thumbnail.placeholder {
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 2rem;
  }
  
  .asset-name {
    margin-top: 8px;
    text-align: center;
    font-size: 0.8rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: 100%;
  }
  
  .lock-indicator {
    position: absolute;
    top: 4px;
    right: 4px;
    background: rgba(255,255,255,0.7);
    border-radius: 50%;
    padding: 2px;
  }
  
  .empty-state {
    grid-column: 1 / -1;
    text-align: center;
    padding: 40px 0;
    color: #888;
  }
  
  .drag-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(33, 150, 243, 0.1);
    border: 2px dashed #2196f3;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 10;
  }
  
  .drag-message {
    background: white;
    padding: 20px 40px;
    border-radius: 8px;
    font-weight: bold;
    color: #2196f3;
  }
  
  .active-drag {
    background-color: rgba(33, 150, 243, 0.05);
  }
</style>