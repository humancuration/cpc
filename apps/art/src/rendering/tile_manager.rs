//! Tile management for the Art application rendering system
//!
//! This module handles dividing layers into tiles, tracking visibility,
//! and managing dirty regions for optimized rendering.

use crate::core::models::{Project, Layer, LayerType};
use crate::rendering::{CameraController, Rect as LayerRect};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents a single tile in the rendering system
#[derive(Debug, Clone)]
pub struct Tile {
    pub layer_id: Uuid,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub dirty: bool,
}

impl Tile {
    /// Create a new tile
    pub fn new(layer_id: Uuid, x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            layer_id,
            x,
            y,
            width,
            height,
            dirty: true, // New tiles are dirty by default
        }
    }
    
    /// Get the bounds of this tile as a rectangle
    pub fn bounds(&self) -> LayerRect {
        LayerRect {
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
    
    /// Check if this tile intersects with a viewport rectangle
    pub fn intersects_viewport(&self, viewport: &LayerRect) -> bool {
        let tile_bounds = self.bounds();
        !(tile_bounds.x > viewport.x + viewport.width ||
          tile_bounds.x + tile_bounds.width < viewport.x ||
          tile_bounds.y > viewport.y + viewport.height ||
          tile_bounds.y + tile_bounds.height < viewport.y)
    }
}

/// Manages tiles for all layers in a project
#[derive(Debug, Default)]
pub struct TileManager {
    /// Tiles organized by layer ID
    tiles: HashMap<Uuid, Vec<Tile>>,
    /// Tile size in pixels
    tile_size: u32,
}

impl TileManager {
    /// Create a new tile manager with the specified tile size
    pub fn new(tile_size: u32) -> Self {
        Self {
            tiles: HashMap::new(),
            tile_size,
        }
    }
    
    /// Divide a layer into tiles
    pub fn divide_layer_into_tiles(&mut self, layer: &Layer) {
        let layer_id = layer.id;
        let mut tiles = Vec::new();
        
        let layer_width = layer.bounds.width as u32;
        let layer_height = layer.bounds.height as u32;
        
        // Calculate number of tiles needed
        let tiles_x = (layer_width + self.tile_size - 1) / self.tile_size;
        let tiles_y = (layer_height + self.tile_size - 1) / self.tile_size;
        
        // Create tiles
        for y in 0..tiles_y {
            for x in 0..tiles_x {
                let tile_x = x * self.tile_size;
                let tile_y = y * self.tile_size;
                
                // Calculate actual tile dimensions (may be smaller at edges)
                let tile_width = if tile_x + self.tile_size > layer_width {
                    layer_width - tile_x
                } else {
                    self.tile_size
                };
                
                let tile_height = if tile_y + self.tile_size > layer_height {
                    layer_height - tile_y
                } else {
                    self.tile_size
                };
                
                tiles.push(Tile::new(layer_id, tile_x, tile_y, tile_width, tile_height));
            }
        }
        
        self.tiles.insert(layer_id, tiles);
    }
    
    /// Get tiles for a specific layer
    pub fn get_layer_tiles(&self, layer_id: Uuid) -> Option<&Vec<Tile>> {
        self.tiles.get(&layer_id)
    }
    
    /// Calculate which tiles are visible in the current viewport
    pub fn calculate_visible_tiles(&self, layer_id: Uuid, camera: &CameraController, project: &Project) -> Vec<&Tile> {
        // Calculate viewport in world coordinates
        let zoom = camera.zoom;
        let pan_x = camera.pan_x;
        let pan_y = camera.pan_y;
        
        // Get window dimensions (assuming 1920x1080 for now, in a real implementation this would come from Bevy)
        let window_width = 1920.0;
        let window_height = 1080.0;
        
        // Calculate viewport bounds in world coordinates
        let viewport_width = window_width / zoom;
        let viewport_height = window_height / zoom;
        
        let viewport = LayerRect {
            x: pan_x - viewport_width / 2.0,
            y: pan_y - viewport_height / 2.0,
            width: viewport_width,
            height: viewport_height,
        };
        
        // Get tiles for this layer and filter visible ones
        if let Some(tiles) = self.tiles.get(&layer_id) {
            tiles.iter()
                .filter(|tile| tile.intersects_viewport(&viewport))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Mark tiles as dirty based on a dirty region
    pub fn mark_dirty_region(&mut self, layer_id: Uuid, region: &LayerRect) {
        if let Some(tiles) = self.tiles.get_mut(&layer_id) {
            for tile in tiles {
                if tile.intersects_viewport(region) {
                    tile.dirty = true;
                }
            }
        }
    }
    
    /// Get dirty tiles for a layer
    pub fn get_dirty_tiles(&self, layer_id: Uuid) -> Vec<&Tile> {
        if let Some(tiles) = self.tiles.get(&layer_id) {
            tiles.iter()
                .filter(|tile| tile.dirty)
                .collect()
        } else {
            Vec::new()
        }
    }
    
    /// Clear dirty flag for tiles
    pub fn clear_dirty_tiles(&mut self, layer_id: Uuid) {
        if let Some(tiles) = self.tiles.get_mut(&layer_id) {
            for tile in tiles {
                tile.dirty = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::{Layer, LayerType};
    
    #[test]
    fn test_tile_creation() {
        let tile = Tile::new(Uuid::new_v4(), 0, 0, 256, 256);
        assert_eq!(tile.x, 0);
        assert_eq!(tile.y, 0);
        assert_eq!(tile.width, 256);
        assert_eq!(tile.height, 256);
        assert!(tile.dirty);
    }
    
    #[test]
    fn test_tile_bounds() {
        let tile = Tile::new(Uuid::new_v4(), 100, 200, 256, 256);
        let bounds = tile.bounds();
        assert_eq!(bounds.x, 100.0);
        assert_eq!(bounds.y, 200.0);
        assert_eq!(bounds.width, 256.0);
        assert_eq!(bounds.height, 256.0);
    }
    
    #[test]
    fn test_tile_intersects_viewport() {
        let tile = Tile::new(Uuid::new_v4(), 100, 100, 256, 256);
        
        // Viewport that intersects the tile
        let viewport = LayerRect {
            x: 50.0,
            y: 50.0,
            width: 200.0,
            height: 200.0,
        };
        assert!(tile.intersects_viewport(&viewport));
        
        // Viewport that doesn't intersect the tile
        let viewport = LayerRect {
            x: 400.0,
            y: 400.0,
            width: 100.0,
            height: 100.0,
        };
        assert!(!tile.intersects_viewport(&viewport));
    }
    
    #[test]
    fn test_divide_layer_into_tiles() {
        let mut tile_manager = TileManager::new(256);
        let layer = Layer::new("Test Layer".to_string(), 600, 400, LayerType::Raster);
        
        tile_manager.divide_layer_into_tiles(&layer);
        
        let tiles = tile_manager.get_layer_tiles(layer.id).unwrap();
        // 600x400 layer with 256 tile size should create 3x2 = 6 tiles
        assert_eq!(tiles.len(), 6);
        
        // Check first tile
        assert_eq!(tiles[0].x, 0);
        assert_eq!(tiles[0].y, 0);
        assert_eq!(tiles[0].width, 256);
        assert_eq!(tiles[0].height, 256);
        
        // Check last tile (should be smaller)
        assert_eq!(tiles[5].x, 512);
        assert_eq!(tiles[5].y, 256);
        assert_eq!(tiles[5].width, 88);  // 600 - 512 = 88
        assert_eq!(tiles[5].height, 144); // 400 - 256 = 144
    }
}