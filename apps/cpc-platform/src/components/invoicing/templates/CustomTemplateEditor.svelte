<script>
  import { onMount } from 'svelte';
  
  export let templateId = null;
  
  let template = {
    name: '',
    header: '',
    footer: '',
    color_scheme: '#007bff',
    font_family: 'Arial',
    show_logo: true,
    show_due_date: true,
    show_payment_terms: true,
    payment_terms: 'Net 30'
  };

  let loading = true;
  let saving = false;
  let error = null;

  onMount(async () => {
    if (templateId) {
      await loadTemplate();
    } else {
      loading = false;
    }
  });

  async function loadTemplate() {
    try {
      const response = await fetch(`/api/invoicing/templates/${templateId}`);
      if (!response.ok) throw new Error('Failed to load template');
      
      template = await response.json();
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  async function saveTemplate() {
    saving = true;
    error = null;

    try {
      const url = templateId ? `/api/invoicing/templates/${templateId}` : '/api/invoicing/templates';
      const method = templateId ? 'PUT' : 'POST';
      
      const response = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(template)
      });
      
      if (!response.ok) throw new Error('Failed to save template');
      
      const saved = await response.json();
      if (!templateId) {
        templateId = saved.id;
      }
      
      // Show success message
    } catch (err) {
      error = err.message;
    } finally {
      saving = false;
    }
  }

  async function deleteTemplate() {
    if (!confirm('Are you sure you want to delete this template?')) return;
    
    try {
      const response = await fetch(`/api/invoicing/templates/${templateId}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) throw new Error('Failed to delete template');
      
      // Navigate back to template gallery
      window.location.href = '/invoicing/templates';
    } catch (err) {
      error = err.message;
    }
  }

  function addPlaceholder(placeholder) {
    template.header += ` {{${placeholder}}}`;
  }
</script>

<div class="template-editor">
  <h2>{templateId ? 'Edit Template' : 'Create Template'}</h2>
  
  {#if loading}
    <div class="loading">Loading template...</div>
  {:else}
    <form on:submit|preventDefault={saveTemplate}>
      <div class="form-section">
        <h3>Template Details</h3>
        
        <label>
          Template Name
          <input type="text" bind:value={template.name} required />
        </label>
      </div>

      <div class="form-section">
        <h3>Design</h3>
        
        <label>
          Color Scheme
          <input type="color" bind:value={template.color_scheme} />
        </label>

        <label>
          Font Family
          <select bind:value={template.font_family}>
            <option value="Arial">Arial</option>
            <option value="Helvetica">Helvetica</option>
            <option value="Times New Roman">Times New Roman</option>
            <option value="Georgia">Georgia</option>
            <option value="Courier New">Courier New</option>
          </select>
        </label>

        <div class="checkbox-group">
          <label>
            <input type="checkbox" bind:checked={template.show_logo} />
            Show Company Logo
          </label>

          <label>
            <input type="checkbox" bind:checked={template.show_due_date} />
            Show Due Date
          </label>

          <label>
            <input type="checkbox" bind:checked={template.show_payment_terms} />
            Show Payment Terms
          </label>
        </div>
      </div>

      <div class="form-section">
        <h3>Header</h3>
        <textarea bind:value={template.header} rows="6" placeholder="Enter template header..."></textarea>
        
        <div class="placeholders">
          <small>Available placeholders:</small>
          <button type="button" on:click={() => addPlaceholder('company_name')}>Company Name</button>
          <button type="button" on:click={() => addPlaceholder('invoice_number')}>Invoice #</button>
          <button type="button" on:click={() => addPlaceholder('date')}>Date</button>
          <button type="button" on:click={() => addPlaceholder('recipient_name')}>Recipient Name</button>
        </div>
      </div>

      <div class="form-section">
        <h3>Footer</h3>
        <textarea bind:value={template.footer} rows="4" placeholder="Enter template footer..."></textarea>
      </div>

      <div class="form-section">
        <h3>Payment Terms</h3>
        <input type="text" bind:value={template.payment_terms} placeholder="e.g., Net 30, Due on receipt" />
      </div>

      <div class="preview">
        <h3>Preview</h3>
        <div class="preview-content" style="color: {template.color_scheme}; font-family: {template.font_family}">
          <div class="preview-header">{template.header || '[Header will appear here]'}</div>
          <div class="preview-footer">{template.footer || '[Footer will appear here]'}</div>
        </div>
      </div>

      <div class="actions">
        <button type="submit" disabled={saving}>
          {saving ? 'Saving...' : 'Save Template'}
        </button>
        
        {#if templateId}
          <button type="button" class="danger" on:click={deleteTemplate}>
            Delete Template
          </button>
        {/if}
      </div>

      {#if error}
        <div class="error">{error}</div>
      {/if}
    </form>
  {/if}
</div>

<style>
  .template-editor {
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

  .checkbox-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .checkbox-group label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .checkbox-group input[type="checkbox"] {
    width: auto;
  }

  .placeholders {
    margin-top: 1rem;
  }

  .placeholders button {
    background: #e9ecef;
    color: #495057;
    border: 1px solid #ced4da;
    padding: 0.25rem 0.5rem;
    margin-right: 0.5rem;
    margin-bottom: 0.5rem;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .preview {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 2rem;
  }

  .preview-content {
    padding: 1rem;
    border: 1px dashed #ccc;
    border-radius: 4px;
    min-height: 200px;
  }

  .preview-header, .preview-footer {
    margin-bottom: 1rem;
    white-space: pre-wrap;
  }

  .actions {
    display: flex;
    gap: 1rem;
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

  button.danger {
    background: #dc3545;
  }

  button.danger:hover {
    background: #c82333;
  }

  .loading {
    text-align: center;
    padding: 2rem;
    color: #6c757d;
  }

  .error {
    background: #f8d7da;
    color: #721c24;
    padding: 1rem;
    border-radius: 4px;
    margin-bottom: 1rem;
  }
</style>