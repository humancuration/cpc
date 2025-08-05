# Real-time Collaborative Cursors Implementation Plan

This document outlines the implementation strategy for real-time collaborative cursors in our document editor, building on existing presence indicators infrastructure.

## 1. Coordinate Translation System

### Design Requirements:
- Handle variable font metrics (size, family, weight)
- Account for mixed-content layouts (text, images, tables)
- Support wrapped lines and complex formatting
- Implement efficient caching

### API Design:
```rust
pub struct CursorPositionTranslator {
    // Cached measurements
    line_metrics: HashMap<usize, LineMetrics>,
    char_metrics: HashMap<(FontId, char), CharMetrics>,
    scroll_offset: (f64, f64),
    viewport_size: (f64, f64),
    // Cache for document-to-screen mappings
    position_cache: LruCache<(usize, usize), (f64, f64)>,
}

pub struct LineMetrics {
    top: f64,
    height: f64,
    wrapped_ranges: Vec<(usize, usize)>,
}

pub struct CharMetrics {
    width: f64,
    kerning: f64,
}

impl CursorPositionTranslator {
    /// Update font metrics for a character
    pub fn update_font_metrics(&mut self, font_id: FontId, char: char, metrics: CharMetrics);
    
    /// Update line metrics
    pub fn update_line_metrics(&mut self, line: usize, metrics: LineMetrics);
    
    /// Convert document position to screen coordinates
    pub fn doc_to_screen(&self, line: usize, col: usize) -> (f64, f64);
    
    /// Convert screen coordinates to document position
    pub fn screen_to_doc(&self, x: f64, y: f64) -> (usize, usize);
    
    /// Clear measurement cache
    pub fn invalidate_cache(&mut self);
}
```

### Implementation Steps:
1. **Measurement Hooks**:
   - Create `useFontMetrics` hook to track font dimensions
   - Implement `useLineMetrics` hook to measure line heights and wrapping
   - Add resize observers for viewport changes

2. **Caching Layer**:
   - Implement LRU cache for position translations
   - Cache key: `(line, col, font_hash, line_height_hash)`
   - Cache invalidation on:
     - Font/style changes
     - Window resize
     - Content reflow

3. **Coordinate Transformation**:
   - Add support for:
     - Bidirectional text
     - RTL languages
     - Variable-width fonts
     - Line wrapping
   - Implement efficient screen-to-doc lookup using spatial indexing

## 2. Viewport Synchronization

### Design Requirements:
- Detect and share viewport changes
- Efficient delta compression
- Throttling mechanism
- Graceful handling of scroll jitter

### Protocol Extension:
```rust
pub enum SignalingMessage {
    // Existing messages...
    ViewportUpdate(ViewportUpdate),
}

pub struct ViewportUpdate {
    pub user_id: Uuid,
    pub document_id: Uuid,
    pub viewport: Rect,
    pub resolution: f64, // LOD resolution for cursors
}
```

### Implementation:
1. **Viewport Tracking**:
   - Add scroll/resize listeners to editor
   - Calculate visible document region
   - Detect meaningful viewport changes (>5% change)

2. **Delta Compression**:
   - Compare current viewport with previous
   - Only send changes when:
     - Visible region changes > 10%
     - Resolution level changes
   - Use run-length encoding for similar viewports

3. **Throttling**:
   - 250ms minimum between updates
   - Priority queue for urgent updates
   - Smart batching with `PresenceUpdateBatcher`

## 3. Cursor Synchronization

### Design Requirements:
- Smooth cursor movement interpolation
- Position prediction for latency compensation
- Visual indicators for connection quality
- Customizable cursor styles

### Protocol Extension:
```rust
pub struct CursorUpdate {
    pub position: (usize, usize),
    pub velocity: (f64, f64), // Pixels/sec (for prediction)
    pub timestamp: DateTime<Utc>,
    pub style: CursorStyle,
}

pub enum CursorStyle {
    Default,
    Highlighted,
    Writing,
    Selecting,
}
```

### Implementation:
1. **Position Interpolation**:
   - Store last two cursor positions per user
   - Use linear interpolation between updates
   - Apply easing functions for smooth transitions

2. **Prediction System**:
   - Calculate movement velocity vector
   - Extrapolate position during network latency
   - Snap to actual position when update arrives

3. **Visual Feedback**:
   - Color-coded latency indicators
   - Animation states for typing/selecting
   - Configurable cursor styles via user preferences

## 4. Performance Optimization

### Strategies:
1. **Occlusion Culling**:
   - Skip rendering cursors outside viewport
   - Implement spatial partitioning (QuadTree)
   - Batch render calls by cursor style

2. **Level of Detail (LOD)**:
   - High detail: Within 100px of focus
   - Medium detail: Visible viewport
   - Low detail: Outside viewport (minimal indicator)
   - Adjust update frequency by LOD level

3. **Rendering Optimizations**:
   - GPU-accelerated cursor rendering
   - Shared texture atlas for cursor images
   - Instanced rendering for identical cursors

4. **Network Optimization**:
   - Differential updates (send only changed fields)
   - 10Hz update cap (configurable)
   - Priority channels for active users

### Benchmarks:
- Target: 1000 concurrent cursors @ 60fps
- Metrics:
  - CPU usage < 15%
  - Memory < 50MB for cursor system
  - Network < 50KB/s at peak

## 5. Testing Plan

### Unit Tests:
1. **Coordinate Translation**:
   - Fixed vs variable font sizes
   - Wrapped line scenarios
   - Mixed content layouts
   - Cache hit/miss performance

2. **Viewport Synchronization**:
   - Delta compression accuracy
   - Throttling behavior
   - Error handling for invalid regions

3. **Cursor Prediction**:
   - Position extrapolation accuracy
   - Interpolation smoothness
   - Error correction mechanisms

### Integration Tests:
1. **Multi-user Scenarios**:
   - 2 users editing same document
   - 10 users with varying latency
   - 100 users (stress test)

2. **Failure Cases**:
   - Network dropout recovery
   - Clock skew handling
   - High-latency environments (300ms+)

### Performance Tests:
1. **Cursors Scaling**:
   - Measure FPS vs cursor count
   - Network bandwidth usage
   - Memory consumption

2. **Stress Tests**:
   - Rapid viewport changes
   - High-frequency cursor updates
   - Mixed content complexity

## Dependencies

1. Enhanced font metrics service
2. Spatial indexing library
3. Updated signaling protocol (v2)
4. GPU rendering pipeline

## Timeline

1. Week 1: Coordinate system & viewport sync
2. Week 2: Cursor interpolation & prediction
3. Week 3: Performance optimizations
4. Week 4: Testing & refinement