<script>
  import { onMount } from 'svelte';
  import { timelineStore } from '$stores/social/TimelineViewModel.js';
  import PostList from './PostList.svelte';
  
  export let filters = {};
  export let enableInfiniteScroll = true;
  
  onMount(() => {
    // Set filters if provided
    if (Object.keys(filters).length > 0) {
      timelineStore.setFilters(filters);
    } else {
      // Load initial timeline
      timelineStore.loadTimeline(true);
    }
    
    return () => {
      // Cleanup when component unmounts
      timelineStore.reset();
    };
  });
  
  function handleLoadMore() {
    timelineStore.loadMorePosts();
  }
  
  function handleRefresh() {
    timelineStore.refreshTimeline();
  }
</script>

<div class="timeline-container">
  <PostList
    posts={$timelineStore.posts}
    loading={$timelineStore.loading}
    error={$timelineStore.error}
    hasMore={$timelineStore.hasMore}
    onLoadMore={handleLoadMore}
    onRefresh={handleRefresh}
    {enableInfiniteScroll}
  />
</div>

<style>
  .timeline-container {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px 0;
  }
</style>