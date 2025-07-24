<script>
  import { createInvoice } from '$lib/graphql/invoicing';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  
  let newInvoice = {
    recipient_id: '',
    items: [{ description: '', quantity: 1, unit_price: 0 }],
    due_date: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
    template_id: '',
    notes: '',
    tax_rate: 0,
    discount: 0
  };

  let templates = [];
  let contacts = [];
  let loading = false;
  let error = null;

  onMount(async () => {
    // Load templates and contacts
    try {
      const [templatesRes, contactsRes] = await Promise.all([
        fetch('/api/invoicing/templates'),
        fetch('/api/contacts')
      ]);
      
      templates = await templatesRes.json();
      contacts = await contactsRes.json();
    } catch (err) {
      error = 'Failed to load data';
    }
  });

  async function handleSubmit() {
    loading = true;
    error = null;
    
    try {
      const result = await createInvoice(newInvoice);
      if (result.data?.createInvoice) {
        goto(`/invoicing/invoice/${result.data.createInvoice.id}`);
      }
    } catch (err) {
      error = err.message || 'Failed to create invoice';
    } finally {
      loading = false;
    }
  }

  function addItem() {
    newInvoice.items = [...newInvoice.items, { description: '', quantity: 1, unit_price: 0 }];
  }

  function removeItem(index) {
    newInvoice.items = newInvoice.items.filter((_, i) => i !== index);
  }

  function updateItem(index, field, value) {
    newInvoice.items = newInvoice.items.map((item, i) => 
      i === index ? { ...item, [field]: value } : item
    );
  }

  $: subtotal = newInvoice.items.reduce((sum, item) => sum + (item.quantity * item.unit_price), 0);
  $: tax = subtotal * (newInvoice.tax_rate / 100);
  $: total = subtotal + tax - newInvoice.discount;
</script>

<div class="invoice-creator">
  <h2>Create New Invoice</h2>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-section">
      <h3>Invoice Details</h3>
      
      <label>
        Recipient
        <select bind:value={newInvoice.recipient_id} required>
          <option value="">Select recipient</option>
          {#each contacts as contact}
            <option value={contact.id}>{contact.name} ({contact.email})</option>
          {/each}
        </select>
      </label>

      <label>
        Due Date
        <input type="date" bind:value={newInvoice.due_date} required />
      </label>

      <label>
        Template
        <select bind:value={newInvoice.template_id}>
          <option value="">Default template</option>
          {#each templates as template}
            <option value={template.id}>{template.name}</option>
          {/each}
        </select>
      </label>
    </div>

    <div class="form-section">
      <h3>Items</h3>
      
      {#each newInvoice.items as item, index}
        <div class="item-row">
          <input 
            type="text" 
            placeholder="Description" 
            bind:value={item.description}
            on:input={(e) => updateItem(index, 'description', e.target.value)}
            required
          />
          <input 
            type="number" 
            placeholder="Quantity" 
            bind:value={item.quantity}
            on:input={(e) => updateItem(index, 'quantity', parseFloat(e.target.value) || 0)}
            min="0"
            step="0.01"
            required
          />
          <input 
            type="number" 
            placeholder="Unit Price" 
            bind:value={item.unit_price}
            on:input={(e) => updateItem(index, 'unit_price', parseFloat(e.target.value) || 0)}
            min="0"
            step="0.01"
            required
          />
          <span>${(item.quantity * item.unit_price).toFixed(2)}</span>
          <button type="button" on:click={() => removeItem(index)}>Remove</button>
        </div>
      {/each}
      
      <button type="button" on:click={addItem}>Add Item</button>
    </div>

    <div class="form-section">
      <h3>Pricing</h3>
      
      <label>
        Tax Rate (%)
        <input type="number" bind:value={newInvoice.tax_rate} min="0" max="100" step="0.01" />
      </label>

      <label>
        Discount ($)
        <input type="number" bind:value={newInvoice.discount} min="0" step="0.01" />
      </label>

      <div class="summary">
        <p>Subtotal: ${subtotal.toFixed(2)}</p>
        <p>Tax: ${tax.toFixed(2)}</p>
        <p>Discount: -${newInvoice.discount.toFixed(2)}</p>
        <p><strong>Total: ${total.toFixed(2)}</strong></p>
      </div>
    </div>

    <div class="form-section">
      <h3>Notes</h3>
      <textarea bind:value={newInvoice.notes} rows="4" placeholder="Additional notes..."></textarea>
    </div>

    <button type="submit" disabled={loading}>
      {loading ? 'Creating...' : 'Create Invoice'}
    </button>
  </form>
</div>

<style>
  .invoice-creator {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  .form-section {
    margin-bottom: 2rem;
    padding: 1rem;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
  }

  .form-section h3 {
    margin-top: 0;
    color: #333;
  }

  label {
    display: block;
    margin-bottom: 1rem;
  }

  input, select, textarea {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
    margin-top: 0.25rem;
  }

  .item-row {
    display: grid;
    grid-template-columns: 2fr 1fr 1fr 1fr auto;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .item-row input {
    margin-top: 0;
  }

  .summary {
    background: #f5f5f5;
    padding: 1rem;
    border-radius: 4px;
  }

  button {
    background: #007bff;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .error {
    background: #f8d7da;
    color: #721c24;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }
</style>