<script>
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { invoke } from '@tauri-apps/api/tauri';

  let yewHtml = '';
  let error = '';

  onMount(async () => {
    try {
      const route = `/invoices/${$page.params.id}`;
      yewHtml = await invoke('render_yew_component', { route });
    } catch (e) {
      error = `Failed to render Yew component: ${e}`;
      console.error(error);
    }
  });
</script>

<div class="prose max-w-none">
  {#if error}
    <div class="alert alert-error">
      <p>{error}</p>
    </div>
  {:else if yewHtml}
    {@html yewHtml}
  {:else}
    <p>Loading invoice details...</p>
  {/if}
</div>