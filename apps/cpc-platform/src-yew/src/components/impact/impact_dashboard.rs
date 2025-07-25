use yew::prelude::*;
use yewdux::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::store::{Store, Action, set_loading, set_error, set_impact_data};
use crate::types::impact::ImpactDashboardData;
use crate::services::impact::{get_impact_report, recalculate_impact, subscribe_impact_updates};
use crate::components::impact::{
    CarbonFootprintCard,
    CommunityInvestmentCard,
    DiversityMetricsCard,
    SupplyChainEthicsChart,
};

#[function_component(ImpactDashboard)]
pub fn impact_dashboard() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let recalculating = use_state(|| false);
    
    // Fetch data on mount and when org_id changes
    {
        let dispatch = dispatch.clone();
        let org_id = store.org_id.to_string();
        
        use_effect_with_deps(
            move |_| {
                dispatch.reduce(set_loading(true));
                
                spawn_local(async move {
                    match get_impact_report(&org_id, 2024).await {
                        Ok(data) => {
                            dispatch.reduce(set_impact_data(data));
                            dispatch.reduce(set_error(None));
                        }
                        Err(err) => {
                            dispatch.reduce(set_error(Some(err)));
                        }
                    }
                    dispatch.reduce(set_loading(false));
                });
                
                || ()
            },
            store.org_id.clone(),
        );
    }
    
    // Subscribe to impact updates
    {
        let dispatch = dispatch.clone();
        let org_id = store.org_id.to_string();
        
        use_effect_with_deps(
            move |_| {
                let callback = Closure::wrap(Box::new(move |data: JsValue| {
                    if let Ok(json_str) = data.as_string() {
                        if let Ok(report) = serde_json::from_str::<ImpactDashboardData>(&json_str) {
                            dispatch.reduce(set_impact_data(report));
                        }
                    }
                }) as Box<dyn FnMut(JsValue)>);
                
                spawn_local(async move {
                    let _ = subscribe_impact_updates(&org_id, callback.as_ref().unchecked_ref()).await;
                    callback.forget(); // Prevent cleanup so subscription remains active
                });
                
                || () // No cleanup needed
            },
            store.org_id.clone(),
        );
    }
    
    // Handle refresh button click
    let on_refresh = {
        let dispatch = dispatch.clone();
        let recalculating = recalculating.clone();
        let org_id = store.org_id.to_string();
        
        Callback::from(move |_| {
            recalculating.set(true);
            let dispatch = dispatch.clone();
            let recalculating = recalculating.clone();
            let org_id = org_id.clone();
            
            spawn_local(async move {
                dispatch.reduce(set_loading(true));
                if let Err(err) = recalculate_impact(&org_id, 2024).await {
                    dispatch.reduce(set_error(Some(err)));
                }
                dispatch.reduce(set_loading(false));
                recalculating.set(false);
            });
        })
    };

    // Loading state
    if store.loading {
        return html! {
            <div class="impact-dashboard loading">
                <div class="spinner"></div>
                <p>{"Loading impact report..."}</p>
            </div>
        };
    }

    // Error state
    if let Some(err) = &store.error {
        return html! {
            <div class="impact-dashboard error">
                <div class="error-message">
                    <h3>{"Error loading impact report"}</h3>
                    <p>{err}</p>
                    <button class="btn-refresh" onclick={on_refresh.clone()}>
                        {"Retry"}
                    </button>
                </div>
            </div>
        };
    }

    // Data state
    let data = match &store.impact {
        Some(data) => data,
        None => return html! {
            <div class="impact-dashboard">
                <EmptyImpactState />
                <div class="refresh-container">
                    <button class="btn-refresh" onclick={on_refresh}>
                        {"Generate Report"}
                    </button>
                </div>
            </div>
        },
    };

    html! {
        <div class="impact-dashboard">
            <div class="dashboard-header">
                <h1>{"Impact Report"}</h1>
                <div class="header-actions">
                    <div class="dashboard-year">{"2024"}</div>
                    <button class="btn-refresh" onclick={on_refresh} disabled={*recalculating}>
                        { if *recalculating {
                            html! {<span class="spinner-small"></span>}
                        } else {
                            html! {"Refresh Data"}
                        }}
                    </button>
                </div>
            </div>
            
            <div class="dashboard-summary">
                <p>{"Comprehensive overview of environmental, social, and economic impact"}</p>
            </div>

            <div class="dashboard-grid">
                <div class="grid-item">
                    <CarbonFootprintCard data={data.carbon_footprint.clone()} />
                </div>
                
                <div class="grid-item">
                    <CommunityInvestmentCard data={data.community_investment.clone()} />
                </div>
                
                <div class="grid-item">
                    <DiversityMetricsCard data={data.diversity_metrics.clone()} />
                </div>
                
                <div class="grid-item">
                    <SupplyChainEthicsChart data={data.supply_chain_score.clone()} />
                </div>
            </div>

            <div class="dashboard-footer">
                <p class="report-note">
                    {"Data reflects impact metrics for the current reporting period."}
                </p>
                { if *recalculating {
                    html! {
                        <div class="recalculation-status">
                            <span class="spinner-small"></span>
                            {"Recalculating impact data..."}
                        </div>
                    }
                } else {
                    html! {}
                }}
            </div>
        </div>
    }
}

// Helper component for when no data is available
#[function_component(EmptyImpactState)]
fn empty_impact_state() -> Html {
    html! {
        <div class="empty-state">
            <div class="empty-icon">
                <i class="icon-leaf"></i>
            </div>
            <h3>{"No Impact Data"}</h3>
            <p>{"Impact data will be available once reports are generated."}</p>
        </div>
    }
}