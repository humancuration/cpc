<script>
  import { fade, fly } from 'svelte/transition';
  import { toastStore } from '$lib/stores/toast';
  
  const icons = {
    success: '✅',
    error: '❌',
    warning: '⚠️',
    info: 'ℹ️'
  };
  
  function remove(id) {
    toastStore.remove(id);
  }
</script>

<div class="toast-container">
  {#each $toastStore as toast (toast.id)}
    <div
      class="toast toast-{toast.type}"
      transition:fly={{ y: -50, duration: 300 }}
      on:click={() => remove(toast.id)}
    >
      <div class="toast-icon">{icons[toast.type] || icons.info}</div>
      <div class="toast-message">{toast.message}</div>
      <button class="toast-close" on:click={() => remove(toast.id)}>×</button>
    </div>
  {/each}
</div>

<style>
  .toast-container {
    position: fixed;
    top: 20px;
    right: 20px;
    z-index: 1000;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .toast {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    cursor: pointer;
    min-width: 300px;
    max-width: 400px;
  }

  .toast-success {
    background: #d4edda;
    color: #155724;
    border-left: 4px solid #28a745;
  }

  .toast-error {
    background: #f8d7da;
    color: #721c24;
    border-left: 4px solid #dc3545;
  }

  .toast-warning {
    background: #fff3cd;
    color: #856404;
    border-left: 4px solid #ffc107;
  }

  .toast-info {
    background: #d1ecf1;
    color: #0c5460;
    border-left: 4px solid #17a2b8;
  }

  .toast-icon {
    font-size: 16px;
  }

  .toast-message {
    flex: 1;
    font-size: 14px;
  }

  .toast-close {
    background: none;
    border: none;
    font-size: 18px;
    cursor: pointer;
    opacity: 0.7;
  }

  .toast-close:hover {
    opacity: 1;
  }

  @media (max-width: 640px) {
    .toast-container {
      top: 10px;
      right: 10px;
      left: 10px;
    }

    .toast {
      min-width: auto;
    }
  }
</style>