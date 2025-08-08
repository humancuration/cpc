//! Unified Impact Dashboard Component
//!
//! This component integrates all four impact measurement systems (learning, volunteer,
//! financial, cause) to show their interconnected nature and collective contribution
//! to community transformation.

use yew::prelude::*;
use wasm_bindgen_futures::JsFuture;
use gloo_timers::callback::Timeout;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use learning_impact_tracker::tracker::{LearningImpactTracker, ImpactMetrics as LearningMetrics};
use volunteer_impact_tracker::tracker::{VolunteerImpactTracker, ImpactMetrics as VolunteerMetrics};
use financial_impact_tracker::tracker::FinancialImpactTracker;
use cause_impact_tracker::tracker::{CauseImpactTracker, ImpactMetrics as CauseMetrics};
use impact_viz::core::{ImpactVisualization, VisualizationStyle, ImpactMetric, CommunityStory};
use impact_viz::values::ValuesTranslator;
use consent_manager::domain::consent::DataSharingLevel;

use crate::models::{UnifiedImpactData, ImpactInterconnection, CommunityWellbeing};
use crate::components::{InterconnectionVisualization, CommunityTransformationMetrics, MemberImpactView};
use crate::services::ImpactDataService;

/// Properties for the UnifiedImpactDashboard component
#[derive(Properties, PartialEq)]
pub struct UnifiedDashboardProps {
    /// User ID for personalized views
    #[prop_or_default]
    pub user_id: Option<String>,
    
    /// Consent level for data collection
    #[prop_or(DataSharingLevel::Standard)]
    pub consent_level: DataSharingLevel,
}

/// State for the UnifiedImpactDashboard component
#[derive(Clone, PartialEq)]
pub struct UnifiedDashboardState {
    /// Unified impact data from all four systems
    pub impact_data: Option<UnifiedImpactData>,
    
    /// Loading state
    pub loading: bool,
    
    /// Error state
    pub error: Option<String>,
    
    /// Current visualization style
    pub visualization_style: VisualizationStyle,
    
    /// Whether to show individual or community view
    pub view_mode: ViewMode,
}

/// View mode for the dashboard
#[derive(Clone, PartialEq)]
pub enum ViewMode {
    Individual,
    Community,
}

impl Default for UnifiedDashboardState {
    fn default() -> Self {
        Self {
            impact_data: None,
            loading: true,
            error: None,
            visualization_style: VisualizationStyle::Comparative,
            view_mode: ViewMode::Individual,
        }
    }
}

/// Unified Impact Dashboard Component
#[function_component(UnifiedImpactDashboard)]
pub fn unified_impact_dashboard(props: &UnifiedDashboardProps) -> Html {
    let state = use_state(|| UnifiedDashboardState::default());
    let data_service = use_state(|| ImpactDataService::new());
    
    // Load data when component mounts
    {
        let state = state.clone();
        let data_service = data_service.clone();
        let user_id = props.user_id.clone();
        let consent_level = props.consent_level.clone();
        
        use_effect_with((), move |_| {
            let state = state.clone();
            let data_service = data_service.clone();
            let user_id = user_id.clone();
            
            // Set loading state
            state.set(UnifiedDashboardState {
                loading: true,
                ..(*state).clone()
            });
            
            // Load data asynchronously
            wasm_bindgen_futures::spawn_local(async move {
                match data_service.load_unified_impact_data(user_id, consent_level).await {
                    Ok(data) => {
                        state.set(UnifiedDashboardState {
                            impact_data: Some(data),
                            loading: false,
                            error: None,
                            ..(*state).clone()
                        });
                    }
                    Err(e) => {
                        state.set(UnifiedDashboardState {
                            loading: false,
                            error: Some(e.to_string()),
                            ..(*state).clone()
                        });
                    }
                }
            });
            
            || ()
        });
    }
    
    let on_refresh = {
        let state = state.clone();
        let data_service = data_service.clone();
        let user_id = props.user_id.clone();
        let consent_level = props.consent_level.clone();
        
        Callback::from(move |_| {
            let state = state.clone();
            let data_service = data_service.clone();
            let user_id = user_id.clone();
            
            // Set loading state
            state.set(UnifiedDashboardState {
                loading: true,
                ..(*state).clone()
            });
            
            // Load data asynchronously
            wasm_bindgen_futures::spawn_local(async move {
                match data_service.load_unified_impact_data(user_id, consent_level).await {
                    Ok(data) => {
                        state.set(UnifiedDashboardState {
                            impact_data: Some(data),
                            loading: false,
                            error: None,
                            ..(*state).clone()
                        });
                    }
                    Err(e) => {
                        state.set(UnifiedDashboardState {
                            loading: false,
                            error: Some(e.to_string()),
                            ..(*state).clone()
                        });
                    }
                }
            });
        })
    };
    
    let on_toggle_view = {
        let state = state.clone();
        Callback::from(move |_| {
            let current_mode = (*state).view_mode.clone();
            let new_mode = match current_mode {
                ViewMode::Individual => ViewMode::Community,
                ViewMode::Community => ViewMode::Individual,
            };
            
            state.set(UnifiedDashboardState {
                view_mode: new_mode,
                ..(*state).clone()
            });
        })
    };
    
    let on_style_change = {
        let state = state.clone();
        Callback::from(move |style: VisualizationStyle| {
            state.set(UnifiedDashboardState {
                visualization_style: style,
                ..(*state).clone()
            });
        })
    };
    
    html! {
        <div class="unified-impact-dashboard">
            <div class="dashboard-header">
                <h1>{"Unified Community Impact Dashboard"}</h1>
                <div class="dashboard-controls">
                    <button 
                        class="btn btn-secondary" 
                        onclick={on_refresh}
                        disabled={state.loading}
                    >
                        {if state.loading { "Refreshing..." } else { "Refresh Data" }}
                    </button>
                    <button 
                        class="btn btn-primary" 
                        onclick={on_toggle_view}
                    >
                        {match state.view_mode {
                            ViewMode::Individual => "Switch to Community View",
                            ViewMode::Community => "Switch to Individual View",
                        }}
                    </button>
                </div>
            </div>
            
            {if let Some(error) = &state.error {
                html! {
                    <div class="error-message">
                        <p>{"Error loading dashboard data: "}{error}</p>
                        <button class="btn btn-primary" onclick={on_refresh}>{"Try Again"}</button>
                    </div>
                }
            } else {
                html! {}
            }}
            
            {if state.loading {
                html! {
                    <div class="loading-spinner">
                        <p>{"Loading impact data..."}</p>
                    </div>
                }
            } else {
                html! {}
            }}
            
            {if let Some(impact_data) = &state.impact_data {
                match state.view_mode {
                    ViewMode::Individual => {
                        html! {
                            <IndividualDashboardView 
                                data={impact_data.clone()} 
                                style={state.visualization_style.clone()}
                                on_style_change={on_style_change.clone()}
                            />
                        }
                    }
                    ViewMode::Community => {
                        html! {
                            <CommunityDashboardView 
                                data={impact_data.clone()} 
                                style={state.visualization_style.clone()}
                                on_style_change={on_style_change.clone()}
                            />
                        }
                    }
                }
            } else {
                html! {}
            }}
        </div>
    }
}

/// Properties for the IndividualDashboardView component
#[derive(Properties, PartialEq)]
pub struct IndividualDashboardViewProps {
    pub data: UnifiedImpactData,
    pub style: VisualizationStyle,
    pub on_style_change: Callback<VisualizationStyle>,
}

/// Individual Dashboard View Component
#[function_component(IndividualDashboardView)]
fn individual_dashboard_view(props: &IndividualDashboardViewProps) -> Html {
    html! {
        <div class="individual-dashboard-view">
            <div class="dashboard-section">
                <h2>{"Your Interconnected Impact"}</h2>
                <InterconnectionVisualization 
                    data={props.data.interconnections.clone()} 
                    style={props.style.clone()}
                />
            </div>
            
            <div class="dashboard-section">
                <h2>{"Your Impact Evolution"}</h2>
                <MemberImpactView 
                    learning_metrics={props.data.learning_metrics.clone()}
                    volunteer_metrics={props.data.volunteer_metrics.clone()}
                    financial_metrics={props.data.financial_metrics.clone()}
                    cause_metrics={props.data.cause_metrics.clone()}
                    style={props.style.clone()}
                />
            </div>
            
            <div class="dashboard-section">
                <h2>{"Community Transformation Indicators"}</h2>
                <CommunityTransformationMetrics 
                    wellbeing={props.data.community_wellbeing.clone()} 
                    style={props.style.clone()}
                />
            </div>
            
            <VisualizationStyleSelector 
                current_style={props.style.clone()} 
                on_change={props.on_style_change.clone()} 
            />
        </div>
    }
}

/// Properties for the CommunityDashboardView component
#[derive(Properties, PartialEq)]
pub struct CommunityDashboardViewProps {
    pub data: UnifiedImpactData,
    pub style: VisualizationStyle,
    pub on_style_change: Callback<VisualizationStyle>,
}

/// Community Dashboard View Component
#[function_component(CommunityDashboardView)]
fn community_dashboard_view(props: &CommunityDashboardViewProps) -> Html {
    html! {
        <div class="community-dashboard-view">
            <div class="dashboard-section">
                <h2>{"Community Impact Interconnections"}</h2>
                <InterconnectionVisualization 
                    data={props.data.interconnections.clone()} 
                    style={props.style.clone()}
                />
            </div>
            
            <div class="dashboard-section">
                <h2>{"Community Transformation Metrics"}</h2>
                <CommunityTransformationMetrics 
                    wellbeing={props.data.community_wellbeing.clone()} 
                    style={props.style.clone()}
                />
            </div>
            
            <div class="dashboard-section">
                <h2>{"Community Impact Stories"}</h2>
                <CommunityStoriesView stories={props.data.community_stories.clone()} />
            </div>
            
            <VisualizationStyleSelector 
                current_style={props.style.clone()} 
                on_change={props.on_style_change.clone()} 
            />
        </div>
    }
}

/// Properties for the VisualizationStyleSelector component
#[derive(Properties, PartialEq)]
pub struct VisualizationStyleSelectorProps {
    pub current_style: VisualizationStyle,
    pub on_change: Callback<VisualizationStyle>,
}

/// Visualization Style Selector Component
#[function_component(VisualizationStyleSelector)]
fn visualization_style_selector(props: &VisualizationStyleSelectorProps) -> Html {
    let on_style_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |event: Event| {
            let select = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = select.value();
            
            let style = match value.as_str() {
                "narrative" => VisualizationStyle::Narrative,
                "comparative" => VisualizationStyle::Comparative,
                "trend" => VisualizationStyle::TrendBased,
                "quantitative" => VisualizationStyle::Quantitative,
                "qualitative" => VisualizationStyle::Qualitative,
                _ => VisualizationStyle::Comparative,
            };
            
            on_change.emit(style);
        })
    };
    
    html! {
        <div class="visualization-style-selector">
            <label for="viz-style">{"Visualization Style: "}</label>
            <select id="viz-style" onchange={on_style_change}>
                <option value="narrative" selected={matches!(props.current_style, VisualizationStyle::Narrative)}>
                    {"Narrative"}
                </option>
                <option value="comparative" selected={matches!(props.current_style, VisualizationStyle::Comparative)}>
                    {"Comparative"}
                </option>
                <option value="trend" selected={matches!(props.current_style, VisualizationStyle::TrendBased)}>
                    {"Trend-Based"}
                </option>
                <option value="quantitative" selected={matches!(props.current_style, VisualizationStyle::Quantitative)}>
                    {"Quantitative"}
                </option>
                <option value="qualitative" selected={matches!(props.current_style, VisualizationStyle::Qualitative)}>
                    {"Qualitative"}
                </option>
            </select>
        </div>
    }
}

/// Properties for the CommunityStoriesView component
#[derive(Properties, PartialEq)]
pub struct CommunityStoriesViewProps {
    pub stories: Vec<CommunityStory>,
}

/// Community Stories View Component
#[function_component(CommunityStoriesView)]
fn community_stories_view(props: &CommunityStoriesViewProps) -> Html {
    html! {
        <div class="community-stories-view">
            {for props.stories.iter().map(|story| {
                html! {
                    <div class="story-card">
                        <h3>{&story.title}</h3>
                        <p>{&story.narrative}</p>
                        <div class="story-metrics">
                            {for story.metrics.iter().map(|metric| {
                                html! {
                                    <span class="metric-tag">
                                        {&metric.name}: {metric.value}
                                    </span>
                                }
                            })}
                        </div>
                    </div>
                }
            })}
        </div>
    }
}