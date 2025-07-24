<script>
  import { timelineStore } from '$stores/social/TimelineViewModel.js';
  import { onMount } from 'svelte';
  
  let selectedContentType = 'all';
  let selectedAuthor = '';
  let selectedVisibility = 'all';
  let dateFrom = '';
  let dateTo = '';
  let cooperativeOnly = false;
  
  const contentTypeOptions = [
    { value: 'all', label: 'All Content' },
    { value: 'posts', label: 'Posts Only' },
    { value: 'replies', label: 'Replies Only' },
    { value: 'media', label: 'With Media' }
  ];
  
  const visibilityOptions = [
    { value: 'all', label: 'All Visibility' },
    { value: 'PUBLIC', label: 'Public' },
    { value: 'COOPERATIVE', label: 'Cooperative' },
    { value: 'PRIVATE', label: 'Private' }
  ];
  
  onMount(() => {
    // Load any saved filters from store
    const unsubscribe = timelineStore.subscribe(state => {
      if (state.filters) {
        selectedContentType = state.filters.content_type || 'all';
        selectedAuthor = state.filters.author_id || '';
        selectedVisibility = state.filters.visibility || 'all';
        dateFrom = state.filters.date_from || '';
        dateTo = state.filters.date_to || '';
        cooperativeOnly = state.filters.cooperative_only || false;
      }
    });
    
    return unsubscribe;
  });
  
  function handleFilterChange() {
    const filters = {
      content_type: selectedContentType === 'all' ? null : selectedContentType,
      author_id: selectedAuthor || null,
      visibility: selectedVisibility === 'all' ? null : selectedVisibility,
      date_from: dateFrom ? new Date(dateFrom).toISOString() : null,
      date_to: dateTo ? new Date(dateTo).toISOString() : null,
      cooperative_only: cooperativeOnly
    };
    
    timelineStore.setFilters(filters);
  }
  
  function resetFilters() {
    selectedContentType = 'all';
    selectedAuthor = '';
    selectedVisibility = 'all';
    dateFrom = '';
    dateTo = '';
    cooperativeOnly = false;
    handleFilterChange();
  }
</script>

<div class="timeline-filters">
  <div class="filter-section">
    <h3>Timeline Filters</h3>
    
    <div class="filter-grid">
      <div class="filter-group">
        <label for="content-type">Content Type:</label>
        <select
          id="content-type"
          bind:value={selectedContentType}
          on:change={handleFilterChange}
        >
          {#each contentTypeOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
      
      <div class="filter-group">
        <label for="visibility">Visibility:</label>
        <select
          id="visibility"
          bind:value={selectedVisibility}
          on:change={handleFilterChange}
        >
          {#each visibilityOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </div>
      
      <div class="filter-group">
        <label for="author">Author ID:</label>
        <input
          id="author"
          type="text"
          placeholder="Enter author ID..."
          bind:value={selectedAuthor}
          on:input={handleFilterChange}
        />
      </div>
      
      <div class="filter-group">
        <label for="date-from">From Date:</label>
        <input
          id="date-from"
          type="date"
          bind:value={dateFrom}
          on:change={handleFilterChange}
        />
      </div>
      
      <div class="filter-group">
        <label for="date-to">To Date:</label>
        <input
          id="date-to"
          type="date"
          bind:value={dateTo}
          on:change={handleFilterChange}
        />
      </div>
      
      <div class="filter-group checkbox-group">
        <label>
          <input
            type="checkbox"
            bind:checked={cooperativeOnly}
            on:change={handleFilterChange}
          />
          Cooperative Only
        </label>
      </div>
    </div>
    
    <div class="filter-actions">
      <button class="reset-btn" on:click={resetFilters}>
        Reset Filters
      </button>
    </div>
  </div>
</div>

<style>
  .timeline-filters {
    background: var(--bg-secondary);
    border-radius: 8px;
    margin-bottom: 20px;
    border: 1px solid var(--border-color);
  }
  
  .filter-section {
    padding: 20px;
  }
  
  .filter-section h3 {
    margin: 0 0 16px 0;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-primary);
  }
  
  .filter-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
    margin-bottom: 16px;
  }
  
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  
  .filter-group label {
    font-size: 0.85rem;
    font-weight: 500;
    color: var(--text-secondary);
  }
  
  select, input {
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 0.9rem;
  }
  
  select:focus, input:focus {
    outline: none;
    border-color: var(--accent-color);
  }
  
  .checkbox-group {
    flex-direction: row;
    align-items: center;
    gap: 8px;
  }
  
  .checkbox-group input[type="checkbox"] {
    width: auto;
    margin: 0;
  }
  
  .filter-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
  }
  
  .reset-btn {
    padding: 6px 12px;
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--text-secondary);
    font-size: 0.85rem;
    cursor: pointer;
    transition: all 0.2s;
  }
  
  .reset-btn:hover {
    background: var(--bg-tertiary);
    color: var(--text-primary);
  }
  
  @media (max-width: 600px) {
    .filter-grid {
      grid-template-columns: 1fr;
    }
  }
</style>