<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let yewHtml = '';
  let error = '';

  onMount(async () => {
    try {
      yewHtml = await invoke('render_yew_component', { route: '/invoices' });
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
    <p>Loading invoices...</p>
  {/if}
</div>