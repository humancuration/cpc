import { gql } from '@apollo/client/core';

export const CREATE_POST = gql`
  mutation CreatePost($input: CreatePostInput!) {
    createPost(input: $input) {
      id
      content
      visibility
      createdAt
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
        description
      }
    }
  }
`;

export const LIKE_POST = gql`
  mutation LikePost($postId: ID!) {
    likePost(postId: $postId) {
      id
      likes
    }
  }
`;

export const UNLIKE_POST = gql`
  mutation UnlikePost($postId: ID!) {
    unlikePost(postId: $postId) {
      id
      likes
    }
  }
`;

export const DELETE_POST = gql`
  mutation DeletePost($postId: ID!) {
    deletePost(postId: $postId) {
      id
    }
  }
`;

export const UPLOAD_MEDIA = gql`
  mutation UploadMedia($files: [Upload!]!) {
    uploadMedia(files: $files) {
      id
      url
      type
      thumbnailUrl
      description
    }
  }
`;

// BI-related mutations
export const GENERATE_IMPACT_REPORT = gql`
  mutation GenerateImpactReport($userId: ID!) {
    generateImpactReport(userId: $userId) {
      jobId
      status
      progress
      message
      estimatedCompletion
    }
  }
`;