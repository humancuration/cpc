<script>
  import { onMount } from 'svelte';
  import { postStore } from '../../stores/social.js';
  
  export let postId = null;
  
  let newComment = '';
  let submittingComment = false;
  
  $: if (postId) {
    postStore.loadPost(postId);
  }
  
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
    if ($postStore.post) {
      await postStore.toggleLike($postStore.post.id);
    }
  }
  
  async function handleCommentSubmit(e) {
    e.preventDefault();
    
    if (!newComment.trim() || submittingComment) return;
    
    submittingComment = true;
    try {
      await postStore.addComment(postId, newComment.trim());
      newComment = '';
    } finally {
      submittingComment = false;
    }
  }
  
  function handleShare() {
    if (navigator.share) {
      navigator.share({
        title: 'Check out this post',
        text: $postStore.post?.content || 'View this post on CPC',
        url: window.location.href
      });
    } else {
      // Fallback: copy to clipboard
      navigator.clipboard.writeText(window.location.href);
      alert('Link copied to clipboard!');
    }
  }
</script>

<div class="post-detail">
  {#if $postStore.loading}
    <div class="loading">
      <p>Loading post...</p>
    </div>
  {:else if $postStore.error}
    <div class="error">
      <p>Failed to load post: {$postStore.error}</p>
      <button on:click={() => postStore.loadPost(postId)}>Retry</button>
    </div>
  {:else if $postStore.post}
    <article class="post">
      <header class="post-header">
        <div class="author-info">
          {#if $postStore.post.author?.avatarUrl}
            <img 
              src={$postStore.post.author.avatarUrl} 
              alt={$postStore.post.author.displayName || $postStore.post.author.username}
              class="avatar"
              on:error={(e) => e.target.style.display = 'none'}
            />
          {:else}
            <div class="avatar-placeholder">
              {($postStore.post.author?.displayName || $postStore.post.author?.username || '?').charAt(0).toUpperCase()}
            </div>
          {/if}
          <div class="author-details">
            <h3>{$postStore.post.author?.displayName || $postStore.post.author?.username || 'Unknown User'}</h3>
            <span class="username">@{$postStore.post.author?.username || 'unknown'}</span>
            <span class="timestamp">{formatDate($postStore.post.createdAt)}</span>
          </div>
        </div>
        <span class="visibility-badge">{formatVisibility($postStore.post.visibility)}</span>
      </header>
      
      <div class="post-content">
        <p>{$postStore.post.content}</p>
        
        {#if $postStore.post.media && $postStore.post.media.length > 0}
          <div class="media-container">
            {#each $postStore.post.media as media}
              {#if media.type === 'IMAGE'}
                <img src={media.url} alt="Post media" class="post-image" />
              {:else if media.type === 'VIDEO'}
                <video controls class="post-video">
                  <source src={media.url} type="video/mp4" />
                  Your browser does not support the video tag.
                </video>
              {/if}
            {/each}
          </div>
        {/if}
      </div>
      
      <div class="post-stats">
        <span class="stat">{$postStore.post.likeCount || 0} likes</span>
        <span class="stat">{$postStore.post.commentCount || 0} comments</span>
      </div>
      
      <footer class="post-actions">
        <button 
          class="action-btn" 
          class:liked={$postStore.post.isLikedByCurrentUser}
          on:click={handleLike}
          title={$postStore.post.isLikedByCurrentUser ? 'Unlike' : 'Like'}
        >
          <svg width="20" height="20" viewBox="0 0 24 24" fill={$postStore.post.isLikedByCurrentUser ? "currentColor" : "none"} stroke="currentColor" stroke-width="2">
            <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
          </svg>
          <span>Like</span>
        </button>
        
        <button class="action-btn" title="Comment">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z"/>
          </svg>
          <span>Comment</span>
        </button>
        
        <button class="action-btn" on:click={handleShare} title="Share">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
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
    
    <section class="comments-section">
      <h3>Comments</h3>
      
      <form class="comment-form" on:submit={handleCommentSubmit}>
        <textarea 
          bind:value={newComment}
          placeholder="Write a comment..."
          disabled={submittingComment}
          rows="3"
        ></textarea>
        <button type="submit" disabled={!newComment.trim() || submittingComment}>
          {submittingComment ? 'Posting...' : 'Post Comment'}
        </button>
      </form>
      
      <div class="comments-list">
        {#if $postStore.comments.length === 0}
          <p class="no-comments">No comments yet. Be the first to comment!</p>
        {:else}
          {#each $postStore.comments as comment (comment.id)}
            <div class="comment">
              <div class="comment-header">
                <div class="comment-author">
                  {#if comment.author?.avatarUrl}
                    <img 
                      src={comment.author.avatarUrl} 
                      alt={comment.author.displayName || comment.author.username}
                      class="comment-avatar"
                    />
                  {:else}
                    <div class="comment-avatar-placeholder">
                      {(comment.author?.displayName || comment.author?.username || '?').charAt(0).toUpperCase()}
                    </div>
                  {/if}
                  <div class="comment-author-info">
                    <strong>{comment.author?.displayName || comment.author?.username || 'Unknown'}</strong>
                    <span class="comment-timestamp">{formatDate(comment.createdAt)}</span>
                  </div>
                </div>
              </div>
              <p class="comment-content">{comment.content}</p>
            </div>
          {/each}
        {/if}
      </div>
    </section>
  {/if}
</div>

<style>
  .post-detail {
    max-width: 600px;
    margin: 0 auto;
    padding: 16px;
  }

  .loading, .error {
    text-align: center;
    padding: 32px;
    color: #666;
  }

  .error button {
    background: #007bff;
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    margin-top: 8px;
    cursor: pointer;
  }

  .post {
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.12);
    margin-bottom: 16px;
  }

  .post-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    padding: 16px;
    border-bottom: 1px solid #eee;
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
    background: #007bff;
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

  .username, .timestamp {
    color: #666;
    font-size: 14px;
  }

  .visibility-badge {
    background: #f0f0
