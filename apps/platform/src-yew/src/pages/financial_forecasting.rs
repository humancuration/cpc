use yew::prelude::*;
use crate::components::financials::FinancialDashboard;

#[function_component(FinancialForecastingPage)]
pub fn financial_forecasting_page() -> Html {
    html! {
        <div class="page-container">
            <FinancialDashboard />
        </div>
    }
}