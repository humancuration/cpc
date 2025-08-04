//! Component for uploading media files

use yew::prelude::*;
use stylist::yew::styled_component;
use stylist::Style;
use uuid::Uuid;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use web_sys::{File, HtmlInputElement};
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;

/// Properties for the MediaUpload component
#[derive(Properties, PartialEq)]
pub struct MediaUploadProps {
    /// The ID of the conversation to upload media to
    pub conversation_id: Uuid,
    
    /// Callback when media is successfully uploaded
    pub on_upload: Callback<Uuid>,
}

/// A component that allows users to upload media files
#[styled_component(MediaUpload)]
pub fn media_upload(props: &MediaUploadProps) -> Html {
    let css = Style::new(r#"
        .media-upload {
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 20px;
            margin-top: 16px;
        }
        
        .upload-area {
            border: 2px dashed #ccc;
            border-radius: 8px;
            padding: 30px;
            text-align: center;
            cursor: pointer;
            transition: all 0.2s ease;
            background: #fafafa;
        }
        
        .upload-area:hover {
            border-color: #007bff;
            background: #f0f8ff;
        }
        
        .upload-area.drag-over {
            border-color: #007bff;
            background: #e6f2ff;
        }
        
        .upload-icon {
            font-size: 48px;
            color: #999;
            margin-bottom: 16px;
        }
        
        .upload-text {
            font-size: 16px;
            color: #666;
            margin-bottom: 8px;
        }
        
        .upload-hint {
            font-size: 12px;
            color: #999;
        }
        
        .file-input {
            display: none;
        }
        
        .upload-progress {
            margin-top: 20px;
        }
        
        .progress-bar {
            height: 8px;
            background: #f0f0f0;
            border-radius: 4px;
            overflow: hidden;
        }
        
        .progress-fill {
            height: 100%;
            background: #007bff;
            transition: width 0.3s ease;
        }
        
        .upload-status {
            font-size: 14px;
            color: #666;
            margin-top: 8px;
            text-align: center;
        }
        
        .error {
            color: #d32f2f;
            font-size: 14px;
            margin-top: 12px;
            text-align: center;
        }
        
        .success {
            color: #388e3c;
            font-size: 14px;
            margin-top: 12px;
            text-align: center;
        }
    "#).expect("style");

    let file_input_ref = use_node_ref();
    let drag_over = use_state(|| false);
    let uploading = use_state(|| false);
    let progress = use_state(|| 0u32);
    let error = use_state(|| Option::<String>::None);
    let success = use_state(|| false);

    let on_click_upload_area = {
        let file_input_ref = file_input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    let on_file_change = {
        let file_input_ref = file_input_ref.clone();
        let uploading = uploading.clone();
        let progress = progress.clone();
        let error = error.clone();
        let success = success.clone();
        let conversation_id = props.conversation_id;
        let on_upload = props.on_upload.clone();
        
        Callback::from(move |_| {
            if *uploading {
                return;
            }
            
            let file_input_ref = file_input_ref.clone();
            let uploading = uploading.clone();
            let progress = progress.clone();
            let error = error.clone();
            let success = success.clone();
            let conversation_id = conversation_id;
            let on_upload = on_upload.clone();
            
            spawn_local(async move {
                if let Some(input) = file_input_ref.cast::<HtmlInputElement>() {
                    if let Some(file_list) = input.files() {
                        if file_list.length() > 0 {
                            let file = file_list.get(0).unwrap();
                            upload_file(file, &uploading, &progress, &error, &success, conversation_id, &on_upload).await;
                        }
                    }
                }
            });
        })
    };

    let on_drag_enter = {
        let drag_over = drag_over.clone();
        Callback::from(move |_| {
            drag_over.set(true);
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |_| {
            drag_over.set(false);
        })
    };

    let on_drag_over = Callback::from(move |e: DragEvent| {
        e.prevent_default();
    });

    let on_drop = {
        let file_input_ref = file_input_ref.clone();
        let drag_over = drag_over.clone();
        let uploading = uploading.clone();
        let progress = progress.clone();
        let error = error.clone();
        let success = success.clone();
        let conversation_id = props.conversation_id;
        let on_upload = props.on_upload.clone();
        
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(false);
            
            if *uploading {
                return;
            }
            
            let file_input_ref = file_input_ref.clone();
            let uploading = uploading.clone();
            let progress = progress.clone();
            let error = error.clone();
            let success = success.clone();
            let conversation_id = conversation_id;
            let on_upload = on_upload.clone();
            
            spawn_local(async move {
                if let Some(data_transfer) = e.data_transfer() {
                    if let Some(file_list) = data_transfer.files() {
                        if file_list.length() > 0 {
                            let file = file_list.get(0).unwrap();
                            upload_file(file, &uploading, &progress, &error, &success, conversation_id, &on_upload).await;
                        }
                    }
                }
            });
        })
    };

    let drag_class = if *drag_over { "drag-over" } else { "" };

    html! {
        <div class={css}>
            <div class="media-upload">
                <div 
                    class={classes!("upload-area", drag_class)}
                    onclick={on_click_upload_area}
                    ondragenter={on_drag_enter}
                    ondragleave={on_drag_leave}
                    ondragover={on_drag_over}
                    ondrop={on_drop}
                >
                    <div class="upload-icon">{"📁"}</div>
                    <div class="upload-text">{"Click or drag files to upload"}</div>
                    <div class="upload-hint">{"Supports images, documents, audio, and video"}</div>
                </div>
                
                <input
                    ref={file_input_ref}
                    type="file"
                    class="file-input"
                    onchange={on_file_change}
                    accept="image/*,audio/*,video/*,.pdf,.doc,.docx,.xls,.xlsx"
                />
                
                if *uploading {
                    <div class="upload-progress">
                        <div class="progress-bar">
                            <div class="progress-fill" style={format!("width: {}%", *progress)}></div>
                        </div>
                        <div class="upload-status">{"Uploading... "}{*progress}{"%"}</div>
                    </div>
                }
                
                if *success {
                    <div class="success">{"File uploaded successfully!"}</div>
                }
                
                if let Some(err) = &*error {
                    <div class="error">{err}</div>
                }
            </div>
        </div>
    }
}

/// Upload a file to the server
async fn upload_file(
    file: File,
    uploading: &UseStateHandle<bool>,
    progress: &UseStateHandle<u32>,
    error: &UseStateHandle<Option<String>>,
    success: &UseStateHandle<bool>,
    conversation_id: Uuid,
    on_upload: &Callback<Uuid>,
) {
    uploading.set(true);
    progress.set(0);
    error.set(None);
    success.set(false);
    
    // In a real implementation, we would:
    // 1. Read the file content
    // 2. Determine the media type
    // 3. Upload to the server via GraphQL mutation
    
    // For now, we'll simulate the upload process
    for i in 0..=100 {
        gloo_timers::future::TimeoutFuture::new(20).await;
        progress.set(i);
    }
    
    // Simulate a successful upload
    uploading.set(false);
    success.set(true);
    
    // In a real implementation, we would call the GraphQL mutation:
    // uploadMedia(conversationId: $conversationId, file: $file)
    // and emit the media ID when successful
    
    // For now, we'll emit a placeholder ID
    on_upload.emit(Uuid::new_v4());
    
    // Reset success message after 3 seconds
    gloo_timers::future::TimeoutFuture::new(3000).await;
    success.set(false);
}