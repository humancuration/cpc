import { gql } from '@apollo/client/core';

export const mediaProcessingSubscription = gql`
  subscription MediaProcessingStatus($mediaId: ID!) {
    mediaProcessingStatus(mediaId: $mediaId) {
      id
      status
      progress
      error
      metadata {
        type
        size
        url
        thumbnailUrl
      }
    }
  }
`;

export const postMediaSubscription = gql`
  subscription PostMediaStatus($postId: ID!) {
    postMediaStatus(postId: $postId) {
      id
      media {
        id
        url
        type
        processingStatus
        progress
      }
    }
  }
`;

// BI-related subscriptions
export const IMPACT_REPORT_SUBSCRIPTION = gql`
  subscription ImpactReportUpdated($userId: ID!) {
    impactReportUpdated(userId: $userId) {
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