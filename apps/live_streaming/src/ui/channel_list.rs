//! Channel list component using Yew and Stylist

use yew::prelude::*;
use stylist::{style, yew::styled_component};

/// Properties for the ChannelList component
#[derive(Properties, PartialEq)]
pub struct ChannelListProps {
    /// List of channels
    pub channels: Vec<ChannelInfo>,
    
    /// Callback for when a channel is selected
    #[prop_or_default]
    pub on_select: Callback<String>,
}

/// Information about a channel
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelInfo {
    /// Channel ID
    pub id: String,
    
    /// Channel name
    pub name: String,
    
    /// Channel description
    pub description: String,
    
    /// Current viewer count
    pub viewer_count: u32,
    
    /// URL to the channel's profile image
    pub profile_image_url: String,
}

/// Channel list component
#[styled_component(ChannelList)]
pub fn channel_list(props: &ChannelListProps) -> Html {
    let style = style!(
        r#"
        .channel-list {
            background-color: #1f1f1f;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
            padding: 1rem;
        }
        
        .channel-list-title {
            color: #ffffff;
            margin-bottom: 1rem;
            font-size: 1.5rem;
        }
        
        .channel-item {
            display: flex;
            align-items: center;
            padding: 0.75rem;
            border-bottom: 1px solid #333333;
            cursor: pointer;
            transition: background-color 0.3s;
        }
        
        .channel-item:last-child {
            border-bottom: none;
        }
        
        .channel-item:hover {
            background-color: #2d2d2d;
        }
        
        .channel-avatar {
            width: 50px;
            height: 50px;
            border-radius: 50%;
            margin-right: 1rem;
            background-color: #333333;
            object-fit: cover;
        }
        
        .channel-info {
            flex: 1;
        }
        
        .channel-name {
            color: #ffffff;
            font-weight: bold;
            margin-bottom: 0.25rem;
        }
        
        .channel-description {
            color: #aaaaaa;
            font-size: 0.9rem;
            margin-bottom: 0.25rem;
        }
        
        .channel-viewers {
            color: #888888;
            font-size: 0.8rem;
        }
    "#
    ).expect("Failed to create style");

    html! {
        <div class={style}>
            <div class="channel-list">
                <h2 class="channel-list-title">{"Live Channels"}</h2>
                {for props.channels.iter().map(|channel| {
                    let on_select = props.on_select.clone();
                    let channel_id = channel.id.clone();
                    let onclick = Callback::from(move |_| on_select.emit(channel_id.clone()));
                    
                    html! {
                        <div class="channel-item" {onclick}>
                            <img 
                                src={channel.profile_image_url.clone()} 
                                alt={format!("{}'s profile", channel.name)}
                                class="channel-avatar"
                            />
                            <div class="channel-info">
                                <div class="channel-name">{&channel.name}</div>
                                <div class="channel-description">{&channel.description}</div>
                                <div class="channel-viewers">{format!("{} viewers", channel.viewer_count)}</div>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
}