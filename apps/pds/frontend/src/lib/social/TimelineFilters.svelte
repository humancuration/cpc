<script>
  import { timelineStore } from '$stores/social/TimelineViewModel.js';
  
  let selectedFilter = 'all';
  let selectedAuthor = '';
  
  const filterOptions = [
    { value: 'all', label: 'All Posts' },
    { value: 'posts', label: 'Posts Only' },
    { value: 'replies', label: 'Replies Only' },
    { value: 'media', label: 'With Media' }
  ];
  
  function handleFilterChange() {
    timelineStore.setFilters({
      type: selectedFilter,
      authorId: selectedAuthor || null
    });
  }
  
  function handleAuthorChange() {
    handleFilterChange();
  }
</script>

<div class="timeline-filters">
  <div class="filter-group">
    <label for="filter-type">Filter by type:</label>
    <select 
      id="filter-type" 
      bind:value={selectedFilter}
      on:change={handleFilterChange}
    >
      {#each filterOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  </div>
  
  <div class="filter-group">
    <label for="filter-author">Filter by author:</label>
    <input 
      id="filter-author"
      type="text"
      placeholder="Enter author ID..."
      bind:value={selectedAuthor}
      on:input={handleAuthorChange}
    />
  </div>
</div>

<style>
  .timeline-filters {
    display: flex;
    gap: 20px;
    padding: 20px;
    background: var(--bg-secondary);
    border-radius: 8px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }
  
  .filter-group {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  
  label {
    font-size: 0.9rem;
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
  
  @media (max-width: 600px) {
    .timeline-filters {
      flex-direction: column;
      gap: 15px;
    }
  }
</style>