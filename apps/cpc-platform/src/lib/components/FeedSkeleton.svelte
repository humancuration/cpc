<script>
  export let count = 3;
  export let compact = false;
</script>

<div class="feed-skeleton" class:compact aria-busy="true" aria-label="Loading posts">
  {#each Array(count) as _, i}
    <div class="skeleton-post" aria-hidden="true">
      <div class="skeleton-header">
        <div class="skeleton-avatar" role="presentation"></div>
        <div class="skeleton-meta">
          <div class="skeleton-line skeleton-line-sm" role="presentation"></div>
          <div class="skeleton-line skeleton-line-xs" role="presentation"></div>
        </div>
      </div>
      
      <div class="skeleton-content">
        <div class="skeleton-line" role="presentation"></div>
        <div class="skeleton-line skeleton-line-md" role="presentation"></div>
        <div class="skeleton-line skeleton-line-sm" role="presentation"></div>
      </div>
      
      {#if !compact}
        <div class="skeleton-media">
          <div class="skeleton-media-item" role="presentation"></div>
          <div class="skeleton-media-item" role="presentation"></div>
        </div>
      {/if}
      
      <div class="skeleton-actions">
        <div class="skeleton-action" role="presentation"></div>
        <div class="skeleton-action" role="presentation"></div>
        <div class="skeleton-action" role="presentation"></div>
      </div>
    </div>
  {/each}
</div>

<style>
  .feed-skeleton {
    width: 100%;
  }
  
  .skeleton-post {
    background: white;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 16px;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  }
  
  .compact .skeleton-post {
    padding: 16px;
    margin-bottom: 8px;
  }
  
  .skeleton-header {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
  }
  
  .skeleton-avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: #e0e0e0;
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  .skeleton-meta {
    margin-left: 12px;
    flex: 1;
  }
  
  .skeleton-line {
    height: 16px;
    background: #e0e0e0;
    border-radius: 4px;
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  .skeleton-line-sm {
    width: 60%;
    height: 14px;
  }
  
  .skeleton-line-xs {
    width: 40%;
    height: 12px;
    margin-top: 4px;
  }
  
  .skeleton-line-md {
    width: 80%;
  }
  
  .skeleton-content {
    margin-bottom: 16px;
  }
  
  .skeleton-content .skeleton-line {
    margin-bottom: 8px;
  }
  
  .skeleton-media {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
    gap: 8px;
    margin-bottom: 16px;
  }
  
  .skeleton-media-item {
    aspect-ratio: 1;
    background: #e0e0e0;
    border-radius: 8px;
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  .skeleton-actions {
    display: flex;
    gap: 16px;
  }
  
  .skeleton-action {
    height: 32px;
    background: #e0e0e0;
    border-radius: 16px;
    flex: 1;
    animation: pulse 1.5s ease-in-out infinite;
  }
  
  @keyframes pulse {
    0% {
      opacity: 1;
    }
    50% {
      opacity: 0.4;
    }
    100% {
      opacity: 1;
    }
  }
  
  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .skeleton-avatar,
    .skeleton-line,
    .skeleton-media-item,
    .skeleton-action {
      animation: none;
    }
  }
</style>