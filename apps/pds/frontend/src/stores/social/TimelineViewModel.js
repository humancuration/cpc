import { writable, derived } from 'svelte/store';
import { SocialRepository } from '$lib/social/SocialRepository.js';

// Timeline state management
function createTimelineStore() {
  const { subscribe, set, update } = writable({
    posts: [],
    loading: false,
    error: null,
    hasMore: true,
    filters: {
      type: 'all', // 'all', 'following', 'cooperative'
      authorId: null
    },
    pagination: {
      limit: 20,
      offset: 0
    }
  });

  const repository = new SocialRepository();

  return {
    subscribe,
    
    // Load timeline with pagination
    async loadTimeline(reset = false) {
      update(state => ({
        ...state,
        loading: true,
        error: null,
        pagination: reset ? { ...state.pagination, offset: 0 } : state.pagination
      }));

      try {
        let currentState;
        subscribe(state => { currentState = state; })();
        
        const { posts, hasMore } = await repository.getTimeline(
          currentState.filters,
          currentState.pagination.limit,
          currentState.pagination.offset
        );

        update(state => ({
          ...state,
          posts: reset ? posts : [...state.posts, ...posts],
          loading: false,
          hasMore,
          pagination: {
            ...state.pagination,
            offset: state.pagination.offset + posts.length
          }
        }));
      } catch (error) {
        update(state => ({
          ...state,
          loading: false,
          error: error.message || 'Failed to load timeline'
        }));
      }
    },

    // Refresh timeline (clear and reload)
    async refreshTimeline() {
      await this.loadTimeline(true);
    },

    // Load more posts (pagination)
    async loadMorePosts() {
      let currentState;
      subscribe(state => { currentState = state; })();
      
      if (!currentState.loading && currentState.hasMore) {
        await this.loadTimeline(false);
      }
    },

    // Set filters and reload
    async setFilters(filters) {
      update(state => ({
        ...state,
        filters: { ...state.filters, ...filters },
        pagination: { ...state.pagination, offset: 0 },
        posts: [] // Clear posts when filters change
      }));
      await this.loadTimeline(true);
    },

    // Add a new post to the timeline
    addPost(newPost) {
      update(state => ({
        ...state,
        posts: [newPost, ...state.posts]
      }));
    },

    // Update an existing post
    updatePost(updatedPost) {
      update(state => ({
        ...state,
        posts: state.posts.map(post => 
          post.id === updatedPost.id ? updatedPost : post
        )
      }));
    },

    // Remove a post
    removePost(postId) {
      update(state => ({
        ...state,
        posts: state.posts.filter(post => post.id !== postId)
      }));
    },

    // Clear all posts and reset state
    reset() {
      set({
        posts: [],
        loading: false,
        error: null,
        hasMore: true,
        filters: { type: 'all', authorId: null },
        pagination: { limit: 20, offset: 0 }
      });
    }
  };
}

// Create the timeline store instance
export const timelineStore = createTimelineStore();

// Derived stores for computed values
export const isLoading = derived(timelineStore, $store => $store.loading);
export const hasError = derived(timelineStore, $store => $store.error !== null);
export const errorMessage = derived(timelineStore, $store => $store.error);
export const canLoadMore = derived(timelineStore, $store => $store.hasMore && !$store.loading);
export const postCount = derived(timelineStore, $store => $store.posts.length);