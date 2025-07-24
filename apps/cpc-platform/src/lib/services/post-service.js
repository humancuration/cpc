import { graphqlClient } from '$lib/graphql/client';
import { CREATE_POST, LIKE_POST, UNLIKE_POST, DELETE_POST } from '$lib/graphql/mutations';
import { GET_POSTS } from '$lib/graphql/queries';

export async function createPost({ content, visibility, cooperativeId, mediaIds = [] }) {
  try {
    const response = await graphqlClient.mutate({
      mutation: CREATE_POST,
      variables: {
        input: {
          content: content.trim(),
          visibility,
          cooperativeId,
          mediaIds
        }
      }
    });
    
    if (!response.data?.createPost) {
      throw new Error('Failed to create post: No response data');
    }
    
    return response.data.createPost;
  } catch (error) {
    console.error('Error creating post:', error);
    throw new Error(error.message || 'Failed to create post');
  }
}

export async function getPosts({ cooperativeId, limit = 20, offset = 0 }) {
  const response = await graphqlClient.query({
    query: GET_POSTS,
    variables: {
      cooperativeId,
      limit,
      offset
    }
  });
  
  return response.data.posts;
}

export async function likePost(postId) {
  const response = await graphqlClient.mutate({
    mutation: LIKE_POST,
    variables: { postId }
  });
  
  return response.data.likePost;
}

export async function unlikePost(postId) {
  const response = await graphqlClient.mutate({
    mutation: UNLIKE_POST,
    variables: { postId }
  });
  
  return response.data.unlikePost;
}

export async function deletePost(postId) {
  const response = await graphqlClient.mutate({
    mutation: DELETE_POST,
    variables: { postId }
  });
  
  return response.data.deletePost;
}