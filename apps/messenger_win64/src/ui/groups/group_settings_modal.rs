//! Component for group settings modal

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;

/// Properties for the GroupSettingsModal component
#[derive(Properties, PartialEq)]
pub struct GroupSettingsModalProps {
    /// The ID of the conversation to update settings for
    pub conversation_id: Uuid,
    
    /// Callback when the modal is closed
    pub on_close: Callback<()>,
}

/// Group settings form data
#[derive(Debug, Clone)]
pub struct GroupSettings {
    pub name: String,
    pub description: String,
    pub require_approval: bool,
}

/// A component that displays a modal for group settings
#[styled_component(GroupSettingsModal)]
pub fn group_settings_modal(props: &GroupSettingsModalProps) -> Html {
    let css = Style::new(r#"
        .modal-overlay {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.5);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1000;
        }
        
        .modal-content {
            background: white;
            border-radius: 8px;
            box-shadow: 0 4px 20px rgba(0,0,0,0.15);
            width: 90%;
            max-width: 500px;
            max-height: 90vh;
            overflow-y: auto;
        }
        
        .modal-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px 20px;
            border-bottom: 1px solid #eee;
        }
        
        .modal-title {
            font-size: 18px;
            font-weight: bold;
            margin: 0;
        }
        
        .close-button {
            background: none;
            border: none;
            font-size: 24px;
            cursor: pointer;
            color: #999;
            padding: 0;
            width: 30px;
            height: 30px;
            display: flex;
            align-items: center;
            justify-content: center;
            border-radius: 50%;
        }
        
        .close-button:hover {
            background: #f0f0f0;
            color: #333;
        }
        
        .modal-body {
            padding: 20px;
        }
        
        .form-group {
            margin-bottom: 20px;
        }
        
        .form-label {
            display: block;
            margin-bottom: 8px;
            font-weight: 500;
            color: #333;
        }
        
        .form-input, .form-textarea {
            width: 100%;
            padding: 10px;
            border: 1px solid #ddd;
            border-radius: 4px;
            font-size: 14px;
            box-sizing: border-box;
        }
        
        .form-textarea {
            min-height: 80px;
            resize: vertical;
        }
        
        .form-checkbox {
            display: flex;
            align-items: center;
            margin-top: 8px;
        }
        
        .form-checkbox input {
            margin-right: 8px;
        }
        
        .modal-footer {
            display: flex;
            justify-content: flex-end;
            gap: 12px;
            padding: 16px 20px;
            border-top: 1px solid #eee;
        }
        
        .btn {
            padding: 8px 16px;
            border-radius: 4px;
            font-size: 14px;
            cursor: pointer;
            border: none;
        }
        
        .btn-secondary {
            background: #f0f0f0;
            color: #333;
        }
        
        .btn-primary {
            background: #007bff;
            color: white;
        }
        
        .btn:disabled {
            opacity: 0.6;
            cursor: not-allowed;
        }
        
        .btn:hover:not(:disabled) {
            opacity: 0.9;
        }
        
        .loading {
            text-align: center;
            padding: 20px;
        }
        
        .error {
            color: #d32f2f;
            font-size: 14px;
            margin-top: 8px;
        }
    "#).expect("style");

    let settings = use_state(|| GroupSettings {
        name: String::new(),
        description: String::new(),
        require_approval: false,
    });
    
    let loading = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let saving = use_state(|| false);

    // Fetch current settings when the component mounts
    {
        let settings = settings.clone();
        let loading = loading.clone();
        let error = error.clone();
        let conversation_id = props.conversation_id;
        
        use_effect_with(conversation_id, move |_| {
            let settings = settings.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                // In a real implementation, we would fetch the current group settings
                // For now, we'll just use empty/default settings
                settings.set(GroupSettings {
                    name: String::new(),
                    description: String::new(),
                    require_approval: false,
                });
                
                loading.set(false);
            });
            
            || ()
        });
    }

    let on_name_input = {
        let settings = settings.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.name = input.value();
            settings.set(current_settings);
        })
    };

    let on_description_input = {
        let settings = settings.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.description = input.value();
            settings.set(current_settings);
        })
    };

    let on_require_approval_change = {
        let settings = settings.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_settings = (*settings).clone();
            current_settings.require_approval = input.checked();
            settings.set(current_settings);
        })
    };

    let on_save = {
        let settings = settings.clone();
        let saving = saving.clone();
        let error = error.clone();
        let on_close = props.on_close.clone();
        let conversation_id = props.conversation_id;
        
        Callback::from(move |_| {
            let settings = settings.clone();
            let saving = saving.clone();
            let error = error.clone();
            let on_close = on_close.clone();
            let conversation_id = conversation_id;
            
            spawn_local(async move {
                if *saving {
                    return;
                }
                
                saving.set(true);
                error.set(None);
                
                // Call the GraphQL mutation to update group settings
                let settings_data = settings.as_ref();
                let query = format!(r#"
                    mutation {{
                        updateGroupSettings(
                            conversationId: "{}",
                            settings: {{
                                name: {},
                                description: {},
                                requireApproval: {}
                            }}
                        )
                    }}
                "#, 
                conversation_id,
                if settings_data.name.is_empty() { "null".to_string() } else { format!("\"{}\"", settings_data.name) },
                if settings_data.description.is_empty() { "null".to_string() } else { format!("\"{}\"", settings_data.description) },
                if settings_data.require_approval { "true" } else { "false" }
                );
                
                let response = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(query)
                    .send()
                    .await;
                
                match response {
                    Ok(resp) => {
                        match resp.text().await {
                            Ok(_text) => {
                                // In a real implementation, we would parse the GraphQL response properly
                                // For now, we'll just close the modal
                                saving.set(false);
                                on_close.emit(());
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to read response: {:?}", e)));
                                saving.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to update settings: {:?}", e)));
                        saving.set(false);
                    }
                }
            });
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| on_close.emit(()))
    };

    if *loading {
        return html! {
            <div class={css}>
                <div class="modal-overlay">
                    <div class="modal-content">
                        <div class="loading">{"Loading settings..."}</div>
                    </div>
                </div>
            </div>
        };
    }

    let current_settings = settings.as_ref();

    html! {
        <div class={css}>
            <div class="modal-overlay">
                <div class="modal-content">
                    <div class="modal-header">
                        <h2 class="modal-title">{"Group Settings"}</h2>
                        <button class="close-button" onclick={on_close}>{"×"}</button>
                    </div>
                    
                    <div class="modal-body">
                        <div class="form-group">
                            <label class="form-label">{"Group Name"}</label>
                            <input
                                type="text"
                                class="form-input"
                                value={current_settings.name.clone()}
                                oninput={on_name_input}
                                placeholder="Enter group name"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label class="form-label">{"Description"}</label>
                            <textarea
                                class="form-textarea"
                                value={current_settings.description.clone()}
                                oninput={on_description_input}
                                placeholder="Enter group description"
                            />
                        </div>
                        
                        <div class="form-group">
                            <div class="form-checkbox">
                                <input
                                    type="checkbox"
                                    id="require-approval"
                                    checked={current_settings.require_approval}
                                    onchange={on_require_approval_change}
                                />
                                <label for="require-approval">{"Require approval for new members"}</label>
                            </div>
                        </div>
                        
                        if let Some(err) = &*error {
                            <div class="error">{err}</div>
                        }
                    </div>
                    
                    <div class="modal-footer">
                        <button class="btn btn-secondary" onclick={on_close}>
                            {"Cancel"}
                        </button>
                        <button 
                            class="btn btn-primary" 
                            onclick={on_save}
                            disabled={*saving}
                        >
                            if *saving { "Saving..." } else { "Save Changes" }
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}