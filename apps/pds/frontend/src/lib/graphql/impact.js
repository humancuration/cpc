import { gql } from '@apollo/client';

export const GET_IMPACT_REPORT = gql`
  query GetImpactReport($userId: ID!) {
    impactReport(userId: $userId) {
      userId
      totalImpact
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

export const GENERATE_IMPACT_REPORT = gql`
  mutation GenerateImpactReport($userId: ID!) {
    generateImpactReport(userId: $userId) {
      success
      message
      reportId
    }
  }
`;