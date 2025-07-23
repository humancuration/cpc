<script>
  import { onMount } from 'svelte';
  import { timelineStore } from '$stores/social/TimelineViewModel.js';
  
  let testResults = [];
  
  onMount(async () => {
    // Test 1: Initial load
    testResults.push('Testing initial timeline load...');
    await timelineStore.loadTimeline(true);
    testResults.push(`✓ Loaded ${$timelineStore.posts.length} posts`);
    
    // Test 2: Refresh
    testResults.push('Testing refresh...');
    await timelineStore.refreshTimeline();
    testResults.push('✓ Refresh completed');
    
    // Test 3: Filters
    testResults.push('Testing filters...');
    await timelineStore.setFilters({ type: 'posts' });
    testResults.push(`✓ Filter applied, ${$timelineStore.posts.length} posts`);
    
    // Test 4: Reset
    testResults.push('Testing reset...');
    timelineStore.reset();
    testResults.push(`✓ Reset completed, ${$timelineStore.posts.length} posts`);
  });
</script>

<div class="test-container">
  <h3>Timeline Integration Test</h3>
  <ul>
    {#each testResults as result}
      <li>{result}</li>
    {/each}
  </ul>
  
  <div class="state-info">
    <h4>Current State:</h4>
    <p>Posts: {$timelineStore.posts.length}</p>
    <p>Loading: {$timelineStore.loading}</p>
    <p>Error: {$timelineStore.error || 'None'}</p>
    <p>Has More: {$timelineStore.hasMore}</p>
  </div>
</div>

<style>
  .test-container {
    padding: 20px;
    background: #f5f5f5;
    border-radius: 8px;
    margin: 20px;
  }
  
  ul {
    list-style: none;
    padding: 0;
  }
  
  li {
    padding: 5px 0;
    font-family: monospace;
  }
  
  .state-info {
    margin-top: 20px;
    padding: 10px;
    background: white;
    border-radius: 4px;
  }
</style>