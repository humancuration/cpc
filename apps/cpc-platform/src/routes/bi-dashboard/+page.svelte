<script>
  import { onMount } from 'svelte';
  import { authStore } from '$lib/stores/auth.js';
  import BIDashboard from '$lib/bi/BIDashboard.svelte';
  
  let userId = null;
  
  onMount(() => {
    // For now, use a mock user ID. In a real app, this would come from auth
    userId = 'user-123'; // This should be replaced with actual user ID from authStore
  });
</script>

<svelte:head>
  <title>Impact Dashboard - CPC Platform</title>
</svelte:head>

<div class="page-container">
  <main>
    {#if userId}
      <BIDashboard {userId} />
    {:else}
      <div class="auth-required">
        <h2>Authentication Required</h2>
        <p>Please log in to view your impact dashboard.</p>
      </div>
    {/if}
  </main>
</div>

<style>
  .page-container {
    min-height: 100vh;
    background-color: #f5f5f5;
  }

  main {
    padding: 2rem;
  }

  .auth-required {
    text-align: center;
    padding: 4rem 2rem;
  }

  .auth-required h2 {
    color: #666;
    margin-bottom: 1rem;
  }

  .auth-required p {
    color: #999;
  }

  @media (max-width: 768px) {
    main {
      padding: 1rem;
    }
  }
</style>