use yew::prelude::*;
use web_sys::HtmlInputElement;
use shared_packages::collaborative_docs::core::{DocumentContent, DocumentMetadata};
use shared_packages::collaborative_docs::crdt::CrdtDocument;
use uuid::Uuid;
use serde_json::Value;
use shared_packages::operational_transformation::{Operation, TextOperation, VersionVector};
use std::collections::HashMap;

#[derive(Properties, PartialEq)]
pub struct ConflictResolutionDialogProps {
    pub document_id: Uuid,
    pub current_version: DocumentContent,
    pub conflicting_version: DocumentContent,
    pub pending_operations: Vec<TextOperation>,
    pub version_vector: VersionVector,
    pub on_resolve: Callback<ResolutionAction>,
    pub on_close: Callback<()>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolutionAction {
    AcceptCurrent,
    AcceptIncoming,
    MergeCustom(String),
}

#[function_component(ConflictResolutionDialog)]
pub fn conflict_resolution_dialog(props: &ConflictResolutionDialogProps) -> Html {
    let selected_version = use_state(|| "current".to_string());
    let merge_preview = use_state(|| String::new());
    let conflict_details = use_state(|| Vec::<String>::new());
    
    // Parse document content to text for display
    let current_text = document_content_to_text(&props.current_version);
    let conflicting_text = document_content_to_text(&props.conflicting_version);
    
    // Analyze conflicts between pending operations and incoming operations
    let conflicts = analyze_conflicts(&props.pending_operations, &props.version_vector);
    conflict_details.set(conflicts);
    
    // Update merge preview when selection changes
    {
        let selected = selected_version.clone();
        let current = current_text.clone();
        let conflicting = conflicting_text.clone();
        let merge_preview = merge_preview.clone();
        use_effect_with(selected.clone(), move |_| {
            let preview = match selected.as_str() {
                "current" => current.clone(),
                "incoming" => conflicting.clone(),
                _ => format!("Merged version:\n\n{}\n\n---\n\n{}", current, conflicting),
            };
            merge_preview.set(preview);
        });
    }
    
    let on_version_select = {
        let selected_version = selected_version.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            selected_version.set(input.value());
        })
    };
    
    let on_accept_current = {
        let on_resolve = props.on_resolve.clone();
        Callback::from(move |_| {
            on_resolve.emit(ResolutionAction::AcceptCurrent);
        })
    };
    
    let on_accept_incoming = {
        let on_resolve = props.on_resolve.clone();
        Callback::from(move |_| {
            on_resolve.emit(ResolutionAction::AcceptIncoming);
        })
    };
    
    let on_merge_custom = {
        let merge_preview = merge_preview.clone();
        let on_resolve = props.on_resolve.clone();
        Callback::from(move |_| {
            on_resolve.emit(ResolutionAction::MergeCustom((*merge_preview).clone()));
        })
    };
    
    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };
    
    html! {
        <div class="conflict-resolution-dialog">
            <div class="dialog-overlay">
                <div class="dialog-content">
                    <div class="dialog-header">
                        <h2>{"Conflict Resolution"}</h2>
                        <button class="close-button" onclick={on_close}>{"×"}</button>
                    </div>
                    
                    <div class="dialog-body">
                        <div class="version-comparison">
                            <div class="version-panel">
                                <h3>{"Current Version"}</h3>
                                <div class="version-content">
                                    <pre>{&current_text}</pre>
                                </div>
                            </div>
                            
                            <div class="version-panel">
                                <h3>{"Conflicting Version"}</h3>
                                <div class="version-content">
                                    <pre>{&conflicting_text}</pre>
                                </div>
                            </div>
                        </div>
                        
                        <div class="resolution-controls">
                            <div class="version-selector">
                                <label>{"Select version to keep:"}</label>
                                <select onchange={on_version_select} value={(*selected_version).clone()}>
                                    <option value="current">{"Current Version"}</option>
                                    <option value="incoming">{"Incoming Version"}</option>
                                    <option value="merge">{"Merge Both"}</option>
                                </select>
                            </div>
                            
                            <div class="conflict-details">
                                <h3>{"Conflict Details"}</h3>
                                <ul>
                                    {for (*conflict_details).iter().map(|detail| {
                                        html! {
                                            <li>{detail}</li>
                                        }
                                    })}
                                </ul>
                            </div>
                            
                            <div class="merge-preview">
                                <h3>{"Merge Preview"}</h3>
                                <div class="preview-content">
                                    <pre>{&*merge_preview}</pre>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <div class="dialog-footer">
                        <button class="btn btn-secondary" onclick={on_close}>
                            {"Cancel"}
                        </button>
                        <button class="btn btn-primary" onclick={on_accept_current}>
                            {"Accept Current"}
                        </button>
                        <button class="btn btn-warning" onclick={on_accept_incoming}>
                            {"Accept Incoming"}
                        </button>
                        <button class="btn btn-success" onclick={on_merge_custom}>
                            {"Merge Versions"}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Convert document content to text representation for display
fn document_content_to_text(content: &DocumentContent) -> String {
    if content.format == "crdt" {
        // For CRDT documents, we would need to parse the CRDT data
        // This is a simplified implementation
        if let Some(crdt_data) = content.data.get("crdt_data") {
            format!("CRDT Document: {:?}", crdt_data)
        } else {
/// Analyze conflicts between pending operations and the version vector
fn analyze_conflicts(pending_ops: &Vec<TextOperation>, version_vector: &VersionVector) -> Vec<String> {
    let mut conflicts = Vec::new();
    
    for op in pending_ops {
        if !version_vector.is_causally_ready(op) {
            conflicts.push(format!(
                "Operation from user {} at version {} is not causally ready (current: {})",
                op.user_id,
                op.version,
                version_vector.get(&op.user_id)
            ));
        }
    }
    
    conflicts
}

/// Convert document content to text representation for display
fn document_content_to_text(content: &DocumentContent) -> String {
    if content.format == "crdt" {
        // For CRDT documents, we would need to parse the CRDT data
        // This is a simplified implementation
        if let Some(crdt_data) = content.data.get("crdt_data") {
            format!("CRDT Document: {:?}", crdt_data)
        } else {
            "Invalid CRDT document format".to_string()
        }
    } else {
        // For other formats, try to convert to string
        match serde_json::to_string_pretty(&content.data) {
            Ok(text) => text,
            Err(_) => format!("{:?}", content.data),
        }
    }
}
            "Invalid CRDT document format".to_string()
        }
    } else {
        // For other formats, try to convert to string
        match serde_json::to_string_pretty(&content.data) {
            Ok(text) => text,
            Err(_) => format!("{:?}", content.data),
        }
    }
}