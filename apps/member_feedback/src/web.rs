//! Web Frontend for Member Feedback App
//!
//! A Yew-based web frontend for community members to provide feedback on financial visualizations.

use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use gloo_net::http::Request as HttpRequest;
use serde::{Deserialize, Serialize};

// Define routes
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    FeedbackForm,
    #[at("/success")]
    Success,
}

// Main application component
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container">
                <header class="app-header">
                    <h1>{"Financial Visualization Feedback"}</h1>
                    <p>{"Help us improve financial tools for the community"}</p>
                </header>
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
                <footer class="app-footer">
                    <p>{"Your feedback helps us create better financial tools for everyone."}</p>
                </footer>
            </div>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::FeedbackForm => html! { <FeedbackForm /> },
        Route::Success => html! { <SuccessPage /> },
    }
}

// Feedback form component
#[function_component(FeedbackForm)]
fn feedback_form() -> Html {
    let user_id = use_state(|| String::new());
    let component_id = use_state(|| String::new());
    let rating = use_state(|| 0);
    let comment = use_state(|| String::new());
    let helpful = use_state(|| true);
    let impact_rating = use_state(|| 0);
    let understanding_rating = use_state(|| 0);
    let confidence_rating = use_state(|| 0);
    let is_submitting = use_state(|| false);
    let error = use_state(|| None);
    
    let on_user_id_change = {
        let user_id = user_id.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            user_id.set(input.value());
        })
    };
    
    let on_component_id_change = {
        let component_id = component_id.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            component_id.set(input.value());
        })
    };
    
    let on_rating_change = {
        let rating = rating.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value: u32 = input.value().parse().unwrap_or(0);
            rating.set(value);
        })
    };
    
    let on_comment_change = {
        let comment = comment.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            comment.set(input.value());
        })
    };
    
    let on_helpful_change = {
        let helpful = helpful.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            helpful.set(input.checked());
        })
    };
    
    let on_impact_rating_change = {
        let impact_rating = impact_rating.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value: u32 = input.value().parse().unwrap_or(0);
            impact_rating.set(value);
        })
    };
    
    let on_understanding_rating_change = {
        let understanding_rating = understanding_rating.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value: u32 = input.value().parse().unwrap_or(0);
            understanding_rating.set(value);
        })
    };
    
    let on_confidence_rating_change = {
        let confidence_rating = confidence_rating.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let value: u32 = input.value().parse().unwrap_or(0);
            confidence_rating.set(value);
        })
    };
    
    let on_submit = {
        let user_id = user_id.clone();
        let component_id = component_id.clone();
        let rating = rating.clone();
        let comment = comment.clone();
        let helpful = helpful.clone();
        let impact_rating = impact_rating.clone();
        let understanding_rating = understanding_rating.clone();
        let confidence_rating = confidence_rating.clone();
        let is_submitting = is_submitting.clone();
        let error = error.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let user_id = user_id.clone();
            let component_id = component_id.clone();
            let rating = rating.clone();
            let comment = comment.clone();
            let helpful = helpful.clone();
            let impact_rating = impact_rating.clone();
            let understanding_rating = understanding_rating.clone();
            let confidence_rating = confidence_rating.clone();
            let is_submitting = is_submitting.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                is_submitting.set(true);
                error.set(None);
                
                let feedback_data = DetailedFeedbackRequest {
                    user_id: (*user_id).clone(),
                    component_id: (*component_id).clone(),
                    rating: *rating,
                    comment: if comment.is_empty() { None } else { Some((*comment).clone()) },
                    helpful: *helpful,
                    impact_rating: if *impact_rating > 0 { Some(*impact_rating) } else { None },
                    understanding_rating: if *understanding_rating > 0 { Some(*understanding_rating) } else { None },
                    confidence_rating: if *confidence_rating > 0 { Some(*confidence_rating) } else { None },
                };
                
                match submit_detailed_feedback(feedback_data).await {
                    Ok(_) => {
                        // Navigate to success page
                        if let Err(e) = Navigator::push(&Route::Success) {
                            error.set(Some(format!("Navigation failed: {}", e)));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to submit feedback: {}", e)));
                    }
                }
                
                is_submitting.set(false);
            });
        })
    };
    
    html! {
        <div class="feedback-form-container">
            <form onsubmit={on_submit}>
                <div class="form-group">
                    <label for="user_id">{"Your User ID"}</label>
                    <input
                        type="text"
                        id="user_id"
                        value={(*user_id).clone()}
                        oninput={on_user_id_change}
                        required=true
                    />
                </div>
                
                <div class="form-group">
                    <label for="component_id">{"Visualization Component ID"}</label>
                    <input
                        type="text"
                        id="component_id"
                        value={(*component_id).clone()}
                        oninput={on_component_id_change}
                        required=true
                    />
                </div>
                
                <div class="form-group">
                    <label>{"Was this visualization helpful?"}</label>
                    <div class="radio-group">
                        <label>
                            <input
                                type="radio"
                                name="helpful"
                                checked={*helpful}
                                onchange={on_helpful_change.clone()}
                            />
                            {"Yes"}
                        </label>
                        <label>
                            <input
                                type="radio"
                                name="helpful"
                                checked={!*helpful}
                                onchange={on_helpful_change}
                            />
                            {"No"}
                        </label>
                    </div>
                </div>
                
                <div class="form-group">
                    <label for="rating">{"Overall Rating (1-5)"}</label>
                    <select id="rating" onchange={on_rating_change}>
                        <option value="0">{"Select rating"}</option>
                        <option value="1">{"1 - Poor"}</option>
                        <option value="2">{"2 - Fair"}</option>
                        <option value="3">{"3 - Good"}</option>
                        <option value="4">{"4 - Very Good"}</option>
                        <option value="5">{"5 - Excellent"}</option>
                    </select>
                </div>
                
                <div class="form-group">
                    <label for="comment">{"Additional Comments"}</label>
                    <textarea
                        id="comment"
                        value={(*comment).clone()}
                        oninput={on_comment_change}
                        rows="4"
                    />
                </div>
                
                <div class="rating-group">
                    <h3>{"Detailed Ratings"}</h3>
                    
                    <div class="form-group">
                        <label for="impact_rating">{"Impact on Financial Decisions (1-10)"}</label>
                        <select id="impact_rating" onchange={on_impact_rating_change}>
                            <option value="0">{"Select rating"}</option>
                            {for (1..=10).map(|i| html! { <option value={i.to_string()}>{i}</option> })}
                        </select>
                    </div>
                    
                    <div class="form-group">
                        <label for="understanding_rating">{"Understanding Improvement (1-10)"}</label>
                        <select id="understanding_rating" onchange={on_understanding_rating_change}>
                            <option value="0">{"Select rating"}</option>
                            {for (1..=10).map(|i| html! { <option value={i.to_string()}>{i}</option> })}
                        </select>
                    </div>
                    
                    <div class="form-group">
                        <label for="confidence_rating">{"Confidence Improvement (1-10)"}</label>
                        <select id="confidence_rating" onchange={on_confidence_rating_change}>
                            <option value="0">{"Select rating"}</option>
                            {for (1..=10).map(|i| html! { <option value={i.to_string()}>{i}</option> })}
                        </select>
                    </div>
                </div>
                
                if let Some(err) = &*error {
                    <div class="error-message">
                        {err}
                    </div>
                }
                
                <button
                    type="submit"
                    disabled=*is_submitting
                >
                    if *is_submitting {
                        {"Submitting..."}
                    } else {
                        {"Submit Feedback"}
                    }
                </button>
            </form>
        </div>
    }
}

// Success page component
#[function_component(SuccessPage)]
fn success_page() -> Html {
    let on_submit_another = Callback::from(|_| {
        if let Err(e) = Navigator::push(&Route::FeedbackForm) {
            web_sys::console::error_1(&format!("Navigation failed: {}", e).into());
        }
    });
    
    html! {
        <div class="success-page">
            <div class="success-icon">{"✓"}</div>
            <h2>{"Thank You for Your Feedback!"}</h2>
            <p>{"Your input helps us create better financial tools for the community."}</p>
            <button onclick={on_submit_another}>{"Submit Another Feedback"}</button>
        </div>
    }
}

// Data structures for API requests
#[derive(Serialize)]
struct DetailedFeedbackRequest {
    user_id: String,
    component_id: String,
    rating: u32,
    comment: Option<String>,
    helpful: bool,
    impact_rating: Option<u32>,
    understanding_rating: Option<u32>,
    confidence_rating: Option<u32>,
}

// API functions
async fn submit_detailed_feedback(feedback: DetailedFeedbackRequest) -> Result<(), anyhow::Error> {
    let resp = HttpRequest::post("/api/feedback/detailed")
        .json(&feedback)?
        .send()
        .await?;
    
    if resp.status() == 200 {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to submit feedback"))
    }
}

// Main entry point
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}