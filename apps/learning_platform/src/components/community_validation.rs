use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct CommunityValidationProps {
    pub component_id: String,
    pub on_validation_submit: Callback<ValidationData>,
}

#[derive(Clone, Debug)]
pub struct ValidationData {
    pub component_id: String,
    pub validation_type: ValidationType,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationType {
    Endorsement,
    Critique,
    Suggestion,
    Question,
}

#[styled_component(CommunityValidation)]
pub fn community_validation(props: &CommunityValidationProps) -> Html {
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
        flex-wrap: wrap;
    "#
    ).unwrap();

    let validation_button_style = style!(
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

    let textarea_style = style!(
        r#"
        width: 100%;
        padding: 0.5rem;
        border: 1px solid var(--border-color);
        border-radius: 4px;
        margin-bottom: 1rem;
        font-family: inherit;
        min-height: 100px;
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

    let validation_type = use_state(|| None::<ValidationType>);
    let content = use_state(|| String::new());

    let set_validation_type = {
        let validation_type = validation_type.clone();
        Callback::from(move |vt: ValidationType| validation_type.set(Some(vt)))
    };

    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_submit = {
        let validation_type = validation_type.clone();
        let content = content.clone();
        let component_id = props.component_id.clone();
        let on_validation_submit = props.on_validation_submit.clone();
        
        Callback::from(move |_| {
            if let Some(vt) = &*validation_type {
                let validation_data = ValidationData {
                    component_id: component_id.clone(),
                    validation_type: vt.clone(),
                    content: content.to_string(),
                };
                on_validation_submit.emit(validation_data);
                
                // Reset form
                validation_type.set(None);
                content.set(String::new());
            }
        })
    };

    let is_submit_disabled = validation_type.is_none() || content.is_empty();

    html! {
        <div class={container_style}>
            <h4 class={title_style}>{"Community Validation"}</h4>
            
            <div class={button_group_style.clone()}>
                <button 
                    class={validation_button_style.clone() + if *validation_type == Some(ValidationType::Endorsement) { " selected" } else { "" }}
                    onclick={move |_| set_validation_type.emit(ValidationType::Endorsement)}
                >
                    {"👍 Endorse"}
                </button>
                <button 
                    class={validation_button_style.clone() + if *validation_type == Some(ValidationType::Critique) { " selected" } else { "" }}
                    onclick={move |_| set_validation_type.emit(ValidationType::Critique)}
                >
                    {"👎 Critique"}
                </button>
                <button 
                    class={validation_button_style.clone() + if *validation_type == Some(ValidationType::Suggestion) { " selected" } else { "" }}
                    onclick={move |_| set_validation_type.emit(ValidationType::Suggestion)}
                >
                    {"💡 Suggest"}
                </button>
                <button 
                    class={validation_button_style.clone() + if *validation_type == Some(ValidationType::Question) { " selected" } else { "" }}
                    onclick={move |_| set_validation_type.emit(ValidationType::Question)}
                >
                    {"❓ Question"}
                </button>
            </div>
            
            if validation_type.is_some() {
                <div>
                    <h4 class={title_style.clone()}>
                        {match &*validation_type {
                            Some(ValidationType::Endorsement) => "Endorse this visualization",
                            Some(ValidationType::Critique) => "Provide constructive critique",
                            Some(ValidationType::Suggestion) => "Suggest improvements",
                            Some(ValidationType::Question) => "Ask a question",
                            None => "Enter your feedback",
                        }}
                    </h4>
                    <textarea
                        class={textarea_style}
                        placeholder={match &*validation_type {
                            Some(ValidationType::Endorsement) => "What do you appreciate about this visualization?",
                            Some(ValidationType::Critique) => "How could this visualization be improved?",
                            Some(ValidationType::Suggestion) => "What specific improvements would you suggest?",
                            Some(ValidationType::Question) => "What would you like to know about this visualization?",
                            None => "Enter your feedback...",
                        }}
                        value={(*content).clone()}
                        oninput={on_content_input}
                    />
                    
                    <button 
                        class={submit_button_style} 
                        onclick={on_submit}
                        disabled={is_submit_disabled}
                    >
                        {"Submit Validation"}
                    </button>
                </div>
            }
        </div>
    }
}