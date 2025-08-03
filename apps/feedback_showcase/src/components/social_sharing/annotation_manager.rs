//! Annotation manager component for adding and displaying annotations on visualizations

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use crate::components::visualization::types::{Annotation, Permission, PermissionLevel};
use crate::services::collaboration::CollaborationService;
use uuid::Uuid;
use chrono::Utc;
use std::rc::Rc;

#[derive(Properties, PartialEq)]
pub struct AnnotationManagerProps {
    pub share_id: String,
    pub on_add_annotation: Callback<Annotation>,
    pub annotations: Vec<Annotation>,
    #[prop_or_default]
    pub initial_position: Option<(f32, f32)>,
    #[prop_or_default]
    pub container_dimensions: (u32, u32),
    #[prop_or(false)]
    pub show_form: bool,
    #[prop_or_default]
    pub on_form_toggle: Callback<()>,
    #[prop_or_default]
    pub current_user_id: String,
    #[prop_or_default]
    pub collaboration_service: Option<Rc<CollaborationService>>,
}

#[function_component(AnnotationManager)]
pub fn annotation_manager(props: &AnnotationManagerProps) -> Html {
    let show_form = use_state(|| props.show_form);
    let content = use_state(|| "".to_string());
    let position = use_state(|| props.initial_position);
    
    // Update position when initial_position changes
    {
        let position = position.clone();
        let initial_position = props.initial_position;
        use_effect_with(initial_position, move |&initial_position| {
            position.set(initial_position);
            || ()
        });
    }
    
    // Update show_form when props.show_form changes
    {
        let show_form = show_form.clone();
        let props_show_form = props.show_form;
        use_effect_with(props_show_form, move |&props_show_form| {
            show_form.set(props_show_form);
            || ()
        });
    }
    
    let on_toggle_form = {
        let show_form = show_form.clone();
        let position = position.clone();
        let initial_position = props.initial_position;
        let on_form_toggle = props.on_form_toggle.clone();
        Callback::from(move |_| {
            show_form.set(!*show_form);
            // Reset position when toggling form
            if *show_form {
                position.set(initial_position);
            }
            on_form_toggle.emit(());
        })
    };
    
    let on_content_input = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_dyn_into::<HtmlTextAreaElement>().unwrap();
            content.set(target.value());
        })
    };
    
    let on_submit = {
        let show_form = show_form.clone();
        let content = content.clone();
        let position = position.clone();
        let on_add_annotation = props.on_add_annotation.clone();
        let share_id = props.share_id.clone();
        let current_user_id = props.current_user_id.clone();
        let collaboration_service = props.collaboration_service.clone();
        Callback::from(move |_| {
            if !content.is_empty() {
                // Parse mentions from content
                let mentions = if let Some(ref service) = collaboration_service {
                    service.parse_mentions(&content)
                } else {
                    Vec::new()
                };
                
                let annotation = Annotation::new(
                    Uuid::new_v4(),
                    share_id.clone(),
                    current_user_id.clone(),
                    content.to_string(),
                    *position,
                );
                
                // Update mentions and version
                let mut annotation = annotation;
                annotation.mentions = mentions;
                annotation.version = 1;
                
                // Broadcast update via collaboration service if available
                if let Some(ref service) = collaboration_service {
                    service.broadcast_update(&share_id, &annotation);
                }
                
                on_add_annotation.emit(annotation);
                content.set("".to_string());
                show_form.set(false);
            }
        })
    };
    
    let on_cancel = {
        let show_form = show_form.clone();
        let content = content.clone();
        Callback::from(move |_| {
            content.set("".to_string());
            show_form.set(false);
        })
    };
    
    html! {
        <div class="annotation-manager">
            <div class="annotation-header">
                <h3>{"Annotations"}</h3>
                <button 
                    class="add-annotation-btn" 
                    onclick={on_toggle_form}
                >
                    if *show_form {
                        {"Cancel"}
                    } else {
                        {"Add Annotation"}
                    }
                </button>
            </div>
            
            if *show_form {
                <div
                    class="annotation-form positioned"
                    style={
                        if let Some((x, y)) = props.initial_position {
                            let (width, height) = props.container_dimensions;
                            format!("left: {}px; top: {}px;", x * width as f32, y * height as f32)
                        } else {
                            "left: 0; top: 0;".to_string()
                        }
                    }
                >
                    <textarea
                        placeholder="Enter your annotation..."
                        value={(*content).clone()}
                        oninput={on_content_input}
                        rows="3"
                    />
                    <div class="annotation-form-actions">
                        <button
                            class="cancel-annotation-btn"
                            onclick={on_cancel}
                        >
                            {"Cancel"}
                        </button>
                        <button
                            class="submit-annotation-btn"
                            onclick={on_submit}
                            disabled={content.is_empty()}
                        >
                            {"Add Annotation"}
                        </button>
                    </div>
                </div>
            }
            
            <div class="annotations-list">
                if props.annotations.is_empty() {
                    <p class="no-annotations">{"No annotations yet. Be the first to add one!"}</p>
                } else {
                    {for props.annotations.iter().map(|annotation| {
                        // Ensure backward compatibility for existing annotations
                        let mut annotation = annotation.clone();
                        annotation.ensure_compatibility(&props.current_user_id);
                        
                        let can_edit = if let Some(ref service) = props.collaboration_service {
                            service.check_permission(&annotation, &props.current_user_id, PermissionLevel::Edit)
                        } else {
                            annotation.user_id == props.current_user_id
                        };
                        
                        let content = if !annotation.mentions.is_empty() {
                            // Highlight mentions in the content
                            let mut highlighted_content = annotation.content.clone();
                            for mention in &annotation.mentions {
                                highlighted_content = highlighted_content.replace(
                                    &format!("@{}", mention),
                                    &format!("<mark>@{}</mark>", mention)
                                );
                            }
                            highlighted_content
                        } else {
                            annotation.content.clone()
                        };
                        
                        html! {
                            <div class="annotation-item">
                                <div class="annotation-content">
                                    {Html::from_html_unchecked(content.into())}
                                </div>
                                <div class="annotation-meta">
                                    <span class="annotation-user">
                                        if annotation.user_id == props.current_user_id {
                                            {"You"}
                                        } else {
                                            {format!("User: {}", annotation.user_id)}
                                        }
                                    </span>
                                    <span class="annotation-timestamp">
                                        {annotation.timestamp.format("%Y-%m-%d %H:%M").to_string()}
                                    </span>
                                    if !annotation.mentions.is_empty() {
                                        <span class="annotation-mentions">
                                            {"Mentions: "}
                                            {for annotation.mentions.iter().map(|mention| {
                                                html! { <span class="mention-tag">@{mention}</span> }
                                            })}
                                        </span>
                                    }
                                </div>
                                if can_edit {
                                    <div class="annotation-actions">
                                        <button class="edit-annotation-btn">{"Edit"}</button>
                                        <button class="delete-annotation-btn">{"Delete"}</button>
                                    </div>
                                }
                            </div>
                        }
                    })}
                }
            </div>
        </div>
    }
}