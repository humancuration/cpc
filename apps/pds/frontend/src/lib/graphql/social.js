import { gql } from '@apollo/client';

// Post mutations
export const CREATE_POST = gql`
  mutation CreatePost($content: String!, $visibility: Visibility!) {
    createPost(content: $content, visibility: $visibility) {
      id
      content
      authorId
      visibility
      createdAt
      updatedAt
    }
  }
`;

export const UPDATE_POST = gql`
  mutation UpdatePost($id: UUID!, $content: String!, $visibility: Visibility!) {
    updatePost(id: $id, content: $content, visibility: $visibility) {
      id
      content
      visibility
      updatedAt
    }
  }
`;

export const DELETE_POST = gql`
  mutation DeletePost($id: UUID!) {
    deletePost(id: $id)
  }
`;

// Post queries
export const GET_POSTS_BY_USER = gql`
  query GetPostsByUser($userId: UUID!, $limit: Int, $offset: Int) {
    postsByUser(userId: $userId, limit: $limit, offset: $offset) {
      id
      content
      authorId
      visibility
      createdAt
      updatedAt
      author {
        id
        username
        displayName
        avatarUrl
      }
      likeCount
      commentCount
      shareCount
      likedByCurrentUser
      mediaUrls
    }
  }
`;

export const GET_TIMELINE = gql`
  query GetTimeline($limit: Int, $offset: Int) {
    timeline(limit: $limit, offset: $offset) {
      id
      content
      authorId
      visibility
      createdAt
      updatedAt
      author {
        id
        username
        displayName
        avatarUrl
      }
      likeCount
      commentCount
      shareCount
      likedByCurrentUser
      mediaUrls
    }
  }
`;

// Relationship mutations
export const FOLLOW_USER = gql`
  mutation FollowUser($userId: UUID!) {
    followUser(userId: $userId) {
      id
      followerId
      followingId
      createdAt
    }
  }
`;

export const UNFOLLOW_USER = gql`
  mutation UnfollowUser($userId: UUID!) {
    unfollowUser(userId: $userId)
  }
`;

// Relationship queries
export const GET_FOLLOWERS = gql`
  query GetFollowers($userId: UUID!) {
    followers(userId: $userId) {
      id
      follower {
        id
        username
        displayName
        avatarUrl
      }
      createdAt
    }
  }
`;

export const GET_FOLLOWING = gql`
  query GetFollowing($userId: UUID!) {
    following(userId: $userId) {
      id
      following {
        id
        username
        displayName
        avatarUrl
      }
      createdAt
    }
  }
`;

// Post detail queries
export const GET_POST = gql`
  query GetPost($id: UUID!) {
    post(id: $id) {
      id
      content
      authorId
      visibility
      createdAt
      updatedAt
      author {
        id
        username
        displayName
        avatarUrl
      }
      media {
        id
        url
        type
        thumbnailUrl
      }
      likeCount
      commentCount
      isLikedByCurrentUser
      comments {
        id
        content
        createdAt
        author {
          id
          username
          displayName
          avatarUrl
        }
      }
    }
  }
`;

// Comment mutations
export const CREATE_COMMENT = gql`
  mutation CreateComment($postId: UUID!, $content: String!) {
    createComment(postId: $postId, content: $content) {
      id
      content
      createdAt
      author {
        id
        username
        displayName
        avatarUrl
      }
    }
  }
`;

export const DELETE_COMMENT = gql`
  mutation DeleteComment($id: UUID!) {
    deleteComment(id: $id)
  }
`;

// Like mutations
export const LIKE_POST = gql`
  mutation LikePost($postId: UUID!) {
    likePost(postId: $postId) {
      id
      postId
      userId
      createdAt
    }
  }
`;

export const UNLIKE_POST = gql`
  mutation UnlikePost($postId: UUID!) {
    unlikePost(postId: $postId)
  }
`;

// User search
export const SEARCH_USERS = gql`
  query SearchUsers($query: String!, $limit: Int) {
    searchUsers(query: $query, limit: $limit) {
      id
      username
      displayName
      avatarUrl
      bio
    }
  }
`;