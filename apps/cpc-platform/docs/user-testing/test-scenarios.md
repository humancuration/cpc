# Financial Forecasting Module Test Scenarios

## Basic Forecast Creation
1. Navigate to Financial Forecasting module
2. Click "Create New Forecast Job" button.
3. Enter scenario name: "Baseline Projection"
4. Set parameters:
   - Growth Rate: 1.15
   - Cost Increase: 1.07
5. Submit the job.
6. Verify a `taskId` is returned and the UI enters a "processing" state.
7. Verify a GraphQL subscription for the `taskId` is initiated.
8. After the job completes, verify the `ForecastChart` displays the new projection.
9. Verify the forecast results are persisted in the database, linked to the job.

![ScenarioEditor](lib/business-tools/financial_forecasting/SensitivityPanel.svelte)

## Scenario Comparison
1. Create 2 scenarios: "Optimistic" (growth=1.25) and "Pessimistic" (growth=1.05)
2. In ComparisonView component:
   - Select both scenarios
   - Set date range: Next 6 months
3. Verify variance percentages are calculated correctly:
   ```svelte
   // ComparisonView.svelte
   <span class:positive={variance > 0} class:negative={variance < 0}>
     ${scenario.netProfit} ({variance}%)
   </span>
   ```
4. Check visual indicators show appropriate colors (green/red)
5. Verify key changes are listed for each scenario

![ComparisonView](lib/business-tools/financial_forecasting/ComparisonView.svelte)

## Sensitivity Analysis
1. Open existing scenario in FinancialForecastingDashboard
2. In `SensitivityPanel`, adjust parameters.
3. Click "Run Sensitivity Analysis Job".
4. Verify a `taskId` is returned and the UI shows a loading indicator for the analysis.
5. Verify a notification is received via GraphQL subscription: "Sensitivity analysis completed".
6. Verify the `ForecastChart` updates with the new projections.
7. Save the modified scenario.

## Training Session Scheduling
1. In FinancialForecastingDashboard, select forecast scenario
2. Click "Schedule Training Session".
3. Set date/time.
4. Select attendees.
5. Verify the scenario is attached to the calendar event.
6. Confirm notifications are sent to attendees (this test may be manual pending full notification system implementation).