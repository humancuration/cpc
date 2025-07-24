<script>
  import { onMount } from 'svelte';
  import { 
    toolRegistryService, 
    getFeaturedTools, 
    getRecentTools,
    TOOL_CATEGORIES 
  } from './tool-registry.js';
  
  export let userId = null;
  export let onToolSelect = () => {};
  
  let featuredTools = [];
  let recentTools = [];
  let categories = [];
  
  onMount(() => {
    toolRegistryService.initialize();
    
    // Subscribe to featured and recent tools
    const unsubscribeFeatured = getFeaturedTools.subscribe(tools => {
      featuredTools = tools;
    });
    
    const unsubscribeRecent = getRecentTools.subscribe(tools => {
      recentTools = tools;
    });
    
    categories = Object.values(TOOL_CATEGORIES);
    
    return () => {
      unsubscribeFeatured();
      unsubscribeRecent();
    };
  });
  
  function handleToolSelect(tool) {
    toolRegistryService.trackUsage(tool.id);
    onToolSelect(tool);
  }
  
  function getToolsByCategory(categoryId) {
    // This would normally use the derived store
    return [];
  }
</script>

<div class="dashboard-landing">
  <div class="hero-section">
    <h1>Business Tools Suite</h1>
    <p class="subtitle">Comprehensive tools for managing your cooperative or business</p>
  </div>
  
  <!-- Quick Actions -->
  <div class="quick-actions">
    <h2>Quick Actions</h2>
    <div class="action-grid">
      <button class="action-card" on:click={() => handleToolSelect({id: 'accounting-dashboard'})}>
        <div class="action-icon">💰</div>
        <h3>View Finances</h3>
        <p>Check your financial overview</p>
      </button>
      
      <button class="action-card" on:click={() => handleToolSelect({id: 'inventory-manager'})}>
        <div class="action-icon">📦</div>
        <h3>Manage Inventory</h3>
        <p>Track stock and suppliers</p>
      </button>
      
      <button class="action-card" on:click={() => handleToolSelect({id: 'impact-reports'})}>
        <div class="action-icon">📊</div>
        <h3>Generate Report</h3>
        <p>Create impact and financial reports</p>
      </button>
      
      <button class="action-card" on:click={() => handleToolSelect({id: 'member-directory'})}>
        <div class="action-icon">👥</div>
        <h3>Manage Members</h3>
        <p>Cooperative member management</p>
      </button>
    </div>
  </div>
  
  <!-- Featured Tools -->
  {#if featuredTools.length > 0}
    <div class="featured-tools">
      <h2>Featured Tools</h2>
      <div class="tools-grid">
        {#each featuredTools as tool}
          <div class="tool-card" on:click={() => handleToolSelect(tool)}>
            <div class="tool-header">
              <span class="tool-icon">{tool.icon}</span>
              {#if tool.isBeta}
                <span class="beta-badge">Beta</span>
              {/if}
            </div>
            <h3>{tool.name}</h3>
            <p>{tool.description}</p>
            <div class="tool-footer">
              <span class="tool-category">{TOOL_CATEGORIES[tool.category.toUpperCase()]?.name || tool.category}</span>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
  
  <!-- Recent Tools -->
  {#if recentTools.length > 0}
    <div class="recent-tools">
      <h2>Recently Used</h2>
      <div class="recent-list">
        {#each recentTools as tool}
          <button class="recent-item" on:click={() => handleToolSelect(tool)}>
            <span class="recent-icon">{tool.icon}</span>
            <div class="recent-info">
              <h4>{tool.name}</h4>
              <p>{tool.description}</p>
            </div>
            <svg class="recent-arrow" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
            </svg>
          </button>
        {/each}
      </div>
    </div>
  {/if}
  
  <!-- Categories Overview -->
  <div class="categories-overview">
    <h2>Tool Categories</h2>
    <div class="categories-grid">
      {#each categories as category}
        <button 
          class="category-card" 
          style="--category-color: {category.color}"
          on:click={() => toggleCategory(category.id)}
        >
          <div class="category-icon">{category.icon}</div>
          <h3>{category.name}</h3>
          <p>{category.description}</p>
          <div class="category-stats">
            <span>{getToolsByCategory(category.id).length} tools</span>
          </div>
        </button>
      {/each}
    </div>
  </div>
</div>

<style>
  .dashboard-landing {
    padding: 2rem;
    max-width: 1200px;
    margin: 0 auto;
  }

  .hero-section {
    text-align: center;
    margin-bottom: 3rem;
  }

  .hero-section h1 {
    font-size: 2.5rem;
    font-weight: 700;
    color: #1a1a1a;
    margin-bottom: 0.5rem;
  }

  .subtitle {
    font-size: 1.25rem;
    color: #666;
  }

  .quick-actions,
  .featured-tools,
  .recent-tools,
  .categories-overview {
    margin-bottom: 3rem;
  }

  .quick-actions h2,
  .featured-tools h2,
  .recent-tools h2,
  .categories-overview h2 {
    font-size: 1.5rem;
    font-weight: 600;
    color: #1a1a1a;
    margin-bottom: 1.5rem;
  }

  .action-grid,
  .tools-grid,
  .categories-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .action-card,
  .tool-card,
  .category-card {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    padding: 1.5rem;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
  }

  .action-card:hover,
  .tool-card:hover,
  .category-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }

  .action-icon,
  .tool-icon,
  .category-icon {
    font-size: 2rem;
    margin-bottom: 1rem;
  }

  .tool-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.5rem;
  }

  .tool-card h3,
  .action-card h3,
  .category-card h3 {
    font-size: 1.125rem;
    font-weight: 600;
    color: #1a1a1a;
    margin-bottom: 0.5rem;
  }

  .tool-card p,
  .action-card p,
  .category-card p {
    color: #666;
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .tool-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .tool-category {
    font-size: 0.75rem;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .beta-badge {
    font-size: 0.75rem;
    padding: 0.125rem 0.375rem;
    background-color: #fef3c7;
    color: #92400e;
    border-radius: 0.25rem;
    font-weight: 500;
  }

  .recent-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .recent-item {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s;
    text-align: left;
    width: 100%;
  }

  .recent-item:hover {
    background-color: #f9fafb;
  }

  .recent-icon {
    font-size: 1.5rem;
  }

  .recent-info {
    flex: 1;
  }

  .recent-info h4 {
    font-size: 1rem;
    font-weight: 500;
    color: #1a1a1a;
    margin-bottom: 0.25rem;
  }

  .recent-info p {
    font-size: 0.875rem;
    color: #666;
  }

  .recent-arrow {
    width: 1.25rem;
    height: 1.25rem;
    color: #9ca3af;
  }

  .category-stats {
    font-size: 0.875rem;
    color: var(--category-color);
    font-weight: 500;
  }

  @media (max-width: 768px) {
    .dashboard-landing {
      padding: 1rem;
    }

    .hero-section h1 {
      font-size: 2rem;
    }

    .action-grid,
    .tools-grid,
    .categories-grid {
      grid-template-columns: 1fr;
    }
  }
</style>