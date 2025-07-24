<script>
  import { createEventDispatcher } from 'svelte';
  import { timelineStore } from '$stores/social/TimelineViewModel.js';
  
  export let post;
  
  const dispatch = createEventDispatcher();
  
  let isLiked = false;
  let likeCount = 0;
  let commentCount = 0;
  let shareCount = 0;
  let isProcessing = false;
  
  $: isLiked = post.likedByCurrentUser || false;
  $: likeCount = post.likeCount || 0;
  $: commentCount = post.commentCount || 0;
  $: shareCount = post.shareCount || 0;
  
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
  
  async function handleLike() {
    if (isProcessing) return;
    
    isProcessing = true;
    try {
      await timelineStore.toggleLike(post.id);
      
      // Optimistic update
      if (isLiked) {
        likeCount--;
        isLiked = false;
      } else {
        likeCount++;
        isLiked = true;
      }
      
    } catch (error) {
      console.error('Error toggling like:', error);
      // Revert optimistic update on error
      if (isLiked) {
        likeCount--;
      } else {
        likeCount++;
      }
    } finally {
      isProcessing = false;
    }
  }
  
  async function handleComment() {
    dispatch('comment', { postId: post.id });
  }
  
  async function handleShare() {
    try {
      if (navigator.share) {
        await navigator.share({
          title: 'Check out this post',
          text: post.content,
          url: `${window.location.origin}/social/${post.id}`
        });
      } else {
        // Fallback to clipboard
        await navigator.clipboard.writeText(`${window.location.origin}/social/${post.id}`);
        dispatch('showToast', { message: 'Link copied to clipboard!' });
      }
    } catch (error) {
      console.error('Error sharing:', error);
    }
  }
  
  function handlePostClick() {
    dispatch('postClick', { post });
  }
</script>

<article class="post-card">
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
  
  <div class="post-content" on:click={handlePostClick}>
    <p>{post.content}</p>
    
    {#if post.mediaUrls && post.mediaUrls.length > 0}
      <div class="media-preview">
        {#each post.mediaUrls as url}
          <img src={url} alt="Post media" class="media-item" />
        {/each}
      </div>
    {/if}
  </div>
  
  <div class="post-stats">
    {#if likeCount > 0}
      <span class="stat">{likeCount} {likeCount === 1 ? 'like' : 'likes'}</span>
    {/if}
    {#if commentCount > 0}
      <span class="stat">{commentCount} {commentCount === 1 ? 'comment' : 'comments'}</span>
    {/if}
    {#if shareCount > 0}
      <span class="stat">{shareCount} {shareCount === 1 ? 'share' : 'shares'}</span>
    {/if}
  </div>
  
  <footer class="post-actions">
    <button 
      class="action-btn like-btn" 
      class:liked={isLiked}
      title="Like"
      on:click={handleLike}
      disabled={isProcessing}
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill={isLiked ? "currentColor" : "none"} stroke="currentColor" stroke-width="2">
        <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
      </svg>
      <span>Like</span>
    </button>
    
    <button 
      class="action-btn" 
      title="Comment"
      on:click={handleComment}
    >
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
      </svg>
      <span>Comment</span>
    </button>
    
    <button 
      class="action-btn" 
      title="Share"
      on:click={handleShare}
    >
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

<style>
  .post-card {
    background: white;
    border-radius: 8px;
    padding: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    transition: transform 0.2s, box-shadow 0.2s;
  }
  
  .post-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0,0,0,0.15);
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
    margin-bottom: 12px;
    cursor: pointer;
  }
  
  .post-content p {
    margin: 0 0 12px 0;
    line-height: 1.5;
    white-space: pre-wrap;
  }
  
  .media-preview {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: 8px;
    margin-top: 12px;
  }
  
  .media-item {
    width: 100%;
    height: 150px;
    object-fit: cover;
    border-radius: 8px;
  }
  
  .post-stats {
    display: flex;
    gap: 16px;
    margin-bottom: 12px;
    font-size: 14px;
    color: #666;
  }
  
  .stat {
    font-weight: 500;
  }
  .post-actions {
    display: flex;
    gap: 16px;
    padding-top: 12px;
    border-top: 1px solid #eee;
  }
  
  .action-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 12px;
    border: none;
    background: transparent;
    color: #666;
    font-size: 14px;
    cursor: pointer;
    border-radius: 4px;
    transition: all 0.2s;
  }
  
  .action-btn:hover {
    background: #f5f5f5;
    color: #333;
  }
  
  .action-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .like-btn.liked {
    color: #e91e63;
  }
  
  .like-btn:hover {
    background: #fce4ec;
  }
  
  @media (max-width: 600px) {
    .post-card {
      padding: 16px;
    }
    
    .author-info {
      gap: 8px;
    }
    
    .avatar, .avatar-placeholder {
      width: 32px;
      height: 32px;
      font-size: 14px;
    }
    
    .author-details h3 {
      font-size: 14px;
    }
    
    .username {
      font-size: 12px;
    }
    
    .media-preview {
      grid-template-columns: 1fr;
    }
    
    .media-item {
      height: 200px;
    }
    
    .post-actions {
      justify-content: space-around;
    }
    
    .action-btn {
      flex-direction: column;
      gap: 4px;
      font-size: 12px;
    }
  }
</style>
   