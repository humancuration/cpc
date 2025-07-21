//! Integration between Bevy's asset system and our custom asset manager

use bevy::prelude::*;
use bevy::asset::{AssetServer, LoadContext};
use crate::asset_manager::AssetManager;
use std::path::Path;

pub struct BevyAssetBridge {
    asset_manager: AssetManager,
}

impl BevyAssetBridge {
    pub fn new(asset_manager: AssetManager) -> Self {
        Self { asset_manager }
    }

    pub fn setup_bevy(&self, app: &mut App) {
        app.add_asset_loader(Self::custom_asset_loader());
    }

    fn custom_asset_loader() -> Box<dyn bevy::asset::AssetLoader> {
        Box::new(CustomAssetLoader)
    }
}

struct CustomAssetLoader;

impl bevy::asset::AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let path = load_context.path();
            let extension = path.extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_lowercase();

            // Handle different asset types
            match extension.as_str() {
                "png" | "jpg" | "jpeg" => self.load_image(bytes, load_context).await,
                "wav" | "mp3" | "ogg" => self.load_audio(bytes, load_context).await,
                "gltf" | "glb" | "obj" => self.load_model(bytes, load_context).await,
                _ => Err(anyhow::anyhow!("Unsupported asset type: {}", extension)),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["png", "jpg", "jpeg", "wav", "mp3", "ogg", "gltf", "glb", "obj"]
    }
}

impl CustomAssetLoader {
    async fn load_image<'a>(
        &self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> Result<(), anyhow::Error> {
        // Process image using our asset manager
        // This would typically generate mipmaps, convert formats, etc.
        load_context.set_default_asset(bevy::asset::LoadedAsset::new(
            bevy::render::texture::Image::from_buffer(
                bytes,
                bevy::render::texture::ImageFormat::from_extension(
                    load_context.path().extension().unwrap().to_str().unwrap(),
                )
                .unwrap(),
            )
            .unwrap(),
        ));
        Ok(())
    }

    async fn load_audio<'a>(
        &self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> Result<(), anyhow::Error> {
        // Convert audio to WAV format if needed
        load_context.set_default_asset(bevy::asset::LoadedAsset::new(
            bevy::audio::AudioSource { bytes: bytes.into() },
        ));
        Ok(())
    }

    async fn load_model<'a>(
        &self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> Result<(), anyhow::Error> {
        // Process 3D model
        load_context.set_default_asset(bevy::asset::LoadedAsset::new(
            bevy::gltf::Gltf::from_slice(bytes)?,
        ));
        Ok(())
    }
}