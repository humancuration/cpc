<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  
  let templates = [];
  let loading = true;
  let error = null;

  onMount(async () => {
    await loadTemplates();
  });

  async function loadTemplates() {
    try {
      const response = await fetch('/api/invoicing/templates');
      if (!response.ok) throw new Error('Failed to load templates');
      
      templates = await response.json();
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }

  function createNewTemplate() {
    goto('/invoicing/templates/new');
  }

  function editTemplate(id) {
    goto(`/invoicing/templates/${id}/edit`);
  }

  async function duplicateTemplate(template) {
    try {
      const response = await fetch('/api/invoicing/templates', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ...template,
          name: `${template.name} (Copy)`,
          id: undefined
        })
      });
      
      if (!response.ok) throw new Error('Failed to duplicate template');
      
      await loadTemplates();
    } catch (err) {
      error = err.message;
    }
  }

  async function deleteTemplate(id) {
    if (!confirm('Are you sure you want to delete this template?')) return;
    
    try {
      const response = await fetch(`/api/invoicing/templates/${id}`, {
        method: 'DELETE'
      });
      
      if (!response.ok) throw new Error('Failed to delete template');
      
      templates = templates.filter(t => t.id !== id);
    } catch (err) {
      error = err.message;
    }
  }

  function previewTemplate(template) {
    // Open preview modal or new tab
    window.open(`/invoicing/templates/${template.id}/preview`, '_blank');
  }
</script>

<div class="template-gallery">
  <div class="header">
    <h2>Invoice Templates</h2>
    <button on:click={createNewTemplate}>Create New Template</button>
  </div>

  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if loading}
    <div class="loading">Loading templates...</div>
  {:else if templates.length === 0}
    <div class="empty">
      <h3>No templates found</h3>
      <p>Create your first invoice template to get started.</p>
      <button on:click={createNewTemplate}>Create Template</button>
    </div>
  {:else}
    <div class="templates-grid">
      {#each templates as template}
        <div class="template-card">
          <div class="template-preview">
            <div 
              class="preview-box" 
              style="background-color: {template.color_scheme}20; color: {template.color_scheme}"
            >
              <div class="template-name">{template.name}</div>
              <div class="template-font" style="font-family: {template.font_family}">
                Sample Invoice
              </div>
            </div>
          </div>
          
          <div class="template-info">
            <h4>{template.name}</h4>
            <p>Created {new Date(template.created_at).toLocaleDateString()}</p>
            <p class="usage">
              Used in {template.usage_count || 0} invoices
            </p>
          </div>
          
          <div class="template-actions">
            <button on:click={() => previewTemplate(template)}>Preview</button>
            <button on:click={() => editTemplate(template.id)}>Edit</button>
            <button on:click={() => duplicateTemplate(template)}>Duplicate</button>
            <button 
              class="danger" 
              on:click={() => deleteTemplate(template.id)}
              disabled={template.is_default}
            >
              Delete
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .template-gallery {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .header h2 {
    margin: 0;
  }

  .header button {
    background: #007bff;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .templates-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 2rem;
  }

  .template-card {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    overflow: hidden;
    transition: box-shadow 0.3s;
  }

  .template-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .template-preview {
    height: 150px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f8f9fa;
  }

  .preview-box {
    width: 80%;
    height: 80%;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    text-align: center;
  }

  .template-name {
    font-weight: bold;
    margin-bottom: 0.5rem;
  }

  .template-font {
    font-size: 0.875rem;
  }

  .template-info {
    padding: 1rem;
  }

  .template-info h4 {
    margin: 0 0 0.5rem 0;
  }

  .template-info p {
    margin: 0.25rem 0;
    color: #666;
    font-size: 0.875rem;
  }

  .usage {
    font-weight: bold;
    color: #007bff !important;
  }

  .template-actions {
    padding: 1rem;
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .template-actions button {
    background: none;
    border: 1px solid #007bff;
    color: #007bff;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.875rem;
  }

  .template-actions button:hover {
    background: #007bff;
    color: white;
  }

  .template-actions button.danger {
    border-color: #dc3545;
    color: #dc3545;
  }

  .template-actions button.danger:hover {
    background: #dc3545;
    color: white;
  }

  .template-actions button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .empty {
    text-align: center;
    padding: 3rem;
  }

  .empty h3 {
    margin-bottom: 1rem;
  }

  .empty p {
    margin-bottom: 2rem;
    color: #666;
  }

  .empty button {
    background: #007bff;
    color: white;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .loading, .error {
    text-align: center;
    padding: 2rem;
  }

  .error {
    background: #f8d7da;
    color: #721c24;
    border-radius: 4px;
  }
</style>