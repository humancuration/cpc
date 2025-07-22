use cpc_core::scene::SceneData;
use std::sync::Mutex;

/// Manages the current scene in memory
pub struct SceneManager {
    current_scene: Mutex<Option<SceneData>>,
}

impl SceneManager {
    /// Creates a new SceneManager with an empty scene
    pub fn new() -> Self {
        SceneManager {
            current_scene: Mutex::new(None),
        }
    }

    /// Sets the current scene
    pub fn set_current_scene(&self, scene: SceneData) {
        let mut current_scene = self.current_scene.lock().unwrap();
        *current_scene = Some(scene);
    }

    /// Gets a clone of the current scene
    pub fn current_scene(&self) -> Option<SceneData> {
        let current_scene = self.current_scene.lock().unwrap();
        current_scene.clone()
    }

    /// Clears the current scene
    pub fn clear(&self) {
        let mut current_scene = self.current_scene.lock().unwrap();
        *current_scene = None;
    }
}