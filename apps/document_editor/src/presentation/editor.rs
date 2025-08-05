use yew::prelude::*;
use collaboration_engine::presence::UserPresence;
use collaboration_engine::core::Position;
use realtime_signaling::SignalingClient;
use uuid::Uuid;
use std::collections::HashMap;
use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus, SignalingMessage, PresenceUpdate, CursorPosition};
use crate::presentation::{PresenceSidebar, CursorOverlay, StatusIndicator, AvatarBadge};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use shared_packages::operational_transformation::{Operation, TextOperation, VersionVector, transform_text_operations, apply, OtError, merge_ranges};
use std::collections::VecDeque;

#[derive(Properties, PartialEq)]
pub struct DocumentEditorProps {
    pub document_id: String,
    pub signaling_client: SignalingClient,
}

#[function_component(DocumentEditor)]
pub fn document_editor(props: &DocumentEditorProps) -> Html {
    let document_content = use_state(|| String::from("Start typing your document here..."));
    let signaling_client = use_state(|| props.signaling_client.clone());
    
    // OT state
    let pending_operations = use_state(|| VecDeque::<TextOperation>::new());
    let version_vector = use_state(|| VersionVector::new());
    let document_version = use_state(|| 0u64);
    let logical_clock = use_state(|| 0u64);
    
    // Presence state
    let presence_users = use_state(|| HashMap::<Uuid, PresenceUser>::new());
    let cursor_positions = use_state(|| HashMap::<Uuid, (usize, usize)>::new()); // line, column
    
    // Current user information (in a real implementation, this would come from auth context)
    let current_user_id = Uuid::new_v4(); // Placeholder
    let current_user_status = use_state(|| PresenceStatus::Online);
    let current_user_color = "#007bff".to_string(); // Bootstrap blue
    let current_user_avatar = None; // No avatar for now
    let is_typing = use_state(|| false);
    
/// Process an incoming text operation
fn process_operation(
    op: TextOperation,
    document_content: &str,
    pending_operations: &VecDeque<TextOperation>,
    version_vector: &VersionVector,
    document_version: u64,
) -> Result<(String, VersionVector, u64), OtError> {
    let mut transformed_op = op.clone();
    
    for pending_op in pending_operations {
        if pending_op.user_id != op.user_id {
            let (_, transformed) = transform_text_operations(pending_op, &op)?;
            transformed_op = TextOperation {
                op: transformed,
                ..transformed_op
            };
        }
    }
    
    // Apply transformed_op and update state...
    // Apply the transformed operation to the document
    let new_content = apply(document_content, &transformed_op.op)?;
    
    // Update version vector
    let mut vv = version_vector.clone();
    vv.update_with(&transformed_op);
    
    Ok((new_content, vv, document_version + 1))
}
    // Connect to signaling server on component mount
    {
        let signaling_client = signaling_client.clone();
        let presence_users = presence_users.clone();
        let cursor_positions = cursor_positions.clone();
        let current_user_status = current_user_status.clone();
        
        use_effect_with((), move |_| {
            let signaling_client = signaling_client.clone();
            let presence_users = presence_users.clone();
            let cursor_positions = cursor_positions.clone();
            let current_user_status = current_user_status.clone();
            
            spawn_local(async move {
                // Set up message handler for presence updates
                let client = signaling_client.clone();
                spawn_local(async move {
                    // In a real implementation, we would set up a message handler
                    // This is a simplified example
                    client.on_message(|message| {
                        match message {
                            SignalingMessage::PresenceUpdate(update) => {
                                let mut users = (*presence_users).clone();
                                users.insert(update.user_id, PresenceUser {
                                    avatar_url: update.avatar_url.clone(),
                                    color: update.color.clone(),
                                    status: PresenceStatus::Online, // Simplified
                                });
                                presence_users.set(users);
                                
                                if let Some(cursor) = update.cursor {
                                    let mut positions = (*cursor_positions).clone();
                                    positions.insert(update.user_id, (cursor.line, cursor.column));
                                    cursor_positions.set(positions);
                                }
                            },
                            SignalingMessage::CursorUpdate(cursor_update) => {
                                let mut positions = (*cursor_positions).clone();
                                positions.insert(cursor_update.user_id, (cursor_update.position.line, cursor_update.position.column));
                                cursor_positions.set(positions);
                            },
                            SignalingMessage::PresenceStatus { status, .. } => {
                                // Update current user status
                                let status = match status.as_str() {
                                    "online" => PresenceStatus::Online,
                                    "away" => PresenceStatus::Away,
                                    "busy" => PresenceStatus::Busy,
                                    _ => PresenceStatus::Offline,
                                };
                                current_user_status.set(status);
                            },
                            SignalingMessage::TextOperation(op) => {
                                // Process incoming text operation
                                let current_content = (*document_content).clone();
                                let pending = (*pending_operations).clone();
                                let vv = (*version_vector).clone();
                                let version = *document_version;
                                
                                match process_operation(op, &current_content, &pending, &vv, version) {
                                    Ok((new_content, new_vv, new_version)) => {
                                        document_content.set(new_content);
                                        version_vector.set(new_vv);
                                        document_version.set(new_version);
                                    }
                                    Err(e) => {
                                        web_sys::console::error_1(&format!("Failed to process operation: {:?}", e).into());
                                    }
                                }
                            },
                            _ => {}
                        }
                    });
                });
            });
            
            || ()
        });
    }
    
    let oninput = {
        let document_content = document_content.clone();
        let signaling_client = signaling_client.clone();
        let is_typing = is_typing.clone();
        let pending_operations = pending_operations.clone();
        let version_vector = version_vector.clone();
        let document_version = document_version.clone();
        let logical_clock = logical_clock.clone();
        let current_user_id = current_user_id.clone();
        
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let new_value = input.value();
            
            // In a real implementation, we would compare with the previous value
            // to generate the appropriate operation
            // For now, we'll just send a placeholder operation
            let op = TextOperation {
                op: Operation::Retain { length: new_value.len() },
                user_id: current_user_id,
                version: *document_version,
                logical_clock: *logical_clock,
                timestamp: chrono::Utc::now(),
            };
            
            // Increment logical clock for next operation
            logical_clock.set(*logical_clock + 1);
            
            // Add to pending operations
            let mut pending = (*pending_operations).clone();
            pending.push_back(op.clone());
            pending_operations.set(pending);
            
            // Update document content
            document_content.set(new_value.clone());
            
            // Update typing state
            is_typing.set(!new_value.is_empty());
            
            // Send operation to other clients
            let signaling_client = signaling_client.clone();
            let op_clone = op.clone();
            spawn_local(async move {
                let message = SignalingMessage::TextOperation(op_clone);
                if let Err(e) = signaling_client.send_message(&message).await {
                    web_sys::console::error_1(&format!("Failed to send operation: {:?}", e).into());
                }
            });
        })
    };
    
    let on_user_selected = Callback::from(|user_id: Uuid| {
        web_sys::console::log_1(&format!("User selected: {}", user_id).into());
    });
    
    html! {
        <div class="document-editor">
            // Toolbar with presence indicators
            <div class="toolbar" style="display: flex; justify-content: space-between; padding: 1rem; background-color: #f8f9fa; border-bottom: 1px solid #dee2e6;">
                <div>
                    <h1>{"Document Editor"}</h1>
                </div>
                <div style="display: flex; align-items: center; gap: 1rem;">
                    <StatusIndicator status={(*current_user_status).clone()} />
                    <AvatarBadge 
                        avatar_url={current_user_avatar.clone()} 
                        color={current_user_color.clone()} 
                        user_id={current_user_id}
                        is_typing={*is_typing}
                    />
                </div>
            </div>
            
            // Main editor area with presence indicators
            <div class="editor-container" style="position: relative; display: flex;">
                <div style="flex: 1; position: relative;">
                    <textarea
                        class="editor-textarea"
                        value={(*document_content).clone()}
                        oninput={oninput}
                        rows="20"
                        cols="80"
                        style="width: 100%; height: 100%; padding: 1rem; box-sizing: border-box;"
                    />
                    
                    // Presence indicators
                    <CursorOverlay 
                        users={(*presence_users).clone()} 
                        cursor_positions={(*cursor_positions).clone()} 
                    />
                </div>
                
                <PresenceSidebar 
                    users={(*presence_users).clone()} 
                    on_user_click={on_user_selected} 
                />
            </div>
            
            <div class="editor-actions" style="padding: 1rem; background-color: #f8f9fa; border-top: 1px solid #dee2e6;">
                <button>{"Save"}</button>
                <button>{"Export"}</button>
            </div>
        </div>
    }
}