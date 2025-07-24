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
    },

    // Toggle like on a post
    async toggleLike(postId) {
      let currentState;
      subscribe(state => { currentState = state; })();
      
      const post = currentState.posts.find(p => p.id === postId);
      if (!post) {
        throw new Error('Post not found');
      }

      const isLiked = post.likedByCurrentUser || false;
      
      try {
        await repository.toggleLike(postId, isLiked);
        
        // Update the post in the timeline
        update(state => ({
          ...state,
          posts: state.posts.map(p =>
            p.id === postId
              ? {
                  ...p,
                  likedByCurrentUser: !isLiked,
                  likeCount: isLiked ? (p.likeCount || 1) - 1 : (p.likeCount || 0) + 1
                }
              : p
          )
        }));
        
        return true;
      } catch (error) {
        console.error('Error toggling like:', error);
        throw error;
      }
    },

    // Create a comment on a post
    async createComment(postId, content) {
      try {
        const comment = await repository.createComment(postId, content);
        
        // Update the post in the timeline with the new comment
        update(state => ({
          ...state,
          posts: state.posts.map(p =>
            p.id === postId
              ? {
                  ...p,
                  commentCount: (p.commentCount || 0) + 1,
                  comments: [...(p.comments || []), comment]
                }
              : p
          )
        }));
        
        return comment;
      } catch (error) {
        console.error('Error creating comment:', error);
        throw error;
      }
    },

    // Delete a comment
    async deleteComment(commentId, postId) {
      try {
        await repository.deleteComment(commentId);
        
        // Update the post in the timeline
        update(state => ({
          ...state,
          posts: state.posts.map(p =>
            p.id === postId
              ? {
                  ...p,
                  commentCount: Math.max(0, (p.commentCount || 1) - 1),
                  comments: (p.comments || []).filter(c => c.id !== commentId)
                }
              : p
          )
        }));
        
        return true;
      } catch (error) {
        console.error('Error deleting comment:', error);
        throw error;
      }
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