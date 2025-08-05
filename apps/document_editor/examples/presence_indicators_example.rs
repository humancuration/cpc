//! Example showing how to use presence indicators in the document editor

use cpc_document_editor::presentation::{
    PresenceSidebar, CursorOverlay, StatusIndicator, AvatarBadge
};
use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};
use uuid::Uuid;
use std::collections::HashMap;
use yew::prelude::*;

/// Example component showing presence indicators in action
#[function_component(PresenceExample)]
fn presence_example() -> Html {
    // Create some sample presence data
    let mut users = HashMap::new();
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    users.insert(user1_id, PresenceUser {
        avatar_url: Some("https://example.com/user1.png".to_string()),
        color: "#ff0000".to_string(),
        status: PresenceStatus::Online,
    });
    
    users.insert(user2_id, PresenceUser {
        avatar_url: None,
        color: "#00ff00".to_string(),
        status: PresenceStatus::Away,
    });
    
    // Sample cursor positions
    let mut cursor_positions = HashMap::new();
    cursor_positions.insert(user1_id, (5, 10)); // Line 5, Column 10
    cursor_positions.insert(user2_id, (3, 25)); // Line 3, Column 25
    
    let on_user_click = Callback::from(|user_id: Uuid| {
        web_sys::console::log_1(&format!("User clicked: {}", user_id).into());
    });

    html! {
        <div style="position: relative; width: 100%; height: 100vh;">
            <div style="padding: 20px;">
                <h1>{"Document Editor with Presence Indicators"}</h1>
                <p>{"This example shows how presence indicators work in the CPC Document Editor."}</p>
                
                // Status indicators example
                <div style="margin: 20px 0;">
                    <h3>{"Status Indicators:"}</h3>
