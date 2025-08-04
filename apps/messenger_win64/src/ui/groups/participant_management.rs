//! Component for managing group participants

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

/// Properties for the ParticipantManagement component
#[derive(Properties, PartialEq)]
pub struct ParticipantManagementProps {
    /// The ID of the conversation to manage participants for
    pub conversation_id: Uuid,
}

/// A participant in the group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Participant {
    pub user_id: Uuid,
    pub username: String,
    pub is_admin: bool,
    pub can_send_messages: bool,
    pub can_manage_participants: bool,
    pub can_change_settings: bool,
    pub can_delete_messages: bool,
    pub can_moderate_content: bool,
}

/// Permissions for a participant
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParticipantPermissions {
    pub can_send_messages: bool,
    pub can_manage_participants: bool,
    pub can_change_settings: bool,
    pub can_delete_messages: bool,
    pub can_moderate_content: bool,
    pub is_admin: bool,
}

/// A component that manages group participants
#[styled_component(ParticipantManagement)]
pub fn participant_management(props: &ParticipantManagementProps) -> Html {
    let css = Style::new(r#"
        .participant-management {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 16px;
            margin-top: 16px;
        }
        
        .section-title {
            font-size: 18px;
            font-weight: bold;
            margin: 0 0 16px 0;
            color: #333;
        }
        
        .participants-list {
            max-height: 400px;
            overflow-y: auto;
        }
        
        .participant-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 12px 0;
            border-bottom: 1px solid #f0f0f0;
        }
        
        .participant-item:last-child {
            border-bottom: none;
        }
        
        .participant-info {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        
        .participant-avatar {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background: #007bff;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: bold;
        }
        
        .participant-details {
            display: flex;
            flex-direction: column;
        }
        
        .participant-name {
            font-weight: 500;
            color: #333;
        }
        
        .participant-role {
            font-size: 12px;
            color: #666;
        }
        
        .participant-actions {
            display: flex;
            gap: 8px;
        }
        
        .action-button {
            background: none;
            border: 1px solid #ddd;
            border-radius: 4px;
            padding: 6px 10px;
            font-size: 12px;
            cursor: pointer;
            color: #333;
        }
        
        .action-button:hover {
            background: #f0f0f0;
        }
        
        .action-button.danger:hover {
            background: #d32f2f;
            color: white;
            border-color: #d32f2f;
        }
        
        .permissions-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
            gap: 12px;
            margin-top: 8px;
        }
        
        .permission-item {
            display: flex;
            align-items: center;
            gap: 6px;
        }
        
        .permission-item input {
            margin: 0;
        }
        
        .permission-item label {
            font-size: 12px;
            color: #666;
        }
        
        .loading, .error {
            text-align: center;
            padding: 20px;
        }
        
        .error {
            color: #d32f2f;
        }
        
        .no-participants {
            text-align: center;
            padding: 20px;
            color: #666;
        }
    "#).expect("style");

    let participants = use_state(|| Vec::<Participant>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let editing_permissions = use_state(|| Option::<Uuid>::None);

    // Fetch participants when the component mounts
    {
        let participants = participants.clone();
        let loading = loading.clone();
        let error = error.clone();
        let conversation_id = props.conversation_id;
        
        use_effect_with(conversation_id, move |_| {
            let participants = participants.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                // In a real implementation, we would fetch the participants from GraphQL
                // For now, we'll just use placeholder data
                participants.set(vec![
                    Participant {
                        user_id: Uuid::new_v4(),
                        username: "Alice".to_string(),
                        is_admin: true,
                        can_send_messages: true,
                        can_manage_participants: true,
                        can_change_settings: true,
                        can_delete_messages: true,
                        can_moderate_content: true,
                    },
                    Participant {
                        user_id: Uuid::new_v4(),
                        username: "Bob".to_string(),
                        is_admin: false,
                        can_send_messages: true,
                        can_manage_participants: false,
                        can_change_settings: false,
                        can_delete_messages: false,
                        can_moderate_content: false,
                    },
                    Participant {
                        user_id: Uuid::new_v4(),
                        username: "Charlie".to_string(),
                        is_admin: false,
                        can_send_messages: true,
                        can_manage_participants: false,
                        can_change_settings: false,
                        can_delete_messages: false,
                        can_moderate_content: false,
                    },
                ]);
                
                loading.set(false);
            });
            
            || ()
        });
    }

    let on_toggle_permissions = {
        let editing_permissions = editing_permissions.clone();
        Callback::from(move |user_id: Uuid| {
            let editing_permissions = editing_permissions.clone();
            let current = *editing_permissions;
            
            if let Some(current_id) = current {
                if current_id == user_id {
                    editing_permissions.set(None);
                } else {
                    editing_permissions.set(Some(user_id));
                }
            } else {
                editing_permissions.set(Some(user_id));
            }
        })
    };

    let on_update_permissions = {
        let participants = participants.clone();
        let editing_permissions = editing_permissions.clone();
        let conversation_id = props.conversation_id;
        
        Callback::from(move |(user_id, permissions): (Uuid, ParticipantPermissions)| {
            let participants = participants.clone();
            let editing_permissions = editing_permissions.clone();
            let conversation_id = conversation_id;
            
            spawn_local(async move {
                // Call the GraphQL mutation to update participant permissions
                let query = format!(r#"
                    mutation {{
                        updateParticipantPermissions(
                            conversationId: "{}",
                            userId: "{}",
                            permissions: {{
                                canSendMessages: {}
                                canManageParticipants: {}
                                canChangeSettings: {}
                                canDeleteMessages: {}
                                canModerateContent: {}
                                isAdmin: {}
                            }}
                        )
                    }}
                "#, 
                conversation_id, user_id,
                if permissions.can_send_messages { "true" } else { "false" },
                if permissions.can_manage_participants { "true" } else { "false" },
                if permissions.can_change_settings { "true" } else { "false" },
                if permissions.can_delete_messages { "true" } else { "false" },
                if permissions.can_moderate_content { "true" } else { "false" },
                if permissions.is_admin { "true" } else { "false" }
                );
                
                let response = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(query)
                    .send()
                    .await;
                
                match response {
                    Ok(_resp) => {
                        // In a real implementation, we would update the local state
                        // For now, we'll just close the permissions editor
                        editing_permissions.set(None);
                    }
                    Err(e) => {
                        // In a real implementation, we would show an error
                        gloo_console::error!(format!("Failed to update permissions: {:?}", e));
                    }
                }
            });
        })
    };

    let on_remove_participant = {
        let conversation_id = props.conversation_id;
        Callback::from(move |user_id: Uuid| {
            let conversation_id = conversation_id;
            spawn_local(async move {
                // Call the GraphQL mutation to remove participant
                let query = format!(r#"
                    mutation {{
                        banParticipant(
                            conversationId: "{}",
                            userId: "{}"
                        )
                    }}
                "#, conversation_id, user_id);
                
                let response = Request::post("/graphql")
                    .header("Content-Type", "application/json")
                    .body(query)
                    .send()
                    .await;
                
                match response {
                    Ok(_resp) => {
                        // In a real implementation, we would update the local state
                        gloo_console::log!("Participant removed");
                    }
                    Err(e) => {
                        gloo_console::error!(format!("Failed to remove participant: {:?}", e));
                    }
                }
            });
        })
    };

    if *loading {
        return html! {
            <div class={css}>
                <div class="participant-management">
                    <div class="loading">{"Loading participants..."}</div>
                </div>
            </div>
        };
    }

    if let Some(err) = &*error {
        return html! {
            <div class={css}>
                <div class="participant-management">
                    <div class="error">{"Error: "}{err}</div>
                </div>
            </div>
        };
    }

    html! {
        <div class={css}>
            <div class="participant-management">
                <h3 class="section-title">{"Group Participants"}</h3>
                
                if participants.is_empty() {
                    <div class="no-participants">{"No participants in this group."}</div>
                } else {
                    <div class="participants-list">
                        {participants.iter().map(|participant| {
                            let is_editing = *editing_permissions == Some(participant.user_id);
                            let on_toggle = on_toggle_permissions.clone();
                            let user_id = participant.user_id;
                            let on_remove = on_remove_participant.clone();
                            
                            html! {
                                <>
                                    <div class="participant-item">
                                        <div class="participant-info">
                                            <div class="participant-avatar">
                                                {participant.username.chars().next().unwrap_or('U').to_uppercase().to_string()}
                                            </div>
                                            <div class="participant-details">
                                                <div class="participant-name">{&participant.username}</div>
                                                <div class="participant-role">
                                                    {if participant.is_admin { "Admin" } else { "Member" }}
                                                </div>
                                            </div>
                                        </div>
                                        
                                        <div class="participant-actions">
                                            <button 
                                                class="action-button"
                                                onclick={Callback::from(move |_| on_toggle.emit(user_id))}
                                            >
                                                {if is_editing { "Cancel" } else { "Edit" }}
                                            </button>
                                            <button 
                                                class="action-button danger"
                                                onclick={Callback::from(move |_| on_remove.emit(user_id))}
                                            >
                                                {"Remove"}
                                            </button>
                                        </div>
                                    </div>
                                    
                                    if is_editing {
                                        <ParticipantPermissionsEditor 
                                            participant={participant.clone()}
                                            on_update={on_update_permissions.clone()}
                                            on_cancel={on_toggle_permissions.clone()}
                                        />
                                    }
                                </>
                            }
                        }).collect::<Html>()}
                    </div>
                }
            </div>
        </div>
    }
}

/// Properties for the ParticipantPermissionsEditor component
#[derive(Properties, PartialEq)]
struct ParticipantPermissionsEditorProps {
    participant: Participant,
    on_update: Callback<(Uuid, ParticipantPermissions)>,
    on_cancel: Callback<Uuid>,
}

/// A component for editing participant permissions
#[styled_component(ParticipantPermissionsEditor)]
fn participant_permissions_editor(props: &ParticipantPermissionsEditorProps) -> Html {
    let css = Style::new(r#"
        .permissions-editor {
            background: #f9f9f9;
            border-radius: 4px;
            padding: 16px;
            margin: 8px 0 16px 0;
        }
        
        .permissions-title {
            font-size: 14px;
            font-weight: 500;
            margin: 0 0 12px 0;
            color: #333;
        }
        
        .permissions-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
            gap: 12px;
        }
        
        .permission-item {
            display: flex;
            align-items: center;
            gap: 6px;
        }
        
        .permission-item input {
            margin: 0;
        }
        
        .permission-item label {
            font-size: 12px;
            color: #666;
        }
        
        .editor-actions {
            display: flex;
            justify-content: flex-end;
            gap: 8px;
            margin-top: 16px;
        }
        
        .btn {
            padding: 6px 12px;
            border-radius: 4px;
            font-size: 12px;
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
    "#).expect("style");

    let permissions = use_state(|| ParticipantPermissions {
        can_send_messages: props.participant.can_send_messages,
        can_manage_participants: props.participant.can_manage_participants,
        can_change_settings: props.participant.can_change_settings,
        can_delete_messages: props.participant.can_delete_messages,
        can_moderate_content: props.participant.can_moderate_content,
        is_admin: props.participant.is_admin,
    });

    let on_permission_change = {
        let permissions = permissions.clone();
        Callback::from(move |(field, value): (String, bool)| {
            let mut current = (*permissions).clone();
            
            match field.as_str() {
                "can_send_messages" => current.can_send_messages = value,
                "can_manage_participants" => current.can_manage_participants = value,
                "can_change_settings" => current.can_change_settings = value,
                "can_delete_messages" => current.can_delete_messages = value,
                "can_moderate_content" => current.can_moderate_content = value,
                "is_admin" => current.is_admin = value,
                _ => {}
            }
            
            permissions.set(current);
        })
    };

    let on_save = {
        let permissions = permissions.clone();
        let on_update = props.on_update.clone();
        let user_id = props.participant.user_id;
        Callback::from(move |_| {
            let permissions = permissions.clone();
            let on_update = on_update.clone();
            let user_id = user_id;
            on_update.emit((user_id, (*permissions).clone()));
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        let user_id = props.participant.user_id;
        Callback::from(move |_| on_cancel.emit(user_id))
    };

    let current_permissions = permissions.as_ref();

    html! {
        <div class={css}>
            <div class="permissions-editor">
                <h4 class="permissions-title">{"Edit Permissions"}</h4>
                
                <div class="permissions-grid">
                    <div class="permission-item">
                        <input
                            type="checkbox"
                            id="can-send-messages"
                            checked={current_permissions.can_send_messages}
                            onchange={Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                on_permission_change.emit(("can_send_messages".to_string(), input.checked()));
                            })}
                        />
                        <label for="can-send-messages">{"Send Messages"}</label>
                    </div>
                    
                    <div class="permission-item">
                        <input
                            type="checkbox"
                            id="can-manage-participants"
                            checked={current_permissions.can_manage_participants}
                            onchange={Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                on_permission_change.emit(("can_manage_participants".to_string(), input.checked()));
                            })}
                        />
                        <label for="can-manage-participants">{"Manage Participants"}</label>
                    </div>
                    
                    <div class="permission-item">
                        <input
                            type="checkbox"
                            id="can-change-settings"
                            checked={current_permissions.can_change_settings}
                            onchange={Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                on_permission_change.emit(("can_change_settings".to_string(), input.checked()));
                            })}
                        />
                        <label for="can-change-settings">{"Change Settings"}</label>
                    </div>
                    
                    <div class="permission-item">
                        <input
                            type="checkbox"
                            id="can-delete-messages"
                            checked={current_permissions.can_delete_messages}
                            onchange={Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                on_permission_change.emit(("can_delete_messages".to_string(), input.checked()));
                            })}
                        />
                        <label for="can-delete-messages">{"Delete Messages"}</label>
                    </div>
                    
                    <div class="permission-item">
                        <input
                            type="checkbox"
                            id="can-moderate-content"
                            checked={current_permissions.can_moderate_content}
                            onchange={Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                on_permission_change.emit(("can_moderate_content".to_string(), input.checked()));
                            })}
                        />
                        <label for="can-moderate-content">{"Moderate Content"}</label>
                    </div>
                    
                    <div class="permission-item">
                        <input
                            type="checkbox"
                            id="is-admin"
                            checked={current_permissions.is_admin}
                            onchange={Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                on_permission_change.emit(("is_admin".to_string(), input.checked()));
                            })}
                        />
                        <label for="is-admin">{"Admin"}</label>
                    </div>
                </div>
                
                <div class="editor-actions">
                    <button class="btn btn-secondary" onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                    <button class="btn btn-primary" onclick={on_save}>
                        {"Save"}
                    </button>
                </div>
            </div>
        </div>
    }
}