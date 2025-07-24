<script>
  export let media = [];
  export let compact = false;
  
  function getMediaIcon(type) {
    switch (type?.toLowerCase()) {
      case 'image':
        return '🖼️';
      case 'video':
        return '🎥';
      case 'audio':
        return '🎵';
      default:
        return '📁';
    }
  }
  
  function getStatusColor(status) {
    switch (status?.toLowerCase()) {
      case 'completed':
        return 'text-green-600';
      case 'processing':
        return 'text-blue-600';
      case 'failed':
        return 'text-red-600';
      default:
        return 'text-gray-600';
    }
  }
</script>

<div class="media-attachments {compact ? 'compact' : ''}">
  {#each media as item}
    <div class="media-item">
      {#if item.type === 'Image'}
        <img 
          src={item.url} 
          alt={item.description || 'Attached image'} 
          class="media-image"
          loading="lazy"
        />
      {:else if item.type === 'Video'}
        <video 
          src={item.url} 
          controls
          class="media-video"
          poster={item.thumbnailUrl}
        ></video>
      {:else if item.type === 'Audio'}
        <div class="audio-player">
          <audio controls src={item.url}></audio>
        </div>
      {/if}
      
      {#if item.processingStatus && item.processingStatus !== 'completed'}
        <div class="processing-overlay">
          <span class="status {getStatusColor(item.processingStatus)}">
            {item.processingStatus}
          </span>
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .media-attachments {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
    margin: 1rem 0;
  }
  
  .media-attachments.compact {
    grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
    gap: 0.5rem;
  }
  
  .media-item {
    position: relative;
    border-radius: 8px;
    overflow: hidden;
    background: #f8f9fa;
  }
  
  .media-image,
  .media-video {
    width: 100%;
    height: 200px;
    object-fit: cover;
  }
  
  .compact .media-image,
  .compact .media-video {
    height: 120px;
  }
  
  .audio-player {
    padding: 1rem;
    background: #f8f9fa;
  }
  
  .processing-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-size: 0.875rem;
  }
  
  .text-green-600 { color: #10b981; }
  .text-blue-600 { color: #3b82f6; }
  .text-red-600 { color: #ef4444; }
  .text-gray-600 { color: #6b7280; }
</style>
</write_to_file>