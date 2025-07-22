use bevy::prelude::*;

/// Resource that holds the paths of essential textures that must be reloaded after a pause.
/// The game should populate this resource with the paths of textures that are essential and must be reloaded on resume.
#[derive(Resource, Default)]
pub struct TextureManifest {
    pub paths: Vec<String>,
}