import { writable, derived } from 'svelte/store';
import { graphqlClient } from '$lib/graphql/client';
import { GET_POST, CREATE_COMMENT, LIKE_POST, UNLIKE_POST } from '$lib/graphql/social';

// Post detail store
function createPostStore() {
  const { subscribe, set, update } = writable({
    post: null,
    comments: [],
    loading: false,
    error: null
  });

  return {
    subscribe,
    
    async loadPost(postId) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const { data } = await graphqlClient.query({
          query: GET_POST,
          variables: { id: postId }
        });
        
        update(state => ({
          ...state,
          post: data.post,
          comments: data.post?.comments || [],
          loading: false,
          error: null
        }));
      } catch (error) {
        update(state => ({
          ...state,
          loading: false,
          error: error.message
        }));
      }
    },
    
    async addComment(postId, content) {
      try {
        const { data } = await graphqlClient.mutate({
          mutation: CREATE_COMMENT,
          variables: { postId, content }
        });
        
        update(state => ({
          ...state,
          comments: [...state.comments, data.createComment]
        }));
      } catch (error) {
        console.error('Failed to add comment:', error);
      }
    },
    
    async toggleLike(postId) {
      const currentState = get({ subscribe });
      const isLiked = currentState.post?.isLikedByCurrentUser || false;
      
      try {
        if (isLiked) {
          await graphqlClient.mutate({
            mutation: UNLIKE_POST,
            variables: { postId }
          });
        } else {
          await graphqlClient.mutate({
            mutation: LIKE_POST,
            variables: { postId }
          });
        }
        
        update(state => ({
          ...state,
          post: {
            ...state.post,
            isLikedByCurrentUser: !isLiked,
            likeCount: isLiked ? state.post.likeCount - 1 : state.post.likeCount + 1
          }
        }));
      } catch (error) {
        console.error('Failed to toggle like:', error);
      }
    },
    
    reset() {
      set({
        post: null,
        comments: [],
        loading: false,
        error: null
      });
    }
  };
}

// Create the post store instance
export const postStore = createPostStore();

// Derived stores for computed values
export const isPostOwner = derived(
  postStore,
  $postStore => {
    // This would need to be connected to user auth
    return false;
  }
);

export const canComment = derived(
  postStore,
  $postStore => {
    // Check if user can comment based on post visibility and user permissions
    return true;
  }
);