import { writable, derived } from 'svelte/store';
import { browser } from '$app/environment';

// Social feed state management
function createSocialStore() {
  const { subscribe, set, update } = writable({
    posts: [],
    loading: false,
    error: null,
    hasMore: true,
    offset: 0,
    cache: new Map(),
    mediaCache: new Map()
  });

  return {
    subscribe,
    
    // Cache management
    cachePost: (post) => update(state => {
      const newCache = new Map(state.cache);
      newCache.set(post.id, post);
      return { ...state, cache: newCache };
    }),
    
    cacheMedia: (mediaId, media) => update(state => {
      const newMediaCache = new Map(state.mediaCache);
      newMediaCache.set(mediaId, media);
      return { ...state, mediaCache: newMediaCache };
    }),
    
    getCachedPost: (postId) => derived(
      this,
      $store => $store.cache.get(postId)
    ),
    
    getCachedMedia: (mediaId) => derived(
      this,
      $store => $store.mediaCache.get(mediaId)
    ),
    
    // Feed state management
    setLoading: (loading) => update(state => ({ ...state, loading })),
    setError: (error) => update(state => ({ ...state, error })),
    setHasMore: (hasMore) => update(state => ({ ...state, hasMore })),
    setOffset: (offset) => update(state => ({ ...state, offset })),
    
    // Post management
    addPosts: (newPosts) => update(state => ({
      ...state,
      posts: [...state.posts, ...newPosts],
      offset: state.offset + newPosts.length
    })),
    
    prependPost: (post) => update(state => ({
      ...state,
      posts: [post, ...state.posts],
      offset: state.offset + 1
    })),
    
    updatePost: (postId, updates) => update(state => ({
      ...state,
      posts: state.posts.map(p => 
        p.id === postId ? { ...p, ...updates } : p
      )
    })),
    
    removePost: (postId) => update(state => ({
      ...state,
      posts: state.posts.filter(p => p.id !== postId),
      offset: Math.max(0, state.offset - 1)
    })),
    
    clearFeed: () => update(state => ({
      ...state,
      posts: [],
      offset: 0,
      hasMore: true
    })),
    
    // Cache persistence for desktop
    saveToStorage: () => {
      if (!browser) return;
      
      update(state => {
        try {
          const cacheData = {
            posts: state.posts.slice(0, 50), // Keep last 50 posts
            cache: Array.from(state.cache.entries()).slice(-100), // Keep last 100 cached
            mediaCache: Array.from(state.mediaCache.entries()).slice(-50)
          };
          
          localStorage.setItem('cpc-social-cache', JSON.stringify(cacheData));
        } catch (error) {
          console.error('Failed to save social cache:', error);
        }
        return state;
      });
    },
    
    loadFromStorage: () => {
      if (!browser) return;
      
      update(state => {
        try {
          const cached = localStorage.getItem('cpc-social-cache');
          if (cached) {
            const data = JSON.parse(cached);
            return {
              ...state,
              posts: data.posts || [],
              cache: new Map(data.cache || []),
              mediaCache: new Map(data.mediaCache || [])
            };
          }
        } catch (error) {
          console.error('Failed to load social cache:', error);
        }
        return state;
      });
    }
  };
}

export const socialStore = createSocialStore();

// Derived stores for specific use cases
export const feedStats = derived(socialStore, $store => ({
  totalPosts: $store.posts.length,
  hasCachedPosts: $store.cache.size > 0,
  hasCachedMedia: $store.mediaCache.size > 0
}));

export const isFeedEmpty = derived(socialStore, $store => 
  $store.posts.length === 0 && !$store.loading
);

// Media loading state
export const mediaLoading = writable({
  activeUploads: new Map(),
  processing: new Set(),
  errors: new Map()
});

// Desktop-specific media state
export const desktopMedia = writable({
  selectedFiles: [],
  desktopMode: false,
  nativeDialog: false
});