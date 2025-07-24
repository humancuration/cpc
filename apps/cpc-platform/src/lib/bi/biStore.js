import { writable, derived } from 'svelte/store';
import { graphqlClient } from '$lib/graphql/client';
import { GET_IMPACT_REPORT } from '$lib/graphql/queries';
import { IMPACT_REPORT_SUBSCRIPTION } from '$lib/graphql/subscriptions';
import { GENERATE_IMPACT_REPORT } from '$lib/graphql/mutations';

// Main BI store
export const biStore = writable({
  impactReport: null,
  loading: false,
  error: null,
  processingStatus: null
});

// Derived store for computed values
export const totalImpact = derived(biStore, $biStore => 
  $biStore.impactReport?.totalImpact || 0
);

export const impactBreakdown = derived(biStore, $biStore => 
  $biStore.impactReport?.breakdown || []
);

export const impactDistribution = derived(biStore, $biStore => 
  $biStore.impactReport?.distribution || []
);

export const impactTimeline = derived(biStore, $biStore => 
  $biStore.impactReport?.timeline || []
);

// Actions
export async function fetchImpactReport(userId) {
  biStore.update(state => ({ ...state, loading: true, error: null }));
  
  try {
    const { data } = await graphqlClient.query({
      query: GET_IMPACT_REPORT,
      variables: { userId },
      fetchPolicy: 'network-only'
    });
    
    biStore.update(state => ({
      ...state,
      impactReport: data.getImpactReport,
      loading: false,
      error: null
    }));
    
    return data.getImpactReport;
  } catch (error) {
    console.error('Error fetching impact report:', error);
    biStore.update(state => ({
      ...state,
      loading: false,
      error: error.message
    }));
    throw error;
  }
}

export async function generateImpactReport(userId) {
  biStore.update(state => ({ ...state, loading: true, error: null }));
  
  try {
    const { data } = await graphqlClient.mutate({
      mutation: GENERATE_IMPACT_REPORT,
      variables: { userId }
    });
    
    biStore.update(state => ({
      ...state,
      processingStatus: data.generateImpactReport,
      loading: false
    }));
    
    return data.generateImpactReport;
  } catch (error) {
    console.error('Error generating impact report:', error);
    biStore.update(state => ({
      ...state,
      loading: false,
      error: error.message
    }));
    throw error;
  }
}

// Subscription management
let subscriptionUnsubscribe = null;

export function subscribeToImpactReport(userId) {
  if (subscriptionUnsubscribe) {
    subscriptionUnsubscribe();
  }
  
  const subscription = graphqlClient.subscribe({
    query: IMPACT_REPORT_SUBSCRIPTION,
    variables: { userId }
  }).subscribe({
    next: ({ data }) => {
      if (data?.impactReportUpdated) {
        biStore.update(state => ({
          ...state,
          impactReport: data.impactReportUpdated,
          loading: false
        }));
      }
    },
    error: (error) => {
      console.error('Impact report subscription error:', error);
      biStore.update(state => ({
        ...state,
        error: error.message
      }));
    }
  });
  
  subscriptionUnsubscribe = () => subscription.unsubscribe();
  
  return subscriptionUnsubscribe;
}

export function unsubscribeFromImpactReport() {
  if (subscriptionUnsubscribe) {
    subscriptionUnsubscribe();
    subscriptionUnsubscribe = null;
  }
}