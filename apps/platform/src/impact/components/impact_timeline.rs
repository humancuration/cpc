use yew::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub impact_value: f64,
    pub category: String,
}

#[derive(Properties, PartialEq)]
pub struct ImpactTimelineProps {
    pub timeline: Vec<TimelineEvent>,
}

#[function_component(ImpactTimelineComponent)]
pub fn impact_timeline_component(props: &ImpactTimelineProps) -> Html {
    let sorted_timeline = {
        let mut timeline = props.timeline.clone();
        timeline.sort_by(|a, b| b.date.cmp(&a.date));
        timeline
    };

    let format_date = |date: DateTime<Utc>| {
        date.format("%b %d, %Y").to_string()
    };

    let get_category_color = |category: &str| -> &'static str {
        match category.to_lowercase().as_str() {
            "environment" => "#10b981",
            "community" => "#f59e0b",
            "workers" => "#3b82f6",
            _ => "#6b7280",
        }
    };

    html! {
        <div class="impact-timeline">
            <h3>{ "Impact Timeline" }</h3>
            <div class="timeline-container">
                if sorted_timeline.is_empty() {
                    <div class="empty-state">
                        <p>{ "No impact events recorded yet." }</p>
                    </div>
                } else {
                    <div class="timeline">
                        {for sorted_timeline.iter().map(|event| {
                            let color = get_category_color(&event.category);
                            html! {
                                <div class="timeline-item">
                                    <div class="timeline-marker" style={format!("background-color: {}", color)}></div>
                                    <div class="timeline-content">
                                        <div class="timeline-header">
                                            <h4>{ &event.title }</h4>
                                            <span class="timeline-date">{ format_date(event.date) }</span>
                                        </div>
                                        <p class="timeline-description">{ &event.description }</p>
                                        <div class="timeline-impact">
                                            <span class="impact-value" style={format!("color: {}", color)}>
                                                { format!("+{:.2}", event.impact_value) }
                                            </span>
                                            <span class="impact-category">{ &event.category }</span>
                                        </div>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                }
            </div>
        </div>
    }
}
