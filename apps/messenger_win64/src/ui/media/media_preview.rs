//! Component for previewing media files

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

/// Properties for the MediaPreview component
#[derive(Properties, PartialEq)]
pub struct MediaPreviewProps {
    /// The ID of the media to preview
    pub media_id: Uuid,
}

/// A media reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaReference {
    pub id: Uuid,
    pub media_type: String, // "image", "document", "audio", "video"
    pub storage_location: String,
    pub thumbnail: Option<ThumbnailReference>,
    pub size_bytes: u64,
    pub filename: Option<String>,
}

/// A thumbnail reference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThumbnailReference {
    pub storage_location: String,
    pub width: u32,
    pub height: u32,
}

/// A component that displays a preview of media files
#[styled_component(MediaPreview)]
pub fn media_preview(props: &MediaPreviewProps) -> Html {
    let css = Style::new(r#"
        .media-preview {
            display: inline-block;
            margin: 4px;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
            background: white;
            max-width: 300px;
        }
        
        .media-content {
            width: 100%;
            height: auto;
            display: block;
        }
        
        .media-placeholder {
            width: 200px;
            height: 200px;
            display: flex;
            align-items: center;
            justify-content: center;
            background: #f5f5f5;
            color: #999;
            font-size: 48px;
        }
        
        .media-info {
            padding: 12px;
            font-size: 12px;
            color: #666;
        }
        
        .media-name {
            font-weight: 500;
            margin-bottom: 4px;
            word-break: break-all;
        }
        
        .media-meta {
            display: flex;
            justify-content: space-between;
        }
        
        .media-type {
            text-transform: capitalize;
        }
        
        .media-size {
            color: #999;
        }
        
        .loading, .error {
            padding: 20px;
            text-align: center;
        }
        
        .error {
            color: #d32f2f;
        }
        
        .thumbnail {
            max-width: 100%;
            height: auto;
            display: block;
        }
    "#).expect("style");

    let media = use_state(|| Option::<MediaReference>::None);
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);

    // Fetch media information when the component mounts or media_id changes
    {
        let media = media.clone();
        let loading = loading.clone();
        let error = error.clone();
        let media_id = props.media_id;
        
        use_effect_with(media_id, move |_| {
            let media = media.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                // Call the GraphQL query to get media information
                let query = format!(r#"
                    query {{
                        getMediaUrl(mediaId: "{}")
                    }}
                "#, media_id);
                
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
                                // For now, we'll just use placeholder data
                                media.set(Some(MediaReference {
                                    id: media_id,
                                    media_type: "image".to_string(),
                                    storage_location: format!("/media/{}", media_id),
                                    thumbnail: None,
                                    size_bytes: 1024 * 1024, // 1MB
                                    filename: Some("example.jpg".to_string()),
                                }));
                                loading.set(false);
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to read response: {:?}", e)));
                                loading.set(false);
                            }
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to fetch media: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            
            || ()
        });
    }

    if *loading {
        return html! {
            <div class={css}>
                <div class="media-preview">
                    <div class="loading">{"Loading media..."}</div>
                </div>
            </div>
        };
    }

    if let Some(err) = &*error {
        return html! {
            <div class={css}>
                <div class="media-preview">
                    <div class="error">{"Error: "}{err}</div>
                </div>
            </div>
        };
    }

    let media_data = match &*media {
        Some(m) => m,
        None => return html! {
            <div class={css}>
                <div class="media-preview">
                    <div class="error">{"Media not found"}</div>
                </div>
            </div>
        },
    };

    // Determine what to display based on media type
    let content = match media_data.media_type.as_str() {
        "image" => {
            html! {
                <img 
                    class="media-content" 
                    src={media_data.storage_location.clone()} 
                    alt={media_data.filename.clone().unwrap_or("Image".to_string())}
                />
            }
        }
        "video" => {
            html! {
                <video 
                    class="media-content" 
                    controls=true
                >
                    <source src={media_data.storage_location.clone()} type="video/mp4" />
                    {"Your browser does not support the video tag."}
                </video>
            }
        }
        "audio" => {
            html! {
                <audio 
                    class="media-content" 
                    controls=true
                >
                    <source src={media_data.storage_location.clone()} type="audio/mpeg" />
                    {"Your browser does not support the audio tag."}
                </audio>
            }
        }
        _ => {
            // For documents and other file types, show a placeholder with icon
            let icon = match media_data.media_type.as_str() {
                "document" => "📄",
                _ => "📁",
            };
            
            html! {
                <div class="media-placeholder">
                    {icon}
                </div>
            }
        }
    };

    let file_size = format_file_size(media_data.size_bytes);

    html! {
        <div class={css}>
            <div class="media-preview">
                {content}
                <div class="media-info">
                    <div class="media-name">
                        {media_data.filename.clone().unwrap_or("Unnamed file".to_string())}
                    </div>
                    <div class="media-meta">
                        <span class="media-type">{&media_data.media_type}</span>
                        <span class="media-size">{file_size}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Format file size in a human-readable way
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}