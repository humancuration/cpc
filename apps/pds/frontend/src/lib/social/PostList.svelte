<script>
  import { onMount } from 'svelte';
  
  export let posts = [];
  export let loading = false;
  export let error = null;
  export let hasMore = true;
  export let onLoadMore = null;
  export let onRefresh = null;
  export let enableInfiniteScroll = true;
  
  let observer;
  let loadingTrigger;
  
  function formatDate(dateString) {
    const date = new Date(dateString);
    const now = new Date();
    const diffMs = now - date;
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);
    
    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  }
  
  function formatVisibility(visibility) {
    const labels = {
      'PUBLIC': 'Public',
      'COOPERATIVE': 'Cooperative',
      'PRIVATE': 'Private'
    };
    return labels[visibility] || visibility;
  }
  
  // Infinite scroll observer
  onMount(() => {
    if (!enableInfiniteScroll || !onLoadMore) return;
    
    observer = new IntersectionObserver(
      (entries) => {
        const [entry] = entries;
        if (entry.isIntersecting && !loading && hasMore && onLoadMore) {
          onLoadMore();
        }
      },
      { threshold: 0.1 }
    );
    
    if (loadingTrigger) {
      observer.observe(loadingTrigger);
    }
    
    return () => {
      if (observer && loadingTrigger) {
        observer.unobserve(loadingTrigger);
      }
    };
  });
  
  function handleRefresh() {
    if (onRefresh) {
      onRefresh();
    }
  }
</script>

<div class="post-list">
  {#if loading && posts.length === 0}
    <div class="loading">
      <p>Loading posts...</p>
    </div>
  {:else if posts.length === 0}
    <div class="empty">
      <p>No posts yet. Be the first to share something!</p>
    </div>
  {:else}
    {#each posts as post (post.id)}
      <article class="post">
        <a href="/social/{post.id}" class="post-link">
          <header class="post-header">
            <div class="author-info">
              {#if post.author?.avatarUrl}
                <img
                  src={post.author.avatarUrl}
                  alt={post.author.displayName || post.author.username}
                  class="avatar"
                  on:error={(e) => e.target.style.display = 'none'}
                />
              {:else}
                <div class="avatar-placeholder">
                  {(post.author?.displayName || post.author?.username || '?').charAt(0).toUpperCase()}
                </div>
              {/if}
              <div class="author-details">
                <h3>{post.author?.displayName || post.author?.username || 'Unknown User'}</h3>
                <span class="username">@{post.author?.username || 'unknown'}</span>
                <span class="timestamp">{formatDate(post.createdAt)}</span>
              </div>
            </div>
            <span class="visibility-badge">{formatVisibility(post.visibility)}</span>
          </header>
          
          <div class="post-content">
            <p>{post.content}</p>
          </div>
        </a>
        
        <footer class="post-actions">
          <button class="action-btn" title="Like">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
            </svg>
            <span>Like</span>
          </button>
          
          <button class="action-btn" title="Comment">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
            </svg>
            <span>Comment</span>
          </button>
          
          <button class="action-btn" title="Share">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="18" cy="5" r="3"/>
              <circle cx="6" cy="12" r="3"/>
              <circle cx="18" cy="19" r="3"/>
              <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
              <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
            </svg>
            <span>Share</span>
          </button>
        </footer>
      </article>
    {/each}
    
    {#if onLoadMore && !enableInfiniteScroll}
      <button class="load-more" on:click={onLoadMore} disabled={loading}>
        {loading ? 'Loading...' : 'Load More'}
      </button>
    {/if}
    
    <!-- End of list indicator -->
    {#if !hasMore && posts.length > 0}
      <div class="end-of-list">
        <p>You've reached the end!</p>
      </div>
    {/if}
  {/if}
</div>

<style>
  .post-list {
    max-width: 600px;
    margin: 0 auto;
  }
  
  .post {
    background: white;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 16px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .post-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 12px;
  }
  
  .author-info {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .avatar, .avatar-placeholder {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    object-fit: cover;
  }
  
  .avatar-placeholder {
    background: var(--accent);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 18px;
  }
  
  .author-details h3 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }
  
  .username {
    color: #666;
    font-size: 14px;
  }
  
  .timestamp {
    color: #999;
    font-size: 12px;
    margin-left: 8px;
  }
  
  .visibility-badge {
    background: #f0f0f0;
    color: #666;
    padding: 4px 8px;
    border-radius: 12px;
    font-size: 12px;
  }
  
  .post-content {
    margin-bottom: 16px;
  }
  
  .post-content p {
    margin: 0;
    line-height: 1.5;
    white-space: pre-wrap;
  }

  .post-link {
    text-decoration: none;
    color: inherit;
    display: block;
    margin: -20px;
    padding: 20px;
    border-radius: 8px;
    transition: background-color 0.2s;
  }

  .post-link:hover {
    background: #f9f9f9;
  }
  
  .post-actions {
    display: flex;
    gap: 16px;
    border-top: 1px solid #eee;
    padding-top: 12px;
  }
  
  .action-btn {
    display: flex;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    color: #666;
    font-size: 14px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    transition: background-color 0.2s;
  }
  
  .action-btn:hover {
    background: #f5f5f5;
  }
  
  .loading, .empty {
    text-align: center;
    padding: 40px;
    color: #666;
  }
  
  .load-more {
    display: block;
    margin: 20px auto;
    padding: 12px 24px;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .load-more:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .refresh-container {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 16px;
  }
  
  .refresh-btn, .retry-btn, .create-post-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
    transition: background-color 0.2s;
  }
  
  .refresh-btn:hover, .retry-btn:hover, .create-post-btn:hover {
    background: var(--accent-dark);
  }
  
  .refresh-btn:disabled, .retry-btn:disabled, .create-post-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .error {
    text-align: center;
    padding: 40px;
    color: #e74c3c;
  }
  
  .error .retry-btn {
    margin-top: 16px;
  }
  
  .empty {
    text-align: center;
    padding: 60px 40px;
    color: #666;
  }
  
  .empty svg {
    margin: 0 auto 16px;
    opacity: 0.5;
  }
  
  .create-post-btn {
    margin: 16px auto 0;
  }
  
  .loading, .loading-more {
    text-align: center;
    padding: 40px;
    color: #666;
  }
  
  .loading-more {
    padding: 20px;
  }
  
  .spinner {
    width: 24px;
    height: 24px;
    border: 3px solid #f3f3f3;
    border-top: 3px solid var(--accent);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin: 0 auto 16px;
  }
  
  @keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  
  .infinite-scroll-trigger {
    height: 20px;
  }
  
  .end-of-list {
    text-align: center;
    padding: 40px;
    color: #999;
    font-style: italic;
  }
</style>