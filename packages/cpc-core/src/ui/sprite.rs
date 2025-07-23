use bevy::prelude::*;
use crate::texture_manifest::TextureManifest;

// A marker component for our user-generated sprite
#[derive(Component)]
struct UserImageSprite;

pub struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
           .add_systems(Update, display_user_image);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn display_user_image(
    mut commands: Commands,
    mut texture_manifest: ResMut<TextureManifest>,
    assets: Res<Assets<Image>>,
    query: Query<Entity, With<UserImageSprite>>,
) {
    if let Some(handle) = texture_manifest.textures.remove("user_selected_image") {
        // Despawn any existing user image sprite
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }

        if let Some(_image) = assets.get(&handle) {
            commands.spawn((
                SpriteBundle {
                    texture: handle,
                    ..default()
                },
                UserImageSprite,
            ));
        }
    }
}