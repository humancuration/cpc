<script>
  import Navigation from '$lib/components/Navigation.svelte';
</script>

<Navigation />

<main>
  <slot />
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
    background: #f5f5f5;
  }
  
  :global(html) {
    box-sizing: border-box;
  }
  
  :global(*), :global(*:before), :global(*:after) {
    box-sizing: inherit;
  }
  
  main {
    min-height: calc(100vh - 60px);
  }
</style>