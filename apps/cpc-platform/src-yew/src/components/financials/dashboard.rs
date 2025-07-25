use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use uuid::Uuid;
use crate::types::financial_forecasting::*;
use crate::components::financials::ForecastChart;

#[derive(Debug, Clone, PartialEq)]
enum DashboardState {
    Idle,
    Loading,
    Error(String),
    ForecastSubmitted(String),
    ForecastComplete(ForecastResult),
}

#[function_component(FinancialDashboard)]
pub fn financial_dashboard() -> Html {
    let scenario = use_state(|| create_default_scenario());
    let state = use_state(|| DashboardState::Idle);
    
    // Helper function to create default scenario
    fn create_default_scenario() -> ForecastScenario {
        ForecastScenario {
            id: Uuid::new_v4().to_string(),
            name: "New Scenario".to_string(),
            description: "Financial forecast scenario".to_string(),
            initial_statement: FinancialStatement {
                id: Uuid::new_v4().to_string(),
                start_date: "2024-01-01".to_string(),
                end_date: "2024-12-31".to_string(),
                revenue_items: vec![
                    RevenueItem {
                        id: Uuid::new_v4().to_string(),
                        name: "Product Sales".to_string(),
                        amount: 50000.0,
                        growth_rate_monthly: 0.02,
                    },
                ],
                expense_items: vec![
                    ExpenseItem {
                        id: Uuid::new_v4().to_string(),
                        name: "Rent".to_string(),
                        amount: 2000.0,
                        is_fixed: true,
                    },
                    ExpenseItem {
                        id: Uuid::new_v4().to_string(),
                        name: "Utilities".to_string(),
                        amount: 500.0,
                        is_fixed: true,
                    },
                ],
            },
            forecast_horizon_months: 12,
            assumptions: vec![],
        }
    }

    // Callback to update scenario name
    let on_name_change = {
        let scenario = scenario.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
