//! Position translation service for accurate cursor positioning

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use lru::LruCache;
use std::num::NonZeroUsize;
use uuid::Uuid;

/// Metrics for a single line
#[derive(Debug, Clone)]
pub struct LineMetrics {
    pub top: f64,
    pub height: f64,
    pub wrapped_ranges: Vec<(usize, usize)>,
}

/// Metrics for a single character
#[derive(Debug, Clone)]
pub struct CharMetrics {
    pub width: f64,
    pub kerning: f64,
}

/// Font identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FontId {
    pub family: String,
    pub size: f64,
    pub weight: u32,
}

/// Rectangle representing a viewport or region
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
    
    pub fn contains(&self, x: f64, y: f64) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}

/// Position translator for converting document coordinates to screen coordinates
pub struct PositionTranslator {
    /// Cached line metrics
    line_metrics: HashMap<usize, LineMetrics>,
    
    /// Cached character metrics by font
    char_metrics: HashMap<(FontId, char), CharMetrics>,
    
    /// Scroll offset
    scroll_offset: (f64, f64),
    
    /// Viewport size
    viewport_size: (f64, f64),
    
    /// Cache for document-to-screen mappings
    position_cache: LruCache<(usize, usize), (f64, f64)>,
}

impl PositionTranslator {
    /// Create a new position translator with default values
    pub fn new() -> Self {
        let cache_capacity = NonZeroUsize::new(1000).unwrap_or(NonZeroUsize::new(100).unwrap());
        Self {
            line_metrics: HashMap::new(),
            char_metrics: HashMap::new(),
            scroll_offset: (0.0, 0.0),
            viewport_size: (800.0, 600.0), // Default viewport size
            position_cache: LruCache::new(cache_capacity),
        }
    }
    
    /// Create a new position translator with custom viewport size
    pub fn with_viewport_size(width: f64, height: f64) -> Self {
        let mut translator = Self::new();
        translator.viewport_size = (width, height);
        translator
    }
    
    /// Convert document coordinates to screen coordinates
    pub fn document_to_screen(&mut self, line: usize, col: usize) -> (f64, f64) {
        // Check cache first
        if let Some(cached) = self.position_cache.get(&(line, col)) {
            return *cached;
        }
        
        // Calculate position based on line metrics and character metrics
        let x = if let Some(line_metrics) = self.line_metrics.get(&line) {
            // For now, use a simplified calculation
            // In a real implementation, this would account for variable-width fonts and wrapping
            col as f64 * 8.0 // Default character width
        } else {
            col as f64 * 8.0 // Default character width
        };
        
        let y = if let Some(line_metrics) = self.line_metrics.get(&line) {
            line_metrics.top
        } else {
            line as f64 * 20.0 // Default line height
        };
        
        let result = (x - self.scroll_offset.0, y - self.scroll_offset.1);
        
        // Cache the result
        self.position_cache.put((line, col), result);
        
        result
    }
    
    /// Convert screen coordinates to document position
    pub fn screen_to_doc(&self, x: f64, y: f64) -> (usize, usize) {
        // Add scroll offset back to screen coordinates
        let screen_x = x + self.scroll_offset.0;
        let screen_y = y + self.scroll_offset.1;
        
        // Find the line based on y coordinate
        let line = if let Some((line_idx, _)) = self.line_metrics
            .iter()
            .find(|(_, metrics)| screen_y >= metrics.top && screen_y <= metrics.top + metrics.height) {
            *line_idx
        } else {
            // Fallback to simple calculation
            (screen_y / 20.0) as usize // Default line height
        };
        
        // Find the column based on x coordinate
        let col = if !self.char_metrics.is_empty() {
            // For now, use a simplified approach
            (screen_x / 8.0) as usize // Default character width
        } else {
            (screen_x / 8.0) as usize // Default character width
        };
        
        (line, col)
    }
    
    /// Update scroll offset
    pub fn set_scroll_offset(&mut self, x: f64, y: f64) {
        self.scroll_offset = (x, y);
        // Invalidate cache when scroll changes significantly
        if x.abs() > 50.0 || y.abs() > 50.0 {
            self.position_cache.clear();
        }
    }
    
    /// Update viewport size
    pub fn set_viewport_size(&mut self, width: f64, height: f64) {
        self.viewport_size = (width, height);
    }
    
    /// Update font metrics for a character
    pub fn update_font_metrics(&mut self, font_id: FontId, ch: char, metrics: CharMetrics) {
        self.char_metrics.insert((font_id, ch), metrics);
        // Invalidate cache when font metrics change
        self.position_cache.clear();
    }
    
    /// Update line metrics
    pub fn update_line_metrics(&mut self, line: usize, metrics: LineMetrics) {
        self.line_metrics.insert(line, metrics);
        // Invalidate cache when line metrics change
        self.position_cache.clear();
    }
    
    /// Clear measurement cache
    pub fn invalidate_cache(&mut self) {
        self.position_cache.clear();
        self.line_metrics.clear();
        self.char_metrics.clear();
    }
    
    /// Get the current viewport
    pub fn viewport(&self) -> Rect {
        Rect::new(
            self.scroll_offset.0,
            self.scroll_offset.1,
            self.viewport_size.0,
            self.viewport_size.1,
        )
    }
}

impl Default for PositionTranslator {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared position translator that can be used across components
#[derive(Clone)]
pub struct SharedPositionTranslator {
    inner: Rc<RefCell<PositionTranslator>>,
}

impl SharedPositionTranslator {
    /// Create a new shared position translator
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(PositionTranslator::new())),
        }
    }
    
    /// Create a new shared position translator with custom viewport size
    pub fn with_viewport_size(width: f64, height: f64) -> Self {
        Self {
            inner: Rc::new(RefCell::new(PositionTranslator::with_viewport_size(width, height))),
        }
    }
    
    /// Convert document coordinates to screen coordinates
    pub fn document_to_screen(&self, line: usize, col: usize) -> (f64, f64) {
        self.inner.borrow_mut().document_to_screen(line, col)
    }
    
    /// Convert screen coordinates to document position
    pub fn screen_to_doc(&self, x: f64, y: f64) -> (usize, usize) {
        self.inner.borrow().screen_to_doc(x, y)
    }
    
    /// Update scroll offset
    pub fn set_scroll_offset(&self, x: f64, y: f64) {
        self.inner.borrow_mut().set_scroll_offset(x, y);
    }
    
    /// Update viewport size
    pub fn set_viewport_size(&self, width: f64, height: f64) {
        self.inner.borrow_mut().set_viewport_size(width, height);
    }
    
    /// Update font metrics for a character
    pub fn update_font_metrics(&self, font_id: FontId, ch: char, metrics: CharMetrics) {
        self.inner.borrow_mut().update_font_metrics(font_id, ch, metrics);
    }
    
    /// Update line metrics
    pub fn update_line_metrics(&self, line: usize, metrics: LineMetrics) {
        self.inner.borrow_mut().update_line_metrics(line, metrics);
    }
    
    /// Clear measurement cache
    pub fn invalidate_cache(&self) {
        self.inner.borrow_mut().invalidate_cache();
    }
    
    /// Get the current viewport
    pub fn viewport(&self) -> Rect {
        self.inner.borrow().viewport()
    }
}

impl Default for SharedPositionTranslator {
    fn default() -> Self {
        Self::new()
    }
}