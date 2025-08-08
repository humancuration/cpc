use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct FeedbackCollectorProps {
    pub component_id: String,
    pub on_feedback_submit: Callback<FeedbackData>,
}

#[derive(Clone, Debug)]
pub struct FeedbackData {
    pub component_id: String,
    pub helpful: bool,
    pub rating: Option<u8>, // 1-5 stars
    pub comment: Option<String>,
}

#[styled_component(FeedbackCollector)]
pub fn feedback_collector(props: &FeedbackCollectorProps) -> Html {
    let container_style = style!(
        r#"
        background: var(--surface);
        border-radius: 8px;
        padding: 1rem;
        box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        margin-top: 1rem;
    "#
    ).unwrap();

    let title_style = style!(
        r#"
        margin-top: 0;
        margin-bottom: 1rem;
        font-size: 1rem;
        color: var(--text-primary);
    "#
    ).unwrap();

    let button_group_style = style!(
        r#"
        display: flex;
        gap: 0.5rem;
        margin-bottom: 1rem;
    "#
    ).unwrap();

    let feedback_button_style = style!(
        r#"
        background: var(--background-secondary);
        border: 1px solid var(--border-color);
        border-radius: 4px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 0.9rem;
        
        &:hover {
            background: var(--primary);
            color: white;
        }
        
        &.selected {
            background: var(--primary);
            color: white;
        }
    "#
    ).unwrap();

    let comment_style = style!(
        r#"
        width: 100%;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        margin-bottom: 1rem;
        font-family: inherit;
    "#
    ).unwrap();

    let submit_button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        border-radius: 4px;
        padding: 0.5rem 1rem;
        cursor: pointer;
        font-size: 0.9rem;
        
        &:hover {
            background: #2980b9;
        }
    "#
    ).unwrap();

    let helpful = use_state(|| None::<bool>);
    let rating = use_state(|| None::<u8>);
    let comment = use_state(|| String::new());

    let on_helpful_click = {
        let helpful = helpful.clone();
        Callback::from(move |_| helpful.set(Some(true)))
    };

    let on_not_helpful_click = {
        let helpful = helpful.clone();
        Callback::from(move |_| helpful.set(Some(false)))
    };

    let on_rating_click = {
        let rating = rating.clone();
        Callback::from(move |value: u8| rating.set(Some(value)))
    };

    let on_comment_input = {
        let comment = comment.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            comment.set(input.value());
        })
    };

    let on_submit = {
        let helpful = helpful.clone();
        let rating = rating.clone();
        let comment = comment.clone();
        let component_id = props.component_id.clone();
        let on_feedback_submit = props.on_feedback_submit.clone();
        
        Callback::from(move |_| {
            let feedback_data = FeedbackData {
                component_id: component_id.clone(),
                helpful: helpful.as_ref().copied().unwrap_or(false),
                rating: *rating,
                comment: if comment.is_empty() { None } else { Some(comment.to_string()) },
            };
            on_feedback_submit.emit(feedback_data);
            
            // Reset form
            helpful.set(None);
            rating.set(None);
            comment.set(String::new());
        })
    };

    html! {
        <div class={container_style}>
            <h4 class={title_style}>{"Was this visualization helpful?"}</h4>
            
            <div class={button_group_style.clone()}>
                <button 
                    class={feedback_button_style.clone() + if *helpful == Some(true) { " selected" } else { "" }}
                    onclick={on_helpful_click}
                >
                    {"Yes"}
                </button>
                <button 
                    class={feedback_button_style.clone() + if *helpful == Some(false) { " selected" } else { "" }}
                    onclick={on_not_helpful_click}
                >
                    {"No"}
                </button>
            </div>
            
            if helpful.is_some() {
                <div>
                    <h4 class={title_style.clone()}>{"Rate this visualization"}</h4>
                    <div class={button_group_style}>
                        {(1..=5).map(|i| {
                            let rating_callback = on_rating_click.clone();
                            let rating_class = feedback_button_style.clone() + if *rating == Some(i) { " selected" } else { "" };
                            html! {
                                <button 
                                    class={rating_class}
                                    onclick={move |_| rating_callback.emit(i)}
                                >
                                    {i}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                    
                    <h4 class={title_style.clone()}>{"Additional comments (optional)"}</h4>
                    <textarea
                        class={comment_style}
                        placeholder="How could this visualization be improved?"
                        value={(*comment).clone()}
                        oninput={on_comment_input}
                    />
                    
                    <button class={submit_button_style} onclick={on_submit}>
                        {"Submit Feedback"}
                    </button>
                </div>
            }
        </div>
    }
}