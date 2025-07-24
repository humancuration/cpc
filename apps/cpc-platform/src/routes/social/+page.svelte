<script>
  import { onMount } from 'svelte';
  import FeedView from '$lib/components/FeedView.svelte';
  import PostComposer from '$lib/components/PostComposer.svelte';
  import { currentUser } from '$lib/stores/auth';
  import { socialStore } from '$lib/stores/social';
  import { desktopMedia } from '$lib/stores/social';
  
  let cooperativeId = null; // Can be set based on route or user selection
  let showComposer = false;
  let isDesktop = false;
  
  onMount(() => {
    // Detect desktop mode
    isDesktop = window.innerWidth >= 768;
    desktopMedia.update(state => ({ ...state, desktopMode: isDesktop }));
    
    // Load cached data
    socialStore.loadFromStorage();
    
    // Save cache on page unload
    window.addEventListener('beforeunload', () => {
      socialStore.saveToStorage();
    });
  });
  
  function handlePostCreated(event) {
    const { post } = event.detail;
    socialStore.prependPost(post);
    showComposer = false;
  }
  
  function handleSignIn() {
    // Navigate to auth flow
    window.location.href = '/auth/login';
  }
</script>

<svelte:head>
  <title>Social Feed - CPC Platform</title>
  <meta name="description" content="Share updates, media, and collaborate with your cooperative" />
</svelte:head>

<div class="social-page" class:desktop={isDesktop}>
  <main class="social-main">
    {#if $currentUser}
      <div class="feed-container">
        <FeedView {cooperativeId} showComposer={!isDesktop} />
        
        {#if isDesktop && showComposer}
          <aside class="composer-sidebar" role="complementary" aria-label="Post composer">
            <div class="composer-wrapper">
              <PostComposer
                {cooperativeId}
                on:postCreated={handlePostCreated}
                on:cancel={() => showComposer = false}
              />
            </div>
          </aside>
        {/if}
      </div>
    {:else}
      <div class="auth-prompt" role="main" aria-label="Authentication required">
        <h2>Welcome to the CPC Social Feed</h2>
        <p>Sign in to start sharing and connecting with your cooperative</p>
        <button class="sign-in-btn" on:click={handleSignIn} aria-label="Sign in to your account">
          Sign In
        </button>
      </div>
    {/if}
  </main>
</div>

<style>
  .social-page {
    min-height: 100vh;
    background: #f5f5f5;
  }
  
  .auth-prompt {
    max-width: 400px;
    margin: 100px auto;
    text-align: center;
    padding: 40px;
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }
  
  .auth-prompt h2 {
    margin: 0 0 16px 0;
    color: #333;
  }
  
  .auth-prompt p {
    margin: 0 0 24px 0;
    color: #666;
  }
  
  .sign-in-btn {
    background: #007bff;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 24px;
    font-size: 16px;
    cursor: pointer;
  }
  
  .sign-in-btn:hover {
    background: #0056b3;
  }
  
  /* Desktop optimizations */
  .desktop .social-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0;
  }
  
  .social-main {
    padding: 0;
  }
  
  .feed-container {
    display: flex;
    gap: 20px;
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .composer-sidebar {
    flex: 0 0 400px;
    position: sticky;
    top: 20px;
    height: fit-content;
  }
  
  .composer-wrapper {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    overflow: hidden;
  }
  
  /* Responsive design */
  @media (max-width: 768px) {
    .feed-container {
      flex-direction: column;
      padding: 10px;
    }
    
    .composer-sidebar {
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: rgba(0, 0, 0, 0.5);
      z-index: 1000;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 20px;
    }
    
    .composer-wrapper {
      max-width: 600px;
      width: 100%;
      max-height: 90vh;
      overflow-y: auto;
    }
  }
  
  /* Accessibility improvements */
  @media (prefers-reduced-motion: reduce) {
    * {
      transition: none !important;
      animation: none !important;
    }
  }
  
  /* Focus styles */
  button:focus-visible,
  [tabindex]:focus-visible {
    outline: 2px solid #007bff;
    outline-offset: 2px;
  }
</style>