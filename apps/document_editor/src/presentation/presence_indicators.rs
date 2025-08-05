use yew::prelude::*;
use stylist::{style, yew::styled_component};
use uuid::Uuid;
use std::collections::HashMap;
use shared_packages::realtime_signaling::message::{PresenceUser, PresenceStatus};
use crate::presentation::position_translator::PositionTranslator;
use crate::presentation::presence_batcher::CursorVirtualizer;

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

    // Memoize users to avoid re-rendering when users haven't changed
    let prev_users = use_state(|| HashMap::<Uuid, PresenceUser>::new());
    let should_render = use_memo(&props.users, |current_users| {
        // Only re-render if users have changed
        !prev_users.eq(current_users) || prev_users.len() != current_users.len()
    });
    
    // Update previous users
    {
        let prev_users = prev_users.clone();
        let current_users = props.users.clone();
        use_effect_with(current_users.clone(), move |_| {
            prev_users.set(current_users);
        });
    }

    let users_list = if *should_render {
        props.users.iter().map(|(user_id, user)| {
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
        }).collect::<Html>()
    } else {
        // Reuse previous users list if no changes
        html! {}
    };

    html! {
        <div class={sidebar_style}>
            <h3>{"Present Users"}</h3>
            {users_list}
        </div>
    
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
    
        // Memoize cursor positions to avoid re-rendering when positions haven't changed
        let prev_positions = use_state(|| HashMap::<Uuid, (usize, usize)>::new());
        let should_render = use_memo((&props.cursor_positions, &props.users), |(current_positions, current_users)| {
            // Only re-render if positions or users have changed
            !prev_positions.eq(current_positions) || prev_positions.len() != current_positions.len()
        });
        
        // Update previous positions
        {
            let prev_positions = prev_positions.clone();
            let current_positions = props.cursor_positions.clone();
            use_effect_with(current_positions.clone(), move |_| {
                prev_positions.set(current_positions);
            });
        }
    
        let cursors = if *should_render {
            // Create a position translator for accurate positioning
            let translator = PositionTranslator::new();
            
            // Create a cursor virtualizer for performance
            let mut virtualizer = CursorVirtualizer::new(1920.0, 1080.0); // Default viewport size
            virtualizer.set_char_dimensions(8.0, 20.0); // Default character dimensions
            
            // Filter to only visible cursors
            let visible_cursors = virtualizer.filter_visible_cursors(&props.cursor_positions);
            
            visible_cursors.iter().map(|(user_id, (line, column))| {
                if let Some(user) = props.users.get(user_id) {
                    // Use accurate positioning instead of approximations
                    let (left, top) = translator.document_to_screen(*line, *column);
                    
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
                        left = left,
                        top = top
                    }
                    .expect("Failed to create cursor style");
    
                    html! {
                        <div class={cursor_style}></div>
                    }
                } else {
                    html! {}
                }
            }).collect::<Html>()
        } else {
            // Reuse previous cursors if no changes
            html! {}
        };
};
    };

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