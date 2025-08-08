//! Demo of the feedback collector component
//!
//! This example demonstrates how to use the feedback collector component
//! in a volunteer impact visualization.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use impact_viz::components::feedback_collector::{FeedbackCollector, FeedbackData};

/// Demo component showing the feedback collector in action
#[styled_component(FeedbackCollectorDemo)]
fn feedback_collector_demo() -> Html {
    let demo_style = style!(
        r#"
        .demo-container {
            max-width: 800px;
            margin: 20px auto;
            padding: 20px;
            font-family: Arial, sans-serif;
        }
        
        .demo-header {
            text-align: center;
            margin-bottom: 30px;
        }
        
        .demo-title {
            color: #2c3e50;
            margin-bottom: 10px;
        }
        
        .demo-description {
            color: #7f8c8d;
            margin-bottom: 20px;
        }
        
        .visualization-demo {
            background: #f8f9fa;
            border-radius: 8px;
            padding: 20px;
            margin-bottom: 20px;
            text-align: center;
        }
        
        .visualization-placeholder {
            height: 200px;
            background: #e9ecef;
            border-radius: 4px;
            display: flex;
            align-items: center;
            justify-content: center;
            color: #6c757d;
            margin-bottom: 20px;
        }
        
        .feedback-results {
            background: white;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            margin-top: 20px;
        }
        
        .results-title {
            margin-top: 0;
            color: #2c3e50;
        }
        
        .result-item {
            padding: 10px;
            border-bottom: 1px solid #eee;
        }
        
        .result-item:last-child {
            border-bottom: none;
        }
    "#
    ).unwrap();

    let feedback_results = use_state(|| Vec::<FeedbackData>::new());

    let on_feedback_submit = {
        let feedback_results = feedback_results.clone();
        Callback::from(move |feedback: FeedbackData| {
            let mut results = (*feedback_results).clone();
            results.push(feedback);
            feedback_results.set(results);
        })
    };

    html! {
        <div class={demo_style}>
            <div class="demo-header">
                <h1 class="demo-title">{"Feedback Collector Demo"}</h1>
                <p class="demo-description">{"Demonstrating the feedback collector component for volunteer impact visualizations"}</p>
            </div>
            
            <div class="visualization-demo">
                <h2>{"Volunteer Impact Visualization"}</h2>
                <div class="visualization-placeholder">
                    {"[Visualization Content Would Appear Here]"}
                </div>
                <p>{"This is a placeholder for an actual volunteer impact visualization."}</p>
                
                <FeedbackCollector 
                    component_id="volunteer_impact_demo_001"
                    on_feedback_submit={on_feedback_submit}
                />
            </div>
            
            if !feedback_results.is_empty() {
                <div class="feedback-results">
                    <h3 class="results-title">{"Feedback Submissions"}</h3>
                    { for feedback_results.iter().map(|feedback| {
                        html! {
                            <div class="result-item">
                                <strong>{format!("Helpful: {}", if feedback.helpful { "Yes" } else { "No" })}</strong>
                                if let Some(rating) = feedback.rating {
                                    <p>{format!("Rating: {} stars", rating)}</p>
                                }
                                if let Some(comment) = &feedback.comment {
                                    <p>{format!("Comment: {}", comment)}</p>
                                }
                                if let Some(impact) = &feedback.decision_impact {
                                    <p>{format!("Decision Impact: {}", impact)}</p>
                                }
                            </div>
                        }
                    }) }
                </div>
            }
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<FeedbackCollectorDemo>::new().render();
}