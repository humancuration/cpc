use yew::prelude::*;
use yew_hooks::use_async;
use crate::context::auth::use_auth;
use crate::api::graphql::queries::{GetBusinessMetrics, get_business_metrics};
use crate::api::impact::{get_impact_report, GetImpactReport};

mod components {
    use yew::prelude::*;
    use crate::business_tools::hooks::use_business_data;

    #[function_component(QuickActions)]
    pub fn quick_actions() -> Html {
        let on_generate_report = Callback::from(|_| {
            web_sys::console::log_1(&"Generating impact report...".into());
        });

        let on_export_data = Callback::from(|_| {
            web_sys::console::log_1(&"Exporting business data...".into());
        });

        let on_analyze_trends = Callback::from(|_| {
            web_sys::console::log_1(&"Analyzing business trends...".into());
        });

        html! {
            <div class="quick-actions">
                <button class="action-button" onclick={on_generate_report}>
                    { "Generate Impact Report" }
                </button>
                <button class="action-button" onclick={on_export_data}>
                    { "Export Data" }
                </button>
                <button class="action-button secondary" onclick={on_analyze_trends}>
                    { "Analyze Trends" }
                </button>
                <button class="action-button secondary">
                    { "Settings" }
                </button>
            </div>
        }
    }

    #[function_component(MetricsOverview)]
    pub fn metrics_overview() -> Html {
        let business_data = use_business_data();
        
        html! {
            <div class="dashboard-grid">
                <div class="dashboard-card">
                    <h3>{ "Total Revenue" }</h3>
                    <p class="metric-value">{ format!("${:.2}", business_data.total_revenue) }</p>
                    <span class="metric-change positive">
                        { format!("+{:.1}%", business_data.revenue_growth) }
                    </span>
                </div>
                
                <div class="dashboard-card">
                    <h3>{ "Impact Score" }</h3>
                    <p class="metric-value">{ format!("{:.1}/10", business_data.impact_score) }</p>
                    <span class="metric-change positive">
                        { format!("+{:.1}", business_data.impact_improvement) }
                    </span>
                </div>
                
                <div class="dashboard-card">
                    <h3>{ "Active Projects" }</h3>
                    <p class="metric-value">{ business_data.active_projects }</p>
                    <span class="metric-info">{ "Currently running" }</span>
                </div>
                
                <div class="dashboard-card">
                    <h3>{ "Community Investment" }</h3>
                    <p class="metric-value">{ format!("${:.2}", business_data.community_investment) }</p>
                    <span class="metric-change positive">
                        { format!("+{:.1}%", business_data.investment_growth) }
                    </span>
                </div>
            </div>
        }
    }

    #[function_component(RecentActivity)]
    pub fn recent_activity() -> Html {
        html! {
            <div class="dashboard-card">
                <h3>{ "Recent Activity" }</h3>
                <div class="activity-list">
                    <div class="activity-item">
                        <span class="activity-time">{ "2 hours ago" }</span>
                        <span class="activity-text">{ "Impact report generated for Q3" }</span>
                    </div>
                    <div class="activity-item">
                        <span class="activity-time">{ "1 day ago" }</span>
                        <span class="activity-text">{ "New cooperative member joined" }</span>
                    </div>
                    <div class="activity-item">
                        <span class="activity-time">{ "3 days ago" }</span>
                        <span class="activity-text">{ "Supply chain audit completed" }</span>
                    </div>
                </div>
            </div>
        }
    }
}

use components::*;

#[function_component(BusinessDashboard)]
pub fn business_dashboard() -> Html {
    let auth = use_auth();
    let metrics = use_async({
        let user_id = auth.user_id.clone();
        async move {
            match user_id {
                Some(id) => get_business_metrics(id).await,
                None => Err("User not authenticated".to_string()),
            }
        }
    });

    let impact_report = use_async({
        let user_id = auth.user_id.clone();
        async move {
            match user_id {
                Some(id) => get_impact_report(id).await,
                None => Err("User not authenticated".to_string()),
            }
        }
    });

    use_effect_with_deps(
        move |_| {
            if auth.is_authenticated && !auth.loading {
                metrics.run();
                impact_report.run();
            }
            || ()
        },
        (auth.is_authenticated, auth.loading),
    );

    html! {
        <div class="business-dashboard">
            <h1>{ "Business Intelligence Dashboard" }</h1>
            
            if auth.loading {
                <div class="loading-container">
                    <div class="spinner"></div>
                    <p>{ "Loading business data..." }</p>
                </div>
            } else if !auth.is_authenticated {
                <div class="auth-required">
                    <h2>{ "Authentication Required" }</h2>
                    <p>{ "Please log in to access business tools." }</p>
                </div>
            } else {
                <>
                    <QuickActions />
                    <MetricsOverview />
                    
                    <div class="dashboard-grid">
                        <RecentActivity />
                        <div class="dashboard-card">
                            <h3>{ "Impact Overview" }</h3>
                            if let Some(report) = &impact_report.data {
                                <p>{ format!("Current impact score: {:.1}", 
                                    report.get_impact_report.as_ref().map(|r| r.total_impact).unwrap_or(0.0)) }
                                </p>
                            } else {
                                <p>{ "Loading impact data..." }</p>
                            }
                        </div>
                    </div>
                </>
            }
        </div>
    }
}
