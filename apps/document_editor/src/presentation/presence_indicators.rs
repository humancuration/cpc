use yew::prelude::*;
use stylist::{style, yew::styled_component};
use uuid::Uuid;
use std::collections::HashMap;
use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};

// Properties for the PresenceSidebar component
#[derive(Properties, PartialEq)]
pub struct PresenceSidebarProps {
    pub users: HashMap<Uuid, PresenceUser>,
    pub on_user_click: Callback<Uuid>,
}

// Properties for the CursorOverlay component
#[derive(Properties, PartialEq)]
pub struct CursorOverlayProps {
    pub users: HashMap<Uuid, PresenceUser>,
    pub cursor_positions: HashMap<Uuid, (usize, usize)>, // line, column
}

// Properties for the StatusIndicator component
#[derive(Properties, PartialEq)]
pub struct StatusIndicatorProps {
    pub status: PresenceStatus,
}

// Properties for the AvatarBadge component
#[derive(Properties, PartialEq)]
pub struct AvatarBadgeProps {
    pub avatar_url: Option<String>,
    pub color: String,
    pub user_id: Uuid,
    pub is_typing: bool,
}

/// Sidebar component showing all users present in the document
#[styled_component(PresenceSidebar)]
pub fn presence_sidebar(props: &PresenceSidebarProps) -> Html {
    let sidebar_style = style! {
        r#"
        position: absolute;
        right: 0;
        top: 0;
        width: 250px;
        height: 100%;
        background-color: #f8f9fa;
        border-left: 1px solid #dee2e6;
        padding: 1rem;
        overflow-y: auto;
        z-index: 100;
    "#
    }
    .expect("Failed to create sidebar style");

    let user_item_style = style! {
        r#"
        display: flex;
        align-items: center;
        padding: 0.5rem;
        margin-bottom: 0.5rem;
        border-radius: 4px;
        cursor: pointer;
        transition: background-color 0.2s;
        
        &:hover {
            background-color: #e9ecef;
        }
    "#
    }
    .expect("Failed to create user item style");

    let users_list = props.users.iter().map(|(user_id, user)| {
        let user_id = *user_id;
        let on_click = {
            let on_user_click = props.on_user_click.clone();
            Callback::from(move |_| on_user_click.emit(user_id))
        };

        html! {
            <div 
                class={user_item_style.clone()} 
                onclick={on_click}
            >
                <AvatarBadge 
                    avatar_url={user.avatar_url.clone()} 
                    color={user.color.clone()} 
                    user_id={user_id}
                    is_typing={matches!(user.status, PresenceStatus::Online)}
                />
                <div style="margin-left: 0.5rem;">
                    <div>{format!("User {}", &user_id.to_string()[..8])}</div>
                    <StatusIndicator status={user.status.clone()} />
                </div>
            </div>
        }
    }).collect::<Html>();

    html! {
        <div class={sidebar_style}>
            <h3>{"Present Users"}</h3>
            {users_list}
        </div>
    }
}

/// Overlay component showing cursor positions of other users
#[styled_component(CursorOverlay)]
pub fn cursor_overlay(props: &CursorOverlayProps) -> Html {
    let overlay_style = style! {
        r#"
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        pointer-events: none;
        z-index: 10;
    "#
    }
    .expect("Failed to create overlay style");

    let cursors = props.cursor_positions.iter().map(|(user_id, (line, column))| {
        if let Some(user) = props.users.get(user_id) {
            let cursor_style = style! {
                r#"
                position: absolute;
                width: 2px;
                height: 1.2em;
                background-color: ${color};
                left: ${left}px;
                top: ${top}px;
                animation: blink 1s infinite;
                
                @keyframes blink {
                    0% { opacity: 1; }
                    50% { opacity: 0.5; }
                    100% { opacity: 1; }
                }
            "#,
                color = user.color,
                left = column * 8, // Approximate character width
                top = line * 20    // Approximate line height
            }
            .expect("Failed to create cursor style");

            html! {
                <div class={cursor_style}></div>
            }
        } else {
            html! {}
        }
    }).collect::<Html>();

    html! {
        <div class={overlay_style}>
            {cursors}
        </div>
    }
}

/// Status indicator showing user presence status
#[styled_component(StatusIndicator)]
pub fn status_indicator(props: &StatusIndicatorProps) -> Html {
    let (color, text) = match props.status {
        PresenceStatus::Online => ("#28a745", "Online"),
        PresenceStatus::Away => ("#ffc107", "Away"),
        PresenceStatus::Busy => ("#dc3545", "Busy"),
        PresenceStatus::Offline => ("#6c757d", "Offline"),
    };

    let indicator_style = style! {
        r#"
        display: inline-block;
        width: 8px;
        height: 8px;
        border-radius: 50%;
        background-color: ${color};
        margin-right: 0.5rem;
    "#,
        color = color
    }
    .expect("Failed to create indicator style");

    html! {
        <div>
            <span class={indicator_style}></span>
            <span>{text}</span>
        </div>
    }
}

/// Avatar badge showing user avatar or colored initial
#[styled_component(AvatarBadge)]
pub fn avatar_badge(props: &AvatarBadgeProps) -> Html {
    let avatar_style = style! {
        r#"
        display: inline-block;
        width: 32px;
        height: 32px;
        border-radius: 50%;
        background-color: ${color};
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-weight: bold;
        ${typing_animation}
    "#,
        color = props.color,
        typing_animation = if props.is_typing {
            "animation: pulse 1s infinite;"
        } else {
            ""
        }
    }
    .expect("Failed to create avatar style");

    let typing_style = style! {
        r#"
        @keyframes pulse {
            0% { transform: scale(1); }
            50% { transform: scale(1.1); }
            100% { transform: scale(1); }
        }
        "#
    }
    .expect("Failed to create typing style");

    if let Some(url) = &props.avatar_url {
        html! {
            <img 
                src={url.clone()} 
                alt="User avatar" 
                class={avatar_style}
                style="object-fit: cover;"
            />
        }
    } else {
        let initial = format!("U{}", &props.user_id.to_string()[0..1]);
        html! {
            <div class={avatar_style}>
                <style>{typing_style}</style>
                {initial}
            </div>
        }
    }
}