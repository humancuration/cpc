//! Web Frontend for Finance Admin Dashboard
//!
//! A Yew-based web frontend for the finance admin dashboard.

use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use gloo_net::http::Request as HttpRequest;

// Import UI components
use crate::ui::*;

// Define routes
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Dashboard,
    #[at("/engagement")]
    Engagement,
    #[at("/feedback")]
    Feedback,
    #[at("/improvements")]
    Improvements,
    #[at("/ab-testing")]
    ABTesting,
    #[at("/cross-system")]
    CrossSystem,
}

// Main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container">
                <nav class="sidebar">
                    <h2>{"Finance Impact Dashboard"}</h2>
                    <ul>
                        <li><Link<Route> to={Route::Dashboard}>{"Dashboard"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Engagement}>{"Engagement Metrics"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Feedback}>{"Feedback Analysis"}</Link<Route>></li>
                        <li><Link<Route> to={Route::Improvements}>{"Improvement Suggestions"}</Link<Route>></li>
                        <li><Link<Route> to={Route::ABTesting}>{"A/B Testing"}</Link<Route>></li>
                        <li><Link<Route> to={Route::CrossSystem}>{"Cross-System Impact"}</Link<Route>></li>
                    </ul>
                </nav>
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Dashboard => html! { <DashboardPage /> },
        Route::Engagement => html! { <EngagementPage /> },
        Route::Feedback => html! { <FeedbackPage /> },
        Route::Improvements => html! { <ImprovementsPage /> },
        Route::ABTesting => html! { <ABTestingPage /> },
        Route::CrossSystem => html! { <CrossSystemPage /> },
    }
}

// Dashboard page component
#[function_component(DashboardPage)]
fn dashboard_page() -> Html {
    let summary = use_state(|| None);
    let loading = use_state(|| true);
    
    {
        let summary = summary.clone();
        let loading = loading.clone();
        
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match fetch_dashboard_summary().await {
                        Ok(data) => {
                            summary.set(Some(data));
                            loading.set(false);
                        }
                        Err(_) => {
                            loading.set(false);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }
    
    if *loading {
        return html! { <div>{"Loading..."}</div> };
    }
    
    let summary_data = match summary.as_ref() {
        Some(data) => data,
        None => return html! { <div>{"Failed to load dashboard data"}</div> },
    };
    
    html! {
        <div class="dashboard-page">
            <h1>{"Financial Impact Dashboard"}</h1>
            
            <div class="metrics-grid">
                <MetricCard props={MetricCardProps::new(
                    "Total Views".to_string(),
                    summary_data.engagement.total_views.to_string(),
                    "Total visualization views".to_string()
                )} />
                
                <MetricCard props={MetricCardProps::new(
                    "Avg Interaction Time".to_string(),
                    format!("{:.1}s", summary_data.engagement.avg_interaction_time),
                    "Average time spent interacting with visualizations".to_string()
                )} />
                
                <MetricCard props={MetricCardProps::new(
                    "Quality Score".to_string(),
                    format!("{:.2}", summary_data.engagement.quality_score),
                    "Average quality of visualization interactions".to_string()
                )} />
                
                <MetricCard props={MetricCardProps::new(
                    "Participation Rate".to_string(),
                    format!("{:.1}%", summary_data.financial_effectiveness.participation_rate_with_viz * 100.0),
                    "Financial participation rate with visualization usage".to_string()
                ).with_change(
                    (summary_data.financial_effectiveness.participation_rate_with_viz - summary_data.financial_effectiveness.participation_rate_without_viz) * 100.0,
                    ChangeDirection::Up
                )} />
            </div>
            
            <div class="feedback-summary">
                <h2>{"Community Feedback"}</h2>
                <p>{format!("Average Rating: {:.1}", summary_data.feedback.avg_rating)}</p>
                <p>{format!("Helpfulness: {:.1}%", summary_data.feedback.helpful_percentage)}</p>
                <p>{format!("Understanding Improvement: {:.1}", summary_data.feedback.avg_understanding_improvement)}</p>
            </div>
            
            <div class="recommendations">
                <h2>{"Improvement Recommendations"}</h2>
                <ul>
                    {for summary_data.recommendations.iter().take(3).map(|rec| {
                        html! {
                            <li>
                                <strong>{&rec.description}</strong>
                                <span>{format!(" (Priority: {:?})", rec.priority)}</span>
                            </li>
                        }
                    })}
                </ul>
            </div>
        </div>
    }
}

// Engagement metrics page
#[function_component(EngagementPage)]
fn engagement_page() -> Html {
    html! {
        <div class="engagement-page">
            <h1>{"Engagement Metrics"}</h1>
            <p>{"Detailed engagement metrics would be displayed here."}</p>
        </div>
    }
}

// Feedback analysis page
#[function_component(FeedbackPage)]
fn feedback_page() -> Html {
    html! {
        <div class="feedback-page">
            <h1>{"Feedback Analysis"}</h1>
            <p>{"Community feedback analysis would be displayed here."}</p>
        </div>
    }
}

// Improvement suggestions page
#[function_component(ImprovementsPage)]
fn improvements_page() -> Html {
    html! {
        <div class="improvements-page">
            <h1>{"Improvement Suggestions"}</h1>
            <p>{"AI-generated improvement suggestions would be displayed here."}</p>
        </div>
    }
}

// A/B testing page
#[function_component(ABTestingPage)]
fn abtesting_page() -> Html {
    html! {
        <div class="abtesting-page">
            <h1>{"A/B Testing"}</h1>
            <p>{"A/B testing results and experiments would be displayed here."}</p>
        </div>
    }
}

// Cross-system impact page
#[function_component(CrossSystemPage)]
fn cross_system_page() -> Html {
    html! {
        <div class="cross-system-page">
            <h1>{"Cross-System Impact"</h1>
            <p>{"Impact analysis across learning, volunteering, and cause systems would be displayed here."}</p>
        </div>
    }
}

// Metric card component
#[derive(Properties, PartialEq)]
struct MetricCardPropsComponent {
    props: MetricCardProps,
}

#[function_component(MetricCard)]
fn metric_card(MetricCardPropsComponent { props }: &MetricCardPropsComponent) -> Html {
    let change_indicator = if let (Some(change), Some(direction)) = (props.change, &props.change_direction) {
        let symbol = match direction {
            ChangeDirection::Up => "↑",
            ChangeDirection::Down => "↓",
            ChangeDirection::Neutral => "→",
        };
        html! {
            <span class={classes!("change-indicator", direction.to_string().to_lowercase())}>
                {format!("{} {:.1}%", symbol, change)}
            </span>
        }
    } else {
        html! {}
    };
    
    html! {
        <div class="metric-card">
            <h3>{&props.title}</h3>
            <div class="value">{&props.value}</div>
            <div class="description">{&props.description}</div>
            <div class="change">{change_indicator}</div>
        </div>
    }
}

// API functions
async fn fetch_dashboard_summary() -> Result<financial_impact_tracker::analytics::DashboardSummary, anyhow::Error> {
    let resp = HttpRequest::get("/api/dashboard/summary")
        .send()
        .await?;
    
    let json: financial_impact_tracker::analytics::DashboardSummary = resp.json().await?;
    Ok(json)
}

// Main entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}