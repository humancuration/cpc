<script>
  import Timeline from '$lib/social/Timeline.svelte';
  import TimelineFilters from '$lib/social/TimelineFilters.svelte';
</script>

<svelte:head>
  <title>Social Timeline - CPC Platform</title>
</svelte:head>

<div class="social-page">
  <header class="page-header">
    <h1>Social Timeline</h1>
    <p>Explore posts from the CPC community</p>
  </header>
  
  <div class="timeline-content">
    <TimelineFilters />
    <Timeline />
  </div>
</div>

<style>
  .social-page {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .page-header {
    text-align: center;
    margin-bottom: 40px;
  }
  
  .page-header h1 {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
    color: var(--text-primary);
  }
  
  .page-header p {
    font-size: 1.1rem;
    color: var(--text-secondary);
  }
  
  .timeline-content {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
</style>