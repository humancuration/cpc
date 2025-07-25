<script>
  import { onMount } from 'svelte';
  
  // Mock product data for testing
  let product = {
    id: '1',
    name: 'Test Product',
    description: 'This is a test product for demonstration purposes',
    cost_breakdown: [
      { category: 'Materials', amount: 5.00, percentage: 62.5 },
      { category: 'Labor', amount: 3.00, percentage: 37.5 }
    ],
    total_cost: 8.00,
    profit_margin: 38.4,
    validation_status: 'valid',
    image_urls: []
  };

  let validationUpdates = [
    { status: 'valid', message: 'Product data verified', timestamp: '2024-01-15T10:30:00Z' },
    { status: 'processing', message: 'Running quality checks', timestamp: '2024-01-15T10:25:00Z' }
  ];

  function handleBack() {
    console.log('Back button clicked');
  }
</script>

<div class="test-product-page">
  <h1>Product Display Test</h1>
  
  <!-- Product Header -->
  <div class="product-header">
    <h2>{product.name}</h2>
    <p>{product.description}</p>
  </div>
  
  <!-- Product Details -->
  <div class="product-details">
    <h3>Product Details</h3>
    <p><strong>ID:</strong> {product.id}</p>
    <p><strong>Total Cost:</strong> ${product.total_cost.toFixed(2)}</p>
    <p><strong>Profit Margin:</strong> {product.profit_margin.toFixed(1)}%</p>
  </div>
  
  <!-- Cost Breakdown -->
  <div class="cost-breakdown">
    <h3>Cost Breakdown</h3>
    <ul>
      {#each product.cost_breakdown as item}
        <li>
          <span>{item.category}:</span>
          <span>${item.amount.toFixed(2)} ({item.percentage.toFixed(1)}%)</span>
        </li>
      {/each}
    </ul>
  </div>
  
  <!-- Validation Status -->
  <div class="validation-status">
    <h3>Validation Status</h3>
    <p class="status-{product.validation_status}">Status: {product.validation_status}</p>
    
    <h4>Recent Updates</h4>
    <ul>
      {#each validationUpdates as update}
        <li>
          <span>{update.timestamp}:</span>
          <span>{update.message}</span>
        </li>
      {/each}
    </ul>
  </div>
  
  <!-- Back Button -->
  <button on:click={handleBack}>Back to Products</button>
</div>

<style>
  .test-product-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    font-family: Arial, sans-serif;
  }
  
  .product-header {
    background: #f8f9fa;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;
  }
  
  .product-details, .cost-breakdown, .validation-status {
    background: white;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .cost-breakdown ul, .validation-status ul {
    list-style-type: none;
    padding: 0;
  }
  
  .cost-breakdown li, .validation-status li {
    display: flex;
    justify-content: space-between;
    padding: 8px 0;
    border-bottom: 1px solid #eee;
  }
  
  .status-valid {
    color: green;
    font-weight: bold;
  }
  
  button {
    background: #007bff;
    color: white;
    padding: 10px 20px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  button:hover {
    background: #0056b3;
  }
</style>