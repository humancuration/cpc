<script>
  import { onMount } from 'svelte';
  
  // Mock product data for testing
  let products = [
    {
      id: '1',
      name: 'Eco-Friendly Water Bottle',
      description: 'Sustainable water bottle made from recycled materials',
      total_cost: 8.00,
      profit_margin: 38.4,
      validation_status: 'valid'
    },
    {
      id: '2',
      name: 'Organic Cotton T-Shirt',
      description: 'Comfortable t-shirt made from 100% organic cotton',
      total_cost: 12.50,
      profit_margin: 42.0,
      validation_status: 'valid'
    },
    {
      id: '3',
      name: 'Bamboo Cutting Board',
      description: 'Durable cutting board made from sustainable bamboo',
      total_cost: 15.00,
      profit_margin: 35.0,
      validation_status: 'pending'
    }
  ];

  let selectedProductId = null;

  function selectProduct(productId) {
    selectedProductId = productId;
  }

  function goBack() {
    selectedProductId = null;
  }
</script>

<div class="products-page">
  <h1>Product Catalog</h1>
  
  {#if !selectedProductId}
    <div class="products-grid">
      {#each products as product}
        <div class="product-card" on:click={() => selectProduct(product.id)}>
          <h3>{product.name}</h3>
          <p>{product.description}</p>
          <div class="product-meta">
            <span>Cost: ${product.total_cost.toFixed(2)}</span>
            <span>Margin: {product.profit_margin.toFixed(1)}%</span>
            <span class="status-{product.validation_status}">Status: {product.validation_status}</span>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="product-detail">
      <button on:click={goBack} class="back-button">← Back to Products</button>
      <!-- This would be where the ProductDisplay component goes -->
      <div class="product-display-placeholder">
        <h2>Product Display Component</h2>
        <p>This is where the full product display component would be shown.</p>
        <p>Product ID: {selectedProductId}</p>
      </div>
    </div>
  {/if}
</div>

<style>
  .products-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    font-family: Arial, sans-serif;
  }
  
  .products-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
  }
  
  .product-card {
    background: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    cursor: pointer;
    transition: transform 0.2s;
  }
  
  .product-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.15);
  }
  
  .product-card h3 {
    margin-top: 0;
    color: #333;
  }
  
  .product-meta {
    display: flex;
    justify-content: space-between;
    margin-top: 15px;
    font-size: 0.9em;
    color: #666;
  }
  
  .status-valid {
    color: green;
    font-weight: bold;
  }
  
  .status-pending {
    color: orange;
    font-weight: bold;
  }
  
  .product-detail {
    background: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .back-button {
    background: #f8f9fa;
    color: #333;
    border: 1px solid #ddd;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
    margin-bottom: 20px;
  }
  
  .back-button:hover {
    background: #e9ecef;
  }
  
  .product-display-placeholder {
    text-align: center;
    padding: 40px;
    color: #666;
  }
</style>