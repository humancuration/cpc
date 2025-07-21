<script>
  import { invoke } from '@tauri-apps/api';
  import { onMount } from 'svelte';
  import { getFileIcon, getFileCategory } from '../utils/fileUtils';
  
  let currentPath = "/";
  let files = [];
  let filteredFiles = [];
  let isLoading = true;
  let searchQuery = '';
  let selectedCategory = 'all';
  let previewFile = null;
  let showUploadDialog = false;

  async function loadFiles() {
    isLoading = true;
    try {
      files = await invoke('list_files', { path: currentPath });
      filterFiles();
    } catch (err) {
      console.error('Failed to list files:', err);
      files = [];
      filteredFiles = [];
    }
    isLoading = false;
  }

  function filterFiles() {
    filteredFiles = files.filter(file => {
      const matchesSearch = file.toLowerCase().includes(searchQuery.toLowerCase());
      const categoryMatch = selectedCategory === 'all' ||
                           getFileCategory(file) === selectedCategory;
      return matchesSearch && categoryMatch;
    });
  }

  onMount(loadFiles);

  function navigate(path: string) {
    currentPath = path;
    loadFiles();
  }

  async function downloadFile(filename) {
    try {
      await invoke('download_file', {
        path: `${currentPath}/${filename}`
      });
      console.log(`Downloaded file: ${filename}`);
    } catch (err) {
      console.error('Download failed:', err);
    }
  }

  async function previewFile(filename) {
    try {
      const preview = await invoke('get_file_preview', {
        path: `${currentPath}/${filename}`
      });
      previewFile = {
        name: filename,
        type: preview.content_type,
        data: preview.data
      };
    } catch (err) {
      console.error('Preview failed:', err);
    }
  }
</script>

<div class="content-browser">
  <h2>Content Browser</h2>
  
  <div class="path-navigation">
    <button on:click={() => navigate('/')}>Home</button>
    <span>{currentPath}</span>
  </div>

  <div class="controls">
    <input type="text" placeholder="Search files..." bind:value={searchQuery} on:input={filterFiles}>
    <select bind:value={selectedCategory} on:change={filterFiles}>
      <option value="all">All Files</option>
      <option value="documents">Documents</option>
      <option value="images">Images</option>
      <option value="archives">Archives</option>
      <option value="code">Code</option>
    </select>
    <button on:click={() => showUploadDialog = true}>Upload</button>
  </div>

  {#if isLoading}
    <p>Loading files...</p>
  {:else}
    <div class="file-list">
      {#each filteredFiles as file}
        <div class="file-item">
          <span class="file-icon">{getFileIcon(file)}</span>
          <span class="file-name">{file}</span>
          <div class="file-actions">
            <button on:click|stopPropagation={() => downloadFile(file)}>Download</button>
            <button on:click|stopPropagation={() => previewFile(file)}>Preview</button>
          </div>
        </div>
      {:else}
        <p>No files found</p>
      {/each}
    </div>
  {/if}
  
  {#if previewFile}
    <div class="preview-modal">
      <button class="close-btn" on:click={() => previewFile = null}>✕</button>
      <h3>Preview: {previewFile.name}</h3>
      {#if previewFile.type.startsWith('text/')}
        <pre>{previewFile.data}</pre>
      {:else if previewFile.type.startsWith('image/')}
        <img src={`data:${previewFile.type};base64,${previewFile.data}`} alt="Preview">
      {:else}
        <p>Preview not available for this file type</p>
      {/if}
    </div>
  {/if}
</div>

<style>
  .content-browser {
    padding: 20px;
    background: #fff;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    position: relative;
  }
  .path-navigation {
    margin-bottom: 15px;
    display: flex;
    gap: 10px;
    align-items: center;
  }
  .controls {
    display: flex;
    gap: 10px;
    margin-bottom: 15px;
    flex-wrap: wrap;
  }
  .controls input, .controls select {
    padding: 8px;
    border: 1px solid #ddd;
    border-radius: 4px;
  }
  .controls button {
    padding: 8px 12px;
    background: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  .file-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 15px;
  }
  .file-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 10px;
    border: 1px solid #eee;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
    position: relative;
  }
  .file-item:hover {
    background: #f9f9f9;
  }
  .file-icon {
    font-size: 24px;
    margin-bottom: 5px;
  }
  .file-name {
    font-size: 12px;
    text-align: center;
    word-break: break-all;
  }
  .file-actions {
    display: flex;
    gap: 5px;
    margin-top: 5px;
  }
  .file-actions button {
    padding: 4px 8px;
    font-size: 10px;
    background: #f0f0f0;
    color: #333;
    border: 1px solid #ddd;
    border-radius: 3px;
    cursor: pointer;
  }
  .preview-modal {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0,0,0,0.2);
    z-index: 1000;
    max-width: 80vw;
    max-height: 80vh;
    overflow: auto;
  }
  .preview-modal pre {
    max-width: 100%;
    overflow: auto;
    background: #f8f8f8;
    padding: 10px;
    border-radius: 4px;
  }
  .preview-modal img {
    max-width: 100%;
    max-height: 60vh;
    display: block;
    margin: 0 auto;
  }
  .close-btn {
    position: absolute;
    top: 10px;
    right: 10px;
    background: none;
    border: none;
    font-size: 18px;
    cursor: pointer;
    color: #666;
  }
</style>