use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Default)]
pub struct EditorState {
    pub active_scene: String,
    pub selected_entities: Vec<u32>,
    pub resources: Vec<String>,
}

impl EditorState {
    pub fn new() -> Self {
        EditorState::default()
    }
}

pub type SharedEditorState = Arc<Mutex<EditorState>>;