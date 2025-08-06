//! Texture caching system for the Art application
//!
//! This module manages textures for layers and tiles, including texture atlas support
//! for efficient GPU utilization.

use bevy::prelude::*;
use rendering_core::TextureHandle;
use opengl_renderer;
use vulkan_renderer;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use crate::core::models::{Layer, LayerType};
use crate::rendering::Rect as LayerRect; // Use Rect from rendering module
use uuid::Uuid;
use std::collections::HashMap;

/// Coordinate for a tile in an atlas
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub x: u32,
    pub y: u32,
}

/// Rectangle bounds for texture coordinates
#[derive(Debug, Clone)]
pub struct TextureRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Texture atlas for efficient tile management
pub struct TextureAtlas {
    pub texture: Handle<Image>,
    pub tile_mapping: HashMap<TileCoord, TextureRect>,
    pub tile_size: u32,
    pub atlas_size: (u32, u32),
}

impl TextureAtlas {
    /// Create a new texture atlas
    pub fn new(
        images: &mut Assets<Image>,
        tile_size: u32,
        initial_capacity: usize,
    ) -> Self {
        // Calculate atlas dimensions based on capacity
        let tiles_per_row = (initial_capacity as f32).sqrt().ceil() as u32;
        let atlas_width = tiles_per_row * tile_size;
        let atlas_height = tiles_per_row * tile_size;
        
        // Create blank atlas texture
        let size = Extent3d {
            width: atlas_width,
            height: atlas_height,
            depth_or_array_layers: 1,
        };
        
        let mut image = Image::new_fill(
            size,
            TextureDimension::D2,
            &[0, 0, 0, 0], // Transparent black
            TextureFormat::Rgba8UnormSrgb,
        );
        
        // Set texture usage for rendering
        image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
        
        let texture = images.add(image);
        
        Self {
            texture,
            tile_mapping: HashMap::new(),
            tile_size,
            atlas_size: (atlas_width, atlas_height),
        }
    }
    
    /// Allocate space for a tile in the atlas
    pub fn allocate_tile(&mut self, tile_coord: TileCoord) -> Option<TextureRect> {
        // Check if we already have this tile
        if let Some(rect) = self.tile_mapping.get(&tile_coord) {
            return Some(rect.clone());
        }
        
        // Calculate position in atlas
        let atlas_width = self.atlas_size.0;
        let tiles_per_row = atlas_width / self.tile_size;
        
        let index = self.tile_mapping.len() as u32;
        let row = index / tiles_per_row;
        let col = index % tiles_per_row;
        
        // Check if we have space
        if row * self.tile_size >= self.atlas_size.1 {
            // Need to resize atlas - for now just return None
            return None;
        }
        
        let rect = TextureRect {
            x: (col * self.tile_size) as f32,
            y: (row * self.tile_size) as f32,
            width: self.tile_size as f32,
            height: self.tile_size as f32,
        };
        
        self.tile_mapping.insert(tile_coord, rect.clone());
        Some(rect)
    }
    
    /// Get the texture coordinates for a tile
    pub fn get_tile_rect(&self, tile_coord: &TileCoord) -> Option<&TextureRect> {
        self.tile_mapping.get(tile_coord)
    }
}

/// Multi-backend texture handle
pub struct MultiBackendTexture {
    pub bevy_handle: Handle<Image>,
    pub opengl_handle: Option<opengl_renderer::texture::OpenGLTextureHandle>,
    pub vulkan_handle: Option<vulkan_renderer::texture::VulkanTextureHandle>,
}

/// Cache for layer textures and atlases
#[derive(Resource, Default)]
pub struct TextureCache {
    /// Cache of layer textures by layer ID
    pub layer_textures: HashMap<Uuid, MultiBackendTexture>,
    /// Cache of texture atlases by layer ID
    pub layer_atlases: HashMap<Uuid, TextureAtlas>,
    /// Set of dirty layer IDs that need texture updates
    pub dirty_layers: HashMap<Uuid, bool>,
    /// Dirty tiles by layer ID
    pub dirty_tiles: HashMap<Uuid, Vec<TileCoord>>,
}

impl TextureCache {
    /// Mark a layer as dirty (needs texture update)
    pub fn mark_dirty(&mut self, layer_id: Uuid) {
        self.dirty_layers.insert(layer_id, true);
    }
    
    /// Check if a layer is dirty
    pub fn is_dirty(&self, layer_id: Uuid) -> bool {
        self.dirty_layers.get(&layer_id).copied().unwrap_or(false)
    }
    
    /// Clear dirty flag for a layer
    pub fn clear_dirty(&mut self, layer_id: Uuid) {
        self.dirty_layers.remove(&layer_id);
    }
    
    /// Mark a tile as dirty
    pub fn mark_tile_dirty(&mut self, layer_id: Uuid, tile_coord: TileCoord) {
        self.dirty_tiles
            .entry(layer_id)
            .or_insert_with(Vec::new)
            .push(tile_coord);
    }
    
    /// Get dirty tiles for a layer
    pub fn get_dirty_tiles(&self, layer_id: Uuid) -> Option<&Vec<TileCoord>> {
        self.dirty_tiles.get(&layer_id)
    }
    
    /// Clear dirty tiles for a layer
    pub fn clear_dirty_tiles(&mut self, layer_id: Uuid) {
        self.dirty_tiles.remove(&layer_id);
    }
    
    /// Get or create texture for a layer
    pub fn get_or_create_texture(
        &mut self,
        layer: &Layer,
        images: &mut Assets<Image>,
    ) -> &mut MultiBackendTexture {
        if let Some(texture) = self.layer_textures.get(&layer.id) {
            if self.is_dirty(layer.id) {
                // Update existing texture
                let image = layer_to_image(layer);
                images.set(texture.bevy_handle.clone(), image);
                self.clear_dirty(layer.id);
            }
            self.layer_textures.get_mut(&layer.id).unwrap()
        } else {
            // Create new texture
            let image = layer_to_image(layer);
            let handle = images.add(image);
            let multi_texture = MultiBackendTexture {
                bevy_handle: handle.clone(),
                opengl_handle: None,
                vulkan_handle: None,
            };
            self.layer_textures.insert(layer.id, multi_texture);
            self.clear_dirty(layer.id);
            self.layer_textures.get_mut(&layer.id).unwrap()
        }
    }
    
    /// Get or create texture atlas for a layer
    pub fn get_or_create_atlas(
        &mut self,
        layer_id: Uuid,
        images: &mut Assets<Image>,
        tile_size: u32,
    ) -> &mut TextureAtlas {
        if !self.layer_atlases.contains_key(&layer_id) {
            let atlas = TextureAtlas::new(images, tile_size, 16); // Initial capacity of 16 tiles
            self.layer_atlases.insert(layer_id, atlas);
        }
        self.layer_atlases.get_mut(&layer_id).unwrap()
    }
}

/// Convert layer pixels to a Bevy Image texture
fn layer_to_image(layer: &Layer) -> Image {
    // Create image with RGBA8 format
    let width = layer.bounds.width as u32;
    let height = layer.bounds.height as u32;
    
    // Create image data from layer pixels
    // Layer pixels are stored as RGBA values
    let pixel_data = layer.pixels.clone();
    
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    
    let mut image = Image::new(
        size,
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb,
    );
    
    // Set texture usage for rendering
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Layer, LayerType};
    
    #[test]
    fn test_texture_atlas_creation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(AssetPlugin::default());
        
        let mut images = app.world.resource_mut::<Assets<Image>>();
        let atlas = TextureAtlas::new(&mut images, 256, 4);
        
        assert_eq!(atlas.tile_size, 256);
        // Atlas should be 512x512 for 4 tiles (2x2 grid)
        assert_eq!(atlas.atlas_size, (512, 512));
    }
    
    #[test]
    fn test_tile_allocation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(AssetPlugin::default());
        
        let mut images = app.world.resource_mut::<Assets<Image>>();
        let mut atlas = TextureAtlas::new(&mut images, 256, 4);
        
        let coord = TileCoord { x: 0, y: 0 };
        let rect = atlas.allocate_tile(coord.clone());
        
        assert!(rect.is_some());
        let rect = rect.unwrap();
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.width, 256.0);
        assert_eq!(rect.height, 256.0);
        
        // Check that we can retrieve the same rect
        let retrieved = atlas.get_tile_rect(&coord);
        assert!(retrieved.is_some());
        assert_eq!(rect, *retrieved.unwrap());
    }
    
    #[test]
    fn test_texture_cache() {
        let mut cache = TextureCache::default();
        let layer_id = Uuid::new_v4();
        
        assert!(!cache.is_dirty(layer_id));
        
        cache.mark_dirty(layer_id);
        assert!(cache.is_dirty(layer_id));
        cache.clear_dirty(layer_id);
        assert!(!cache.is_dirty(layer_id));
    }
}

/// Convert a Bevy image to an OpenGL texture handle
impl TextureCache {
    pub fn as_opengl_texture(&self, texture: &MultiBackendTexture) -> &opengl_renderer::texture::OpenGLTextureHandle {
        // In a real implementation, we would convert the Bevy image to an OpenGL texture
        // For now, we'll return a placeholder or create one if needed
        texture.opengl_handle.as_ref().unwrap()
    }
    
    /// Convert a Bevy image to a Vulkan texture handle
    pub fn as_vulkan_texture(&self, texture: &MultiBackendTexture) -> &vulkan_renderer::texture::VulkanTextureHandle {
        // In a real implementation, we would convert the Bevy image to a Vulkan texture
        // For now, we'll return a placeholder or create one if needed
        texture.vulkan_handle.as_ref().unwrap()
    }
    
    /// Create OpenGL texture representation if it doesn't exist
    pub fn create_opengl_texture(&mut self, layer_id: Uuid) {
        if let Some(texture) = self.layer_textures.get_mut(&layer_id) {
            if texture.opengl_handle.is_none() {
                // In a real implementation, we would create the OpenGL texture
                // For now, we'll create a placeholder
                texture.opengl_handle = Some(opengl_renderer::texture::OpenGLTextureHandle {
                    texture: unsafe { std::mem::zeroed() }, // Placeholder
                    descriptor: rendering_core::texture::TextureDescriptor {
                        width: 256,
                        height: 256,
                        format: rendering_core::texture::TextureFormat::Rgba8Unorm,
                        usage: rendering_core::texture::TextureUsage::default(),
                    },
                    gl: unsafe { std::mem::zeroed() }, // Placeholder
                });
            }
        }
    }
    
    /// Create Vulkan texture representation if it doesn't exist
    pub fn create_vulkan_texture(&mut self, layer_id: Uuid) {
        if let Some(texture) = self.layer_textures.get_mut(&layer_id) {
            if texture.vulkan_handle.is_none() {
                // In a real implementation, we would create the Vulkan texture
                // For now, we'll create a placeholder
                texture.vulkan_handle = Some(vulkan_renderer::texture::VulkanTextureHandle {
                    image_view: unsafe { std::mem::zeroed() }, // Placeholder
                    descriptor: rendering_core::texture::TextureDescriptor {
                        width: 256,
                        height: 256,
                        format: rendering_core::texture::TextureFormat::Rgba8Unorm,
                        usage: rendering_core::texture::TextureUsage::default(),
                    },
                });
            }
        }
    }
}
}