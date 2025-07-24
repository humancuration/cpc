import { graphqlClient } from '$lib/graphql/client';
import { GET_FEED_POSTS } from '$lib/graphql/queries';

export async function getFeedPosts({ cooperativeId, limit = 20, offset = 0 }) {
  const response = await graphqlClient.query({
    query: GET_FEED_POSTS,
    variables: {
      cooperativeId,
      limit,
      offset
    }
  });
  
  return response.data.feedPosts;
}

export async function refreshFeed({ cooperativeId }) {
  return getFeedPosts({ cooperativeId, limit: 20, offset: 0 });
}

export async function loadMoreFeed({ cooperativeId, offset, limit = 20 }) {
  return getFeedPosts({ cooperativeId, limit, offset });
}