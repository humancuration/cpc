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
