<script>
  export let title = '';
  export let data = [];
  export let columns = [];
  export let pageSize = 10;
  
  let currentPage = 1;
  
  $: paginatedData = data.slice(
    (currentPage - 1) * pageSize,
    currentPage * pageSize
  );
  
  $: totalPages = Math.ceil(data.length / pageSize);
  
  function formatValue(value, format) {
    if (value === null || value === undefined) return '';
    
    switch (format) {
      case 'currency':
        return new Intl.NumberFormat('en-US', {
          style: 'currency',
          currency: 'USD'
        }).format(value);
      case 'number':
        return new Intl.NumberFormat().format(value);
      case 'percentage':
        return `${(value * 100).toFixed(1)}%`;
      default:
        return String(value);
    }
  }
  
  function goToPage(page) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }
</script>

<div class="data-table-widget">
  {#if title}
    <h3 class="table-title">{title}</h3>
  {/if}
  
  {#if data.length > 0}
    <div class="table-container">
      <table>
        <thead>
          <tr>
            {#each columns as column}
              <th>{column.label}</th>
            {/each}
          </tr>
        </thead>
        <tbody>
          {#each paginatedData as row}
            <tr>
              {#each columns as column}
                <td>
                  {formatValue(row[column.key], column.format)}
                </td>
              {/each}
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
    
    {#if totalPages > 1}
      <div class="pagination">
        <button 
          on:click={() => goToPage(currentPage - 1)}
          disabled={currentPage === 1}
        >
          Previous
        </button>
        
        <span class="page-info">
          Page {currentPage} of {totalPages}
        </span>
        
        <button 
          on:click={() => goToPage(currentPage + 1)}
          disabled={currentPage === totalPages}
        >
          Next
        </button>
      </div>
    {/if}
  {:else}
    <div class="empty-state">
      <p>No data available</p>
    </div>
  {/if}
</div>

<style>
  .data-table-widget {
    background: white;
    border-radius: 8px;
    padding: 1.5rem;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
    border: 1px solid #e0e0e0;
  }

  .table-title {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 1rem;
    color: #1a1a1a;
  }

  .table-container {
    overflow-x: auto;
    margin-bottom: 1rem;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }

  th, td {
    padding: 0.75rem;
    text-align: left;
    border-bottom: 1px solid #e0e0e0;
  }

  th {
    font-weight: 600;
    color: #666;
    background-color: #f5f5f5;
  }

  tr:hover {
    background-color: #f9f9f9;
  }

  .pagination {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  .pagination button {
    background: #3498db;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .pagination button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .page-info {
    color: #666;
    font-size: 0.875rem;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #666;
    font-style: italic;
  }

  @media (max-width: 768px) {
    .data-table-widget {
      padding: 1rem;
    }

    th, td {
      padding: 0.5rem;
      font-size: 0.75rem;
    }
  }
</style>