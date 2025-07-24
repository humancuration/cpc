<script>
  import { onMount, onDestroy } from 'svelte';
  import PostCard from './PostCard.svelte';
  import PostComposer from './PostComposer.svelte';
  import { getPosts } from '$lib/services/post-service';
  import { graphqlClient } from '$lib/graphql/client';
  import { FEED_SUBSCRIPTION } from '$lib/graphql/queries';
  import { currentUser } from '$lib/stores/auth';
  
  export let cooperativeId = null;
  export let showComposer = true;
  export let postsPerPage = 10;
  
  let posts = [];
  let loading = false;
  let hasMore = true;
  let offset = 0;
  let error = null;
  let showPostComposer = false;
  let subscription = null;
  
  // Intersection observer for infinite scroll
  let observer;
  let loadMoreTrigger;
  
  onMount(async () => {
    await loadInitialPosts();
    setupInfiniteScroll();
    setupSubscription();
  });
  
  onDestroy(() => {
    if (subscription) {
      subscription.unsubscribe();
    }
    if (observer) {
      observer.disconnect();
    }
  });
  
  async function loadInitialPosts() {
    loading = true;
    error = null;
    
    try {
      const newPosts = await getPosts({
        cooperativeId,
        limit: postsPerPage,
        offset: 0
      });
      
      posts = newPosts;
      offset = newPosts.length;
      hasMore = newPosts.length === postsPerPage;
    } catch (err) {
      error = err.message || 'Failed to load posts';
    } finally {
      loading = false;
    }
  }
  
  async function loadMorePosts() {
    if (loading || !hasMore) return;
    
    loading = true;
    
    try {
      const newPosts = await getPosts({
        cooperativeId,
        limit: postsPerPage,
        offset
      });
      
      posts = [...posts, ...newPosts];
      offset += newPosts.length;
      hasMore = newPosts.length === postsPerPage;
    } catch (err) {
      error = err.message || 'Failed to load more posts';
    } finally {
      loading = false;
    }
  }
  
  function setupInfiniteScroll() {
    observer = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && !loading && hasMore) {
          loadMorePosts();
        }
      },
      {
        rootMargin: '100px'
      }
    );
    
    if (loadMoreTrigger) {
      observer.observe(loadMoreTrigger);
    }
  }
  
  function setupSubscription() {
    subscription = graphqlClient.subscribe({
      query: FEED_SUBSCRIPTION,
      variables: { cooperativeId }
    }).subscribe({
      next: ({ data }) => {
        if (data?.feedUpdate) {
          // Add new post to the beginning of the feed
          const newPost = data.feedUpdate;
          const existingIndex = posts.findIndex(p => p.id === newPost.id);
          
          if (existingIndex === -1) {
            posts = [newPost, ...posts];
            offset += 1;
          }
        }
      },
      error: (err) => {
        console.error('Feed subscription error:', err);
      }
    });
  }
  
  function handlePostCreated(event) {
    const { post } = event.detail;
    posts = [post, ...posts];
    offset += 1;
    showPostComposer = false;
  }
  
  function handlePostUpdate(event) {
    const { post } = event.detail;
    const index = posts.findIndex(p => p.id === post.id);
    if (index !== -1) {
      posts = [
        ...posts.slice(0, index),
        post,
        ...posts.slice(index + 1)
      ];
    }
  }
  
  function handlePostDelete(event) {
    const { postId } = event.detail;
    posts = posts.filter(p => p.id !== postId);
    offset -= 1;
  }
  
  function handleRetry() {
    loadInitialPosts();
  }
  
  function handleRefresh() {
    posts = [];
    offset = 0;
    hasMore = true;
    loadInitialPosts();
  }
</script>

<div class="feed-view">
  <div class="feed-header">
    <h2>Feed</h2>
    <div class="feed-actions">
      {#if showComposer && $currentUser}
        <button 
          class="compose-btn"
          on:click={() => showPostComposer = true}
        >
          ✍️ New Post
        </button>
      {/if}
      <button 
        class="refresh-btn"
        on:click={handleRefresh}
        disabled={loading}
        title="Refresh feed"
      >
        🔄
      </button>
    </div>
  </div>
  
  {#if showComposer && showPostComposer && $currentUser}
    <div class="composer-overlay">
      <div class="composer-modal">
        <PostComposer
          {cooperativeId}
          on:postCreated={handlePostCreated}
          on:cancel={() => showPostComposer = false}
        />
      </div>
    </div>
  {/if}
  
  <div class="feed-content">
    {#if loading && posts.length === 0}
      <div class="loading-state">
        <div class="spinner"></div>
        <p>Loading posts...</p>
      </div>
    {:else if error && posts.length === 0}
      <div class="error-state">
        <div class="error-icon">⚠️</div>
        <p>{error}</p>
        <button class="retry-btn" on:click={handleRetry}>
          Try Again
        </button>
      </div>
    {:else if posts.length === 0}
      <div class="empty-state">
        <div class="empty-icon">📭</div>
        <h3>No posts yet</h3>
        <p>Be the first to share something!</p>
        {#if showComposer && $currentUser}
          <button 
            class="create-first-post-btn"
            on:click={() => showPostComposer = true}
          >
            Create First Post
          </button>
        {/if}
      </div>
    {:else}
      <div class="posts-container">
        {#each posts as post (post.id)}
          <PostCard 
            {post} 
            on:postUpdate={handlePostUpdate}
            on:postDelete={handlePostDelete}
          />
        {/each}
        
        {#if hasMore}
          <div 
            bind:this={loadMoreTrigger} 
            class="load-more-trigger"
          >
            {#if loading}
              <div class="loading-more">
                <div class="spinner"></div>
                <span>Loading more posts...</span>
              </div>
            {/if}
          </div>
        {:else if posts.length > 0}
          <div class="feed-end">
            <p>You've reached the end of your feed</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .feed-view {
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .feed-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
  }
  
  .feed-header h2 {
    margin: 0;
    color: #333;
  }
  
  .feed-actions {
    display: flex;
    gap: 8px;
  }
  
  .compose-btn,
  .refresh-btn,
  .retry-btn,
  .create-first-post-btn {
    padding: 8px 16px;
    border: none;
    border-radius: 20px;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.3s ease;
  }
  
  .compose-btn {
    background: #007bff;
    color: white;
  }
  
  .compose-btn:hover {
    background: #0056b3;
  }
  
  .refresh-btn,
  .retry-btn {
    background: #f8f9fa;
    color: #666;
  }
  
  .refresh-btn:hover:not(:disabled),
  .retry-btn:hover {
    background: #e9ecef;
    color: #333;
  }
  
  .refresh-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .create-first-post-btn {
    background: #007bff;
    color: white;
    margin-top: 16px;
  }
  
  .create-first-post-btn:hover {
    background: #0056b3;
  }
  
  .composer-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  
  .composer-modal {
    background: white;
    border-radius: 12px;
    max-width: 600px;
    width: 90%;
    max-height: 90vh;
    overflow-y: auto;
  }
  
  .feed-content {
    min-height: 400px;
  }
  
  .loading-state,
  .error-state,
  .empty-state {
    text-align: center;
    padding: 60px 20px;
    color: #666;
  }
  
  .spinner {
    border: 3px solid #f3f3f3;
    border-top: 3px solid #007bff;
    border-radius: 50%;
    width: 40px;
    height: 40px;
    animation: spin 1s linear infinite;
    margin: 0 auto 20px;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  .error-icon,
  .empty-icon {
    font-size: 48px;
    margin-bottom: 16px;
  }
  
  .empty-state h3 {
    margin: 0 0 8px 0;
    color: #333;
  }
  
  .posts-container {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  
  .load-more-trigger {
    padding: 20px;
    text-align: center;
  }
  
  .loading-more {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    color: #666;
  }
  
  .loading-more .spinner {
    width: 24px;
    height: 24px;
    margin: 0;
  }
  
  .feed-end {
    text-align: center;
    padding: 40px 20px;
    color: #666;
    font-style: italic;
  }
</style>