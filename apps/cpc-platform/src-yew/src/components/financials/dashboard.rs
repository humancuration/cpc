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
            let mut new_scenario = (*scenario).clone();
            new_scenario.name = input.value();
            scenario.set(new_scenario);
        })
    };

    // Callback to update forecast horizon
    let on_horizon_change = {
        let scenario = scenario.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
            if let Ok(months) = input.value().parse::<u32>() {
                let mut new_scenario = (*scenario).clone();
                new_scenario.forecast_horizon_months = months;
                scenario.set(new_scenario);
            }
        })
    };

    // Callback to add revenue item
    let add_revenue_item = {
        let scenario = scenario.clone();
        Callback::from(move |_| {
            let mut new_scenario = (*scenario).clone();
            new_scenario.initial_statement.revenue_items.push(RevenueItem {
                id: Uuid::new_v4().to_string(),
                name: "New Revenue".to_string(),
                amount: 0.0,
                growth_rate_monthly: 0.0,
            });
            scenario.set(new_scenario);
        })
    };

    // Callback to add expense item
    let add_expense_item = {
        let scenario = scenario.clone();
        Callback::from(move |_| {
            let mut new_scenario = (*scenario).clone();
            new_scenario.initial_statement.expense_items.push(ExpenseItem {
                id: Uuid::new_v4().to_string(),
                name: "New Expense".to_string(),
                amount: 0.0,
                is_fixed: false,
            });
            scenario.set(new_scenario);
        })
    };

    // Callback to update revenue item
    let update_revenue_item = {
        let scenario = scenario.clone();
        Callback::from(move |(index, field, value): (usize, &'static str, String)| {
            let mut new_scenario = (*scenario).clone();
            if let Some(item) = new_scenario.initial_statement.revenue_items.get_mut(index) {
                match field {
                    "name" => item.name = value,
                    "amount" => {
                        if let Ok(val) = value.parse::<f64>() {
                            item.amount = val;
                        }
                    }
                    "growth_rate" => {
                        if let Ok(val) = value.parse::<f64>() {
                            item.growth_rate_monthly = val / 100.0;
                        }
                    }
                    _ => {}
                }
            }
            scenario.set(new_scenario);
        })
    };

    // Callback to update expense item
    let update_expense_item = {
        let scenario = scenario.clone();
        Callback::from(move |(index, field, value): (usize, &'static str, String)| {
            let mut new_scenario = (*scenario).clone();
            if let Some(item) = new_scenario.initial_statement.expense_items.get_mut(index) {
                match field {
                    "name" => item.name = value,
                    "amount" => {
                        if let Ok(val) = value.parse::<f64>() {
                            item.amount = val;
                        }
                    }
                    "is_fixed" => {
                        item.is_fixed = value == "true";
                    }
                    _ => {}
                }
            }
            scenario.set(new_scenario);
        })
    };

    // Callback to remove revenue item
    let remove_revenue_item = {
        let scenario = scenario.clone();
        Callback::from(move |index: usize| {
            let mut new_scenario = (*scenario).clone();
            new_scenario.initial_statement.revenue_items.remove(index);
            scenario.set(new_scenario);
        })
    };

    // Callback to remove expense item
    let remove_expense_item = {
        let scenario = scenario.clone();
        Callback::from(move |index: usize| {
            let mut new_scenario = (*scenario).clone();
            new_scenario.initial_statement.expense_items.remove(index);
            scenario.set(new_scenario);
        })
    };

    // Callback to run forecast
    let run_forecast = {
        let scenario = scenario.clone();
        let state = state.clone();
        Callback::from(move |_| {
            let scenario = (*scenario).clone();
            state.set(DashboardState::Loading);
            
            spawn_local(async move {
                // Simulate API call - in real implementation, use GraphQL client
                // For now, we'll simulate a successful forecast
                let result = ForecastResult {
                    scenario_id: scenario.id.clone(),
                    monthly_projections: generate_sample_projections(&scenario),
                };
                
                state.set(DashboardState::ForecastComplete(result));
            });
        })
    };

    // Generate sample projections for demo
    fn generate_sample_projections(scenario: &ForecastScenario) -> Vec<ProjectedMonth> {
        let mut projections = Vec::new();
        let mut total_revenue = scenario.initial_statement.revenue_items.iter()
            .map(|r| r.amount)
            .sum::<f64>();
        let mut total_expenses = scenario.initial_statement.expense_items.iter()
            .map(|e| e.amount)
            .sum::<f64>();
        
        for month in 1..=scenario.forecast_horizon_months {
            let year = 2024 + ((month - 1) / 12) as i32;
            let month_num = ((month - 1) % 12) + 1;
            
            // Apply growth rates
            let revenue_growth = scenario.initial_statement.revenue_items.iter()
                .map(|r| r.amount * (1.0 + r.growth_rate_monthly).powi(month as i32))
                .sum::<f64>();
            
            let expense_growth = scenario.initial_statement.expense_items.iter()
                .map(|e| {
                    if e.is_fixed {
                        e.amount
                    } else {
                        e.amount * (1.0 + 0.01).powi(month as i32) // 1% monthly growth for variable
                    }
                })
                .sum::<f64>();
            
            projections.push(ProjectedMonth {
                month: month_num,
                year,
                total_revenue: revenue_growth,
                total_expenses: expense_growth,
                profit_loss: revenue_growth - expense_growth,
            });
        }
        
        projections
    }

    html! {
        <div class="financial-dashboard">
            <div class="dashboard-header">
                <h1>{"Financial Forecasting Dashboard"}</h1>
                <p>{"Create and analyze financial projections for your cooperative"}</p>
            </div>

            <div class="dashboard-content">
                <div class="scenario-form">
                    <h2>{"Forecast Scenario"}</h2>
                    
                    <div class="form-group">
                        <label>{"Scenario Name"}</label>
                        <input 
                            type="text" 
                            value={scenario.name.clone()}
                            oninput={on_name_change}
                            placeholder="e.g., Optimistic Growth"
                        />
                    </div>

                    <div class="form-group">
                        <label>{"Forecast Horizon (months)"}</label>
                        <input 
                            type="number" 
                            value={scenario.forecast_horizon_months.to_string()}
                            oninput={on_horizon_change}
                            min="1"
                            max="60"
                        />
                    </div>

                    <div class="form-section">
                        <h3>{"Revenue Items"}</h3>
                        <button onclick={add_revenue_item} class="btn-add">
                            {"Add Revenue Item"}
                        </button>
                        
                        <div class="items-list">
                            {for scenario.initial_statement.revenue_items.iter().enumerate().map(|(i, item)| {
                                html! {
                                    <div class="item-row" key={item.id.clone()}>
                                        <input 
                                            type="text" 
                                            value={item.name.clone()}
                                            placeholder="Item name"
                                            oninput={update_revenue_item.reform(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                (i, "name", input.value())
                                            })}
                                        />
                                        <input 
                                            type="number" 
                                            value={item.amount.to_string()}
                                            placeholder="Amount"
                                            oninput={update_revenue_item.reform(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                (i, "amount", input.value())
                                            })}
                                        />
                                        <input 
                                            type="number" 
                                            value={(item.growth_rate_monthly * 100.0).to_string()}
                                            placeholder="Growth %"
                                            step="0.1"
                                            oninput={update_revenue_item.reform(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                (i, "growth_rate", input.value())
                                            })}
                                        />
                                        <button onclick={remove_revenue_item.reform(move |_| i)} class="btn-remove">
                                            {"Remove"}
                                        </button>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    <div class="form-section">
                        <h3>{"Expense Items"}</h3>
                        <button onclick={add_expense_item} class="btn-add">
                            {"Add Expense Item"}
                        </button>
                        
                        <div class="items-list">
                            {for scenario.initial_statement.expense_items.iter().enumerate().map(|(i, item)| {
                                html! {
                                    <div class="item-row" key={item.id.clone()}>
                                        <input 
                                            type="text" 
                                            value={item.name.clone()}
                                            placeholder="Item name"
                                            oninput={update_expense_item.reform(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                (i, "name", input.value())
                                            })}
                                        />
                                        <input 
                                            type="number" 
                                            value={item.amount.to_string()}
                                            placeholder="Amount"
                                            oninput={update_expense_item.reform(move |e: InputEvent| {
                                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                                (i, "amount", input.value())
                                            })}
                                        />
                                        <select 
                                            value={item.is_fixed.to_string()}
                                            onchange={update_expense_item.reform(move |e: Event| {
                                                let select = e.target_unchecked_into::<web_sys::HtmlSelectElement>();
                                                (i, "is_fixed", select.value())
                                            })}
                                        >
                                            <option value="true">{"Fixed"}</option>
                                            <option value="false">{"Variable"}</option>
                                        </select>
                                        <button onclick={remove_expense_item.reform(move |_| i)} class="btn-remove">
                                            {"Remove"}
                                        </button>
                                    </div>
                                }
                            })}
                        </div>
                    </div>

                    <button onclick={run_forecast} class="btn-primary" disabled={*state == DashboardState::Loading}>
                        { if *state == DashboardState::Loading {
                            "Running Forecast..."
                        } else {
                            "Run Forecast"
                        }}
                    </button>
                </div>

                <div class="forecast-results">
                    {match &*state {
                        DashboardState::Idle => html! {
                            <div class="empty-state">
                                <h3>{"No Forecast Yet"}</h3>
                                <p>{"Configure your scenario above and run a forecast to see projections"}</p>
                            </div>
                        },
                        DashboardState::Loading => html! {
                            <div class="loading-state">
                                <div class="spinner"></div>
                                <p>{"Calculating financial projections..."}</p>
                            </div>
                        },
                        DashboardState::Error(err) => html! {
                            <div class="error-state">
                                <h3>{"Forecast Error"}</h3>
                                <p>{err}</p>
                            </div>
                        },
                        DashboardState::ForecastComplete(result) => html! {
                            <ForecastChart data={result.clone()} />
                        },
                        _ => html! {},
                    }}
                </div>
            </div>
        </div>
    }
}