<script>
  export let progress = 0;
  export let status = '';
  export let size = 'normal'; // 'small', 'normal', 'large'
  export let showPercentage = true;
  
  $: clampedProgress = Math.max(0, Math.min(100, progress));
  
  $: statusColor = getStatusColor(status);
  
  function getStatusColor(status) {
    switch (status?.toLowerCase()) {
      case 'uploading':
        return 'bg-blue-500';
      case 'processing':
        return 'bg-yellow-500';
      case 'completed':
        return 'bg-green-500';
      case 'failed':
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  }
</script>

<div class="progress-bar {size}" class:indeterminate={progress === -1}>
  <div class="progress-track">
    <div 
      class="progress-fill {statusColor}" 
      style="width: {clampedProgress}%"
    ></div>
  </div>
  
  {#if showPercentage && progress >= 0}
    <span class="progress-text">{Math.round(clampedProgress)}%</span>
  {/if}
  
  {#if status}
    <span class="status-text">{status}</span>
  {/if}
</div>

<style>
  .progress-bar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .progress-track {
    flex: 1;
    background: #e5e7eb;
    border-radius: 9999px;
    overflow: hidden;
  }
  
  .progress-fill {
    height: 100%;
    transition: width 0.3s ease;
    border-radius: 9999px;
  }
  
  .progress-text {
    font-size: 0.875rem;
    font-weight: 500;
    color: #374151;
    min-width: 2.5rem;
    text-align: right;
  }
  
  .status-text {
    font-size: 0.75rem;
    color: #6b7280;
    text-transform: capitalize;
  }
  
  /* Size variants */
  .small .progress-track {
    height: 4px;
  }
  
  .normal .progress-track {
    height: 8px;
  }
  
  .large .progress-track {
    height: 12px;
  }
  
  /* Indeterminate state */
  .indeterminate .progress-fill {
    width: 30%;
    animation: indeterminate 1.5s ease-in-out infinite;
  }
  
  @keyframes indeterminate {
    0% {
      transform: translateX(-100%);
    }
    100% {
      transform: translateX(400%);
    }
  }
  
  /* Color variants */
  .bg-blue-500 { background-color: #3b82f6; }
  .bg-yellow-500 { background-color: #eab308; }
  .bg-green-500 { background-color: #10b981; }
  .bg-red-500 { background-color: #ef4444; }
  .bg-gray-500 { background-color: #6b7280; }
</style>
</write_to_file>