import { writable, derived } from 'svelte/store';
import { GraphQLClient } from 'graphql-request';

// GraphQL queries
const GET_FINANCIAL_FORECAST = `
  query GetFinancialForecast($jobId: UUID!) {
    financialForecast(jobId: $jobId) {
      jobId
      projections {
        year
        income
        expenses
        netCashFlow
        cumulativeCashFlow
        netWorth
      }
      scenarios {
        name
        parameters
        finalNetWorth
      }
      sensitivityAnalysis {
        scenarios {
          name
          parameters
          finalNetWorth
          changePercent
        }
      }
      riskMetrics {
        worstCase
        bestCase
        probabilitySuccess
        averageOutcome
      }
    }
  }
`;

const CREATE_FINANCIAL_FORECAST = `
  mutation CreateFinancialForecast($input: CreateFinancialForecastInput!) {
    createFinancialForecast(input: $input) {
      jobId
      status
    }
  }
`;

const FINANCIAL_FORECAST_SUBSCRIPTION = `
  subscription OnFinancialForecastComplete($jobId: UUID!) {
    financialForecastComplete(jobId: $jobId) {
      jobId
      status
      result {
        ... on FinancialForecastResult {
          jobId
          projections {
            year
            income
            expenses
            netCashFlow
            cumulativeCashFlow
            netWorth
          }
          scenarios {
            name
            parameters
            finalNetWorth
          }
          sensitivityAnalysis {
            scenarios {
              name
              parameters
              finalNetWorth
              changePercent
            }
          }
          riskMetrics {
            worstCase
            bestCase
            probabilitySuccess
            averageOutcome
          }
        }
      }
    }
  }
`;

const UPDATE_DASHBOARD_PREFERENCES = `
  mutation UpdateDashboardPreferences($preferences: JSON!) {
    updateDashboardPreferences(preferences: $preferences) {
      success
      preferences
    }
  }
`;

const GET_DASHBOARD_PREFERENCES = `
  query GetDashboardPreferences {
    me {
      dashboardPreferences
    }
  }
`;

// Store state
const createForecastStore = () => {
  const { subscribe, set, update } = writable({
    currentJob: null,
    forecastResult: null,
    scenarios: [],
    activeScenario: 'base',
    parameters: {
      income_growth_rate: 0.03,
      expense_inflation: 0.025,
      investment_return: 0.08,
      inflation_rate: 0.035,
      retirement_age: 65,
      safety_factor: 1.2
    },
    preferences: {
      chartType: 'line',
      currency: 'USD',
      dateFormat: 'YYYY-MM-DD',
      showGrid: true,
      showLegend: true
    },
    loading: false,
    error: null
  });

  // GraphQL client
  const client = new GraphQLClient('/graphql');

  return {
    subscribe,
    
    // Actions
    async loadPreferences() {
      try {
        const response = await client.request(GET_DASHBOARD_PREFERENCES);
        if (response.me?.dashboardPreferences) {
          update(state => ({
            ...state,
            preferences: { ...state.preferences, ...response.me.dashboardPreferences }
          }));
        }
      } catch (error) {
        console.error('Failed to load preferences:', error);
      }
    },

    async savePreferences(preferences) {
      try {
        const response = await client.request(UPDATE_DASHBOARD_PREFERENCES, {
          preferences: { ...preferences }
        });
        if (response.updateDashboardPreferences.success) {
          update(state => ({
            ...state,
            preferences: { ...state.preferences, ...preferences }
          }));
        }
      } catch (error) {
        console.error('Failed to save preferences:', error);
      }
    },

    async createForecast(input) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const response = await client.request(CREATE_FINANCIAL_FORECAST, {
          input: {
            parameters: input.parameters || {},
            scenarios: input.scenarios || []
          }
        });
        
        const jobId = response.createFinancialForecast.jobId;
        update(state => ({ ...state, currentJob: jobId }));
        
        // Start subscription
        this.subscribeToForecast(jobId);
        
        return jobId;
      } catch (error) {
        update(state => ({ ...state, loading: false, error: error.message }));
        throw error;
      }
    },

    async loadForecast(jobId) {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const response = await client.request(GET_FINANCIAL_FORECAST, { jobId });
        
        if (response.financialForecast) {
          update(state => ({
            ...state,
            forecastResult: response.financialForecast,
            scenarios: response.financialForecast.scenarios || [],
            loading: false
          }));
        }
      } catch (error) {
        update(state => ({ ...state, loading: false, error: error.message }));
      }
    },

    subscribeToForecast(jobId) {
      // WebSocket subscription for real-time updates
      const wsUrl = `ws://localhost:3000/graphql?query=${encodeURIComponent(FINANCIAL_FORECAST_SUBSCRIPTION)}`;
      const ws = new WebSocket(wsUrl);
      
      ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        if (data.data?.financialForecastComplete) {
          const result = data.data.financialForecastComplete;
          
          if (result.status === 'COMPLETED') {
            update(state => ({
              ...state,
              forecastResult: result.result,
              scenarios: result.result.scenarios || [],
              loading: false
            }));
          } else if (result.status === 'FAILED') {
            update(state => ({
              ...state,
              loading: false,
              error: 'Forecast calculation failed'
            }));
          }
        }
      };
      
      ws.onerror = (error) => {
        console.error('WebSocket error:', error);
        update(state => ({ ...state, loading: false, error: 'Connection error' }));
      };
      
      return () => ws.close();
    },

    setActiveScenario(scenario) {
      update(state => ({ ...state, activeScenario: scenario }));
    },

    updateParameters(parameters) {
      update(state => ({
        ...state,
        parameters: { ...state.parameters, ...parameters }
      }));
    },

    clearError() {
      update(state => ({ ...state, error: null }));
    },

    reset() {
      set({
        currentJob: null,
        forecastResult: null,
        scenarios: [],
        activeScenario: 'base',
        parameters: {
          income_growth_rate: 0.03,
          expense_inflation: 0.025,
          investment_return: 0.08,
          inflation_rate: 0.035,
          retirement_age: 65,
          safety_factor: 1.2
        },
        preferences: {
          chartType: 'line',
          currency: 'USD',
          dateFormat: 'YYYY-MM-DD',
          showGrid: true,
          showLegend: true
        },
        loading: false,
        error: null
      });
    }
  };
};

// Derived stores
const forecastStore = createForecastStore();

// Computed values
export const cashFlowData = derived(forecastStore, $store => {
  if (!$store.forecastResult?.projections) return [];
  return $store.forecastResult.projections;
});

export const sensitivityScenarios = derived(forecastStore, $store => {
  if (!$store.forecastResult?.sensitivityAnalysis?.scenarios) return [];
  return $store.forecastResult.sensitivityAnalysis.scenarios;
});

export const riskMetrics = derived(forecastStore, $store => {
  if (!$store.forecastResult?.riskMetrics) return null;
  return $store.forecastResult.riskMetrics;
});

export const baseScenario = derived(forecastStore, $store => {
  if (!$store.forecastResult) return null;
  return {
    name: 'Base Scenario',
    parameters: $store.parameters,
    final_net_worth: $store.forecastResult.projections?.[$store.forecastResult.projections.length - 1]?.netWorth || 0
  };
});

export default forecastStore;