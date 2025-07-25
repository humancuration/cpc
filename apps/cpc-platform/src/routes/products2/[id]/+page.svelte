<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  
  // Mock product data for testing
  let product = {
    id: '1',
    name: 'Eco-Friendly Water Bottle',
    description: 'Sustainable water bottle made from recycled materials with a lifetime warranty.',
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
    // Navigate back to products list
    window.location.href = '/products2';
  }
</script>

<div class="product-detail-page">
  <button on:click={handleBack} class="back-button">← Back to Products</button>
  
  <!-- Product Header -->
  <div class="product-header">
    <h1>{product.name}</h1>
    <p>{product.description}</p>
  </div>
  
  <div class="product-content">
    <div class="product-main">
      <!-- Product Details -->
      <div class="product-section">
        <h2>Product Details</h2>
        <div class="details-grid">
          <div class="detail-item">
            <strong>ID:</strong>
            <span>{product.id}</span>
          </div>
          <div class="detail-item">
            <strong>Total Cost:</strong>
            <span>${product.total_cost.toFixed(2)}</span>
          </div>
          <div class="detail-item">
            <strong>Profit Margin:</strong>
            <span>{product.profit_margin.toFixed(1)}%</span>
          </div>
        </div>
      </div>
      
      <!-- Cost Breakdown -->
      <div class="product-section">
        <h2>Cost Breakdown</h2>
        <div class="cost-breakdown-chart">
          <!-- This would be where the plotters-rs chart would go -->
          <div class="chart-placeholder">
            <p>Cost Breakdown Chart (Plotters-rs visualization)</p>
          </div>
        </div>
        <ul class="cost-breakdown-list">
          {#each product.cost_breakdown as item}
            <li class="cost-item">
              <span class="category">{item.category}:</span>
              <span class="amount">${item.amount.toFixed(2)}</span>
              <span class="percentage">({item.percentage.toFixed(1)}%)</span>
            </li>
          {/each}
        </ul>
      </div>
      
      <!-- Supply Chain -->
      <div class="product-section">
        <h2>Supply Chain</h2>
        <div class="supply-chain-placeholder">
          <p>Supply chain visualization would appear here</p>
        </div>
      </div>
    </div>
    
    <div class="product-sidebar">
      <!-- Validation Status -->
      <div class="validation-status">
        <h3>Validation Status</h3>
        <div class="status-indicator status-{product.validation_status}">
          {#if product.validation_status === 'valid'}
            ✅ Validated
          {:else if product.validation_status === 'pending'}
            ⏳ Pending
          {:else}
            ⚠️ Issues Found
          {/if}
        </div>
        
        <h4>Recent Updates</h4>
        <ul class="validation-updates">
          {#each validationUpdates as update}
            <li class="update-item">
              <div class="update-timestamp">{new Date(update.timestamp).toLocaleString()}</div>
              <div class="update-message">{update.message}</div>
            </li>
          {/each}
        </ul>
        
        <button class="revalidate-button">Re-validate Product</button>
      </div>
    </div>
  </div>
</div>

<style>
  .product-detail-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    font-family: Arial, sans-serif;
  }
  
  .back-button {
    background: #f8f9fa;
    color: #333;
    border: 1px solid #ddd;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    margin-bottom: 20px;
    font-size: 14px;
  }
  
  .back-button:hover {
    background: #e9ecef;
  }
  
  .product-header {
    background: #f8f9fa;
    padding: 30px;
    border-radius: 8px;
    margin-bottom: 30px;
  }
  
  .product-header h1 {
    margin-top: 0;
    color: #333;
  }
  
  .product-content {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 30px;
  }
  
  .product-section {
    background: white;
    padding: 25px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    margin-bottom: 25px;
  }
  
  .product-section h2 {
    margin-top: 0;
    color: #333;
    border-bottom: 1px solid #eee;
    padding-bottom: 10px;
  }
  
  .details-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 15px;
  }
  
  .detail-item {
    display: flex;
    justify-content: space-between;
    padding: 10px 0;
    border-bottom: 1px solid #f5f5f5;
  }
  
  .chart-placeholder, .supply-chain-placeholder {
    background: #f8f9fa;
    border: 2px dashed #dee2e6;
    border-radius: 4px;
    padding: 40px;
    text-align: center;
    color: #666;
    margin-bottom: 20px;
  }
  
  .cost-breakdown-list {
    list-style-type: none;
    padding: 0;
    margin: 0;
  }
  
  .cost-item {
    display: flex;
    justify-content: space-between;
    padding: 12px 0;
    border-bottom: 1px solid #f5f5f5;
  }
  
  .category {
    font-weight: 500;
  }
  
  .amount {
    font-weight: 500;
  }
  
  .percentage {
    color: #666;
  }
  
  .validation-status {
    background: white;
    padding: 25px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .validation-status h3 {
    margin-top: 0;
    color: #333;
    border-bottom: 1px solid #eee;
    padding-bottom: 10px;
  }
  
  .status-indicator {
    padding: 15px;
    border-radius: 4px;
    text-align: center;
    font-weight: 500;
    margin-bottom: 20px;
  }
  
  .status-valid {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
  }
  
  .status-pending {
    background: #fff3cd;
    color: #856404;
    border: 1px solid #ffeaa7;
  }
  
  .validation-updates {
    list-style-type: none;
    padding: 0;
    margin: 0 0 20px 0;
  }
  
  .update-item {
    padding: 12px 0;
    border-bottom: 1px solid #f5f5f5;
  }
  
  .update-timestamp {
    font-size: 0.85em;
    color: #666;
    margin-bottom: 4px;
  }
  
  .update-message {
    font-size: 0.95em;
  }
  
  .revalidate-button {
    width: 100%;
    background: #007bff;
    color: white;
    border: none;
    padding: 12px;
    border-radius: 4px;
    cursor: pointer;
    font-weight: 500;
  }
  
  .revalidate-button:hover {
    background: #0056b3;
  }
  
  @media (max-width: 768px) {
    .product-content {
      grid-template-columns: 1fr;
    }
  }
</style>