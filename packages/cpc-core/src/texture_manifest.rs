use bevy::prelude::*;
use std::collections::HashMap;

/// Resource that holds handles to textures that are dynamically added at runtime.
/// This allows other parts of the application to look up textures by a string name.
#[derive(Resource, Default)]
pub struct TextureManifest {
    pub textures: HashMap<String, Handle<Image>>,
}