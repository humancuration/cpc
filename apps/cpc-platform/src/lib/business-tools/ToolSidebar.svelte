<script>
  import { onMount } from 'svelte';
  import { 
    toolRegistry, 
    TOOL_CATEGORIES, 
    getToolsByCategory, 
    getRecentTools,
    searchTools,
    toolRegistryService 
  } from './tool-registry.js';
  
  export let currentTool = null;
  export let onToolSelect = () => {};
  
  let searchQuery = '';
  let expandedCategories = new Set();
  let recentTools = [];
  
  $: categories = $toolRegistry.categories;
  $: searchResults = $searchTools;
  $: isSearching = searchQuery.trim().length > 0;
  
  onMount(() => {
    toolRegistryService.initialize();
    
    // Subscribe to recent tools
    const unsubscribe = getRecentTools.subscribe(tools => {
      recentTools = tools;
    });
    
    return unsubscribe;
  });
  
  function toggleCategory(categoryId) {
    if (expandedCategories.has(categoryId)) {
      expandedCategories.delete(categoryId);
    } else {
      expandedCategories.add(categoryId);
    }
    expandedCategories = expandedCategories;
  }
  
  function handleToolSelect(tool) {
    toolRegistryService.trackUsage(tool.id);
    onToolSelect(tool);
  }
  
  function getToolsForCategory(categoryId) {
    return $getToolsByCategory(categoryId) || [];
  }
  
  function isCategoryActive(categoryId) {
    return currentTool && getToolsForCategory(categoryId).some(tool => tool.id === currentTool.id);
  }
</script>

<div class="tool-sidebar">
  <div class="sidebar-header">
    <h2>Business Tools</h2>
    <div class="search-container">
      <input 
        type="text" 
        placeholder="Search tools..." 
        bind:value={searchQuery}
        class="search-input"
      />
      <svg class="search-icon" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
      </svg>
    </div>
  </div>
  
  {#if isSearching}
    <div class="search-results">
      <h3>Search Results</h3>
      {#if searchResults.length > 0}
        <ul class="tool-list">
          {#each searchResults as tool}
            <li>
              <button 
                class="tool-item {currentTool?.id === tool.id ? 'active' : ''}"
                on:click={() => handleToolSelect(tool)}
              >
                <span class="tool-icon">{tool.icon}</span>
                <span class="tool-name">{tool.name}</span>
                {#if tool.isBeta}
                  <span class="beta-badge">Beta</span>
                {/if}
              </button>
            </li>
          {/each}
        </ul>
      {:else}
        <p class="no-results">No tools found matching "{searchQuery}"</p>
      {/if}
    </div>
  {:else}
    <!-- Recent Tools -->
    {#if recentTools.length > 0}
      <div class="section">
        <h3>Recent Tools</h3>
        <ul class="tool-list">
          {#each recentTools as tool}
            <li>
              <button 
                class="tool-item {currentTool?.id === tool.id ? 'active' : ''}"
                on:click={() => handleToolSelect(tool)}
              >
                <span class="tool-icon">{tool.icon}</span>
                <span class="tool-name">{tool.name}</span>
              </button>
            </li>
          {/each}
        </ul>
      </div>
    {/if}
    
    <!-- Categories -->
    <div class="categories">
      {#each categories as category}
        <div class="category">
          <button 
            class="category-header {isCategoryActive(category.id) ? 'active' : ''}"
            on:click={() => toggleCategory(category.id)}
          >
            <span class="category-icon" style="color: {category.color}">{category.icon}</span>
            <span class="category-name">{category.name}</span>
            <svg 
              class="chevron {expandedCategories.has(category.id) ? 'expanded' : ''}" 
              viewBox="0 0 20 20" 
              fill="currentColor"
            >
              <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
            </svg>
          </button>
          
          {#if expandedCategories.has(category.id)}
            <ul class="tool-list">
              {#each getToolsForCategory(category.id) as tool}
                <li>
                  <button 
                    class="tool-item {currentTool?.id === tool.id ? 'active' : ''}"
                    on:click={() => handleToolSelect(tool)}
                  >
                    <span class="tool-name">{tool.name}</span>
                    {#if tool.isBeta}
                      <span class="beta-badge">Beta</span>
                    {/if}
                  </button>
                </li>
              {/each}
            </ul>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .tool-sidebar {
    width: 280px;
    height: 100vh;
    background: #f8fafc;
    border-right: 1px solid #e2e8f0;
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .sidebar-header {
    padding: 1.5rem;
    border-bottom: 1px solid #e2e8f0;
  }

  .sidebar-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1e293b;
  }

  .search-container {
    position: relative;
  }

  .search-input {
    width: 100%;
    padding: 0.5rem 0.75rem 0.5rem 2.5rem;
    border: 1px solid #d1d5db;
    border-radius: 0.375rem;
    font-size: 0.875rem;
  }

  .search-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  }

  .search-icon {
    position: absolute;
    left: 0.75rem;
    top: 50%;
    transform: translateY(-50%);
    width: 1rem;
    height: 1rem;
    color: #6b7280;
  }

  .section {
    padding: 1rem 1.5rem;
    border-bottom: 1px solid #e2e8f0;
  }

  .section h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .categories {
    flex: 1;
    padding: 0.5rem 0;
  }

  .category {
    border-bottom: 1px solid #e2e8f0;
  }

  .category-header {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1.5rem;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    transition: background-color 0.2s;
  }

  .category-header:hover {
    background-color: #f1f5f9;
  }

  .category-header.active {
    background-color: #eff6ff;
    color: #1d4ed8;
  }

  .category-icon {
    width: 1.25rem;
    height: 1.25rem;
  }

  .category-name {
    flex: 1;
    text-align: left;
  }

  .chevron {
    width: 1rem;
    height: 1rem;
    transition: transform 0.2s;
  }

  .chevron.expanded {
    transform: rotate(180deg);
  }

  .tool-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .tool-list li {
    border-bottom: 1px solid #f1f5f9;
  }

  .tool-list li:last-child {
    border-bottom: none;
  }

  .tool-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1.5rem 0.5rem 3rem;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 0.875rem;
    color: #4b5563;
    transition: background-color 0.2s;
  }

  .tool-item:hover {
    background-color: #f1f5f9;
  }

  .tool-item.active {
    background-color: #eff6ff;
    color: #1d4ed8;
    font-weight: 500;
  }

  .tool-icon {
    width: 1rem;
    height: 1rem;
  }

  .tool-name {
    flex: 1;
    text-align: left;
  }

  .beta-badge {
    font-size: 0.75rem;
    padding: 0.125rem 0.375rem;
    background-color: #fef3c7;
    color: #92400e;
    border-radius: 0.25rem;
    font-weight: 500;
  }

  .search-results {
    padding: 1rem 1.5rem;
  }

  .search-results h3 {
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .no-results {
    color: #6b7280;
    font-size: 0.875rem;
    text-align: center;
    padding: 1rem 0;
  }

  @media (max-width: 768px) {
    .tool-sidebar {
      width: 100%;
      height: auto;
      border-right: none;
      border-bottom: 1px solid #e2e8f0;
    }
  }
</style>