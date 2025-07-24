<script>
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  import MediaUploader from './MediaUploader.svelte';
  import RichTextEditor from './RichTextEditor.svelte';
  import MediaAttachment from './MediaAttachment.svelte';
  import ProgressBar from './ProgressBar.svelte';
  import { createPost } from '$lib/services/post-service';
  import { addToast } from '$lib/stores/toast';
  import { debounce } from '$lib/utils/debounce';
  import { mediaSubscriptionService } from '$lib/services/media-subscription';
  import { desktopMediaService } from '$lib/services/desktop-media-service';
  
  export let cooperativeId = null;
  export let placeholder = "What's on your mind?";
  export let maxLength = 2000;
  export let autoSaveKey = 'post-draft';
  
  const dispatch = createEventDispatcher();
  
  let content = '';
  let visibility = 'PUBLIC';
  let media = [];
  let isComposing = false;
  let isPosting = false;
  let error = null;
  let characterCount = 0;
  let isSavingDraft = false;
  let lastSaved = null;
  let processingMedia = new Map();
  
  // Rich text editor reference
  let richTextEditor;
  
  // Visibility options
  const visibilityOptions = [
    { value: 'PUBLIC', label: 'Public', icon: '👥', description: 'Anyone can see' },
    { value: 'COOPERATIVE', label: 'Cooperative', icon: '🏢', description: 'Cooperative members only' },
    { value: 'PRIVATE', label: 'Private', icon: '🔒', description: 'Only you can see' }
  ];
  
  $: characterCount = content.length;
  $: isValid = (content.trim().length > 0 || media.length > 0) && characterCount <= maxLength;
  $: canPost = isValid && !isPosting && !hasProcessingMedia();
  $: readyMedia = media.filter(m => m.processingStatus === 'completed');
  
  // Auto-save functionality
  const saveDraft = debounce(async () => {
    if (!content.trim() && media.length === 0) return;
    
    isSavingDraft = true;
    try {
      const draft = {
        content,
        visibility,
        media,
        timestamp: new Date().toISOString()
      };
      localStorage.setItem(`draft-${autoSaveKey}`, JSON.stringify(draft));
      lastSaved = new Date();
    } catch (err) {
      console.error('Failed to save draft:', err);
    } finally {
      isSavingDraft = false;
    }
  }, 2000);
  
  // Load draft on mount
  onMount(() => {
    try {
      const savedDraft = localStorage.getItem(`draft-${autoSaveKey}`);
      if (savedDraft) {
        const draft = JSON.parse(savedDraft);
        content = draft.content || '';
        visibility = draft.visibility || 'PUBLIC';
        media = draft.media || [];
        lastSaved = new Date(draft.timestamp);
        
        // Subscribe to media processing status for any existing media
        subscribeToMediaProcessing();
      }
    } catch (err) {
      console.error('Failed to load draft:', err);
    }
  });
  
  // Cleanup on destroy
  onDestroy(() => {
    mediaSubscriptionService.unsubscribeAll();
  });
  
  // Watch for changes to auto-save
  $: if (content || media.length > 0) {
    saveDraft();
  }
  
  function hasProcessingMedia() {
    return media.some(m => m.processingStatus && m.processingStatus !== 'completed');
  }
  
  function handleContentChange(event) {
    content = event.detail.content;
  }
  
  function handleMediaUpload(event) {
    const { media: uploadedMedia } = event.detail;
    const newMedia = uploadedMedia.map(item => ({
      ...item,
      processingStatus: item.processingStatus || 'uploading',
      progress: item.progress || 0
    }));
    
    media = [...media, ...newMedia];
    
    // Subscribe to processing status for new media
    newMedia.forEach(item => {
      if (item.id && item.processingStatus !== 'completed') {
        subscribeToMediaProcessing(item.id);
      }
    });
  }
  
  function handleMediaRemove(index) {
    const removedMedia = media[index];
    if (removedMedia?.id) {
      mediaSubscriptionService.unsubscribeFromMediaStatus(removedMedia.id);
    }
    media = media.filter((_, i) => i !== index);
  }
  
  function subscribeToMediaProcessing(mediaId) {
    mediaSubscriptionService.subscribeToMediaStatus(mediaId, (update) => {
      media = media.map(item => {
        if (item.id === mediaId) {
          return {
            ...item,
            ...update,
            processingStatus: update.status || update.processingStatus
          };
        }
        return item;
      });
    });
  }
  
  function handleVisibilityChange(event) {
    visibility = event.target.value;
  }
  
  async function handleSubmit() {
    if (!canPost) return;
    
    isPosting = true;
    error = null;
    
    try {
      // Filter media to only include completed ones
      const completedMedia = media.filter(m => m.processingStatus === 'completed');
      
      const newPost = await createPost({
        content: content.trim(),
        visibility,
        cooperativeId,
        mediaIds: completedMedia.map(m => m.id)
      });
      
      // Clear draft
      localStorage.removeItem(`draft-${autoSaveKey}`);
      
      // Reset form
      content = '';
      media = [];
      visibility = 'PUBLIC';
      lastSaved = null;
      processingMedia.clear();
      
      if (richTextEditor) {
        richTextEditor.clear();
      }
      
      addToast({
        type: 'success',
        message: 'Post created successfully!',
        duration: 3000
      });
      
      dispatch('postCreated', { post: newPost });
    } catch (err) {
      error = err.message || 'Failed to create post';
      addToast({
        type: 'error',
        message: error,
        duration: 5000
      });
    } finally {
      isPosting = false;
    }
  }
  
  function handleCancel() {
    if (content.trim() || media.length > 0) {
      if (confirm('You have unsaved content. Are you sure you want to cancel?')) {
        clearForm();
      }
    } else {
      clearForm();
    }
  }
  
  function clearForm() {
    content = '';
    media = [];
    visibility = 'PUBLIC';
    error = null;
    processingMedia.clear();
    localStorage.removeItem(`draft-${autoSaveKey}`);
    if (richTextEditor) {
      richTextEditor.clear();
    }
    mediaSubscriptionService.unsubscribeAll();
    dispatch('cancel');
  }
  
  function handleKeyDown(event) {
    if (event.ctrlKey && event.key === 'Enter' && canPost) {
      event.preventDefault();
      handleSubmit();
    }
    
    if (event.key === 'Escape') {
      event.preventDefault();
      handleCancel();
    }
  }
  
  function formatLastSaved() {
    if (!lastSaved) return '';
    return `Last saved: ${lastSaved.toLocaleTimeString()}`;
  }
</script>

<svelte:window on:keydown={handleKeyDown} />

<div class="post-composer">
  <div class="composer-header">
    <h3>Create Post</h3>
    <div class="header-actions">
      {#if lastSaved}
        <span class="draft-status" class:saving={isSavingDraft}>
          {isSavingDraft ? 'Saving...' : formatLastSaved()}
        </span>
      {/if}
      <button class="close-btn" on:click={handleCancel} title="Close (Esc)">&times;</button>
    </div>
  </div>
  
  <div class="composer-content">
    <div class="visibility-selector">
      <label for="visibility">Visibility:</label>
      <select id="visibility" bind:value={visibility} on:change={handleVisibilityChange}>
        {#each visibilityOptions as option}
          <option value={option.value}>
            {option.icon} {option.label} - {option.description}
          </option>
        {/each}
      </select>
    </div>
    
    <div class="editor-container">
      <RichTextEditor
        bind:this={richTextEditor}
        bind:content
        {placeholder}
        {maxLength}
        on:change={handleContentChange}
        disabled={isPosting}
      />
    </div>
    
    <div class="character-count">
      <span class:warning={characterCount > maxLength * 0.9}>
        {characterCount}/{maxLength}
      </span>
      {#if characterCount >= maxLength}
        <span class="error">Maximum length reached</span>
      {/if}
    </div>
    
    {#if media.length > 0}
      <div class="media-preview">
        <h4>Attached Media ({media.length})</h4>
        <div class="media-grid">
          {#each media as item, index}
            <div class="media-item">
              {#if item.type === 'Image'}
                <img src={item.url} alt={item.description || 'Attached image'} />
              {:else if item.type === 'Video'}
                <video src={item.url} muted></video>
                <div class="media-overlay">
                  <span class="media-type">🎥</span>
                </div>
              {:else if item.type === 'Audio'}
                <div class="audio-placeholder">
                  <span class="media-type">🎵</span>
                  <span class="audio-name">{item.description || 'Audio file'}</span>
                </div>
              {/if}
              <button 
                class="remove-media-btn"
                on:click={() => handleMediaRemove(index)}
                disabled={isPosting}
                title="Remove media"
              >
                ✕
              </button>
            </div>
          {/each}
        </div>
      </div>
    {/if}
    
    <MediaUploader
      on:filesSelected={handleMediaUpload}
      on:uploadComplete={handleMediaUpload}
      postId={null}
      maxFiles={10}
      maxFileSize={50 * 1024 * 1024} // 50MB
      acceptedTypes={['image/*', 'video/*', 'audio/*']}
    />
    
    {#if error}
      <div class="error-message">
        <strong>Error:</strong> {error}
      </div>
    {/if}
  </div>
  
  <div class="composer-footer">
    <div class="footer-info">
      <span class="keyboard-shortcuts">
        <kbd>Ctrl+Enter</kbd> to post • <kbd>Esc</kbd> to cancel
      </span>
    </div>
    <div class="footer-actions">
      <button 
        class="cancel-btn"
        on:click={handleCancel}
        disabled={isPosting}
      >
        Cancel
      </button>
      <button 
        class="post-btn"
        on:click={handleSubmit}
        disabled={!canPost}
        class:loading={isPosting}
      >
        {isPosting ? 'Posting...' : 'Post'}
      </button>
    </div>
  </div>
</div>

<style>
  .post-composer {
    background: white;
    border-radius: 12px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    overflow: hidden;
    max-width: 600px;
    margin: 0 auto;
    border: 1px solid #e1e5e9;
  }
  
  .composer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid #eee;
    background: #f8f9fa;
  }
  
  .composer-header h3 {
    margin: 0;
    color: #333;
    font-size: 18px;
    font-weight: 600;
  }
  
  .header-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  
  .draft-status {
    font-size: 12px;
    color: #666;
    font-style: italic;
  }
  
  .draft-status.saving {
    color: #007bff;
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.5; }
  }
  
  .close-btn {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #666;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    transition: all 0.2s;
  }
  
  .close-btn:hover {
    background: #e9ecef;
    color: #333;
  }
  
  .composer-content {
    padding: 20px;
  }
  
  .visibility-selector {
    margin-bottom: 16px;
  }
  
  .visibility-selector label {
    display: block;
    margin-bottom: 4px;
    font-weight: 600;
    color: #333;
  }
  
  .visibility-selector select {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ddd;
    border-radius: 6px;
    font-size: 14px;
    background: white;
  }
  
  .editor-container {
    margin-bottom: 12px;
  }
  
  .character-count {
    text-align: right;
    font-size: 12px;
    color: #666;
    margin-bottom: 12px;
  }
  
  .character-count .warning {
    color: #f39c12;
    font-weight: bold;
  }
  
  .character-count .error {
    color: #e74c3c;
    font-weight: bold;
  }
  
  .media-preview {
    margin: 16px 0;
    padding: 16px;
    background: #f8f9fa;
    border-radius: 8px;
    border: 1px solid #e1e5e9;
  }
  
  .media-preview h4 {
    margin: 0 0 12px 0;
    color: #333;
    font-size: 14px;
    font-weight: 600;
  }
  
  .media-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 8px;
  }
  
  .media-item {
    position: relative;
    border-radius: 8px;
    overflow: hidden;
    background: #e9ecef;
    aspect-ratio: 1;
  }
  
  .media-item img,
  .media-item video {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  
  .audio-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    background: #e9ecef;
    text-align: center;
    padding: 8px;
  }
  
  .media-type {
    font-size: 24px;
    margin-bottom: 4px;
  }
  
  .audio-name {
    font-size: 11px;
    color: #666;
    word-break: break-word;
  }
  
  .remove-media-btn {
    position: absolute;
    top: 4px;
    right: 4px;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    border: none;
    border-radius: 50%;
    width: 24px;
    height: 24px;
    font-size: 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }
  
  .remove-media-btn:hover:not(:disabled) {
    background: rgba(0, 0, 0, 0.9);
    transform: scale(1.1);
  }
  
  .remove-media-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
  
  .error-message {
    color: #e74c3c;
    background: #fdf2f2;
    padding: 12px;
    border-radius: 6px;
    margin-top: 12px;
    font-size: 14px;
    border-left: 4px solid #e74c3c;
  }
  
  .composer-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-top: 1px solid #eee;
    background: #f8f9fa;
  }
  
  .footer-info {
    font-size: 12px;
    color: #666;
  }
  
  .keyboard-shortcuts kbd {
    background: #e9ecef;
    padding: 2px 4px;
    border-radius: 3px;
    font-size: 11px;
    font-family: monospace;
  }
  
  .footer-actions {
    display: flex;
    gap: 12px;
  }
  
  .cancel-btn,
  .post-btn {
    padding: 8px 16px;
    border-radius: 6px;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
    min-width: 80px;
  }
  
  .cancel-btn {
    background: white;
    border: 1px solid #ddd;
    color: #666;
  }
  
  .cancel-btn:hover:not(:disabled) {
    background: #f8f9fa;
    border-color: #bbb;
  }
  
  .post-btn {
    background: #007bff;
    color: white;
    border: none;
  }
  
  .post-btn:hover:not(:disabled) {
    background: #0056b3;
  }
  
  .post-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  .post-btn.loading {
    position: relative;
  }
  
  .post-btn.loading::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 16px;
    height: 16px;
    margin: -8px 0 0 -8px;
    border: 2px solid #ffffff;
    border-radius: 50%;
    border-top-color: transparent;
    animation: spin 1s linear infinite;
  }
  
  @keyframes spin {
    to { transform: rotate(360deg); }
  }
  
  /* Responsive design */
  @media (max-width: 640px) {
    .post-composer {
      margin: 8px;
      border-radius: 8px;
    }
    
    .composer-content {
      padding: 16px;
    }
    
    .composer-footer {
      flex-direction: column;
      gap: 8px;
      align-items: stretch;
    }
    
    .footer-info {
