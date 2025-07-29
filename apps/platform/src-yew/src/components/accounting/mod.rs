use yew::prelude::*;
use yewdux::prelude::*;
use crate::store::{Store, set_dashboard_data};
use crate::services::accounting_service;
use crate::types::{AccountingDashboard, TrendDirection};
use crate::components::common::{MetricCard, Chart, ChartType};
use cpc_core::accounting::Money;

#[function_component(AccountingDashboard)]
pub fn accounting_dashboard() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let dashboard = store.dashboard.clone();
    
    // Fetch data on component mount
    {
        let dispatch = dispatch.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match accounting_service::get_dashboard_data().await {
                        Ok(data) => {
                            dispatch.apply(set_dashboard_data(data));
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Failed to load dashboard: {}", e).into());
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    if dashboard.is_none() {
        return html! {
            <div class="dashboard-loading">
                <div class="spinner"></div>
                <p>{"Loading dashboard..."}</p>
            </div>
        };
    }

    let dashboard_data = dashboard.unwrap();
    
    html! {
        <div class="accounting-dashboard">
            <div class="dashboard-header">
                <h1>{"Accounting Dashboard"}</h1>
                <div class="time-selector">
                    <button class="time-btn active">{"Current"}</button>
                    <button class="time-btn">{"30 Days"}</button>
                    <button class="time-btn">{"90 Days"}</button>
                </div>
            </div>

            <div class="dashboard-grid">
                <div class="metrics-section">
                    <MetricCard 
                        title="Current Ratio"
                        value={dashboard_data.current_ratio}
                        trend={Some(dashboard_data.current_ratio_trend)}
                        icon={Some("📊".to_string())}
                        format={crate::components::common::metric_card::Format::Number}
                    />
                    <MetricCard 
                        title="Quick Ratio"
                        value={dashboard_data.quick_ratio}
                        trend={Some(dashboard_data.quick_ratio_trend)}
                        icon={Some("⚡".to_string())}
                        format={crate::components::common::metric_card::Format::Number}
                    />
                    <MetricCard 
                        title="Cash on Hand"
                        value={dashboard_data.cash_on_hand}
                        trend={Some(dashboard_data.cash_on_hand_trend)}
                        icon={Some("💰".to_string())}
                        format={crate::components::common::metric_card::Format::Currency}
                    />
                    <MetricCard 
                        title="Net Income"
                        value={dashboard_data.net_income}
                        trend={Some(dashboard_data.net_income_trend)}
                        icon={Some("📈".to_string())}
                        format={crate::components::common::metric_card::Format::Currency}
                    />
                </div>

                <div class="charts-section">
                    <div class="chart-card">
                        <h2>{"Income vs Expenses"}</h2>
                        <Chart 
                            chart_type={ChartType::Line}
                            data={dashboard_data.income_expense_data}
                            title="Monthly Income vs Expenses"
                            width={400}
                            height={300}
                        />
                    </div>
                    
                    <div class="chart-card">
                        <h2>{"Revenue by Category"}</h2>
                        <Chart 
                            chart_type={ChartType::Pie}
                            data={dashboard_data.revenue_by_category}
                            title="Revenue Distribution"
                            width={400}
                            height={300}
                        />
                    </div>
                </div>

                <div class="anomaly-section">
                    <h2>{"Anomaly Detection"}</h2>
                    <div class="anomaly-list">
                        {dashboard_data.anomalies.iter().map(|anomaly| {
                            html! {
                                <div class="anomaly-item">
                                    <div class="anomaly-type">{&anomaly.anomaly_type}</div>
                                    <div class="anomaly-description">{&anomaly.description}</div>
                                    <div class="anomaly-severity">{&anomaly.severity}</div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
}