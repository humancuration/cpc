// apps/backend/src/graphql/financial_forecasting.rs
use async_graphql::*;
use cpc_core::financial_forecasting::{models, service::FinancialForecastingService};
use futures::Stream;
use async_stream::stream;
use std::sync::Arc;

// Expose core models via GraphQL types.
// We'll automatically derive `SimpleObject` for our result types.
// For input types, we already derived `InputObject` in the models file.
#[SimpleObject]
#[graphql(name = "ForecastResult")]
struct ForecastResultObject {
    scenario_id: String,
    monthly_projections: Vec<ProjectedMonthObject>,
}

impl From<models::ForecastResult> for ForecastResultObject {
    fn from(value: models::ForecastResult) -> Self {
        Self {
            scenario_id: value.scenario_id,
            monthly_projections: value.monthly_projections.into_iter().map(Into::into).collect(),
        }
    }
}

#[SimpleObject]
#[graphql(name = "ProjectedMonth")]
struct ProjectedMonthObject {
    month: u32,
    year: i32,
    total_revenue: f64,
    total_expenses: f64,
    profit_loss: f64,
}

impl From<models::ProjectedMonth> for ProjectedMonthObject {
    fn from(value: models::ProjectedMonth) -> Self {
        Self {
            month: value.month,
            year: value.year,
            total_revenue: value.total_revenue,
            total_expenses: value.total_expenses,
            profit_loss: value.profit_loss,
        }
    }
}


#[derive(Default)]
pub struct FinancialForecastingQueryRoot;

#[Object]
impl FinancialForecastingQueryRoot {
    /// Placeholder for a query to fetch saved forecast scenarios.
    #[graphql(name = "getSavedScenarios")]
    async fn get_saved_scenarios(&self, _ctx: &Context<'_>) -> Result<Vec<models::ForecastScenario>> {
        // In a real implementation, this would fetch from a database or p2panda.
        unimplemented!()
    }
}

#[derive(Default)]
pub struct FinancialForecastingMutationRoot;

#[Object]
impl FinancialForecastingMutationRoot {
    /// Kicks off a financial forecast. The result will be sent via subscription.
    /// The `scenario` input is mapped from the GraphQL input type to our core model.
    #[graphql(name = "runForecastScenario")]
    async fn run_forecast_scenario(&self, ctx: &Context<'_>, scenario: models::ForecastScenario) -> Result<bool> {
        let service = ctx.data_unchecked::<FinancialForecastingService>();
        // The service runs the forecast in a background task.
        service.run_forecast(scenario).await;
        Ok(true)
    }
}

#[derive(Default)]
pub struct FinancialForecastingSubscriptionRoot;

#[Subscription]
impl FinancialForecastingSubscriptionRoot {
    /// Subscribes to the result of a specific forecast scenario.
    #[graphql(name = "forecastResult")]
    async fn forecast_result(&self, ctx: &Context<'_>, scenario_id: String) -> impl Stream<Item = Arc<ForecastResultObject>> {
        let service = ctx.data_unchecked::<FinancialForecastingService>();
        let mut rx = service.get_result_stream();

        stream! {
            while let Ok(result_arc) = rx.recv().await {
                if result_arc.scenario_id == scenario_id {
                    let result_obj: ForecastResultObject = (*result_arc).clone().into();
                    yield Arc::new(result_obj);
                }
            }
        }
    }
}