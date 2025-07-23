import { writable, derived } from 'svelte/store';
import { graphqlClient } from '../lib/graphql/client';
import { GET_IMPACT_REPORT, GENERATE_IMPACT_REPORT } from '../lib/graphql/impact';

// Mock data for development
const mockImpactReport = {
  userId: 'test-user-123',
  totalImpact: 85.5,
  breakdown: [
    {
      category: 'Environmental',
      amount: 45.2,
      itemName: 'Carbon Footprint Reduction',
      contribution: 52.8,
      impactScore: 8.5
    },
    {
      category: 'Social',
      amount: 25.3,
      itemName: 'Community Engagement',
      contribution: 29.6,
      impactScore: 7.2
    },
    {
      category: 'Economic',
      amount: 15.0,
      itemName: 'Local Economic Support',
      contribution: 17.6,
      impactScore: 6.8
    }
  ],
  distribution: [
    { category: 'Environmental', weight: 0.528 },
    { category: 'Social', weight: 0.296 },
    { category: 'Economic', weight: 0.176 }
  ],
  timeline: [
    {
      date: '2024-01-15',
      description: 'Started using eco-friendly products',
      impactValue: 15.2,
      timestamp: 1705276800000,
      score: 7.5
    },
    {
      date: '2024-03-20',
      description: 'Joined local sustainability group',
      impactValue: 25.3,
      timestamp: 1710892800000,
      score: 8.2
    },
    {
      date: '2024-06-10',
      description: 'Implemented recycling program',
      impactValue: 45.0,
      timestamp: 1717977600000,
      score: 9.1
    }
  ]
};

// Store for impact report data
export const impactReport = writable(null);
export const isLoading = writable(false);
export const error = writable(null);

// Derived stores for computed values
export const totalImpact = derived(impactReport, $report => $report?.totalImpact || 0);
export const breakdownItems = derived(impactReport, $report => $report?.breakdown || []);
export const distributionData = derived(impactReport, $report => $report?.distribution || []);
export const timelinePoints = derived(impactReport, $report => $report?.timeline || []);

// Actions
export async function fetchImpactReport(userId) {
  if (!userId) {
    // Use mock data for development
    impactReport.set(mockImpactReport);
    return;
  }

  isLoading.set(true);
  error.set(null);

  try {
    const response = await graphqlClient.request(GET_IMPACT_REPORT, { userId });
    impactReport.set(response.impactReport);
  } catch (err) {
    console.error('Failed to fetch impact report:', err);
    error.set(err.message || 'Failed to load impact report');
    // Fallback to mock data for development
    impactReport.set(mockImpactReport);
  } finally {
    isLoading.set(false);
  }
}

export async function generateImpactReport(userId) {
  if (!userId) return;

  isLoading.set(true);
  error.set(null);

  try {
    const response = await graphqlClient.request(GENERATE_IMPACT_REPORT, { userId });
    if (response.generateImpactReport.success) {
      await fetchImpactReport(userId);
    }
  } catch (err) {
    console.error('Failed to generate impact report:', err);
    error.set(err.message || 'Failed to generate impact report');
  } finally {
    isLoading.set(false);
  }
}

export function clearImpactReport() {
  impactReport.set(null);
  error.set(null);
  isLoading.set(false);
}