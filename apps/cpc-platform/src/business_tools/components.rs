use yew::prelude::*;
use crate::business_tools::hooks::{use_business_data, use_impact_categories, use_time_periods, use_filters};

#[function_component(ExportButton)]
pub fn export_button() -> Html {
    let onclick = Callback::from(|_| {
        web_sys::console::log_1(&"Exporting business data...".into());
    });

    html! {
        <button class="action-button" {onclick}>
            { "Export Data" }
        </button>
    }
}

#[function_component(FilterPanel)]
pub fn filter_panel() -> Html {
    let (filters, update_filters) = use_filters();
    let categories = use_impact_categories();
    let time_periods = use_time_periods();

    let on_category_change = {
        let update_filters = update_filters.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
            update_filters.emit(FilterState {
                category: if value.is_empty() { None } else { Some(value) },
                ..filters.clone()
            });
        })
    };

    let on_period_change = {
        let update_filters = update_filters.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
            update_filters.emit(FilterState {
                time_period: value,
                ..filters.clone()
            });
        })
    };

    html! {
        <div class="filters-section">
            <h3>{ "Filters" }</h3>
            <div class="filters-grid">
                <div class="filter-group">
                    <label>{ "Impact Category" }</label>
                    <select onchange={on_category_change}>
                        <option value="">{ "All Categories" }</option>
                        {for categories.iter().map(|cat| {
                            html! {
                                <option value={cat.clone()} selected={filters.category.as_ref() == Some(cat)}>
                                    { cat }
                                </option>
                            }
                        })}
                    </select>
                </div>
                
                <div class="filter-group">
                    <label>{ "Time Period" }</label>
                    <select onchange={on_period_change}>
                        {for time_periods.iter().map(|period| {
                            html! {
                                <option value={period.clone()} selected={filters.time_period == *period}>
                                    { period }
                                </option>
                            }
                        })}
                    </select>
                </div>
            </div>
        </div>
    }
}

#[function_component(DataTable)]
pub fn data_table() -> Html {
    let business_data = use_business_data();
    
    html! {
        <div class="dashboard-card">
            <h3>{ "Monthly Performance" }</h3>
            <table class="data-table">
                <thead>
                    <tr>
                        <th>{ "Month" }</th>
                        <th>{ "Revenue" }</th>
                        <th>{ "Impact Score" }</th>
                        <th>{ "Growth" }</th>
                    </tr>
                </thead>
                <tbody>
                    {for business_data.monthly_metrics.iter().map(|(month, revenue)| {
                        let impact_score = 8.5 + (revenue / 10000.0) * 0.1;
                        let growth = if *revenue > 12000.0 { "positive" } else { "neutral" };
                        
                        html! {
                            <tr>
                                <td>{ month }</td>
                                <td>{ format!("${:.2}", revenue) }</td>
                                <td>{ format!("{:.1}", impact_score) }</td>
                                <td>
                                    <span class={format!("status-badge {}", growth)}>
                                        { if *revenue > 12000.0 { "Growing" } else { "Stable" } }
                                    </span>
                                </td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}

#[function_component(ChartPlaceholder)]
pub fn chart_placeholder() -> Html {
    html! {
        <div class="chart-card">
            <h4>{ "Impact Distribution" }</h4>
            <div style="height: 300px; display: flex; align-items: center; justify-content: center; background: #f9fafb; border-radius: 8px;">
                <p style="color: #6b7280;">{ "Chart visualization will be displayed here" }</p>
            </div>
        </div>
    }
}

#[function_component(QuickStats)]
pub fn quick_stats() -> Html {
    let business_data = use_business_data();
    
    html! {
        <div class="charts-container">
            <div class="chart-card">
                <h4>{ "Revenue Trend" }</h4>
                <div style="padding: 1rem;">
                    <p style="font-size: 2rem; font-weight: bold; color: #0066cc;">
                        { format!("${:.0}", business_data.total_revenue) }
                    </p>
                    <p style="color: #10b981; font-size: 0.875rem;">
                        { format!("+{:.1}% growth", business_data.revenue_growth) }
                    </p>
                </div>
            </div>
            
            <div class="chart-card">
                <h4>{ "Impact Score" }</h4>
                <div style="padding: 1rem;">
                    <p style="font-size: 2rem; font-weight: bold; color: #f59e0b;">
                        { format!("{:.1}/10", business_data.impact_score) }
                    </p>
                    <p style="color: #10b981; font-size: 0.875rem;">
                        { format!("+{:.1} improvement", business_data.impact_improvement) }
                    </p>
                </div>
            </div>
        </div>
    }
}

use super::hooks::FilterState;
