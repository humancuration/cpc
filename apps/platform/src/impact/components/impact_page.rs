use yew::prelude::*;
use yew_hooks::use_async;
use uuid::Uuid;
use web_sys::console;

use crate::api::impact::{
    get_impact_report, get_organization_impact_report, GetImpactReport,
    GetOrganizationImpactReport,
};
use crate::api::graphql::queries::GetFeatureFlags;
use crate::context::auth::use_auth;
use super::{
    impact_timeline::{ImpactTimelineComponent, ImpactTimelineProps},
    impact_distribution_chart::{ImpactDistributionChart, ImpactDistributionChartProps},
    impact_breakdown_table::{ImpactBreakdownTable, ImpactBreakdownTableProps},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tab {
    Personal,
    Organization,
}

#[function_component(ImpactPage)]
pub fn impact_page() -> Html {
    let auth = use_auth();
    let active_tab = use_state(|| Tab::Personal);
    
    let user_impact_report = use_async({
        let user_id = auth.user_id.clone();
        async move {
            match user_id {
                Some(id) => get_impact_report(id).await,
                None => Err("User not authenticated".to_string()),
            }
        }
    });
    
    let feature_flags = use_async(async move {
        get_feature_flags().await
    });

    let org_impact_report = use_async({
        let org_id = auth.organization_id;
        async move {
            match org_id {
                Some(id) => get_organization_impact_report(id, 2024).await,
                None => Err("Organization not available".to_string()),
            }
        }
    });

    use_effect_with_deps(
        move |_| {
            if auth.is_authenticated && !auth.loading {
                user_impact_report.run();
                org_impact_report.run();
                feature_flags.run();
            }
            || ()
        },
        (auth.is_authenticated, auth.loading),
    );

    let on_tab_click = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: Tab| {
            active_tab.set(tab);
        })
    };

    let render_user_report = |report: &GetImpactReport| {
        let report_data = report.get_impact_report.as_ref().unwrap();
        html! {
            <div class="impact-report">
                <h2>{ "Personal Impact Report" }</h2>
                <div class="impact-summary">
                    <div class="metric-card">
                        <h3>{ "Total Impact" }</h3>
                        <p class="metric-value">{ format!("{:.2}", report_data.total_impact) }</p>
                    </div>
                </div>
                <ImpactTimelineComponent timeline={report_data.timeline.clone()} />
                <ImpactDistributionChart distribution={report_data.distribution.clone()} />
                <ImpactBreakdownTable
                    breakdown={report_data.breakdown.clone()}
                    threshold={feature_flags.data.as_ref().and_then(|d| d.feature_flags.as_ref()).map(|f| f.ui_degradation_threshold).unwrap_or(0.1)}
                />
            </div>
        }
    };

    let render_org_report = |report: &GetOrganizationImpactReport| {
        let report_data = report.get_organization_impact_report.as_ref().unwrap();
        html! {
            <div class="impact-report">
                <h2>{ "Organization Impact Report" }</h2>
                <div class="impact-metrics">
                    <div class="metric-card">
                        <h3>{ "Carbon Footprint" }</h3>
                        <p class="metric-value">{ format!("{:.2} kg CO₂", report_data.carbon_footprint) }</p>
                    </div>
                    <div class="metric-card">
                        <h3>{ "Community Investment" }</h3>
                        <p class="metric-value">{ format!("${:.2}", report_data.community_investment) }</p>
                    </div>
                    <div class="metric-card">
                        <h3>{ "Supply Chain Score" }</h3>
                        <p class="metric-value">{ format!("{:.1}/10", report_data.supply_chain_score) }</p>
                    </div>
                </div>
            </div>
        }
    };

    let render_loading = || {
        html! {
            <div class="loading-container">
                <div class="spinner"></div>
                <p>{ "Loading impact data..." }</p>
            </div>
        }
    };

    let render_error = |error: &str| {
        html! {
            <div class="error-container">
                <div class="error-message">
                    <h3>{ "Error Loading Data" }</h3>
                    <p>{ error }</p>
                </div>
            </div>
        }
    };

    html! {
        <div class="impact-page">
            <h1>{ "Impact Dashboard" }</h1>

            if auth.loading {
                { render_loading() }
            } else if !auth.is_authenticated {
                <div class="auth-required">
                    <h2>{ "Authentication Required" }</h2>
                    <p>{ "Please log in to view your impact reports." }</p>
                </div>
            } else {
                <div class="impact-container">
                    <div class="tabs">
                        <button 
                            class={if *active_tab == Tab::Personal { "tab active" } else { "tab" }}
                            onclick={on_tab_click.reform(|_| Tab::Personal)}
                        >
                            { "Personal Impact" }
                        </button>
                        <button 
                            class={if *active_tab == Tab::Organization { "tab active" } else { "tab" }}
                            onclick={on_tab_click.reform(|_| Tab::Organization)}
                        >
                            { "Organization Impact" }
                        </button>
                    </div>

                    <div class="report-content">
                        {match *active_tab {
                            Tab::Personal => {
                                if user_impact_report.loading || feature_flags.loading {
                                    render_loading()
                                } else if let Some(report) = &user_impact_report.data {
                                    render_user_report(report)
                                } else if let Some(error) = &user_impact_report.error {
                                    render_error(error)
                                } else if let Some(error) = &feature_flags.error {
                                    render_error(error)
                                } else {
                                    render_error("No data available")
                                }
                            }
                            Tab::Organization => {
                                if org_impact_report.loading {
                                    render_loading()
                                } else if let Some(report) = &org_impact_report.data {
                                    render_org_report(report)
                                } else if let Some(error) = &org_impact_report.error {
                                    render_error(error)
                                } else {
                                    render_error("No organization data available")
                                }
                            }
                        }}
                    </div>
                </div>
            }
        </div>
    }
}
