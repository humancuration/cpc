<script>
  import { onMount } from 'svelte';
  import { createPost, getTimeline } from '$lib/graphql/social';
  import { graphqlClient } from '$lib/graphql/client';
  
  export let posts = [];
  export let loading = false;
  export let error = null;
  
  let newPostContent = '';
  let newPostVisibility = 'PUBLIC';
  let isSubmitting = false;
  
  const visibilityOptions = [
    { value: 'PUBLIC', label: 'Public' },
    { value: 'COOPERATIVE', label: 'Cooperative' },
    { value: 'PRIVATE', label: 'Private' }
  ];
  
  async function loadTimeline() {
    loading = true;
    error = null;
    try {
      const { data } = await graphqlClient.query({
        query: getTimeline,
        variables: { limit: 20, offset: 0 }
      });
      posts = data.timeline;
    } catch (err) {
      error = err.message;
    } finally {
      loading = false;
    }
  }
  
  async function handleCreatePost() {
    if (!newPostContent.trim()) return;
    
    isSubmitting = true;
    try {
      const { data } = await graphqlClient.mutate({
        mutation: createPost,
        variables: {
          content: newPostContent.trim(),
          visibility: newPostVisibility
        }
      });
      
      // Add new post to the beginning of the list
      posts.unshift(data.createPost);
      newPostContent = '';
    } catch (err) {
      error = err.message;
    } finally {
      isSubmitting = false;
    }
  }
  
  onMount(() => {
    loadTimeline();
  });
</script>

<div class="social-service">
  <div class="create-post">
    <h2>Create Post</h2>
    <form on:submit|preventDefault={handleCreatePost}>
      <textarea
        bind:value={newPostContent}
        placeholder="What's on your mind?"
        rows="4"
        maxlength="500"
        disabled={isSubmitting}
      />
      
      <div class="post-controls">
        <select bind:value={newPostVisibility} disabled={isSubmitting}>
          {#each visibilityOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
        
        <button type="submit" disabled={!newPostContent.trim() || isSubmitting}>
          {isSubmitting ? 'Posting...' : 'Post'}
        </button>
      </div>
    </form>
  </div>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
</div>

<style>
  .social-service {
    max-width: 600px;
    margin: 0 auto;
  }
  
  .create-post {
    background: white;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }
  
  .create-post h2 {
    margin-top: 0;
    color: var(--text);
  }
  
  textarea {
    width: 100%;
    padding: 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-family: inherit;
    resize: vertical;
    min-height: 80px;
  }
  
  .post-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 12px;
  }
  
  select {
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: white;
  }
  
  button {
    padding: 8px 20px;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: opacity 0.2s;
  }
  
  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .error {
    background: #fee;
    color: #c33;
    padding: 12px;
    border-radius: 4px;
    margin-bottom: 16px;
  }
</style>