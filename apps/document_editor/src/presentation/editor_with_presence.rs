//! Document editor component with presence indicators integration

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use uuid::Uuid;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus, SignalingMessage, PresenceUpdate, CursorPosition, PresenceSummary, ViewportUpdate};
use shared_packages::realtime_signaling::RedisSignalingService;
use chrono::{Utc, DateTime};
use gloo_timers::future::TimeoutFuture;

// Import our presence components
use crate::presentation::{
    PresenceSidebar,
    CursorOverlay,
    StatusIndicator,
    AvatarBadge
};
use crate::presentation::position_translator::SharedPositionTranslator;
use crate::presentation::presence_state::{PresenceStateManager, UserPresenceState};
use crate::presentation::viewport_sync::ViewportSyncManager;

/// Properties for the document editor with presence indicators
#[derive(Properties, PartialEq)]
pub struct DocumentEditorWithPresenceProps {
    pub document_id: Uuid,
    pub current_user_id: Uuid,
}

/// Document editor component with presence indicators
#[styled_component(DocumentEditorWithPresence)]
pub fn document_editor_with_presence(props: &DocumentEditorWithPresenceProps) -> Html {
    // State for users presence
    let presence_state = use_state(|| PresenceStateManager::new(1000));
    
    // Extract separate state maps for compatibility with existing components
    let users = use_state(|| HashMap::<Uuid, PresenceUser>::new());
    let cursor_positions = use_state(|| HashMap::<Uuid, (usize, usize)>::new());
    let last_activity = use_state(|| HashMap::<Uuid, DateTime<Utc>>::new());
    
    // State for rate limiting cursor updates
    let last_cursor_update = use_state(|| std::time::Instant::now());
    
    // State for typing detection
    let is_typing = use_state(|| false);
    let last_typing_event = use_state(|| std::time::Instant::now());
    
    // State for viewport synchronization
    let viewport_manager = use_state(|| ViewportSyncManager::new());
    let last_viewport_update = use_state(|| std::time::Instant::now());
    
    // Initialize signaling service
    let signaling_service = use_state(|| {
        // Initialize with actual Redis connection using environment variables or defaults
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let channel_prefix = std::env::var("SIGNALING_CHANNEL_PREFIX").unwrap_or_else(|_| "cpc_signaling".to_string());
        
        match RedisSignalingService::new(&redis_url, Some(channel_prefix)) {
            Ok(service) => Rc::new(service),
            Err(e) => {
                web_sys::console::error_1(&format!("Failed to initialize Redis signaling service: {:?}", e).into());
                // Fallback to a default service
                Rc::new(RedisSignalingService::new("redis://localhost:6379", Some("cpc_signaling".to_string())).unwrap())
            }
        }
    });
    
    // Effect to initialize signaling connection
    {
        let presence_state = presence_state.clone();
        let users = users.clone();
        let cursor_positions = cursor_positions.clone();
        let last_activity = last_activity.clone();
        let signaling_service = signaling_service.clone();
        let document_id = props.document_id;
        let current_user_id = props.current_user_id;
        
        use_effect_with((), move |_| {
            // Initialize connection with exponential backoff
            let presence_state = presence_state.clone();
            let users = users.clone();
            let cursor_positions = cursor_positions.clone();
            let last_activity = last_activity.clone();
            let signaling_service = signaling_service.clone();
            let document_id = document_id;
            let current_user_id = current_user_id;
            
            spawn_local(async move {
                const MAX_RETRIES: u32 = 5;
                let mut retries = 0;
                
                loop {
                    match signaling_service.register_connection(document_id).await {
                        Ok(mut receiver) => {
                            // Send join document message
                            let join_message = SignalingMessage::JoinDocument {
                                document_id,
                                user_id: current_user_id,
                            };
                            
                            if let Err(e) = signaling_service.broadcast_message(document_id, &join_message).await {
                                web_sys::console::error_1(&format!("Failed to send join message: {:?}", e).into());
                            }
                            
                            // Reset retry counter on successful connection
                            retries = 0;
                            
                            // Listen for messages
                            loop {
                                match receiver.recv().await {
                                    Ok(message) => {
                                        if let Ok(signaling_message) = serde_json::from_str::<SignalingMessage>(&message) {
                                            handle_signaling_message(
                                                signaling_message,
                                                presence_state.clone(),
                                                users.clone(),
                                                cursor_positions.clone(),
                                                last_activity.clone(),
                                            );
                                        }
                                    },
                                    Err(e) => {
                                        web_sys::console::error_1(&format!("Failed to receive message: {:?}", e).into());
                                        // Break the loop to trigger reconnection
                                        break;
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            web_sys::console::error_1(&format!("Failed to register connection: {:?}", e).into());
                            
                            // Exponential backoff
                            if retries < MAX_RETRIES {
                                let delay = 2u64.pow(retries) * 100; // 100ms, 200ms, 400ms, 800ms, 1600ms
                                TimeoutFuture::new(delay as u32).await;
                                retries += 1;
                            } else {
                                web_sys::console::error_1(&"Max retries exceeded, giving up on connection".into());
                                break;
                            }
                        }
                    }
                }
            });
            
            // Cleanup function
            || ()
        });
    }
    
    // Effect to handle presence expiration
    {
        let presence_state = presence_state.clone();
        let users = users.clone();
        let cursor_positions = cursor_positions.clone();
        let last_activity = last_activity.clone();
        
        use_effect_with((), move |_| {
            let presence_state = presence_state.clone();
            let users = users.clone();
            let cursor_positions = cursor_positions.clone();
            let last_activity = last_activity.clone();
            
            // Set up interval to check for inactive users
            let interval_id = gloo_timers::callback::Interval::new(5000, move || {
                // Update presence state manager
                let mut presence_state_mut = (*presence_state).clone();
                presence_state_mut.update_all_statuses(5, 30); // 5s away, 30s offline
                presence_state.set(presence_state_mut);
                
                // Update separate state maps for compatibility
                let now = Utc::now();
                let mut users_updated = (*users).clone();
                let mut cursor_positions_updated = (*cursor_positions).clone();
                let last_activity_map = (*last_activity).clone();
                
                // Check each user for inactivity
                for (user_id, last_active) in last_activity_map.iter() {
                    let inactive_duration = now.signed_duration_since(*last_active);
                    let inactive_secs = inactive_duration.num_seconds();
                    
                    if inactive_secs > 30 {
                        // Remove user completely
                        users_updated.remove(user_id);
                        cursor_positions_updated.remove(user_id);
                    } else if inactive_secs > 5 {
                        // Mark user as Away
                        if let Some(user) = users_updated.get_mut(user_id) {
                            user.status = PresenceStatus::Away;
                        }
                    }
                }
                
                // Update state
                users.set(users_updated);
                cursor_positions.set(cursor_positions_updated);
            });
            
            // Cleanup function
            move || {
                interval_id.cancel();
            }
        });
    }
    
    
    let editor_style = style! {
        r#"
        position: relative;
        width: 100%;
        height: 100vh;
        display: flex;
        flex-direction: column;
    "#
    }
    .expect("Failed to create editor style");
    
    let toolbar_style = style! {
        r#"
        padding: 1rem;
        background-color: #f8f9fa;
        border-bottom: 1px solid #dee2e6;
        display: flex;
        align-items: center;
        justify-content: space-between;
    "#
    }
    .expect("Failed to create toolbar style");
    
    let content_style = style! {
        r#"
        flex: 1;
        position: relative;
        overflow: hidden;
    "#
    }
    .expect("Failed to create content style");
    
    let document_area_style = style! {
        r#"
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        padding: 2rem;
        box-sizing: border-box;
        overflow: auto;
    "#
    }
    .expect("Failed to create document area style");
    
    let on_user_click = Callback::from(|user_id: Uuid| {
        web_sys::console::log_1(&format!("User clicked: {}", user_id).into());
    });
    
    // Callback to handle typing events with throttling
    let on_typing = {
        let signaling_service = signaling_service.clone();
        let is_typing = is_typing.clone();
        let last_typing_event = last_typing_event.clone();
        let document_id = props.document_id;
        let current_user_id = props.current_user_id;
        
        Callback::from(move |_| {
            let now = std::time::Instant::now();
            let last_event = *last_typing_event;
            
            // Throttle typing updates to 500ms
            if now.duration_since(last_event).as_millis() >= 500 {
                last_typing_event.set(now);
                is_typing.set(true);
                
                let signaling_service = signaling_service.clone();
                let document_id = document_id;
                let current_user_id = current_user_id;
                
                spawn_local(async move {
                    let typing_message = SignalingMessage::TypingIndicator {
                        document_id,
                        user_id: current_user_id,
                        is_typing: true,
                        timestamp: Utc::now(),
                    };
                    
                    if let Err(e) = signaling_service.broadcast_message(document_id, &typing_message).await {
                        web_sys::console::error_1(&format!("Failed to send typing indicator: {:?}", e).into());
                    }
                });
                
                // Set a timeout to clear typing status after 1 second of inactivity
                let is_typing = is_typing.clone();
                let signaling_service = signaling_service.clone();
                let document_id = document_id;
                let current_user_id = current_user_id;
                
                spawn_local(async move {
                    TimeoutFuture::new(1000).await;
                    
                    is_typing.set(false);
                    
                    let typing_message = SignalingMessage::TypingIndicator {
                        document_id,
                        user_id: current_user_id,
                        is_typing: false,
                        timestamp: Utc::now(),
                    };
                    
                    if let Err(e) = signaling_service.broadcast_message(document_id, &typing_message).await {
                        web_sys::console::error_1(&format!("Failed to send typing indicator: {:?}", e).into());
                    }
                });
            }
        })
    };
    
    // Callback to handle cursor position updates with rate limiting
    let on_cursor_update = {
        let signaling_service = signaling_service.clone();
        let last_cursor_update = last_cursor_update.clone();
        let document_id = props.document_id;
        let current_user_id = props.current_user_id;
        
        Callback::from(move |(line, column): (usize, usize)| {
            let now = std::time::Instant::now();
            let last_update = *last_cursor_update;
            
            // Rate limit to 10 updates per second (100ms minimum between updates)
            if now.duration_since(last_update).as_millis() >= 100 {
                last_cursor_update.set(now);
                
                let signaling_service = signaling_service.clone();
                let document_id = document_id;
                let current_user_id = current_user_id;
                
                spawn_local(async move {
                    let cursor_message = SignalingMessage::CursorUpdate(CursorPosition {
                        document_id,
                        user_id: current_user_id,
                        position: shared_packages::realtime_signaling::message::Position {
                            line,
                            column,
                        },
                        timestamp: Utc::now(),
                    });
                    
                    if let Err(e) = signaling_service.broadcast_message(document_id, &cursor_message).await {
                        web_sys::console::error_1(&format!("Failed to send cursor update: {:?}", e).into());
                    }
                });
            }
        })
    };
    
    html! {
        <div class={editor_style}>
            // Toolbar with presence indicators
            <div class={toolbar_style}>
                <div>
                    <h2>{"Document Editor"}</h2>
                </div>
                <div>
                    // Show current user's avatar and status
                    if let Some(current_user) = users.get(&props.current_user_id) {
                        <div style="display: flex; align-items: center;">
                            <AvatarBadge
                                avatar_url={current_user.avatar_url.clone()}
                                color={current_user.color.clone()}
                                user_id={props.current_user_id}
                                is_typing={{
                                    if let Some(state) = presence_state.get(&props.current_user_id) {
                                        state.is_typing
                                    } else {
                                        false
                                    }
                                }}
                            />
                            <div style="margin-left: 0.5rem;">
                                <StatusIndicator status={current_user.status.clone()} />
                            </div>
                        </div>
                    }
                </div>
            </div>
            
            // Main content area with presence indicators
            <div class={content_style}>
                // Document content area
                <div class={document_area_style}>
                    <h1>{"Welcome to the Document Editor"}</h1>
                    <p>{"This is a collaborative document editor with real-time presence indicators."}</p>
                    <p>{"You can see other users in the sidebar on the right."}</p>
                    <p>{"Their cursors are shown as colored bars in the document."}</p>
                </div>
                
                // Presence indicators
                <CursorOverlay
                    users={(*users).clone()}
                    cursor_positions={(*cursor_positions).clone()}
                />
                <PresenceSidebar
                    users={(*users).clone()}
                    on_user_click={on_user_click}
                />
            </div>
        </div>
    }
}

/// Handle incoming signaling messages
fn handle_signaling_message(
    message: SignalingMessage,
    presence_state: UseStateHandle<PresenceStateManager>,
    users: UseStateHandle<HashMap<Uuid, PresenceUser>>,
    cursor_positions: UseStateHandle<HashMap<Uuid, (usize, usize)>>,
    last_activity: UseStateHandle<HashMap<Uuid, DateTime<Utc>>>,
) {
    match message {
        SignalingMessage::PresenceUpdate(update) => {
            handle_presence_update(update, presence_state, users, last_activity);
        },
        SignalingMessage::PresenceSummary(summary) => {
            handle_presence_summary(summary, presence_state, users, last_activity);
        },
        SignalingMessage::CursorUpdate(cursor) => {
            handle_cursor_update(cursor, presence_state, cursor_positions, last_activity);
        },
        SignalingMessage::TypingIndicator { user_id, is_typing, timestamp } => {
            handle_typing_indicator(user_id, is_typing, timestamp, presence_state, users);
        },
        SignalingMessage::ViewportUpdate(viewport) => {
            handle_viewport_update(viewport, presence_state, users, last_activity);
        },
        SignalingMessage::JoinDocument { user_id, .. } => {
            // Update last activity for joined user
            let mut last_activity_map = (*last_activity).clone();
            last_activity_map.insert(user_id, Utc::now());
            last_activity.set(last_activity_map);
        },
        SignalingMessage::LeaveDocument { user_id, .. } => {
            // Remove user from presence
            let mut presence_state_mut = (*presence_state).clone();
            presence_state_mut.remove(&user_id);
            presence_state.set(presence_state_mut);
            
            let mut users_map = (*users).clone();
            let mut cursor_positions_map = (*cursor_positions).clone();
            let mut last_activity_map = (*last_activity).clone();
            
            users_map.remove(&user_id);
            cursor_positions_map.remove(&user_id);
            last_activity_map.remove(&user_id);
            
            users.set(users_map);
            cursor_positions.set(cursor_positions_map);
            last_activity.set(last_activity_map);
        },
        _ => {
            // Ignore other message types
        }
    }
}

/// Handle presence update message
fn handle_presence_update(
    update: PresenceUpdate,
    presence_state: UseStateHandle<PresenceStateManager>,
    users: UseStateHandle<HashMap<Uuid, PresenceUser>>,
    last_activity: UseStateHandle<HashMap<Uuid, DateTime<Utc>>>,
) {
    // Update presence state manager
    let mut presence_state_mut = (*presence_state).clone();
    let state = crate::presentation::presence_state::converters::presence_update_to_state(&update);
    presence_state_mut.insert(update.user_id, state);
    presence_state.set(presence_state_mut);
    
    // Update separate state maps for compatibility
    let mut users_map = (*users).clone();
    let mut last_activity_map = (*last_activity).clone();
    
    // Create presence user from update
    let presence_user = PresenceUser {
        avatar_url: update.avatar_url.clone(),
        color: update.color.clone(),
        status: if update.is_typing {
            PresenceStatus::Online
        } else {
            PresenceStatus::Online // Default to online, expiration logic will handle away status
        },
    };
    
    users_map.insert(update.user_id, presence_user);
    last_activity_map.insert(update.user_id, update.last_active);
    
    users.set(users_map);
    last_activity.set(last_activity_map);
}

/// Handle typing indicator message
fn handle_typing_indicator(
    user_id: Uuid,
    is_typing: bool,
    timestamp: DateTime<Utc>,
    presence_state: UseStateHandle<PresenceStateManager>,
    users: UseStateHandle<HashMap<Uuid, PresenceUser>>,
) {
    // Update presence state manager
    let mut presence_state_mut = (*presence_state).clone();
    if let Some(state) = presence_state_mut.get_mut(&user_id) {
        state.set_typing(is_typing);
    }
    presence_state.set(presence_state_mut);
    
    // Update separate state maps for compatibility
    let mut users_map = (*users).clone();
    if let Some(user) = users_map.get_mut(&user_id) {
        // Update status based on typing state
        if is_typing {
            user.status = PresenceStatus::Online;
        }
    }
    users.set(users_map);
    
    // Update last activity
    // (This is handled by the presence state manager)
}

/// Handle presence summary message
fn handle_presence_summary(
    summary: PresenceSummary,
    presence_state: UseStateHandle<PresenceStateManager>,
    users: UseStateHandle<HashMap<Uuid, PresenceUser>>,
    last_activity: UseStateHandle<HashMap<Uuid, DateTime<Utc>>>,
) {
    // Update presence state manager
    let mut presence_state_mut = (*presence_state).clone();
    for (user_id, presence_user) in &summary.users {
        if let Some(mut state) = presence_state_mut.get_mut(user_id) {
            state.update_user(presence_user.clone());
        } else {
            let state = UserPresenceState::new(presence_user.clone(), Utc::now());
            presence_state_mut.insert(*user_id, state);
        }
    }
    presence_state.set(presence_state_mut);
    
    // Update separate state maps for compatibility
    let mut users_map = (*users).clone();
    let mut last_activity_map = (*last_activity).clone();
    
    // Update all users from summary
    for (user_id, presence_user) in summary.users {
        users_map.insert(user_id, presence_user);
        // For summary, we don't have last_active, so we'll use current time
        last_activity_map.insert(user_id, Utc::now());
    }
    
    users.set(users_map);
    last_activity.set(last_activity_map);
}
/// Handle cursor update message
fn handle_cursor_update(
    cursor: CursorPosition,
    presence_state: UseStateHandle<PresenceStateManager>,
    cursor_positions: UseStateHandle<HashMap<Uuid, (usize, usize)>>,
    last_activity: UseStateHandle<HashMap<Uuid, DateTime<Utc>>>,
) {
    // Update presence state manager
    let mut presence_state_mut = (*presence_state).clone();
    if let Some(state) = presence_state_mut.get_mut(&cursor.user_id) {
        state.update_cursor_position((cursor.position.line, cursor.position.column));
    }
    presence_state.set(presence_state_mut);
    
    // Update separate state maps for compatibility
    let mut cursor_positions_map = (*cursor_positions).clone();
    let mut last_activity_map = (*last_activity).clone();
    
    // Update cursor position
    cursor_positions_map.insert(cursor.user_id, (cursor.position.line, cursor.position.column));
    
    // Update last activity
    last_activity_map.insert(cursor.user_id, cursor.timestamp);
    
    cursor_positions.set(cursor_positions_map);
    last_activity.set(last_activity_map);
}

/// Handle viewport update message
fn handle_viewport_update(
    viewport: ViewportUpdate,
    presence_state: UseStateHandle<PresenceStateManager>,
    users: UseStateHandle<HashMap<Uuid, PresenceUser>>,
    last_activity: UseStateHandle<HashMap<Uuid, DateTime<Utc>>>,
) {
    // Update presence state manager
    let mut presence_state_mut = (*presence_state).clone();
    if let Some(state) = presence_state_mut.get_mut(&viewport.user_id) {
        crate::presentation::presence_state::converters::viewport_update_to_state(&viewport, state);
    }
    presence_state.set(presence_state_mut);
    
    // Update last activity
    let mut last_activity_map = (*last_activity).clone();
    last_activity_map.insert(viewport.user_id, viewport.timestamp);
    last_activity.set(last_activity_map);
}
}