import { graphqlClient } from '$lib/graphql/client.js';
import {
  GET_TIMELINE,
  GET_POSTS_BY_USER,
  CREATE_POST,
  DELETE_POST,
  FOLLOW_USER,
  UNFOLLOW_USER,
  GET_FOLLOWERS,
  GET_FOLLOWING,
  SEARCH_USERS
} from '$lib/graphql/social.js';

export class SocialRepository {
  constructor() {
    this.cache = new Map();
    this.cacheTimeout = 5 * 60 * 1000; // 5 minutes
  }

  /**
   * Get cached data or fetch fresh data
   * @param {string} key - Cache key
   * @param {Function} fetcher - Function to fetch fresh data
   * @returns {Promise<any>} - Cached or fresh data
   */
  async getCachedData(key, fetcher) {
    const cached = this.cache.get(key);
    if (cached && Date.now() - cached.timestamp < this.cacheTimeout) {
      return cached.data;
    }

    const data = await fetcher();
    this.cache.set(key, { data, timestamp: Date.now() });
    return data;
  }

  /**
   * Clear cache for specific key or all cache
   * @param {string} key - Optional cache key to clear
   */
  clearCache(key = null) {
    if (key) {
      this.cache.delete(key);
    } else {
      this.cache.clear();
    }
  }

  /**
   * Get timeline posts with pagination
   * @param {Object} filters - Filter options
   * @param {number} limit - Number of posts to fetch
   * @param {number} offset - Offset for pagination
   * @returns {Promise<{posts: Array, hasMore: boolean}>}
   */
  async getTimeline(filters = {}, limit = 20, offset = 0) {
    const cacheKey = `timeline_${JSON.stringify(filters)}_${limit}_${offset}`;
    
    return this.getCachedData(cacheKey, async () => {
      try {
        const { data } = await graphqlClient.query({
          query: GET_TIMELINE,
          variables: { limit, offset },
          fetchPolicy: 'network-first'
        });

        const posts = data?.timeline || [];
        
        // Check if there are more posts to load
        const hasMore = posts.length === limit;
        
        return { posts, hasMore };
      } catch (error) {
        console.error('Error fetching timeline:', error);
        throw new Error('Failed to load timeline');
      }
    });
  }

  /**
   * Get posts by a specific user
   * @param {string} userId - User ID
   * @param {number} limit - Number of posts to fetch
   * @param {number} offset - Offset for pagination
   * @returns {Promise<{posts: Array, hasMore: boolean}>}
   */
  async getPostsByUser(userId, limit = 20, offset = 0) {
    const cacheKey = `user_posts_${userId}_${limit}_${offset}`;
    
    return this.getCachedData(cacheKey, async () => {
      try {
        const { data } = await graphqlClient.query({
          query: GET_POSTS_BY_USER,
          variables: { userId, limit, offset },
          fetchPolicy: 'network-first'
        });

        const posts = data?.postsByUser || [];
        const hasMore = posts.length === limit;
        
        return { posts, hasMore };
      } catch (error) {
        console.error('Error fetching user posts:', error);
        throw new Error('Failed to load user posts');
      }
    });
  }

  /**
   * Create a new post
   * @param {string} content - Post content
   * @param {string} visibility - Post visibility (PUBLIC, COOPERATIVE, PRIVATE)
   * @returns {Promise<Object>} - Created post
   */
  async createPost(content, visibility = 'PUBLIC') {
    try {
      const { data } = await graphqlClient.mutate({
        mutation: CREATE_POST,
        variables: { content, visibility }
      });

      // Clear timeline cache since we added a new post
      this.clearCache();
      
      return data?.createPost;
    } catch (error) {
      console.error('Error creating post:', error);
      throw new Error('Failed to create post');
    }
  }

  /**
   * Delete a post
   * @param {string} postId - Post ID to delete
   * @returns {Promise<boolean>} - Success status
   */
  async deletePost(postId) {
    try {
      await graphqlClient.mutate({
        mutation: DELETE_POST,
        variables: { id: postId }
      });

      // Clear timeline cache since we deleted a post
      this.clearCache();
      
      return true;
    } catch (error) {
      console.error('Error deleting post:', error);
      throw new Error('Failed to delete post');
    }
  }

  /**
   * Follow a user
   * @param {string} userId - User ID to follow
   * @returns {Promise<Object>} - Relationship object
   */
  async followUser(userId) {
    try {
      const { data } = await graphqlClient.mutate({
        mutation: FOLLOW_USER,
        variables: { userId }
      });

      // Clear cache since following status changed
      this.clearCache();
      
      return data?.followUser;
    } catch (error) {
      console.error('Error following user:', error);
      throw new Error('Failed to follow user');
    }
  }

  /**
   * Unfollow a user
   * @param {string} userId - User ID to unfollow
   * @returns {Promise<boolean>} - Success status
   */
  async unfollowUser(userId) {
    try {
      await graphqlClient.mutate({
        mutation: UNFOLLOW_USER,
        variables: { userId }
      });

      // Clear cache since following status changed
      this.clearCache();
      
      return true;
    } catch (error) {
      console.error('Error unfollowing user:', error);
      throw new Error('Failed to unfollow user');
    }
  }

  /**
   * Get followers for a user
   * @param {string} userId - User ID
   * @returns {Promise<Array>} - List of followers
   */
  async getFollowers(userId) {
    const cacheKey = `followers_${userId}`;
    
    return this.getCachedData(cacheKey, async () => {
      try {
        const { data } = await graphqlClient.query({
          query: GET_FOLLOWERS,
          variables: { userId },
          fetchPolicy: 'cache-first'
        });

        return data?.followers || [];
      } catch (error) {
        console.error('Error fetching followers:', error);
        throw new Error('Failed to load followers');
      }
    });
  }

  /**
   * Get users that a user is following
   * @param {string} userId - User ID
   * @returns {Promise<Array>} - List of following relationships
   */
  async getFollowing(userId) {
    const cacheKey = `following_${userId}`;
    
    return this.getCachedData(cacheKey, async () => {
      try {
        const { data } = await graphqlClient.query({
          query: GET_FOLLOWING,
          variables: { userId },
          fetchPolicy: 'cache-first'
        });

        return data?.following || [];
      } catch (error) {
        console.error('Error fetching following:', error);
        throw new Error('Failed to load following');
      }
    });
  }

  /**
   * Search users
   * @param {string} query - Search query
   * @param {number} limit - Number of results to return
   * @returns {Promise<Array>} - List of matching users
   */
  async searchUsers(query, limit = 10) {
    const cacheKey = `search_${query}_${limit}`;
    
    return this.getCachedData(cacheKey, async () => {
      try {
        const { data } = await graphqlClient.query({
          query: SEARCH_USERS,
          variables: { query, limit },
          fetchPolicy: 'cache-first'
        });

        return data?.searchUsers || [];
      } catch (error) {
        console.error('Error searching users:', error);
        throw new Error('Failed to search users');
      }
    });
  }

  /**
   * Toggle like on a post
   * @param {string} postId - Post ID
   * @param {boolean} isLiked - Current like status
   * @returns {Promise<Object>} - Updated like info
   */
  async toggleLike(postId, isLiked) {
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

      // Clear cache since like status changed
      this.clearCache();
      
      return { success: true };
    } catch (error) {
      console.error('Error toggling like:', error);
      throw new Error('Failed to toggle like');
    }
  }

  /**
   * Create a comment on a post
   * @param {string} postId - Post ID
   * @param {string} content - Comment content
   * @returns {Promise<Object>} - Created comment
   */
  async createComment(postId, content) {
    try {
      const { data } = await graphqlClient.mutate({
        mutation: CREATE_COMMENT,
        variables: { postId, content }
      });

      // Clear cache since comments changed
      this.clearCache();
      
      return data?.createComment;
    } catch (error) {
      console.error('Error creating comment:', error);
      throw new Error('Failed to create comment');
    }
  }

  /**
   * Delete a comment
   * @param {string} commentId - Comment ID
   * @returns {Promise<boolean>} - Success status
   */
  async deleteComment(commentId) {
    try {
      await graphqlClient.mutate({
        mutation: DELETE_COMMENT,
        variables: { id: commentId }
      });

      // Clear cache since comments changed
      this.clearCache();
      
      return true;
    } catch (error) {
      console.error('Error deleting comment:', error);
      throw new Error('Failed to delete comment');
    }
  }
}