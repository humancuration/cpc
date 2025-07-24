<script>
    import { onMount } from 'svelte';
    import { socialStore } from '../../stores/socialStore.js';
    
    export let experienceId;
    export let onCommentAdded = () => {};
    
    let comments = [];
    let newComment = '';
    let isLoading = true;
    let isAdding = false;
    let error = null;
    let unsubscribe;
    
    onMount(async () => {
        await loadComments();
        
        // Subscribe to new comments
        unsubscribe = socialStore.subscribeToComments(experienceId, (comment) => {
            comments = [...comments, comment];
        });
        
        return () => {
            if (unsubscribe) unsubscribe();
        };
    });
    
    async function loadComments() {
        isLoading = true;
        error = null;
        
        try {
            comments = await socialStore.getComments(experienceId);
        } catch (err) {
            error = err.message || 'Failed to load comments';
        } finally {
            isLoading = false;
        }
    }
    
    async function handleAddComment() {
        if (!newComment.trim() || isAdding) return;
        
        isAdding = true;
        error = null;
        
        try {
            const comment = await socialStore.addComment(experienceId, newComment.trim());
            comments = [...comments, comment];
            newComment = '';
            onCommentAdded(comment);
        } catch (err) {
            error = err.message || 'Failed to add comment';
        } finally {
            isAdding = false;
        }
    }
    
    function formatDate(date) {
        return new Date(date).toLocaleString();
    }
</script>

<div class="comment-section">
    <h4>Comments</h4>
    
    {#if isLoading}
        <div class="loading">Loading comments...</div>
    {:else if error}
        <div class="error-message">{error}</div>
    {/if}
    
    <div class="comments-list">
        {#each comments as comment (comment.id)}
            <div class="comment">
                <div class="comment-header">
                    <span class="author">User {comment.authorId.substring(0, 8)}</span>
                    <span class="timestamp">{formatDate(comment.createdAt)}</span>
                </div>
                <div class="comment-content">{comment.content}</div>
            </div>
        {/each}
        
        {#if comments.length === 0 && !isLoading}
            <div class="no-comments">No comments yet. Be the first to comment!</div>
        {/if}
    </div>
    
    <div class="add-comment">
        <textarea
            bind:value={newComment}
            placeholder="Add a comment..."
            rows="3"
            disabled={isAdding}
        ></textarea>
        
        <div class="comment-actions">
            <button
                on:click={handleAddComment}
                disabled={!newComment.trim() || isAdding}
            >
                {isAdding ? 'Posting...' : 'Post Comment'}
            </button>
        </div>
    </div>
</div>

<style>
    .comment-section {
        max-width: 600px;
        margin: 0 auto;
        padding: 1rem;
    }
    
    h4 {
        margin-bottom: 1rem;
        color: var(--text-primary);
    }
    
    .loading, .error-message, .no-comments {
        text-align: center;
        padding: 1rem;
        color: var(--text-secondary);
    }
    
    .error-message {
        color: var(--error-color);
        background: var(--error-bg);
        border-radius: 4px;
    }
    
    .comments-list {
        margin-bottom: 1rem;
    }
    
    .comment {
        border-bottom: 1px solid var(--border-color);
        padding: 0.75rem 0;
    }
    
    .comment:last-child {
        border-bottom: none;
    }
    
    .comment-header {
        display: flex;
        justify-content: space-between;
        margin-bottom: 0.5rem;
        font-size: 0.875rem;
    }
    
    .author {
        font-weight: 600;
        color: var(--text-primary);
    }
    
    .timestamp {
        color: var(--text-secondary);
    }
    
    .comment-content {
        color: var(--text-primary);
        line-height: 1.4;
    }
    
    .add-comment {
        margin-top: 1rem;
        padding-top: 1rem;
        border-top: 1px solid var(--border-color);
    }
    
    textarea {
        width: 100%;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        background: var(--input-bg);
        color: var(--text-primary);
        resize: vertical;
        min-height: 60px;
    }
    
    .comment-actions {
        display: flex;
        justify-content: flex-end;
        margin-top: 0.5rem;
    }
    
    button {
        padding: 0.5rem 1rem;
        background: var(--primary-color);
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }
    
    button:disabled {
        background: var(--disabled-bg);
        cursor: not-allowed;
    }
</style>
</script>