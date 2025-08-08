//! Community Documentation Component
//!
//! This component enables documentation of community validation outcomes
//! and makes them accessible for future reference and learning.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::models::{UnifiedImpactData, community_wellbeing::CooperativeGoalProgress};
use uuid::Uuid;
use chrono::{Utc, DateTime};
use std::collections::HashMap;

/// Properties for the CommunityDocumentation component
#[derive(Properties, PartialEq)]
pub struct CommunityDocumentationProps {
    /// Impact data to document
    pub impact_data: UnifiedImpactData,
    
    /// Callback when documentation is saved
    pub on_save: Callback<CommunityDocumentationRecord>,
}

/// State for the CommunityDocumentation component
#[derive(Clone, PartialEq)]
pub struct CommunityDocumentationState {
    /// Documentation records
    records: Vec<CommunityDocumentationRecord>,
    
    /// Current documentation being edited
    current_documentation: CommunityDocumentationRecord,
    
    /// Filter for viewing records
    filter: DocumentationFilter,
}

/// Documentation filter options
#[derive(Clone, PartialEq)]
pub enum DocumentationFilter {
    All,
    Interpretations,
    Reflections,
    Recommendations,
    ActionItems,
}

/// Community documentation record structure
#[derive(Clone, PartialEq)]
pub struct CommunityDocumentationRecord {
    /// Unique identifier
    pub id: Uuid,
    
    /// Title of the documentation
    pub title: String,
    
    /// Type of documentation
    pub doc_type: DocumentationType,
    
    /// Content of the documentation
    pub content: DocumentationContent,
    
    /// Authors/contributors
    pub authors: Vec<String>,
    
    /// Related impact data snapshot
    pub impact_snapshot: Option<UnifiedImpactData>,
    
    /// Tags for categorization
    pub tags: Vec<String>,
    
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Status
    pub status: DocumentationStatus,
}

/// Documentation types
#[derive(Clone, PartialEq)]
pub enum DocumentationType {
    CommunityInterpretation,
    ReflectionOutcome,
    Recommendation,
    ActionItem,
    LearningResource,
    BestPractice,
}

/// Documentation content structure
#[derive(Clone, PartialEq)]
pub struct DocumentationContent {
    /// Summary of the documentation
    pub summary: String,
    
    /// Detailed content
    pub details: String,
    
    /// Key insights
    pub insights: Vec<String>,
    
    /// Supporting evidence
    pub evidence: Vec<SupportingEvidence>,
    
    /// Related documents
    pub related_documents: Vec<Uuid>,
}

/// Supporting evidence structure
#[derive(Clone, PartialEq)]
pub struct SupportingEvidence {
    /// Evidence description
    pub description: String,
    
    /// Evidence type
    pub evidence_type: EvidenceType,
    
    /// Source
    pub source: String,
    
    /// Link to source (if applicable)
    pub link: Option<String>,
}

/// Evidence types
#[derive(Clone, PartialEq)]
pub enum EvidenceType {
    DataPoint,
    CommunityFeedback,
    Observation,
    Research,
    CaseStudy,
}

/// Documentation status
#[derive(Clone, PartialEq)]
pub enum DocumentationStatus {
    Draft,
    Review,
    Published,
    Archived,
}

/// Community Documentation Component
#[styled_component(CommunityDocumentation)]
pub fn community_documentation(props: &CommunityDocumentationProps) -> Html {
    let state = use_state(|| CommunityDocumentationState {
        records: sample_documentation_records(),
        current_documentation: CommunityDocumentationRecord {
            id: Uuid::new_v4(),
            title: String::new(),
            doc_type: DocumentationType::CommunityInterpretation,
            content: DocumentationContent {
                summary: String::new(),
                details: String::new(),
                insights: Vec::new(),
                evidence: Vec::new(),
                related_documents: Vec::new(),
            },
            authors: vec!["Current User".to_string()],
            impact_snapshot: Some(props.impact_data.clone()),
            tags: Vec::new(),
            timestamp: Utc::now(),
            status: DocumentationStatus::Draft,
        },
        filter: DocumentationFilter::All,
    });
    
    let container_style = style!(
        r#"
        background-color: white;
        border-radius: 8px;
        padding: 2rem;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        margin-bottom: 2rem;
    "#
    ).unwrap();
    
    let tab_style = style!(
        r#"
        padding: 0.75rem 1.5rem;
        border: none;
        background-color: #f8f9fa;
        cursor: pointer;
        border-radius: 4px 4px 0 0;
        margin-right: 0.25rem;
    "#
    ).unwrap();
    
    let active_tab_style = style!(
        r#"
        background-color: #3498db;
        color: white;
    "#
    ).unwrap();
    
    let button_style = style!(
        r#"
        padding: 0.5rem 1rem;
        border: none;
        border-radius: 4px;
        font-size: 0.9rem;
        cursor: pointer;
        margin: 0 0.25rem;
    "#
    ).unwrap();
    
    let primary_button_style = style!(
        r#"
        background-color: #3498db;
        color: white;
    "#
    ).unwrap();
    
    let secondary_button_style = style!(
        r#"
        background-color: #95a5a6;
        color: white;
    "#
    ).unwrap();
    
    let on_filter_change = {
        let state = state.clone();
        Callback::from(move |filter: DocumentationFilter| {
            state.set(CommunityDocumentationState {
                filter,
                ..(*state).clone()
            });
        })
    };
    
    let on_save_documentation = {
        let state = state.clone();
        let on_save = props.on_save.clone();
        
        Callback::from(move |_| {
            let documentation = state.current_documentation.clone();
            on_save.emit(documentation);
        })
    };
    
    let filtered_records = match state.filter {
        DocumentationFilter::All => state.records.clone(),
        DocumentationFilter::Interpretations => {
            state.records.iter()
                .filter(|r| matches!(r.doc_type, DocumentationType::CommunityInterpretation))
                .cloned()
                .collect()
        },
        DocumentationFilter::Reflections => {
            state.records.iter()
                .filter(|r| matches!(r.doc_type, DocumentationType::ReflectionOutcome))
                .cloned()
                .collect()
        },
        DocumentationFilter::Recommendations => {
            state.records.iter()
                .filter(|r| matches!(r.doc_type, DocumentationType::Recommendation))
                .cloned()
                .collect()
        },
        DocumentationFilter::ActionItems => {
            state.records.iter()
                .filter(|r| matches!(r.doc_type, DocumentationType::ActionItem))
                .cloned()
                .collect()
        },
    };
    
    html! {
        <div class={container_style}>
            <h2>{"Community Documentation Center"}</h2>
            <p>{"Document, share, and learn from our community's collective insights and outcomes."}</p>
            
            <div style="margin: 1rem 0;">
                <button 
                    class={classes!(tab_style.clone(), 
                        if matches!(state.filter, DocumentationFilter::All) { 
                            active_tab_style.clone() 
                        } else { 
                            style!("").unwrap() 
                        })}
                    onclick={on_filter_change.reform(|_| DocumentationFilter::All)}
                >
                    {"All Documents"}
                </button>
                <button 
                    class={classes!(tab_style.clone(), 
                        if matches!(state.filter, DocumentationFilter::Interpretations) { 
                            active_tab_style.clone() 
                        } else { 
                            style!("").unwrap() 
                        })}
                    onclick={on_filter_change.reform(|_| DocumentationFilter::Interpretations)}
                >
                    {"Interpretations"}
                </button>
                <button 
                    class={classes!(tab_style.clone(), 
                        if matches!(state.filter, DocumentationFilter::Reflections) { 
                            active_tab_style.clone() 
                        } else { 
                            style!("").unwrap() 
                        })}
                    onclick={on_filter_change.reform(|_| DocumentationFilter::Reflections)}
                >
                    {"Reflections"}
                </button>
                <button 
                    class={classes!(tab_style.clone(), 
                        if matches!(state.filter, DocumentationFilter::Recommendations) { 
                            active_tab_style.clone() 
                        } else { 
                            style!("").unwrap() 
                        })}
                    onclick={on_filter_change.reform(|_| DocumentationFilter::Recommendations)}
                >
                    {"Recommendations"}
                </button>
                <button 
                    class={classes!(tab_style.clone(), 
                        if matches!(state.filter, DocumentationFilter::ActionItems) { 
                            active_tab_style.clone() 
                        } else { 
                            style!("").unwrap() 
                        })}
                    onclick={on_filter_change.reform(|_| DocumentationFilter::ActionItems)}
                >
                    {"Action Items"}
                </button>
            </div>
            
            <div style="margin: 2rem 0;">
                {render_documentation_list(&filtered_records)}
            </div>
            
            <div style="border-top: 1px solid #eee; padding-top: 2rem;">
                <h3>{"Create New Documentation"}</h3>
                {render_documentation_form(&state.current_documentation)}
                
                <div style="display: flex; justify-content: flex-end; margin-top: 1rem;">
                    <button 
                        class={classes!(button_style.clone(), secondary_button_style.clone())}
                    >
                        {"Save Draft"}
                    </button>
                    <button 
                        class={classes!(button_style.clone(), primary_button_style.clone())}
                        onclick={on_save_documentation}
                    >
                        {"Publish Documentation"}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Render the documentation list
fn render_documentation_list(records: &Vec<CommunityDocumentationRecord>) -> Html {
    if records.is_empty() {
        return html! {
            <div style="text-align: center; padding: 2rem; color: #666;">
                <p>{"No documentation records found. Create your first documentation to get started."}</p>
            </div>
        };
    }
    
    html! {
        <div>
            {for records.iter().map(|record| {
                html! {
                    <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;">
                        <div style="display: flex; justify-content: space-between; align-items: center;">
                            <div>
                                <h4>{&record.title}</h4>
                                <p style="margin: 0.25rem 0; color: #666; font-size: 0.9rem;">
                                    {format!("{}", record.timestamp.format("%B %d, %Y"))}
                                    {" • "}
                                    {match &record.doc_type {
                                        DocumentationType::CommunityInterpretation => "Community Interpretation",
                                        DocumentationType::ReflectionOutcome => "Reflection Outcome",
                                        DocumentationType::Recommendation => "Recommendation",
                                        DocumentationType::ActionItem => "Action Item",
                                        DocumentationType::LearningResource => "Learning Resource",
                                        DocumentationType::BestPractice => "Best Practice",
                                    }}
                                </p>
                            </div>
                            <div>
                                <span style={format!("padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem; {}",
                                    match record.status {
                                        DocumentationStatus::Draft => "background-color: #95a5a6; color: white;",
                                        DocumentationStatus::Review => "background-color: #f39c12; color: white;",
                                        DocumentationStatus::Published => "background-color: #27ae60; color: white;",
                                        DocumentationStatus::Archived => "background-color: #34495e; color: white;",
                                    })}>
                                    {match record.status {
                                        DocumentationStatus::Draft => "Draft",
                                        DocumentationStatus::Review => "In Review",
                                        DocumentationStatus::Published => "Published",
                                        DocumentationStatus::Archived => "Archived",
                                    }}
                                </span>
                            </div>
                        </div>
                        
                        <p style="margin: 0.5rem 0;">{&record.content.summary}</p>
                        
                        <div style="display: flex; flex-wrap: wrap; gap: 0.5rem; margin-top: 0.5rem;">
                            {for record.tags.iter().map(|tag| {
                                html! {
                                    <span style="background-color: #3498db; color: white; padding: 0.25rem 0.5rem; border-radius: 4px; font-size: 0.8rem;">
                                        {tag}
                                    </span>
                                }
                            })}
                        </div>
                        
                        <div style="display: flex; gap: 0.5rem; margin-top: 1rem;">
                            <button style="background: none; border: none; color: #3498db; cursor: pointer; font-size: 0.9rem;">
                                {"View Details"}
                            </button>
                            <button style="background: none; border: none; color: #3498db; cursor: pointer; font-size: 0.9rem;">
                                {"Edit"}
                            </button>
                            <button style="background: none; border: none; color: #3498db; cursor: pointer; font-size: 0.9rem;">
                                {"Share"}
                            </button>
                        </div>
                    </div>
                }
            })}
        </div>
    }
}

/// Render the documentation form
fn render_documentation_form(documentation: &CommunityDocumentationRecord) -> Html {
    html! {
        <div>
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Title"}</label>
                <input 
                    type="text" 
                    value={documentation.title.clone()}
                    style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 4px;"
                    placeholder="Enter a descriptive title for this documentation"
                />
            </div>
            
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Type"}</label>
                <select style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 4px;">
                    <option selected={matches!(documentation.doc_type, DocumentationType::CommunityInterpretation)}>{"Community Interpretation"}</option>
                    <option selected={matches!(documentation.doc_type, DocumentationType::ReflectionOutcome)}>{"Reflection Outcome"}</option>
                    <option selected={matches!(documentation.doc_type, DocumentationType::Recommendation)}>{"Recommendation"}</option>
                    <option selected={matches!(documentation.doc_type, DocumentationType::ActionItem)}>{"Action Item"}</option>
                    <option selected={matches!(documentation.doc_type, DocumentationType::LearningResource)}>{"Learning Resource"}</option>
                    <option selected={matches!(documentation.doc_type, DocumentationType::BestPractice)}>{"Best Practice"}</option>
                </select>
            </div>
            
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Summary"}</label>
                <textarea 
                    value={documentation.content.summary.clone()}
                    style="width: 100%; height: 100px; padding: 0.75rem; border: 1px solid #ddd; border-radius: 4px;"
                    placeholder="Provide a brief summary of this documentation"
                >
                </textarea>
            </div>
            
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Details"}</label>
                <textarea 
                    value={documentation.content.details.clone()}
                    style="width: 100%; height: 200px; padding: 0.75rem; border: 1px solid #ddd; border-radius: 4px;"
                    placeholder="Provide detailed information, context, and explanations"
                >
                </textarea>
            </div>
            
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Key Insights"}</label>
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 4px;">
                    <p>{"No insights added yet. Add key insights from this documentation."}</p>
                    <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; margin-top: 0.5rem;">
                        {"Add Insight"}
                    </button>
                </div>
            </div>
            
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Supporting Evidence"}</label>
                <div style="background-color: #f8f9fa; padding: 1rem; border-radius: 4px;">
                    <p>{"No evidence added yet. Add supporting data, feedback, or research."}</p>
                    <button style="background-color: #3498db; color: white; border: none; padding: 0.5rem 1rem; border-radius: 4px; cursor: pointer; margin-top: 0.5rem;">
                        {"Add Evidence"}
                    </button>
                </div>
            </div>
            
            <div style="margin-bottom: 1rem;">
                <label style="display: block; margin-bottom: 0.5rem; font-weight: bold;">{"Tags"}</label>
                <input 
                    type="text" 
                    style="width: 100%; padding: 0.75rem; border: 1px solid #ddd; border-radius: 4px;"
                    placeholder="Add tags separated by commas (e.g., learning, volunteer, financial)"
                />
            </div>
        </div>
    }
}

/// Sample documentation records for demonstration
fn sample_documentation_records() -> Vec<CommunityDocumentationRecord> {
    vec![
        CommunityDocumentationRecord {
            id: Uuid::new_v4(),
            title: "Q2 Community Interpretation: Learning-Volunteer Connection".to_string(),
            doc_type: DocumentationType::CommunityInterpretation,
            content: DocumentationContent {
                summary: "Community identified strong connection between learning programs and volunteer retention.".to_string(),
                details: "Through collaborative interpretation sessions, community members noted that participants who complete learning modules show 40% higher volunteer retention rates. This insight has informed our integrated pathway development.".to_string(),
                insights: vec![
                    "Learning builds commitment to service".to_string(),
                    "Skills development strengthens community bonds".to_string(),
                ],
                evidence: vec![
                    SupportingEvidence {
                        description: "Volunteer retention data Q1-Q2 2025".to_string(),
                        evidence_type: EvidenceType::DataPoint,
                        source: "Community Impact Dashboard".to_string(),
                        link: None,
                    }
                ],
                related_documents: vec![],
            },
            authors: vec!["Community Interpretation Group".to_string()],
            impact_snapshot: None,
            tags: vec!["learning".to_string(), "volunteer".to_string(), "retention".to_string()],
            timestamp: Utc::now() - chrono::Duration::days(45),
            status: DocumentationStatus::Published,
        },
        CommunityDocumentationRecord {
            id: Uuid::new_v4(),
            title: "Annual Community Reflection: Resilience and Connection".to_string(),
            doc_type: DocumentationType::ReflectionOutcome,
            content: DocumentationContent {
                summary: "Annual reflection revealed community resilience and need for deeper connections.".to_string(),
                details: "Our annual community reflection session highlighted three key themes: Community Resilience, Need for Connection, and Excitement for Future. Participants expressed gratitude for our integrated approach while requesting more cross-domain interaction opportunities.".to_string(),
                insights: vec![
                    "Integrated approach strengthens resilience".to_string(),
                    "Connection quality amplifies impact".to_string(),
                    "Technology can enhance collaboration".to_string(),
                ],
                evidence: vec![
                    SupportingEvidence {
                        description: "Participation metrics from reflection session".to_string(),
                        evidence_type: EvidenceType::CommunityFeedback,
                        source: "Community Reflection Platform".to_string(),
                        link: None,
                    }
                ],
                related_documents: vec![],
            },
            authors: vec!["Community Reflection Facilitation Team".to_string()],
            impact_snapshot: None,
            tags: vec!["reflection".to_string(), "resilience".to_string(), "connection".to_string()],
            timestamp: Utc::now() - chrono::Duration::days(30),
            status: DocumentationStatus::Published,
        },
        CommunityDocumentationRecord {
            id: Uuid::new_v4(),
            title: "Recommendation: Strengthen Cross-Domain Connections".to_string(),
            doc_type: DocumentationType::Recommendation,
            content: DocumentationContent {
                summary: "Community recommends creating structured opportunities for cross-domain collaboration.".to_string(),
                details: "Based on community insights from interpretation and reflection sessions, we recommend developing monthly cross-domain coffee chats and a connection-mapping tool to visualize collaboration opportunities.".to_string(),
                insights: vec![
                    "Cross-domain interaction enhances innovation".to_string(),
                    "Structured connection opportunities increase engagement".to_string(),
                ],
                evidence: vec![
                    SupportingEvidence {
                        description: "Community feedback on connection needs".to_string(),
                        evidence_type: EvidenceType::CommunityFeedback,
                        source: "Community Reflection Session".to_string(),
                        link: None,
                    }
                ],
                related_documents: vec![],
            },
            authors: vec!["Community Coordination Team".to_string()],
            impact_snapshot: None,
            tags: vec!["recommendation".to_string(), "connection".to_string(), "collaboration".to_string()],
            timestamp: Utc::now() - chrono::Duration::days(15),
            status: DocumentationStatus::Published,
        },
    ]
}