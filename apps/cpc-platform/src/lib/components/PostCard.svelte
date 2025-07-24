<script>
  import { createEventDispatcher } from 'svelte';
  import { formatDistanceToNow } from 'date-fns';
  import { deletePost, likePost, unlikePost } from '$lib/services/post-service';
  import { currentUser } from '$lib/stores/auth';
  
  export let post;
  
  const dispatch = createEventDispatcher();
  
  let isDeleting = false;
  let isLiking = false;
  let error = null;
  
  $: isOwner = $currentUser?.id === post.author.id;
  $: isLiked = $currentUser && post.likes?.includes($currentUser.id);
  $: likeCount = post.likes?.length || 0;
  $: formattedDate = post.createdAt ? formatDistanceToNow(new Date(post.createdAt), { addSuffix: true }) : '';
  
  async function handleLike() {
    if (!$currentUser) return;
    
    isLiking = true;
    error = null;
    
    try {
      const updatedPost = isLiked 
        ? await unlikePost(post.id)
        : await likePost(post.id);
      
      dispatch('postUpdate', { post: updatedPost });
    } catch (err) {
      error = err.message || 'Failed to update like';
    } finally {
      isLiking = false;
    }
  }
  
  async function handleDelete() {
    if (!confirm('Are you sure you want to delete this post?')) return;
    
    isDeleting = true;
    error = null;
    
    try {
      await deletePost(post.id);
      dispatch('postDelete', { postId: post.id });
    } catch (err) {
      error = err.message || 'Failed to delete post';
      isDeleting = false;
    }
  }
  
  function getVisibilityIcon(visibility) {
    switch (visibility) {
      case 'PUBLIC':
        return '👥';
      case 'COOPERATIVE':
        return '🏢';
      case 'PRIVATE':
        return '🔒';
      default:
        return '📢';
    }
  }
</script>

<div class="post-card">
  <div class="post-header">
    <div class="author-info">
      {#if post.author.avatarUrl}
        <img 
          src={post.author.avatarUrl} 
          alt={post.author.displayName || post.author.username}
          class="avatar"
        />
      {:else}
        <div class="avatar-placeholder">
          {post.author.displayName?.[0] || post.author.username?.[0] || '?'}
        </div>
      {/if}
      <div class="author-details">
        <span class="author-name">
          {post.author.displayName || post.author.username}
        </span>
        <span class="post-meta">
          {formattedDate} · {getVisibilityIcon(post.visibility)}
        </span>
      </div>
    </div>
    
    {#if isOwner}
      <div class="post-actions">
        <button 
          class="delete-btn" 
          on:click={handleDelete}
          disabled={isDeleting}
          title="Delete post"
        >
          🗑️
        </button>
      </div>
    {/if}
  </div>
  
  {#if post.content}
    <div class="post-content">
      {post.content}
    </div>
  {/if}
  
  {#if post.media && post.media.length > 0}
    <div class="post-media">
      {#each post.media as media, index}
        {#if media.type === 'IMAGE'}
          <img 
            src={media.url} 
            alt={media.description || 'Post image'}
            class="post-image"
          />
        {:else if media.type === 'VIDEO'}
          <video 
            controls
            class="post-video"
            src={media.url}
            poster={media.thumbnailUrl}
          >
            Your browser does not support the video tag.
          </video>
        {:else if media.type === 'AUDIO'}
          <audio controls class="post-audio" src={media.url}>
            Your browser does not support the audio tag.
          </audio>
        {/if}
      {/each}
    </div>
  {/if}
  
  <div class="post-footer">
    <div class="post-stats">
      <span class="likes-count">
        {likeCount} {likeCount === 1 ? 'like' : 'likes'}
      </span>
    </div>
    
    <div class="post-actions">
      <button 
        class="like-btn"
        class:liked={isLiked}
        on:click={handleLike}
        disabled={isLiking || !$currentUser}
        title={isLiked ? 'Unlike' : 'Like'}
      >
        {isLiked ? '❤️' : '🤍'} {isLiked ? 'Liked' : 'Like'}
      </button>
    </div>
  </div>
  
  {#if error}
    <div class="error-message">{error}</div>
  {/if}
</div>

<style>
  .post-card {
    background: white;
    border-radius: 8px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    padding: 16px;
    margin-bottom: 15px;
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
    background: #007bff;
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 18px;
  }
  
  .author-details {
    display: flex;
    flex-direction: column;
  }
  
  .author-name {
    font-weight: bold;
    color: #333;
  }
  
  .post-meta {
    font-size: 12px;
    color: #666;
  }
  
  .post-actions {
    display: flex;
    gap: 8px;
  }
  
  .delete-btn, .like-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    font-size: 14px;
  }
  
  .delete-btn:hover {
    background: #f8f9fa;
  }
  
  .like-btn {
    color: #666;
  }
  
  .like-btn.liked {
    color: #dc3545;
  }
  
  .like-btn:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }
  
  .post-content {
    margin-bottom: 12px;
    line-height: 1.5;
    color: #333;
    white-space: pre-wrap;
  }
  
  .post-media {
    margin-bottom: 12px;
  }
  
  .post-image, .post-video {
    width: 100%;
    max-height: 400px;
    object-fit: cover;
    border-radius: 8px;
  }
  
  .post-audio {
    width: 100%;
    margin: 8px 0;
  }
  
  .post-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-top: 12px;
    border-top: 1px solid #eee;
  }
  
  .post-stats {
    font-size: 14px;
    color: #666;
  }
  
  .error-message {
    color: #dc3545;
    font-size: 12px;
    margin-top: 8px;
    padding: 8px;
    background: #f8d7da;
    border-radius: 4px;
  }
</style>