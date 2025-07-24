import { gql } from '@apollo/client/core';

export const GET_POSTS = gql`
  query GetPosts($cooperativeId: ID, $limit: Int, $offset: Int) {
    posts(cooperativeId: $cooperativeId, limit: $limit, offset: $offset) {
      id
      content
      visibility
      createdAt
      updatedAt
      likes
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

export const GET_FEED_POSTS = gql`
  query GetFeedPosts($cooperativeId: ID, $limit: Int, $offset: Int) {
    feedPosts(cooperativeId: $cooperativeId, limit: $limit, offset: $offset) {
      id
      content
      visibility
      createdAt
      updatedAt
      likes
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

export const FEED_SUBSCRIPTION = gql`
  subscription FeedSubscription($cooperativeId: ID) {
    feedUpdate(cooperativeId: $cooperativeId) {
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
      }
    }
  }
`;

// BI-related queries
export const GET_IMPACT_REPORT = gql`
  query GetImpactReport($userId: ID!) {
    getImpactReport(userId: $userId) {
      userId
      totalImpact
      generatedAt
      breakdown {
        category
        amount
        itemName
        contribution
        impactScore
      }
      distribution {
        category
        weight
      }
      timeline {
        date
        description
        impactValue
        timestamp
        score
      }
    }
  }
`;

export const GET_PROCESSING_STATUS = gql`
  query GetProcessingStatus($jobId: ID!) {
    getProcessingStatus(jobId: $jobId) {
      jobId
      status
      progress
      message
      estimatedCompletion
    }
  }
`;