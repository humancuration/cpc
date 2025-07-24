<script>
  import { setContext } from 'svelte';
  
  // Provide responsive grid context
  setContext('widget-grid', {
    columns: 12,
    gap: '1rem'
  });
</script>

<div class="widget-grid">
  <slot />
</div>

<style>
  .widget-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1.5rem;
    width: 100%;
  }

  @media (max-width: 768px) {
    .widget-grid {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
  }
</style>